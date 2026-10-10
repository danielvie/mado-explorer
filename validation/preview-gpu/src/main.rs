use mado_pdf_preview_service::{CpuBudget, CpuBuffer, CpuReservation};
use mado_preview_gpu_probe::{Layout, Ledger, Payload, upload_ready};
use std::{
    future::Future,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    task::{Context, Poll, Wake, Waker},
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, String>;

// Native setup/scope futures only. No UI executor or thread per request.
fn wait_future<T>(future: impl Future<Output = T>) -> Result<T> {
    struct WakeThread(thread::Thread);
    impl Wake for WakeThread {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
    }
    let waker = Waker::from(Arc::new(WakeThread(thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Poll::Ready(result) = future.as_mut().poll(&mut cx) {
            return Ok(result);
        }
        if Instant::now() >= deadline {
            return Err("native future timeout; outcome unknown".into());
        }
        thread::park_timeout(Duration::from_millis(20));
    }
}

struct Scopes([wgpu::ErrorScopeGuard; 3]);
impl Scopes {
    fn new(device: &wgpu::Device) -> Self {
        Self([
            device.push_error_scope(wgpu::ErrorFilter::OutOfMemory),
            device.push_error_scope(wgpu::ErrorFilter::Internal),
            device.push_error_scope(wgpu::ErrorFilter::Validation),
        ])
    }
    fn pop(self, label: &str) -> Result<bool> {
        let [oom, internal, validation] = self.0;
        // Pop on this submitting thread immediately, in reverse order.
        let v = validation.pop();
        let i = internal.pop();
        let o = oom.pop();
        let mut clean = true;
        for (name, error) in [
            ("Validation", wait_future(v)?),
            ("Internal", wait_future(i)?),
            ("OutOfMemory", wait_future(o)?),
        ] {
            if let Some(error) = error {
                eprintln!("SCOPE {label} {name}: {error:#?}");
                clean = false;
            }
        }
        println!("SCOPES {label} clean={clean}");
        Ok(clean)
    }
}

struct ChargedBuffer {
    buffer: wgpu::Buffer,
    _charge: CpuReservation,
}
impl Drop for ChargedBuffer {
    fn drop(&mut self) {
        self.buffer.destroy();
    }
}
struct ChargedTexture {
    texture: wgpu::Texture,
    _charge: Payload,
}
impl Drop for ChargedTexture {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}

// All app-owned resources survive a failed/panicked poll in this outer owner.
// ponytail: one fixed, bounded probe batch, not a production request/cache bridge.
struct Probe {
    device: wgpu::Device,
    queue: wgpu::Queue,
    cpu: CpuBudget,
    gpu: Ledger,
    source: Option<CpuBuffer>,
    buffers: Vec<ChargedBuffer>,
    textures: Vec<Option<ChargedTexture>>,
    renderer: Option<egui_wgpu::Renderer>,
    uncaptured: Arc<AtomicBool>,
    _control: CpuReservation,
}
impl Probe {
    fn buffer(&mut self, size: usize, usage: wgpu::BufferUsages) -> Result<usize> {
        if self.buffers.len() == 8 {
            return Err("fixed buffer bound".into());
        }
        let charge = self.cpu.reserve_capacity(size).map_err(|e| e.to_string())?;
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("p23-owned-buffer"),
            size: size as u64,
            usage,
            mapped_at_creation: false,
        });
        self.buffers.push(ChargedBuffer {
            buffer,
            _charge: charge,
        });
        Ok(self.buffers.len() - 1)
    }
    fn texture(&mut self, width: u32, height: u32, usage: wgpu::TextureUsages) -> Result<usize> {
        if self.textures.len() == 4 {
            return Err("fixed texture bound".into());
        }
        let layout = Layout::checked(width, height, self.device.limits().max_texture_dimension_2d)
            .ok_or("texture size refused before allocation")?;
        let charge = self
            .gpu
            .reserve(layout.source)
            .ok_or("GPU payload refused before allocation")?;
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("p23-owned-texture"),
            size: extent(layout),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage,
            view_formats: &[],
        });
        self.textures.push(Some(ChargedTexture {
            texture,
            _charge: charge,
        }));
        Ok(self.textures.len() - 1)
    }
    fn tex(&self, index: usize) -> &wgpu::Texture {
        &self.textures[index].as_ref().unwrap().texture
    }
    fn encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("p23-owned-encoder"),
            })
    }
    fn wait(&self, submission: Option<wgpu::SubmissionIndex>) -> Result<()> {
        // A timeout or a native panic is NOT a release or acceptance certificate.
        self.device
            .poll(wgpu::PollType::Wait {
                submission_index: submission,
                timeout: Some(Duration::from_secs(10)),
            })
            .map_err(|e| format!("Device::poll failed: {e:?}; outcome unknown"))?;
        if self.uncaptured.load(Ordering::Acquire) {
            return Err("uncaptured GPU error".into());
        }
        Ok(())
    }
    fn map(&self, index: usize, mode: wgpu::MapMode) -> Result<()> {
        let (send, receive) = mpsc::sync_channel(1);
        self.buffers[index]
            .buffer
            .map_async(mode, .., move |result| {
                let _ = send.try_send(result);
            });
        self.wait(None)?;
        receive
            .try_recv()
            .map_err(|e| format!("map callback not delivered: {e}"))?
            .map_err(|e| format!("map failed: {e:?}"))
    }
}
fn extent(l: Layout) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width: l.width,
        height: l.height,
        depth_or_array_layers: 1,
    }
}
fn copy_info(texture: &wgpu::Texture) -> wgpu::TexelCopyTextureInfo<'_> {
    wgpu::TexelCopyTextureInfo {
        texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    }
}
fn copy_buffer(buffer: &wgpu::Buffer, l: Layout, offset: u64) -> wgpu::TexelCopyBufferInfo<'_> {
    wgpu::TexelCopyBufferInfo {
        buffer,
        layout: wgpu::TexelCopyBufferLayout {
            offset,
            bytes_per_row: Some(l.stride as u32),
            rows_per_image: Some(l.height),
        },
    }
}
fn completion(encoder: &wgpu::CommandEncoder) -> Arc<AtomicBool> {
    let done = Arc::new(AtomicBool::new(false));
    let callback = done.clone();
    encoder.on_submitted_work_done(move || callback.store(true, Ordering::Release));
    done
}
fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn rgba(pixel: usize) -> [u8; 4] {
    [
        ((pixel * 37 + 11) % 256) as u8,
        ((pixel * 53 + 29) % 256) as u8,
        ((pixel * 19 + 83) % 256) as u8,
        [0, 1, 127, 128, 254, 255][pixel % 6],
    ]
}
fn scalar(pixel: [u8; 4]) -> [u8; 4] {
    epaint::Color32::from_rgba_unmultiplied(pixel[0], pixel[1], pixel[2], pixel[3]).to_array()
}

fn upload(p: &mut Probe) -> Result<(usize, epaint::TextureId)> {
    println!("CASE actual-mapped-upload 7x3 alpha=0,1,127,128,254,255");
    let scopes = Scopes::new(&p.device);
    let l = Layout::checked(7, 3, p.device.limits().max_texture_dimension_2d).unwrap();
    let mut source = p.cpu.allocate(l.source).map_err(|e| e.to_string())?;
    for (index, pixel) in source.bytes_mut().chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&rgba(index));
    }
    p.source = Some(source);
    let staging = p.buffer(
        l.padded,
        wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
    )?;
    let readback = p.buffer(
        l.padded * 2,
        wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
    )?;
    let texture = p.texture(
        l.width,
        l.height,
        wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
    )?;
    p.map(staging, wgpu::MapMode::Write)?;
    {
        let mut mapped = p.buffers[staging]
            .buffer
            .get_mapped_range_mut(..)
            .map_err(|e| e.to_string())?;
        mapped.slice(..).fill(0xA5);
        for y in 0..l.height as usize {
            for x in 0..l.width as usize {
                let offset = y * l.row + x * 4;
                let source = &p.source.as_ref().unwrap().bytes()[offset..offset + 4];
                mapped
                    .slice(y * l.stride + x * 4..y * l.stride + x * 4 + 4)
                    .copy_from_slice(&scalar(source.try_into().unwrap()));
            }
        }
    }
    p.buffers[staging].buffer.unmap();
    let mut encoder = p.encoder();
    encoder.copy_buffer_to_texture(
        copy_buffer(&p.buffers[staging].buffer, l, 0),
        copy_info(p.tex(texture)),
        extent(l),
    );
    // Mirror actual mapped staging including padding, not an uncharged CPU copy.
    encoder.copy_buffer_to_buffer(
        &p.buffers[staging].buffer,
        0,
        &p.buffers[readback].buffer,
        0,
        l.padded as u64,
    );
    encoder.copy_texture_to_buffer(
        copy_info(p.tex(texture)),
        copy_buffer(&p.buffers[readback].buffer, l, l.padded as u64),
        extent(l),
    );
    let done = completion(&encoder);
    let submission = p.queue.submit([encoder.finish()]);
    let clean = scopes.pop("upload-create-map-encode-finish-submit")?;
    require(
        !upload_ready(None, clean, done.load(Ordering::Acquire), 1, 1, true),
        "Ready without ID",
    )?;
    p.wait(Some(submission))?;
    require(done.load(Ordering::Acquire), "upload fence not fired")?;
    require(clean, "upload scopes rejected")?;
    p.map(readback, wgpu::MapMode::Read)?;
    {
        let bytes = p.buffers[readback]
            .buffer
            .get_mapped_range(..)
            .map_err(|e| e.to_string())?;
        for y in 0..l.height as usize {
            for x in 0..l.width as usize {
                let expected = scalar(rgba(y * l.width as usize + x));
                let offset = y * l.stride + x * 4;
                require(
                    bytes[offset..offset + 4] == expected,
                    "mapped staging scalar mismatch",
                )?;
                require(
                    bytes[l.padded + offset..l.padded + offset + 4] == expected,
                    "GPU upload/readback scalar mismatch",
                )?;
            }
            require(
                bytes[y * l.stride + l.row..(y + 1) * l.stride]
                    .iter()
                    .all(|&b| b == 0xA5),
                "mapped staging row padding corrupted",
            )?;
            require(
                bytes[l.padded + y * l.stride + l.row..l.padded + (y + 1) * l.stride]
                    .iter()
                    .all(|&b| b == 0),
                "texture copy touched readback padding",
            )?;
        }
    }
    p.buffers[readback].buffer.unmap();
    let scopes = Scopes::new(&p.device);
    p.renderer = Some(egui_wgpu::Renderer::new(
        &p.device,
        wgpu::TextureFormat::Rgba8Unorm,
        egui_wgpu::RendererOptions::PREDICTABLE,
    ));
    let view = p
        .tex(texture)
        .create_view(&wgpu::TextureViewDescriptor::default());
    let id = p.renderer.as_mut().unwrap().register_native_texture(
        &p.device,
        &view,
        wgpu::FilterMode::Nearest,
    );
    let registration_clean = scopes.pop("renderer-create-register")?;
    require(
        upload_ready(
            Some(id),
            clean && registration_clean,
            done.load(Ordering::Acquire),
            1,
            1,
            true,
        ),
        "upload not Ready",
    )?;
    println!(
        "PASS upload+scalar+padding+readback Ready id={id:?} source={} staging={} readback={} CPU={} GPU={}",
        l.source,
        l.padded,
        l.padded * 2,
        p.cpu.used(),
        p.gpu.used()
    );
    Ok((texture, id))
}

const CLEAR: [u8; 4] = [17, 29, 43, 255];
fn jobs(id: epaint::TextureId) -> [epaint::ClippedPrimitive; 1] {
    let rect = epaint::emath::Rect::from_min_max(
        epaint::emath::pos2(0.0, 0.0),
        epaint::emath::pos2(16.0, 8.0),
    );
    let uv = epaint::emath::Rect::from_min_max(
        epaint::emath::pos2(0.0, 0.0),
        epaint::emath::pos2(1.0, 1.0),
    );
    let mut mesh = epaint::Mesh::with_texture(id);
    mesh.add_rect_with_uv(rect, uv, epaint::Color32::WHITE);
    [epaint::ClippedPrimitive {
        clip_rect: rect,
        primitive: epaint::Primitive::Mesh(mesh),
    }]
}
fn render(
    p: &mut Probe,
    id: epaint::TextureId,
    target: usize,
    readback: usize,
) -> Result<(wgpu::SubmissionIndex, Arc<AtomicBool>)> {
    let l = Layout::checked(16, 8, 8192).unwrap();
    let screen = egui_wgpu::ScreenDescriptor {
        size_in_pixels: [l.width, l.height],
        pixels_per_point: 1.0,
    };
    let jobs = jobs(id);
    let mut encoder = p.encoder();
    let view = p
        .tex(target)
        .create_view(&wgpu::TextureViewDescriptor::default());
    let renderer = p.renderer.as_mut().unwrap();
    let callbacks = renderer.update_buffers(&p.device, &p.queue, &mut encoder, &jobs, &screen);
    require(callbacks.is_empty(), "unexpected callback command buffers")?;
    {
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("p23-normal-egui-mesh"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: CLEAR[0] as f64 / 255.0,
                        g: CLEAR[1] as f64 / 255.0,
                        b: CLEAR[2] as f64 / 255.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        renderer.render(&mut pass.forget_lifetime(), &jobs, &screen);
    }
    encoder.copy_texture_to_buffer(
        copy_info(p.tex(target)),
        copy_buffer(&p.buffers[readback].buffer, l, 0),
        extent(l),
    );
    let done = completion(&encoder);
    Ok((p.queue.submit([encoder.finish()]), done))
}
fn cleared(p: &Probe, readback: usize) -> Result<bool> {
    p.map(readback, wgpu::MapMode::Read)?;
    let l = Layout::checked(16, 8, 8192).unwrap();
    let all_clear = {
        let bytes = p.buffers[readback]
            .buffer
            .get_mapped_range(..)
            .map_err(|e| e.to_string())?;
        (0..l.height as usize).all(|y| {
            bytes[y * l.stride..y * l.stride + l.row]
                .chunks_exact(4)
                .all(|pixel| pixel == CLEAR)
        })
    };
    p.buffers[readback].buffer.unmap();
    Ok(all_clear)
}
fn retirement(p: &mut Probe, texture: usize, id: epaint::TextureId) -> Result<()> {
    println!("CASE pre-encoding-native-ID-withdrawal");
    let scopes = Scopes::new(&p.device);
    let target = p.texture(
        16,
        8,
        wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
    )?;
    let readback = p.buffer(
        2048,
        wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
    )?;
    let (submission, done) = render(p, id, target, readback)?;
    require(
        scopes.pop("positive-normal-render")?,
        "positive renderer rejected",
    )?;
    p.wait(Some(submission))?;
    require(
        done.load(Ordering::Acquire) && !cleared(p, readback)?,
        "positive mesh did not sample",
    )?;

    let scopes = Scopes::new(&p.device);
    let (_previous, previous_done) = render(p, id, target, readback)?;
    // No sleep, poll, future wait, or ordinary logic-tick assumption before withdrawal.
    let pending_observed = !previous_done.load(Ordering::Acquire);
    let before = p.gpu.used();
    p.renderer.as_mut().unwrap().free_texture(&id);
    require(
        p.renderer.as_ref().unwrap().texture(&id).is_none(),
        "old native binding survived",
    )?;
    require(
        p.gpu.used() == before && p.textures[texture].is_some(),
        "early texture refund",
    )?;
    let replacement = p.texture(
        7,
        3,
        wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    )?;
    let view = p
        .tex(replacement)
        .create_view(&wgpu::TextureViewDescriptor::default());
    let next_id = p.renderer.as_mut().unwrap().register_native_texture(
        &p.device,
        &view,
        wgpu::FilterMode::Nearest,
    );
    require(next_id != id, "numeric ID reused")?;
    require(
        p.gpu.used() == before + 84,
        "current+retiring conservation failed",
    )?;
    // Stale normal meshes after unregister cannot retrieve a native binding.
    let (submission, stale_done) = render(p, id, target, readback)?;
    require(
        scopes.pop("previous-submit-withdraw-replacement-stale-render")?,
        "withdraw renderer rejected",
    )?;
    println!(
        "RETAIN pending_callback_observed={pending_observed} retiring=84 current=84 target=512 GPU={} CPU={}",
        p.gpu.used(),
        p.cpu.used()
    );
    p.wait(Some(submission))?;
    require(
        previous_done.load(Ordering::Acquire) && stale_done.load(Ordering::Acquire),
        "retirement queue completion absent",
    )?;
    require(
        cleared(p, readback)?,
        "withdrawn old mesh sampled instead of retaining clear target",
    )?;
    // A COPY_SRC readback after unregister also proves free_texture did not destroy our native texture.
    let l = Layout::checked(7, 3, 8192).unwrap();
    let scopes = Scopes::new(&p.device);
    let mut encoder = p.encoder();
    encoder.copy_texture_to_buffer(
        copy_info(p.tex(texture)),
        copy_buffer(&p.buffers[readback].buffer, l, 0),
        extent(l),
    );
    let done = completion(&encoder);
    let submission = p.queue.submit([encoder.finish()]);
    require(
        scopes.pop("retained-native-texture-readback")?,
        "unregister destroyed app texture",
    )?;
    p.wait(Some(submission))?;
    require(
        done.load(Ordering::Acquire),
        "retained texture fence absent",
    )?;
    p.map(readback, wgpu::MapMode::Read)?;
    {
        let bytes = p.buffers[readback]
            .buffer
            .get_mapped_range(..)
            .map_err(|e| e.to_string())?;
        for y in 0..l.height as usize {
            for x in 0..l.width as usize {
                require(
                    bytes[y * l.stride + x * 4..y * l.stride + x * 4 + 4]
                        == scalar(rgba(y * l.width as usize + x)),
                    "retained texture bytes changed",
                )?;
            }
        }
    }
    p.buffers[readback].buffer.unmap();
    drop(p.textures[texture].take()); // destroy before Payload's refund
    require(
        p.gpu.used() == before,
        "completed retiring texture refund incorrect",
    )?;
    p.renderer.as_mut().unwrap().free_texture(&next_id);
    println!(
        "PASS positive-sampling + missing-old-mesh-no-sampling + retained-readback + completion-before-destroy/refund; old={id:?} next={next_id:?}"
    );
    Ok(())
}

fn discard_and_reject(p: &mut Probe) -> Result<()> {
    println!("CASE direct-owned-encoder-discard-before-submit");
    let scopes = Scopes::new(&p.device);
    let src = p.buffer(16, wgpu::BufferUsages::COPY_SRC)?;
    let dst = p.buffer(16, wgpu::BufferUsages::COPY_DST)?;
    let mut encoder = p.encoder();
    encoder.copy_buffer_to_buffer(&p.buffers[src].buffer, 0, &p.buffers[dst].buffer, 0, 16);
    let discarded = completion(&encoder);
    drop(encoder); // Direct owner knows there was never a CommandBuffer or submit.
    require(scopes.pop("direct-discard")?, "discard rejected")?;
    p.wait(None)?;
    require(
        !discarded.load(Ordering::Acquire),
        "discarded encoder callback fired",
    )?;
    println!("PASS discard known-non-submission, not callback/drop success inference");

    println!("CASE controlled-submit-validation-rejection own16-byte-destroyed-source");
    let scopes = Scopes::new(&p.device);
    let mut encoder = p.encoder();
    encoder.copy_buffer_to_buffer(&p.buffers[src].buffer, 0, &p.buffers[dst].buffer, 0, 16);
    let rejected = completion(&encoder);
    let commands = encoder.finish();
    // Valid encoding, invalidate ONLY our own small source before submit.
    p.buffers[src].buffer.destroy();
    p.queue.submit([commands]);
    let clean = scopes.pop("controlled-rejected-submit")?;
    require(!clean, "controlled rejection unexpectedly accepted")?;
    // Never wait on the rejected submission index. Establish a separate clean drain.
    let scopes = Scopes::new(&p.device);
    let encoder = p.encoder();
    let drain_done = completion(&encoder);
    let drain = p.queue.submit([encoder.finish()]);
    require(
        scopes.pop("post-rejection-clean-drain")?,
        "post-rejection drain failed",
    )?;
    p.wait(Some(drain))?;
    require(
        drain_done.load(Ordering::Acquire),
        "post-rejection drain callback absent",
    )?;
    require(
        !upload_ready(
            Some(epaint::TextureId::User(100)),
            clean,
            rejected.load(Ordering::Acquire),
            1,
            1,
            true,
        ),
        "rejected submission falsely Ready",
    )?;
    println!(
        "PASS rejected_submit clean={clean} actual_deferred_callback={} Ready=false; resources kept charged until separate clean drain",
        rejected.load(Ordering::Acquire)
    );
    Ok(())
}

fn native() -> Result<()> {
    println!("P23 opt-in generated-only offscreen DX12; no window/files/clipboard/decoder/PDFium");
    println!(
        "IDENTITY host={} os={} arch={} PID={}",
        std::env::var("COMPUTERNAME").unwrap_or_default(),
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::process::id()
    );
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = wait_future(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))?
        .map_err(|e| format!("ENV adapter unavailable: {e:#?}"))?;
    println!("ADAPTER {:#?}", adapter.get_info());
    let (device, queue) = wait_future(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("p23-owned-main-device"),
        ..Default::default()
    }))?
    .map_err(|e| format!("ENV request device: {e:#?}"))?;
    let cpu = CpuBudget::new();
    let control = cpu
        .reserve_capacity(1024 * 1024)
        .map_err(|e| e.to_string())?;
    let uncaptured = Arc::new(AtomicBool::new(false));
    let error_flag = uncaptured.clone();
    device.on_uncaptured_error(Arc::new(move |error| {
        eprintln!("UNEXPECTED UNCAPTURED: {error:#?}");
        error_flag.store(true, Ordering::Release);
    }));
    let mut p = Box::new(Probe {
        device,
        queue,
        cpu,
        gpu: Ledger::default(),
        source: None,
        buffers: Vec::with_capacity(8),
        textures: Vec::with_capacity(4),
        renderer: None,
        uncaptured,
        _control: control,
    });
    let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<()> {
        let (texture, id) = upload(&mut p)?;
        retirement(&mut p, texture, id)?;
        discard_and_reject(&mut p)?;
        // Drain a final known-clean fence before freeing any remaining owned resources.
        let scopes = Scopes::new(&p.device);
        let encoder = p.encoder();
        let done = completion(&encoder);
        let submission = p.queue.submit([encoder.finish()]);
        require(scopes.pop("final-drain")?, "final drain rejected")?;
        p.wait(Some(submission))?;
        require(done.load(Ordering::Acquire), "final drain absent")?;
        p.renderer.take();
        p.textures.clear();
        p.buffers.clear();
        p.source.take();
        require(
            p.gpu.used() == 0 && p.cpu.used() == 1024 * 1024,
            "final accounting did not return to control-only",
        )?;
        println!(
            "PASS final-clean-drain preview CPU=0 GPU=0 control=1048576; physical driver heaps NOT capped"
        );
        Ok(())
    }));
    match outcome {
        Ok(Ok(())) => drop(p),
        failed => {
            eprintln!(
                "FAILED main-device outcome={failed:?}; conservatively retained CPU={} GPU={} resources until process teardown; no Ready/refund certificate",
                p.cpu.used(),
                p.gpu.used()
            );
            std::mem::forget(p);
            return Err("main-device native proof failed; resources retained".into());
        }
    }

    println!("CASE explicit-own-separate-device-destroy NOT hardware-loss");
    let (device, queue) = wait_future(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("p23-separate-destroy-device"),
        ..Default::default()
    }))?
    .map_err(|e| format!("separate device setup: {e:#?}"))?;
    let lost = Arc::new(AtomicBool::new(false));
    let flag = lost.clone();
    device.set_device_lost_callback(move |reason, message| {
        println!("DEVICE LOST CALLBACK reason={reason:?} message={message}");
        flag.store(true, Ordering::Release);
    });
    device.destroy();
    let poll = catch_unwind(AssertUnwindSafe(|| device.poll(wgpu::PollType::Poll)));
    println!(
        "DESTROY exact_poll_outcome={poll:?} callback_observed={}",
        lost.load(Ordering::Acquire)
    );
    require(
        !upload_ready(Some(epaint::TextureId::User(0)), true, true, 1, 2, false),
        "destroyed epoch Ready",
    )?;
    drop(queue);
    drop(device);
    println!(
        "PASS explicit-destroy own device; epoch invalidation is PURE injection, not hardware-loss recovery proof"
    );
    println!(
        "GAP actual eframe hidden/multipass/close ordering and asynchronous UI drain NOT exercised; no Root activation authorized"
    );
    Ok(())
}
fn main() {
    if std::env::args().collect::<Vec<_>>()
        != [std::env::args().next().unwrap(), "--native-owned".into()]
    {
        eprintln!(
            "GPU execution requires explicit --native-owned. Use task test for GPU-free checks."
        );
        std::process::exit(2);
    }
    // One fixed worker performs all pixel conversion, mapping, scope and GPU work.
    let worker = thread::Builder::new()
        .name("p23-fixed-native".into())
        .spawn(native);
    match worker {
        Ok(worker) => match worker.join() {
            Ok(Ok(())) => (),
            result => {
                eprintln!("P23 FAILED {result:?}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("worker setup failed: {error}");
            std::process::exit(1);
        }
    }
}
