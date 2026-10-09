use mado_pdf_controls::{DocumentKey, DocumentState, PdfControls};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(if cfg!(windows) {
        r"C:\generated\example.pdf"
    } else {
        "/generated/example.pdf"
    });
    let key = DocumentKey::new(path, 1).unwrap();
    let mut controls = PdfControls::new(egui::Id::new("generated-pane"));
    controls.select(Some(key.clone())).unwrap();
    controls
        .supply_metadata(&key, 1, DocumentState::Available { page_count: 3 })
        .unwrap();
    let ctx = egui::Context::default();
    let mut request = None;
    let output = ctx.run_ui(egui::RawInput::default(), |ui| {
        if let Some(current) = controls.show(ui).request {
            assert!(request.is_none());
            request = Some(current);
        }
    });
    let request = request.unwrap();
    assert_eq!(request.page_index(), 0);
    assert_eq!(controls.confirmed_page(), None);
    println!(
        "Generated PDF controls: {} shapes, supplied 3 pages, requested index {}. No PDF read or rendered.",
        output.shapes.len(),
        request.page_index()
    );
    output.drop_without_applying_deltas();
}
