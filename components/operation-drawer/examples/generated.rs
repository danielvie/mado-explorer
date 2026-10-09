//! Generated headless request/report demonstration. No desktop or filesystem I/O.
use mado_operation_drawer::*;
use std::path::PathBuf;

fn main() {
    let ctx = egui::Context::default();
    let mut operations = Operations::new(1);
    for sequence in 1..=3 {
        let request = Request::new(
            OperationId {
                window: 1,
                sequence,
            },
            Kind::Copy,
            vec![PathBuf::from(format!(r"C:\generated\source-{sequence}"))],
            Some(PathBuf::from(r"D:\generated\fixed-destination")),
        )
        .unwrap();
        operations.enqueue(&request).unwrap();
    }
    let Some(Command::Start(request)) = operations.next_command() else {
        panic!("missing request");
    };
    operations
        .report(&Report {
            id: request.id(),
            revision: 1,
            update: Update::Running(Status {
                current_item: Some(request.sources()[0].clone()),
                progress: Progress::Known {
                    processed: 1,
                    total: 4,
                },
                counts: Counts {
                    completed: 1,
                    skipped: 0,
                    failed: 0,
                },
            }),
        })
        .unwrap();
    let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
        operations.show(ui);
    });
    output.textures_delta.clear();
    assert_eq!(operations.queued().len(), 2);
    operations.cancel(request.id());
    assert_eq!(
        operations.next_command(),
        Some(Command::Cancel(request.id()))
    );
    assert!(operations.next_command().is_none());
    operations
        .report(&Report {
            id: request.id(),
            revision: 2,
            update: Update::Stopped {
                status: Status {
                    counts: Counts {
                        completed: 1,
                        skipped: 0,
                        failed: 0,
                    },
                    ..Default::default()
                },
                outcome: Outcome::Cancelled,
                issues: vec![],
            },
        })
        .unwrap();
    assert_eq!(operations.history()[0].status().counts.completed, 1);
    println!(
        "Generated drawer: {} shapes; one active/two queued; cancellation waits for terminal acknowledgement. No executor or file changes.",
        output.shapes.len()
    );
}
