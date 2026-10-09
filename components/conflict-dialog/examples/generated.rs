use mado_conflict_dialog::{
    Choice, ConflictDialog, ConflictId, ConflictRequest, ConflictType, EntryKind, OperationId,
};
use std::path::PathBuf;

fn main() {
    let operation = OperationId(1);
    let mut dialog = ConflictDialog::new(operation);
    let request = ConflictRequest::new(
        operation,
        ConflictId(1),
        PathBuf::from(if cfg!(windows) {
            r"C:\generated\source\example.txt"
        } else {
            "/generated/source/example.txt"
        }),
        PathBuf::from(if cfg!(windows) {
            r"C:\generated\destination\example.txt"
        } else {
            "/generated/destination/example.txt"
        }),
        ConflictType {
            source: EntryKind::RegularFile,
            destination: EntryKind::RegularFile,
        },
    )
    .unwrap();
    dialog.present(request).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..2 {
        ctx.run_ui(egui::RawInput::default(), |ui| {
            assert!(dialog.show(ui.ctx()).decision.is_none());
        })
        .drop_without_applying_deltas();
    }
    let input = egui::RawInput {
        events: vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..Default::default()
    };
    let mut decision = None;
    let output = ctx.run_ui(input, |ui| {
        if let Some(current) = dialog.show(ui.ctx()).decision {
            decision = Some(current);
        }
    });
    let decision = decision.unwrap();
    assert_eq!(decision.choice(), Choice::Skip);
    println!(
        "Generated egui dialog: {} shapes, operation {}, conflict {}, choice {:?}. No files read or changed.",
        output.shapes.len(),
        decision.request().operation().0,
        decision.request().conflict().0,
        decision.choice()
    );
    output.drop_without_applying_deltas();
}
