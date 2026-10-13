use crate::{
    actions::Controls,
    platform::{
        bin::{self, BinItem, BinOutcome},
        external,
        operations::Operation,
        windows::OleGuard,
    },
    rename::{RenameOptions, RenamePlan},
    session::{Session, Store},
    statistics::Statistics,
    ui::BrowserUi,
};
use eframe::egui;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::mpsc::{self, Receiver, TryRecvError},
};
use windows::Win32::Foundation::HWND;
#[derive(Default)]
pub struct Advanced {
    stats: Statistics,
    rename_open: bool,
    rename_options: RenameOptions,
    rename_plan: Option<RenamePlan>,
    bin_items: Vec<BinItem>,
    bin_selected: HashSet<Vec<u16>>,
    bin_outcome: Option<Receiver<Vec<BinOutcome>>>,
    bin_pending: Vec<Vec<u16>>,
    confirm_bin: Option<Vec<BinItem>>,
    bin_ledger: Vec<BinOutcome>,
    confirm_delete: Option<Vec<PathBuf>>,
    pub presets: HashMap<String, Session>,
    preset_name: String,
    preset_confirmation: Option<(String, bool)>,
}
impl Advanced {
    pub fn poll(&mut self, browser: &mut BrowserUi) {
        let Some(rx) = &self.bin_outcome else {
            return;
        };
        let results = match rx.try_recv() {
            Ok(results) => results,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => self
                .bin_pending
                .iter()
                .cloned()
                .map(|identity| BinOutcome {
                    identity,
                    result: Err(
                        "Recycle Bin action worker disconnected; refresh before retrying".into(),
                    ),
                })
                .collect(),
        };
        browser.status = results
            .iter()
            .map(|r| format!("{:?}", r.result))
            .collect::<Vec<_>>()
            .join("; ");
        self.bin_ledger = results;
        self.bin_pending.clear();
        self.bin_outcome = None;
        self.bin_items.clear();
        self.bin_selected.clear();
        for (_, tab) in browser.dock.iter_all_tabs_mut() {
            if let Some(view) = &mut tab.bin_view {
                view.refresh();
            }
        }
    }

    pub fn is_busy(&self) -> bool {
        self.bin_outcome.is_some()
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        browser: &mut BrowserUi,
        controls: &mut Controls,
        hwnd: HWND,
        store: Option<&Store>,
    ) {
        let ctx = ui.ctx().clone();
        let current = browser
            .active_tab()
            .map(|t| t.browser.current_path().to_owned());
        self.bin_items.clear();
        self.bin_selected.clear();
        if let Some(tab) = browser.active_any_tab()
            && let Some(view) = tab.bin_view.as_ref()
        {
            self.bin_items = view.selected_items();
            self.bin_selected = self.bin_items.iter().map(|i| i.identity.clone()).collect();
        }
        let selected = browser
            .active_tab()
            .map(|t| t.selection.paths().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        ui.horizontal_wrapped(|ui| {
            if ui.button("Folder totals").clicked()
                && let Some(path) = current.clone()
            {
                self.stats.request(path);
            }
            if ui.button("Batch rename").clicked() {
                self.rename_open = true;
                self.rename_plan = None;
            }
            if ui.button("Recycle Bin").clicked() {
                browser.open_recycle_bin();
            }
            if ui.button("Explorer").clicked()
                && let Some(path) = &current
                && let Err(e) = external::open_explorer(path)
            {
                browser.status = e;
            }
            if ui.button("Terminal").clicked()
                && let Some(path) = &current
                && let Err(e) = external::open_terminal(path)
            {
                browser.status = e;
            }
            if ui.button("Create shortcuts here").clicked()
                && let Some(destination) = &current
            {
                let items = selected
                    .iter()
                    .map(|source| Operation::Shortcut {
                        source: source.clone(),
                        destination: destination.join(format!(
                            "{} - Shortcut.lnk",
                            source.file_name().unwrap_or_default().to_string_lossy()
                        )),
                    })
                    .collect();
                if let Err(e) = controls.submit(items) {
                    browser.status = e;
                }
            }
            if ui.button("Permanently delete…").clicked() && !selected.is_empty() {
                self.confirm_delete = Some(selected.clone());
            }
        });
        if !self.bin_selected.is_empty() {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        !self.is_busy() && !controls.worker.is_busy(),
                        egui::Button::new("Restore selected bin items"),
                    )
                    .clicked()
                {
                    self.bin_action(hwnd, false, self.bin_selected.iter().cloned().collect());
                }
                if ui.button("Delete selected bin items…").clicked() {
                    self.confirm_bin = Some(self.bin_items.clone());
                }
            });
        }
        self.stats.poll();
        if let Some(result) = self.stats.result().cloned() {
            egui::CollapsingHeader::new("Folder statistics").show(ui, |ui| {
                ui.label(format!(
                    "{} files, {} folders, {} bytes • complete: {} • cancelled: {}",
                    result.files, result.dirs, result.bytes, result.complete, result.cancelled
                ));
                ui.label(format!(
                    "Errors: {} • skipped links: {} • limit: {}",
                    result.errors, result.skipped_reparse, result.limit_reached
                ));
                if let Some(message) = &result.message {
                    ui.label(message);
                }
                if ui.button("Cancel scan").clicked() {
                    self.stats.cancel();
                }
            });
        }
        ui.horizontal(|ui| {
            ui.label("Layout preset");
            ui.text_edit_singleline(&mut self.preset_name);
            if ui.button("Save layout").clicked() {
                if self.presets.contains_key(&self.preset_name) {
                    self.preset_confirmation = Some((self.preset_name.clone(), false));
                } else {
                    self.save_preset(self.preset_name.clone(), browser);
                }
            }
            if ui.button("Load layout").clicked()
                && let Some(session) = self.presets.get(&self.preset_name)
            {
                let home = crate::platform::shell::home_directory()
                    .unwrap_or_else(|_| current.clone().unwrap_or_default());
                match crate::persistence::restore(session.clone(), home) {
                    Ok(mut restored) => {
                        restored.settings = browser.settings.clone();
                        *browser = restored;
                    }
                    Err(e) => browser.status = e,
                }
            }
            if ui.button("Delete preset…").clicked() {
                if self.presets.contains_key(&self.preset_name) {
                    self.preset_confirmation = Some((self.preset_name.clone(), true));
                } else {
                    browser.status = "No preset exists with that name".into();
                }
            }
        });
        if let Some((name, delete)) = self.preset_confirmation.clone() {
            egui::Window::new("Confirm preset change").show(&ctx, |ui| {
                ui.label(&name);
                if ui.button("Confirm").clicked() {
                    if delete {
                        self.presets.remove(&name);
                    } else {
                        self.save_preset(name.clone(), browser);
                    }
                    self.preset_confirmation = None;
                }
                if ui.button("Cancel").clicked() {
                    self.preset_confirmation = None;
                }
            });
        }
        let _ = store; // Presets are persisted by the root session mapping.
        if self.rename_plan.as_ref().is_some_and(|p| {
            p.items.iter().map(|i| &i.source).collect::<Vec<_>>()
                != selected.iter().collect::<Vec<_>>()
        }) {
            self.rename_plan = None;
        }
        if self.rename_open {
            let mut open = true;
            egui::Window::new("Batch rename preview").open(&mut open).show(&ctx,|ui|{
            ui.label("Independent policy: numbering starts at 1, extension preserved; optional date is UTC Unix seconds.");
            let mut changed=false;changed|=ui.text_edit_singleline(&mut self.rename_options.prefix).changed();changed|=ui.text_edit_singleline(&mut self.rename_options.suffix).changed();changed|=ui.checkbox(&mut self.rename_options.numbering,"Numbering").changed();changed|=ui.checkbox(&mut self.rename_options.preserve_extension,"Preserve extension").changed();changed|=ui.checkbox(&mut self.rename_options.modified_utc_seconds,"Append modified UTC seconds").changed();
            if changed{self.rename_plan=None;}
            if ui.button("Preview selected items (no changes)").clicked(){self.rename_plan=Some(RenamePlan::build(&selected,self.rename_options.clone()));}
            if let Some(plan)=&self.rename_plan{egui::ScrollArea::vertical().max_height(240.0).show(ui,|ui|{for item in &plan.items{ui.label(format!("{} → {}",item.source.display(),item.target.display()));for e in &item.errors{ui.colored_label(egui::Color32::LIGHT_RED,e);}}for e in &plan.errors{ui.colored_label(egui::Color32::LIGHT_RED,e);}});
                ui.label("Commit is not atomic; completed items stay renamed if later work fails or is cancelled.");
                if ui.add_enabled(plan.is_valid()&&!controls.worker.is_busy()&&!self.is_busy(),egui::Button::new("Confirm and commit this preview")).clicked(){match plan.clone().into_operations(){Ok(items)=>{if let Err(e)=controls.submit(items){browser.status=e;}else{self.rename_plan=None;}},Err(e)=>browser.status=e}}
            }
        });
            self.rename_open = open;
        }
        if let Some(paths) = &self.confirm_delete {
            let mut confirm = false;
            let mut cancel = false;
            egui::Window::new("Permanent deletion — cannot undo").show(&ctx, |ui| {
                for path in paths {
                    ui.label(path.to_string_lossy());
                }
                ui.label("These exact items will not go to the Recycle Bin.");
                confirm = ui
                    .add_enabled(
                        !controls.worker.is_busy() && !self.is_busy(),
                        egui::Button::new("Permanently delete these items"),
                    )
                    .clicked();
                cancel = ui.button("Cancel").clicked();
            });
            if confirm {
                let items = self
                    .confirm_delete
                    .take()
                    .unwrap()
                    .into_iter()
                    .map(|source| Operation::PermanentDelete {
                        source,
                        confirmed: true,
                    })
                    .collect();
                if let Err(e) = controls.submit(items) {
                    browser.status = e;
                }
            }
            if cancel {
                self.confirm_delete = None;
            }
        }
        if let Some(captured) = self.confirm_bin.clone() {
            egui::Window::new("Confirm selected Recycle Bin deletion").show(&ctx, |ui| {
                for item in &captured {
                    ui.label(&item.display_name);
                }
                if ui
                    .add_enabled(
                        !self.is_busy() && !controls.worker.is_busy(),
                        egui::Button::new("Permanently delete only these items"),
                    )
                    .clicked()
                {
                    self.bin_action(
                        hwnd,
                        true,
                        captured.iter().map(|i| i.identity.clone()).collect(),
                    );
                    self.confirm_bin = None;
                }
                if ui.button("Cancel").clicked() {
                    self.confirm_bin = None;
                }
            });
        }
        if !self.bin_ledger.is_empty() {
            ui.collapsing("Recycle Bin operation results", |ui| {
                for item in &self.bin_ledger {
                    ui.label(format!(
                        "{}: {:?}",
                        String::from_utf16_lossy(&item.identity),
                        item.result
                    ));
                }
            });
        }
    }
    fn save_preset(&mut self, name: String, browser: &mut BrowserUi) {
        if name.trim().is_empty() || name.len() > 128 {
            browser.status = "Preset name must contain 1 to 128 bytes and cannot be blank".into();
            return;
        }
        if self.presets.len() >= 32 && !self.presets.contains_key(&name) {
            browser.status =
                "32 preset limit reached; overwrite or delete an existing preset".into();
            return;
        }
        self.presets
            .insert(name, crate::persistence::snapshot(browser));
        browser.status = "Layout preset saved".into();
    }
    fn bin_action(&mut self, hwnd: HWND, delete: bool, ids: Vec<Vec<u16>>) {
        if self.bin_outcome.is_some() {
            return;
        }

        let (tx, rx) = mpsc::sync_channel(1);
        self.bin_outcome = Some(rx);
        let owner = hwnd.0 as usize;
        let retained = ids.clone();
        self.bin_pending = ids.clone();
        if let Err(error) = std::thread::Builder::new()
            .name("wayfarer-bin-action".into())
            .spawn(move || {
                let results = match OleGuard::initialize() {
                    Ok(_guard) => {
                        if delete {
                            bin::delete_selection(HWND(owner as *mut _), &ids, true)
                        } else {
                            bin::restore_selection(HWND(owner as *mut _), &ids)
                        }
                    }
                    Err(e) => Err(e.to_string()),
                };
                let values = results.unwrap_or_else(|e| {
                    ids.into_iter()
                        .map(|identity| BinOutcome {
                            identity,
                            result: Err(e.clone()),
                        })
                        .collect()
                });
                let _ = tx.send(values);
            })
        {
            self.bin_outcome = None;
            self.bin_pending.clear();
            self.bin_ledger = retained
                .into_iter()
                .map(|identity| BinOutcome {
                    identity,
                    result: Err(format!("Cannot start Recycle Bin action: {error}")),
                })
                .collect();
        }
    }
}
