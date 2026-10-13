//! Bounded latest-request preview work. Workers never hold UI references.
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Condvar, Mutex},
    thread,
};
pub const TEXT_LIMIT: usize = 1024 * 1024;
#[derive(Debug)]
pub enum PreviewResult {
    Text(String),
    /// RGBA bytes with premultiplied alpha. Use egui::ColorImage::from_rgba_premultiplied.
    Image {
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    },
    Error(String),
}
#[derive(Default)]
struct State {
    generation: u64,
    pending: Option<(u64, PathBuf)>,
    completed: Option<(u64, PreviewResult)>,
    stop: bool,
}
pub struct Preview {
    shared: Arc<(Mutex<State>, Condvar)>,
    result: Option<PreviewResult>,
}
impl Default for Preview {
    fn default() -> Self {
        Self::new()
    }
}
impl Preview {
    pub fn new() -> Self {
        let shared = Arc::new((Mutex::new(State::default()), Condvar::new()));
        let worker = shared.clone();
        thread::spawn(move || {
            loop {
                let (lock, wake) = &*worker;
                let request = {
                    let mut state = lock.lock().unwrap();
                    while state.pending.is_none() && !state.stop {
                        state = wake.wait(state).unwrap();
                    }
                    if state.stop {
                        return;
                    }
                    state.pending.take().unwrap()
                };
                let result = load(&request.1, || {
                    let state = lock.lock().unwrap();
                    state.stop || state.generation != request.0
                });
                let mut state = lock.lock().unwrap();
                if !state.stop && state.generation == request.0 {
                    state.completed = Some((request.0, result));
                }
            }
        });
        Self {
            shared,
            result: None,
        }
    }
    pub fn request(&mut self, path: PathBuf) {
        let (lock, wake) = &*self.shared;
        let mut state = lock.lock().unwrap();
        state.generation = state.generation.wrapping_add(1);
        state.pending = Some((state.generation, path));
        state.completed = None;
        self.result = None;
        wake.notify_one();
    }
    pub fn poll(&mut self) -> bool {
        let mut state = self.shared.0.lock().unwrap();
        if let Some((generation, result)) = state.completed.take()
            && generation == state.generation
        {
            self.result = Some(result);
            return true;
        }
        false
    }
    pub fn result(&self) -> Option<&PreviewResult> {
        self.result.as_ref()
    }
    pub fn clear(&mut self) {
        let mut state = self.shared.0.lock().unwrap();
        state.generation = state.generation.wrapping_add(1);
        state.pending = None;
        state.completed = None;
        self.result = None;
    }
}
impl Drop for Preview {
    fn drop(&mut self) {
        let mut state = self.shared.0.lock().unwrap();
        state.stop = true;
        state.pending = None;
        state.completed = None;
        self.shared.1.notify_one();
        // Do not block the UI waiting for a filesystem or external codec call.
    }
}
fn load(path: &Path, cancelled: impl Fn() -> bool) -> PreviewResult {
    if cancelled() {
        return PreviewResult::Error("Preview cancelled".into());
    }
    let extension = path
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(
        extension.as_str(),
        "png" | "jpg" | "jpeg" | "bmp" | "gif" | "tif" | "tiff" | "ico"
    ) {
        return match crate::platform::image::decode(path) {
            Ok(image) => PreviewResult::Image {
                width: image.width,
                height: image.height,
                pixels: image.pixels,
            },
            Err(error) => PreviewResult::Error(error),
        };
    }
    match File::open(path)
        .map_err(|e| e.to_string())
        .and_then(|file| read_text_cancellable(file, cancelled))
    {
        Ok(text) => PreviewResult::Text(text),
        Err(error) => PreviewResult::Error(error),
    }
}
#[cfg(test)]
fn read_text(reader: impl Read) -> Result<String, String> {
    read_text_cancellable(reader, || false)
}
fn read_text_cancellable(
    mut reader: impl Read,
    cancelled: impl Fn() -> bool,
) -> Result<String, String> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 8192];
    while bytes.len() <= TEXT_LIMIT {
        if cancelled() {
            return Err("Preview cancelled".into());
        }
        let remaining = (TEXT_LIMIT + 1 - bytes.len()).min(chunk.len());
        let read = match reader.read(&mut chunk[..remaining]) {
            Ok(read) => read,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error.to_string()),
        };
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
    if bytes.len() > TEXT_LIMIT {
        return Err("Text preview exceeds 1 MiB".into());
    }
    if bytes
        .iter()
        .any(|&v| v < 32 && !matches!(v, b'\t' | b'\r' | b'\n'))
    {
        return Err("Binary data cannot be previewed as text".into());
    }
    String::from_utf8(bytes)
        .map_err(|_| "Unsupported encoding: preview requires valid UTF-8".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_preview_limits_and_encoding() {
        assert_eq!(read_text("hello λ\n".as_bytes()).unwrap(), "hello λ\n");
        assert!(read_text(&b"a\0b"[..]).is_err());
        assert!(read_text(&[0xff][..]).is_err());
        assert_eq!(
            read_text(vec![b'a'; TEXT_LIMIT].as_slice()).unwrap().len(),
            TEXT_LIMIT
        );
        assert!(read_text(vec![b'a'; TEXT_LIMIT + 1].as_slice()).is_err());
    }
    #[test]
    fn text_preview_stops_reading_at_bound() {
        let mut reader = std::io::Cursor::new(vec![b'a'; TEXT_LIMIT * 2]);
        assert!(read_text(&mut reader).is_err());
        assert_eq!(reader.position(), (TEXT_LIMIT + 1) as u64);
    }
    #[test]
    fn text_preview_clear_invalidates_results() {
        let mut preview = Preview::new();
        {
            let mut state = preview.shared.0.lock().unwrap();
            state.completed = Some((state.generation, PreviewResult::Text("old".into())));
        }
        preview.clear();
        assert!(!preview.poll());
        assert!(preview.result().is_none());
    }
    #[test]
    fn text_preview_cancels_between_bounded_reads() {
        let checks = std::cell::Cell::new(0);
        let mut reader = std::io::Cursor::new(vec![b'a'; TEXT_LIMIT]);
        let result = read_text_cancellable(&mut reader, || {
            checks.set(checks.get() + 1);
            checks.get() > 1
        });
        assert_eq!(result.unwrap_err(), "Preview cancelled");
        assert_eq!(reader.position(), 8192);
    }
}
