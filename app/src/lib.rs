pub mod platform;

use eframe::egui;
use platform::windows::{NativeHost, OleGuard};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

pub const APP_NAME: &str = "Wayfarer Files";
type AppResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> AppResult {
    run_window(None)
}

fn run_window(smoke_frames: Option<Arc<AtomicUsize>>) -> AppResult {
    let _ole = OleGuard::initialize()?;
    let native = Rc::new(RefCell::new(None::<NativeHost>));
    let owner = Rc::clone(&native);
    eframe::run_native(
        APP_NAME,
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1000.0, 680.0])
                .with_min_inner_size([480.0, 320.0])
                .with_drag_and_drop(false),
            ..Default::default()
        },
        Box::new(move |cc| {
            let host = NativeHost::attach(cc)?;
            host.probe()?;
            *owner.borrow_mut() = Some(host);
            Ok(Box::new(Window {
                smoke_frames,
                native: owner,
                probe_status: String::from("Native callback round trip passed"),
                input: String::new(),
            }))
        }),
    )?;
    let mut native = native.borrow_mut();
    let host = native.as_mut().ok_or("Native host was never created")?;
    {
        if host.is_attached() {
            host.detach()?;
            return Err("Native callback was not detached before shutdown".into());
        }
    }
    Ok(())
}

struct Window {
    smoke_frames: Option<Arc<AtomicUsize>>,
    native: Rc<RefCell<Option<NativeHost>>>,
    probe_status: String,
    input: String,
}

impl eframe::App for Window {
    fn on_exit(&mut self) {
        if let Some(host) = self.native.borrow_mut().as_mut()
            && let Err(error) = host.detach()
        {
            eprintln!("Native teardown failed: {error}");
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::central_panel(ui.style()).show(ui, |ui| {
            ui.heading(APP_NAME);
            ui.label("Independent Windows file manager");
            ui.separator();
            ui.label("Native foundation probe — browsing is not implemented yet.");
            ui.label("Keyboard and IME probe");
            ui.text_edit_singleline(&mut self.input)
                .labelled_by(ui.label("Input").id);
            if ui.button("Probe native callback").clicked() {
                self.probe_status = match self.native.borrow().as_ref().unwrap().probe() {
                    Ok(count) => format!("Native callback round trips: {count}"),
                    Err(error) => format!("Native callback failed: {error}"),
                };
            }
            ui.label(&self.probe_status);
        });
        if let Some(frames) = &self.smoke_frames {
            if frames.fetch_add(1, Ordering::Relaxed) >= 2 {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            ui.ctx().request_repaint();
        }
    }
}

/// Opt-in native lifecycle probe, launched on the executable's main thread.
pub fn window_boot_probe() -> AppResult {
    let frames = Arc::new(AtomicUsize::new(0));
    run_window(Some(Arc::clone(&frames)))?;
    if frames.load(Ordering::Relaxed) < 3 {
        return Err("Window closed before three UI frames were observed".into());
    }
    println!(
        "window_boot: {} UI frames; HWND callback and pre-destruction detach passed",
        frames.load(Ordering::Relaxed)
    );
    Ok(())
}
