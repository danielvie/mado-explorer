use egui::{Context, Event, FullOutput, Key, Modifiers, PointerButton, RawInput, Shape};
use mado_operation_drawer::*;
use std::path::PathBuf;

fn id(sequence: u64) -> OperationId {
    OperationId {
        window: 7,
        sequence,
    }
}
fn request(sequence: u64) -> Request {
    Request::new(
        id(sequence),
        Kind::Copy,
        vec![PathBuf::from(format!(r"C:\generated\source-{sequence}"))],
        Some(PathBuf::from(r"D:\fixed-destination")),
    )
    .unwrap()
}
fn status(progress: Progress) -> Status {
    Status {
        current_item: Some(PathBuf::from(r"C:\generated\current.bin")),
        progress,
        counts: Counts {
            completed: 2,
            skipped: 1,
            failed: 3,
        },
    }
}
fn running(sequence: u64, revision: u64, progress: Progress) -> Report {
    Report {
        id: id(sequence),
        revision,
        update: Update::Running(status(progress)),
    }
}
fn stopped(sequence: u64, revision: u64, outcome: Outcome) -> Report {
    Report {
        id: id(sequence),
        revision,
        update: Update::Stopped {
            status: status(Progress::Known {
                processed: 6,
                total: 6,
            }),
            outcome,
            issues: vec![],
        },
    }
}
fn key(key: Key) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}
fn frame(
    ctx: &Context,
    operations: &mut Operations,
    events: Vec<Event>,
) -> (FullOutput, DrawerResponse) {
    let mut response = None;
    let mut output = ctx.run_ui(
        RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(700.0, 800.0),
            )),
            time: Some(ctx.input(|i| i.time) + 0.2),
            events,
            ..Default::default()
        },
        |ui| response = Some(operations.show(ui)),
    );
    output.textures_delta.clear();
    (output, response.unwrap())
}
fn texts(output: &FullOutput) -> Vec<String> {
    fn collect(shape: &Shape, result: &mut Vec<String>) {
        match shape {
            Shape::Text(text) => result.push(text.galley.job.text.clone()),
            Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, result);
                }
            }
            _ => {}
        }
    }
    let mut result = vec![];
    for shape in &output.shapes {
        collect(&shape.shape, &mut result);
    }
    result
}
fn contains(output: &FullOutput, expected: &str) {
    let text = texts(output);
    assert!(
        text.iter().any(|text| text == expected),
        "{expected:?} missing from {text:?}"
    );
}
fn control(response: &DrawerResponse, sequence: u64, action: Action) -> &egui::Response {
    &response
        .controls
        .iter()
        .find(|control| control.id == id(sequence) && control.action == action)
        .unwrap()
        .response
}
fn click(ctx: &Context, operations: &mut Operations, response: &egui::Response) {
    let pos = response.rect.center();
    for pressed in [true, false] {
        frame(
            ctx,
            operations,
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
    }
}

#[test]
fn p09_fifo_two_queued_one_active_and_queued_cancel_emits_no_start() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    for sequence in 1..=3 {
        operations.enqueue(&request(sequence)).unwrap();
    }
    assert_eq!(operations.next_command(), Some(Command::Start(request(1))));
    assert!(operations.next_command().is_none());
    let (output, response) = frame(&ctx, &mut operations, vec![]);
    contains(&output, "Copy #1");
    contains(&output, "Copy #2");
    contains(&output, "Copy #3");
    contains(
        &output,
        "Pending: 3/16 | Retained reports and reserved slots: 3/32",
    );
    assert_eq!(operations.queued().len(), 2);
    // Actual queued control, focused and activated with Enter.
    control(&response, 2, Action::Cancel).request_focus();
    frame(&ctx, &mut operations, vec![key(Key::Enter)]);
    assert_eq!(operations.queued().len(), 1);
    assert_eq!(operations.history()[0].state(), State::CancelledBeforeStart);
    assert!(operations.next_command().is_none());
    operations
        .report(&stopped(1, 1, Outcome::Completed))
        .unwrap();
    assert_eq!(operations.next_command(), Some(Command::Start(request(3))));
    assert!(operations.next_command().is_none());
    assert!(!operations.cancel(id(2)));
}

#[test]
fn p09_pointer_cancel_waits_through_running_reports_and_late_completion_ack() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.enqueue(&request(2)).unwrap();
    operations.next_command();
    operations
        .report(&running(
            1,
            1,
            Progress::Known {
                processed: 3,
                total: 10,
            },
        ))
        .unwrap();
    let (_, response) = frame(&ctx, &mut operations, vec![]);
    click(&ctx, &mut operations, control(&response, 1, Action::Cancel));
    assert_eq!(operations.active().unwrap().state(), State::Cancelling);
    assert_eq!(operations.next_command(), Some(Command::Cancel(id(1))));
    assert!(operations.next_command().is_none());
    operations
        .report(&running(
            1,
            2,
            Progress::Known {
                processed: 4,
                total: 10,
            },
        ))
        .unwrap();
    assert_eq!(operations.active().unwrap().state(), State::Cancelling);
    assert!(operations.next_command().is_none());
    let (output, response) = frame(&ctx, &mut operations, vec![]);
    contains(
        &output,
        "Cancelling: requested, waiting for worker to stop. An OS call may still finish.",
    );
    assert!(
        !response
            .controls
            .iter()
            .any(|c| c.id == id(1) && c.action == Action::Cancel)
    );
    operations
        .report(&stopped(1, 3, Outcome::Completed))
        .unwrap();
    assert_eq!(
        operations.history()[0].state(),
        State::Stopped(Outcome::Completed)
    );
    assert_eq!(operations.history()[0].status().counts.completed, 2);
    contains(
        &frame(&ctx, &mut operations, vec![]).0,
        "Worker finished after cancellation request",
    );
    assert_eq!(operations.next_command(), Some(Command::Start(request(2))));
}

#[test]
fn p09_actual_cancelled_terminal_ack_does_not_claim_rollback() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.next_command();
    assert!(operations.cancel(id(1)));
    operations.next_command();
    operations
        .report(&stopped(1, 1, Outcome::Cancelled))
        .unwrap();
    contains(
        &frame(&ctx, &mut operations, vec![]).0,
        "Worker acknowledged stop; completed work remains",
    );
    assert_eq!(operations.history()[0].status().counts.completed, 2);
}

#[test]
fn p09_frozen_snapshot_outlives_origin_and_later_selection_navigation() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    // The only origin-owned values are input data. No tab/pane lifetime handle.
    {
        let mut selected = vec![PathBuf::from(r"C:\original\résumé")];
        let mut destination = PathBuf::from(r"D:\original-target");
        let snapshot = Request::new(
            id(1),
            Kind::Move,
            selected.clone(),
            Some(destination.clone()),
        )
        .unwrap();
        operations.enqueue(&snapshot).unwrap();
        selected[0] = PathBuf::from(r"C:\later-selection");
        destination.push("later-navigation");
        assert_ne!(snapshot.sources(), selected);
    }
    let expected = operations.queued()[0].request().clone();
    assert_eq!(
        operations.next_command(),
        Some(Command::Start(expected.clone()))
    );
    assert_eq!(
        expected.destination(),
        Some(std::path::Path::new(r"D:\original-target"))
    );
    contains(
        &frame(&ctx, &mut operations, vec![]).0,
        r"Destination: D:\original-target",
    );
    operations
        .report(&stopped(1, 1, Outcome::Completed))
        .unwrap();
    assert_eq!(operations.history()[0].request(), &expected);
}

#[test]
fn p09_stale_foreign_duplicate_queued_and_completed_reports_cannot_retarget_or_restart() {
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.enqueue(&request(2)).unwrap();
    assert_eq!(
        operations.report(&running(1, 1, Progress::Unknown)),
        Err(Refusal::StaleReport)
    );
    operations.next_command();
    operations
        .report(&running(1, 5, Progress::Enumerating))
        .unwrap();
    let original = operations.active().unwrap().status().clone();
    for report in [
        running(1, 5, Progress::Unknown),
        running(1, 4, Progress::Unknown),
        running(2, 99, Progress::Unknown),
        running(99, 99, Progress::Unknown),
    ] {
        assert_eq!(operations.report(&report), Err(Refusal::StaleReport));
        assert_eq!(operations.active().unwrap().status(), &original);
    }
    let foreign = Report {
        id: OperationId {
            window: 8,
            sequence: 1,
        },
        ..running(1, 10, Progress::Unknown)
    };
    assert_eq!(operations.report(&foreign), Err(Refusal::ForeignWindow));
    operations.report(&stopped(1, 6, Outcome::Failed)).unwrap();
    operations.next_command();
    assert_eq!(
        operations.report(&stopped(1, 7, Outcome::Completed)),
        Err(Refusal::StaleReport)
    );
    assert_eq!(operations.active().unwrap().request().id(), id(2));
    assert!(operations.acknowledge(id(1)));
    assert_eq!(operations.enqueue(&request(1)), Err(Refusal::ReusedId));
    assert!(operations.next_command().is_none());
}

#[test]
fn p09_known_unknown_enumeration_conflict_and_invalid_totals_render_honestly() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.next_command();
    let cases = [
        (Progress::Unknown, "Progress unknown"),
        (Progress::Enumerating, "Enumerating; total unknown"),
        (
            Progress::WaitingForConflict,
            "Waiting for a conflict decision",
        ),
        (
            Progress::Known {
                processed: 1,
                total: 0,
            },
            "Invalid or empty total; progress unknown",
        ),
        (
            Progress::Known {
                processed: 11,
                total: 10,
            },
            "Invalid or empty total; progress unknown",
        ),
        (
            Progress::Known {
                processed: 0,
                total: 0,
            },
            "Invalid or empty total; progress unknown",
        ),
    ];
    for (index, (progress, label)) in cases.into_iter().enumerate() {
        assert!(progress.fraction().is_none());
        operations
            .report(&running(1, index as u64 + 1, progress))
            .unwrap();
        let (output, _) = frame(&ctx, &mut operations, vec![]);
        contains(&output, label);
        contains(&output, "Completed: 2 | Skipped: 1 | Failed: 3");
        contains(&output, r"Current item: C:\generated\current.bin");
        assert!(!texts(&output).iter().any(|text| text.contains('%')));
    }
    let known = Progress::Known {
        processed: 3,
        total: 10,
    };
    assert_eq!(known.fraction(), Some(0.3));
    operations.report(&running(1, 7, known)).unwrap();
    contains(&frame(&ctx, &mut operations, vec![]).0, "3/10");
    assert_eq!(
        Progress::Known {
            processed: u64::MAX,
            total: u64::MAX
        }
        .fraction(),
        Some(1.0)
    );
}

#[test]
fn p09_keyboard_tab_space_enter_focus_and_retained_error_details_acknowledgement() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.next_command();
    frame(&ctx, &mut operations, vec![]);
    frame(&ctx, &mut operations, vec![key(Key::Tab)]);
    let (_, response) = frame(&ctx, &mut operations, vec![]);
    assert!(response.toggle.has_focus());
    frame(&ctx, &mut operations, vec![key(Key::Space)]);
    assert!(!operations.is_open());
    frame(&ctx, &mut operations, vec![key(Key::Enter)]);
    assert!(operations.is_open());
    frame(&ctx, &mut operations, vec![key(Key::Tab)]);
    let (output, response) = frame(&ctx, &mut operations, vec![]);
    let details = control(&response, 1, Action::Details);
    assert!(details.has_focus());
    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, Shape::Rect(rect) if rect.stroke.width == 2.0 && rect.rect == details.rect.expand(1.0))));
    frame(&ctx, &mut operations, vec![key(Key::Space)]);
    frame(&ctx, &mut operations, vec![key(Key::Tab)]);
    let (_, response) = frame(&ctx, &mut operations, vec![]);
    assert!(control(&response, 1, Action::Cancel).has_focus());
    frame(&ctx, &mut operations, vec![key(Key::Enter)]);
    assert_eq!(operations.next_command(), Some(Command::Cancel(id(1))));
    let issue = Issue {
        path: PathBuf::from(r"D:\generated\owned-partial.tmp"),
        error: "Cleanup failed: injected access denied".into(),
    };
    let report = Report {
        id: id(1),
        revision: 1,
        update: Update::Stopped {
            status: status(Progress::Unknown),
            outcome: Outcome::Failed,
            issues: vec![issue.clone()],
        },
    };
    operations.report(&report).unwrap();
    let (output, response) = frame(&ctx, &mut operations, vec![]);
    contains(&output, "Worker stopped with failure");
    contains(&output, &issue.error);
    assert_eq!(operations.history()[0].issues(), &[issue]);
    control(&response, 1, Action::Acknowledge).request_focus();
    frame(&ctx, &mut operations, vec![key(Key::Enter)]);
    assert!(operations.history().is_empty());
    assert!(operations.next_command().is_none());
}

#[test]
fn p09_pending_history_reservations_refuse_visibly_without_losing_failures() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    let failed_report = |sequence| Report {
        id: id(sequence),
        revision: 1,
        update: Update::Stopped {
            status: status(Progress::Unknown),
            outcome: Outcome::Failed,
            issues: vec![Issue {
                path: PathBuf::from(format!("generated-unresolved-{sequence}")),
                error: "Injected failure; recovery needed".into(),
            }],
        },
    };
    for sequence in 1..=MAX_PENDING as u64 {
        operations.enqueue(&request(sequence)).unwrap();
    }
    assert_eq!(
        operations.enqueue(&request(17)),
        Err(Refusal::PendingCapacity)
    );
    contains(
        &frame(&ctx, &mut operations, vec![]).0,
        Refusal::PendingCapacity.message(),
    );
    for sequence in 1..=16 {
        assert_eq!(
            operations.next_command(),
            Some(Command::Start(request(sequence)))
        );
        operations.report(&failed_report(sequence)).unwrap();
    }
    for sequence in 17..=32 {
        operations.enqueue(&request(sequence)).unwrap();
    }
    assert_eq!(
        operations.history().len() + operations.queued().len(),
        MAX_HISTORY
    );
    for sequence in 17..=32 {
        operations.next_command();
        operations.report(&failed_report(sequence)).unwrap();
    }
    assert_eq!(
        operations.enqueue(&request(33)),
        Err(Refusal::HistoryCapacity)
    );
    contains(
        &frame(&ctx, &mut operations, vec![]).0,
        Refusal::HistoryCapacity.message(),
    );
    assert_eq!(operations.history().len(), MAX_HISTORY);
    assert!(
        operations
            .history()
            .iter()
            .all(|record| record.state() == State::Stopped(Outcome::Failed))
    );
    assert!(
        operations
            .history()
            .iter()
            .all(|record| record.issues().len() == 1)
    );
    assert!(operations.acknowledge(id(1)));
    operations.enqueue(&request(33)).unwrap();
    assert_eq!(operations.history().len(), 31);
    assert_eq!(operations.next_command(), Some(Command::Start(request(33))));
    operations.report(&stopped(33, 1, Outcome::Failed)).unwrap();
    assert_eq!(operations.history().len(), MAX_HISTORY);
}

#[test]
fn p09_metadata_refusal_is_atomic_and_never_truncates_native_input() {
    let mut operations = Operations::new(7);
    assert_eq!(
        Request::new(
            id(1),
            Kind::Copy,
            vec![PathBuf::from("x"); MAX_SOURCES + 1],
            None
        ),
        Err(Refusal::MetadataCapacity)
    );
    assert_eq!(
        Request::new(
            id(1),
            Kind::Copy,
            vec![PathBuf::from("x".repeat(MAX_PATH_BYTES + 1))],
            None
        ),
        Err(Refusal::MetadataCapacity)
    );
    assert_eq!(
        Request::new(
            id(1),
            Kind::Copy,
            vec![PathBuf::from("x".repeat(MAX_PATH_BYTES)); 5],
            None
        ),
        Err(Refusal::MetadataCapacity)
    );
    operations.enqueue(&request(1)).unwrap();
    operations.next_command();
    let mut invalid = stopped(1, 1, Outcome::Failed);
    if let Update::Stopped { issues, .. } = &mut invalid.update {
        *issues = vec![
            Issue {
                path: PathBuf::from("generated"),
                error: "error".into()
            };
            MAX_ISSUES + 1
        ];
    }
    assert_eq!(operations.report(&invalid), Err(Refusal::MetadataCapacity));
    assert!(operations.history().is_empty());
    assert_eq!(operations.active().unwrap().state(), State::Active);
    assert_eq!(operations.active().unwrap().status(), &Status::default());
    let report = stopped(1, 1, Outcome::Failed);
    operations.report(&report).unwrap();
    assert_eq!(operations.history().len(), 1);
    let foreign = Request::new(
        OperationId {
            window: 8,
            sequence: 2,
        },
        Kind::Trash,
        vec![],
        None,
    )
    .unwrap();
    assert_eq!(operations.enqueue(&foreign), Err(Refusal::ForeignWindow));
}

#[test]
fn p09_late_pointer_release_does_not_cancel_replacement_operation() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    operations.enqueue(&request(1)).unwrap();
    operations.enqueue(&request(2)).unwrap();
    operations.next_command();
    let (_, response) = frame(&ctx, &mut operations, vec![]);
    let pos = control(&response, 1, Action::Cancel).rect.center();
    frame(
        &ctx,
        &mut operations,
        vec![
            Event::PointerMoved(pos),
            Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            },
        ],
    );
    operations
        .report(&stopped(1, 1, Outcome::Completed))
        .unwrap();
    operations.next_command();
    frame(
        &ctx,
        &mut operations,
        vec![Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::NONE,
        }],
    );
    assert_eq!(operations.active().unwrap().request().id(), id(2));
    assert_eq!(operations.active().unwrap().state(), State::Active);
    assert!(operations.next_command().is_none());
}

#[test]
fn p09_bounded_presenter_ids_across_sequential_operations_including_full_path_hover() {
    let ctx = Context::default();
    ctx.global_style_mut(|style| style.interaction.tooltip_delay = 0.0);
    let mut operations = Operations::new(7);
    let mut warmed = None;
    for sequence in 1..=200 {
        let long_source = PathBuf::from(format!(
            r"C:\generated\{}-{sequence}",
            "long-name".repeat(90)
        ));
        let request =
            Request::new(id(sequence), Kind::Copy, vec![long_source.clone()], None).unwrap();
        operations.enqueue(&request).unwrap();
        operations.next_command();
        operations
            .report(&running(
                sequence,
                1,
                Progress::Known {
                    processed: 1,
                    total: 10,
                },
            ))
            .unwrap();
        let (output, response) = frame(&ctx, &mut operations, vec![]);
        // Source is above the details button; hover a real rendered label.
        let text = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                Shape::Text(text) if text.galley.job.text.starts_with("Source:") => Some(text),
                _ => None,
            })
            .unwrap();
        let pos = text.pos + egui::vec2(5.0, 5.0);
        frame(&ctx, &mut operations, vec![Event::PointerMoved(pos)]);
        let (hover_output, _) = frame(&ctx, &mut operations, vec![]);
        contains(
            &hover_output,
            &format!("Source: {}", long_source.to_string_lossy()),
        );
        assert!(ctx.memory(|memory| {
            memory
                .areas()
                .visible_layer_ids()
                .iter()
                .any(|layer| layer.order == egui::Order::Tooltip)
        }));
        assert_ne!(
            response.toggle.id,
            control(&response, sequence, Action::Details).id
        );
        let retained = (
            ctx.data(|data| data.len()),
            ctx.memory(|memory| {
                format!("{:?}", memory.areas())
                    .matches("AreaState {")
                    .count()
            }),
        );
        if sequence == 3 {
            warmed = Some(retained);
        }
        if sequence > 3 {
            assert_eq!(
                Some(retained),
                warmed,
                "presenter state leaked at operation {sequence}"
            );
        }
        operations
            .report(&stopped(sequence, 2, Outcome::Completed))
            .unwrap();
        frame(&ctx, &mut operations, vec![]);
        assert!(operations.acknowledge(id(sequence)));
        frame(&ctx, &mut operations, vec![Event::PointerGone]);
        assert!(operations.active().is_none());
        assert!(operations.queued().is_empty());
        assert!(operations.history().is_empty());
    }
}

#[test]
fn p09_all_kinds_and_conflict_waiting_leave_browsing_input_usable() {
    let ctx = Context::default();
    let mut operations = Operations::new(7);
    for (index, kind) in [
        Kind::CreateFile,
        Kind::CreateFolder,
        Kind::Rename,
        Kind::Copy,
        Kind::Move,
        Kind::Trash,
        Kind::Delete,
    ]
    .into_iter()
    .enumerate()
    {
        let request = Request::new(id(index as u64 + 1), kind, vec![], None).unwrap();
        operations.enqueue(&request).unwrap();
    }
    operations.next_command();
    operations
        .report(&Report {
            id: id(1),
            revision: 1,
            update: Update::Running(Status {
                progress: Progress::WaitingForConflict,
                ..Default::default()
            }),
        })
        .unwrap();
    let mut browsing_input = String::from("generated location");
    let mut field = None;
    let mut output = ctx.run_ui(RawInput::default(), |ui| {
        operations.show(ui);
        field = Some(ui.text_edit_singleline(&mut browsing_input));
    });
    output.textures_delta.clear();
    field.unwrap().request_focus();
    let mut output = ctx.run_ui(
        RawInput {
            events: vec![Event::Text(" readable".into())],
            ..Default::default()
        },
        |ui| {
            operations.show(ui);
            ui.text_edit_singleline(&mut browsing_input);
        },
    );
    output.textures_delta.clear();
    assert!(browsing_input.contains(" readable"));
    assert_eq!(
        operations.active().unwrap().status().progress,
        Progress::WaitingForConflict
    );
    assert_eq!(operations.queued().len(), 6);
    assert!(operations.next_command().is_none());
}

#[cfg(windows)]
#[test]
fn p09_native_namespace_and_non_unicode_display_collision_never_become_command_data() {
    use std::{
        ffi::OsString,
        os::windows::ffi::{OsStrExt, OsStringExt},
    };
    let paths = [
        PathBuf::from(OsString::from_wide(&[0x78, 0xD800])),
        PathBuf::from(OsString::from_wide(&[0x78, 0xD801])),
    ];
    assert_eq!(paths[0].to_string_lossy(), paths[1].to_string_lossy());
    assert_ne!(paths[0], paths[1]);
    let destination = PathBuf::from(r"\\?\UNC\server\share\é\destination");
    let request =
        Request::new(id(1), Kind::Move, paths.to_vec(), Some(destination.clone())).unwrap();
    let mut operations = Operations::new(7);
    operations.enqueue(&request).unwrap();
    let Some(Command::Start(command)) = operations.next_command() else {
        panic!("missing Start");
    };
    assert_eq!(command, request);
    assert_eq!(
        command.sources()[0]
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        [0x78, 0xD800]
    );
    assert_eq!(
        command.sources()[1]
            .as_os_str()
            .encode_wide()
            .collect::<Vec<_>>(),
        [0x78, 0xD801]
    );
    assert_eq!(command.destination(), Some(destination.as_path()));
    let ctx = Context::default();
    frame(&ctx, &mut operations, vec![]);
    assert_eq!(operations.active().unwrap().request(), &request);
    let mut other = Operations::new(8);
    let mut response = None;
    let mut output = ctx.run_ui(RawInput::default(), |ui| {
        response = Some((operations.show(ui).toggle.id, other.show(ui).toggle.id));
    });
    output.textures_delta.clear();
    let (left, right) = response.unwrap();
    assert_ne!(left, right);
}
