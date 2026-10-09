use mado_delete_confirmation::{Choice, Confirmation, Kind, Request, RequestId};
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(if cfg!(windows) {
        r"C:\generated"
    } else {
        "/generated"
    });
    let request = Request::new(
        RequestId(1),
        Kind::Trash,
        vec![root.join("example.txt")],
        root,
    );
    let mut confirmation = Confirmation::new();
    confirmation.present(request.clone()).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..3 {
        ctx.run_ui(egui::RawInput::default(), |ui| {
            assert!(confirmation.show(ui.ctx()).answer.is_none());
        })
        .drop_without_applying_deltas();
    }
    let answer = confirmation.choose(request.id(), Choice::Cancel).unwrap();
    assert_eq!(answer.request(), &request);
    assert!(answer.deletion_intent().is_none());
    println!("Generated confirmation cancelled. No filesystem access or deletion intent.");
}
