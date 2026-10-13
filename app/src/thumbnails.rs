//! Visible-item thumbnail work. Buffers and textures are bounded independently.
use crate::platform::image;
use eframe::egui;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
};
const TEXTURE_LIMIT: usize = 32 * 1024 * 1024;
const ENTRY_LIMIT: usize = 256;
static TEXTURE_BYTES: AtomicUsize = AtomicUsize::new(0);
static DECODING: AtomicBool = AtomicBool::new(false);
struct DecodePermit;
impl Drop for DecodePermit {
    fn drop(&mut self) {
        DECODING.store(false, Ordering::Release);
    }
}
const GLOBAL_TEXTURE_LIMIT: usize = 32 * 1024 * 1024;
const EDGE: usize = 192;
struct Request {
    generation: u64,
    path: PathBuf,
}
struct Pixels {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}
struct ResultItem {
    generation: u64,
    path: PathBuf,
    pixels: Result<Pixels, String>,
}
struct Cached {
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    bytes: usize,
}
impl Drop for Cached {
    fn drop(&mut self) {
        TEXTURE_BYTES.fetch_sub(self.bytes, Ordering::Relaxed);
    }
}
pub struct Thumbnails {
    requests: SyncSender<Request>,
    results: Receiver<ResultItem>,
    generation: Arc<AtomicU64>,
    cache: HashMap<PathBuf, Cached>,
    order: VecDeque<PathBuf>,
    pending: HashSet<PathBuf>,
    bytes: usize,
}
impl Thumbnails {
    pub fn new() -> Self {
        let (requests, receive) = mpsc::sync_channel::<Request>(32);
        let (send, results) = mpsc::sync_channel(1);
        let generation = Arc::new(AtomicU64::new(0));
        let worker_generation = generation.clone();
        std::thread::spawn(move || {
            while let Ok(request) = receive.recv() {
                if worker_generation.load(Ordering::Relaxed) != request.generation {
                    continue;
                }
                // Only one full-size thumbnail decode across all tabs; waiting never blocks UI.
                let permit = loop {
                    if worker_generation.load(Ordering::Relaxed) != request.generation {
                        break None;
                    }
                    if DECODING
                        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                        .is_ok()
                    {
                        break Some(DecodePermit);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                };
                let Some(permit) = permit else {
                    continue;
                };
                let pixels = image::decode(&request.path).map(downsample);
                drop(permit);
                if worker_generation.load(Ordering::Relaxed) != request.generation {
                    continue;
                }
                if send
                    .send(ResultItem {
                        generation: request.generation,
                        path: request.path,
                        pixels,
                    })
                    .is_err()
                {
                    break;
                }
            }
        });
        Self {
            requests,
            results,
            generation,
            cache: HashMap::new(),
            order: VecDeque::new(),
            pending: HashSet::new(),
            bytes: 0,
        }
    }
    pub fn synchronize(&mut self, generation: u64, context: &egui::Context) {
        if self.generation.load(Ordering::Relaxed) != generation {
            self.generation.store(generation, Ordering::Relaxed);
            self.cache.clear();
            self.order.clear();
            self.pending.clear();
            self.bytes = 0;
        }
        while let Ok(result) = self.results.try_recv() {
            if result.generation != generation {
                continue;
            }
            self.pending.remove(&result.path);
            let cached = match result.pixels {
                Ok(pixels) => {
                    let bytes = pixels.pixels.len();
                    if TEXTURE_BYTES
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |used| {
                            used.checked_add(bytes)
                                .filter(|total| *total <= GLOBAL_TEXTURE_LIMIT)
                        })
                        .is_err()
                    {
                        Cached { texture: None, error: Some("Thumbnail cache budget reached; refresh after closing other icon tabs".into()), bytes: 0 }
                    } else {
                        let image = egui::ColorImage::from_rgba_premultiplied(
                            [pixels.width, pixels.height],
                            &pixels.pixels,
                        );
                        Cached {
                            texture: Some(context.load_texture(
                                format!("thumbnail:{:?}", result.path),
                                image,
                                egui::TextureOptions::LINEAR,
                            )),
                            error: None,
                            bytes,
                        }
                    }
                }
                Err(error) => Cached {
                    texture: None,
                    error: Some(error),
                    bytes: 0,
                },
            };
            while self.bytes + cached.bytes > TEXTURE_LIMIT || self.cache.len() >= ENTRY_LIMIT {
                if let Some(old) = self.order.pop_front() {
                    if let Some(old) = self.cache.remove(&old) {
                        self.bytes -= old.bytes;
                    }
                } else {
                    break;
                }
            }
            self.bytes += cached.bytes;
            self.order.push_back(result.path.clone());
            self.cache.insert(result.path, cached);
        }
        if !self.pending.is_empty() {
            context.request_repaint_after(std::time::Duration::from_millis(50));
        }
    }
    pub fn request(&mut self, path: &Path, context: &egui::Context) {
        if self.cache.contains_key(path) {
            self.order.retain(|item| item != path);
            self.order.push_back(path.to_owned());
            return;
        }
        if self.pending.contains(path) || self.pending.len() >= 32 || !supported(path) {
            return;
        }
        if self
            .requests
            .try_send(Request {
                generation: self.generation.load(Ordering::Relaxed),
                path: path.to_owned(),
            })
            .is_ok()
        {
            self.pending.insert(path.to_owned());
            context.request_repaint_after(std::time::Duration::from_millis(50));
        }
    }
    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.pending.len(),
            self.cache.len(),
            TEXTURE_BYTES.load(Ordering::Relaxed),
        )
    }
    pub fn texture(&self, path: &Path) -> Option<&egui::TextureHandle> {
        self.cache.get(path)?.texture.as_ref()
    }
    pub fn error(&self, path: &Path) -> Option<&str> {
        self.cache.get(path)?.error.as_deref()
    }
}
impl Drop for Thumbnails {
    fn drop(&mut self) {
        self.generation.store(u64::MAX, Ordering::Relaxed);
    }
}
fn supported(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            [
                "png", "jpg", "jpeg", "bmp", "gif", "tif", "tiff", "ico", "webp", "avif",
            ]
            .iter()
            .any(|known| ext.eq_ignore_ascii_case(known))
        })
}
fn downsample(image: image::DecodedImage) -> Pixels {
    let source_width = image.width as usize;
    let source_height = image.height as usize;
    let longest = source_width.max(source_height);
    let width = if longest > EDGE {
        (source_width * EDGE / longest).max(1)
    } else {
        source_width
    };
    let height = if longest > EDGE {
        (source_height * EDGE / longest).max(1)
    } else {
        source_height
    };
    let mut pixels = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let source =
                ((y * source_height / height) * source_width + x * source_width / width) * 4;
            let target = (y * width + x) * 4;
            pixels[target..target + 4].copy_from_slice(&image.pixels[source..source + 4]);
        }
    }
    Pixels {
        width,
        height,
        pixels,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn thumbnails_downsample_bounds_and_content() {
        let image = image::DecodedImage {
            width: 1000,
            height: 500,
            pixels: vec![42; 1000 * 500 * 4],
        };
        let result = downsample(image);
        assert_eq!((result.width, result.height), (192, 96));
        assert_eq!(result.pixels.len(), 192 * 96 * 4);
        assert!(result.pixels.iter().all(|value| *value == 42));
    }
    #[test]
    fn thumbnails_stale_results_discarded() {
        let mut cache = Thumbnails::new();
        let context = egui::Context::default();
        cache.synchronize(1, &context);
        cache.pending.insert(PathBuf::from("old.png"));
        cache.synchronize(2, &context);
        assert!(cache.pending.is_empty());
        assert_eq!(cache.bytes, 0);
        assert!(!supported(Path::new("document.txt")));
        assert!(supported(Path::new("IMAGE.PNG")));
    }
}
