use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use slint::Model;
use slint::ModelRc;
use slint::VecModel;

use crate::core::junk_item::JunkItem;
use crate::core::scan_summary::ScanSummary;
use crate::platform::privileges::is_elevated;

use super::adapters;

slint::include_modules!();

struct Handles {
    safe: Rc<VecModel<CategoryRow>>,
    aggressive: Rc<VecModel<CategoryRow>>,
    results: Rc<VecModel<ResultRow>>,
}

struct Session {
    handles: Rc<Handles>,
    snapshot: Arc<Mutex<Vec<JunkItem>>>,
    age_label: Rc<RefCell<String>>,
}

pub fn run() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let session = Session::new();
    session.apply_initial(&ui);
    session.wire(&ui);
    ui.run()
}

impl Session {
    fn new() -> Rc<Self> {
        let (safe_data, aggressive_data) = adapters::initial_categories();
        let checked = default_checked(&safe_data, &aggressive_data);
        let empty_sizes = HashMap::new();
        let handles = Rc::new(Handles {
            safe: Rc::new(VecModel::from(category_rows(
                &safe_data,
                &checked,
                &empty_sizes,
            ))),
            aggressive: Rc::new(VecModel::from(category_rows(
                &aggressive_data,
                &checked,
                &empty_sizes,
            ))),
            results: Rc::new(VecModel::from(Vec::new())),
        });

        Rc::new(Self {
            handles,
            snapshot: Arc::new(Mutex::new(Vec::new())),
            age_label: Rc::new(RefCell::new("Any age".to_string())),
        })
    }

    fn apply_initial(self: &Rc<Self>, ui: &MainWindow) {
        ui.set_safe_categories(ModelRc::new(self.handles.safe.clone()));
        ui.set_aggressive_categories(ModelRc::new(self.handles.aggressive.clone()));
        ui.set_results(ModelRc::new(self.handles.results.clone()));
        ui.set_is_admin(is_elevated());
    }

    fn wire(self: &Rc<Self>, ui: &MainWindow) {
        self.wire_selection(ui);
        self.wire_scan(ui);
        self.wire_clean(ui);
        self.wire_options(ui);
    }

    fn wire_selection(self: &Rc<Self>, ui: &MainWindow) {
        let session = Rc::clone(self);
        ui.on_toggle_safe(move |index| {
            flip_checked(&session.handles.safe, index);
        });

        let session = Rc::clone(self);
        ui.on_toggle_aggressive(move |index| {
            flip_checked(&session.handles.aggressive, index);
        });

        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_toggle_result(move |index| {
            let model = &session.handles.results;
            if let Some(mut row) = model.row_data(index as usize) {
                row.selected = !row.selected;
                model.set_row_data(index as usize, row);
            }
            if let Some(ui) = weak.upgrade() {
                session.refresh_selection_open(&ui);
            }
        });

        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_select_all(move || {
            set_all_selected(&session.handles.results, true);
            if let Some(ui) = weak.upgrade() {
                session.refresh_selection_open(&ui);
            }
        });

        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_select_none(move || {
            set_all_selected(&session.handles.results, false);
            if let Some(ui) = weak.upgrade() {
                session.refresh_selection_open(&ui);
            }
        });
    }

    fn wire_scan(self: &Rc<Self>, ui: &MainWindow) {
        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_scan_requested(move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            session.start_scan(&ui);
        });
    }

    fn wire_clean(self: &Rc<Self>, ui: &MainWindow) {
        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_clean_requested(move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            session.ask_confirm(&ui);
        });

        let weak = ui.as_weak();
        ui.on_clean_cancelled(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_confirm_open(false);
            }
        });

        let weak = ui.as_weak();
        let session = Rc::clone(self);
        ui.on_clean_confirmed(move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            ui.set_confirm_open(false);
            session.start_clean(&ui);
        });
    }

    fn wire_options(self: &Rc<Self>, ui: &MainWindow) {
        let session = Rc::clone(self);
        ui.on_age_changed(move |label| {
            *session.age_label.borrow_mut() = label.to_string();
        });

        let session = Rc::clone(self);
        ui.on_aggressive_changed(move |shown| {
            if !shown {
                uncheck_all(&session.handles.aggressive);
            }
        });
    }

    fn start_scan(self: &Rc<Self>, ui: &MainWindow) {
        let ids = checked_ids(&self.handles);
        if ids.is_empty() {
            ui.set_status_text("Select at least one category first.".into());
            return;
        }

        let age = adapters::minimum_age_for_label(&self.age_label.borrow());
        ui.set_busy(true);
        ui.set_status_text("Scanning…".into());

        let weak = ui.as_weak();
        let snapshot = Arc::clone(&self.snapshot);
        std::thread::spawn(move || {
            let summary = adapters::run_scan(&ids, age);
            let _ = weak.upgrade_in_event_loop(move |ui| {
                present_scan(&ui, &ids, summary, None, &snapshot);
            });
        });
    }

    fn ask_confirm(&self, ui: &MainWindow) {
        let flags = selected_flags(&self.handles.results);
        let guard = self.snapshot.lock().unwrap();
        let (count, bytes) = adapters::selection_summary(&guard, &flags);
        if count == 0 {
            ui.set_status_text("Nothing selected.".into());
            return;
        }
        ui.set_confirm_detail(adapters::confirm_detail(count, bytes).into());
        ui.set_confirm_open(true);
    }

    fn start_clean(self: &Rc<Self>, ui: &MainWindow) {
        let flags = selected_flags(&self.handles.results);
        let ids = checked_ids(&self.handles);
        let age = adapters::minimum_age_for_label(&self.age_label.borrow());
        let snapshot = self.snapshot.lock().unwrap().clone();

        ui.set_busy(true);
        ui.set_status_text("Cleaning…".into());

        let weak = ui.as_weak();
        let target = Arc::clone(&self.snapshot);
        std::thread::spawn(move || {
            let cleaned = adapters::remove_selection(&snapshot, &flags);
            let summary = adapters::run_scan(&ids, age);
            let status = adapters::clean_status(
                cleaned.removed_files,
                cleaned.freed_bytes,
                cleaned.skipped_files,
                cleaned.failed_files,
            );
            let _ = weak.upgrade_in_event_loop(move |ui| {
                present_scan(&ui, &ids, summary, Some(status), &target);
            });
        });
    }

    fn refresh_selection_open(&self, ui: &MainWindow) {
        let flags = selected_flags(&self.handles.results);
        let guard = self.snapshot.lock().unwrap();
        let (count, bytes) = adapters::selection_summary(&guard, &flags);
        ui.set_selected_count(count as i32);
        ui.set_selection_text(
            format!(
                "{count} files · {size}",
                size = crate::core::size_format::format_bytes(bytes)
            )
            .into(),
        );
    }
}

fn present_scan(
    ui: &MainWindow,
    checked: &[String],
    summary: ScanSummary,
    status: Option<String>,
    snapshot: &Arc<Mutex<Vec<JunkItem>>>,
) {
    let rows = to_result_rows(&summary);
    let sizes = adapters::category_sizes(&summary);
    let (total, files) = adapters::totals_text(&summary);
    let result_count = summary.files;
    let empty = summary.is_empty();
    *snapshot.lock().unwrap() = summary.items;

    ui.set_results(ModelRc::new(Rc::new(VecModel::from(rows))));

    let (safe_data, aggressive_data) = adapters::initial_categories();
    ui.set_safe_categories(ModelRc::new(Rc::new(VecModel::from(category_rows(
        &safe_data, checked, &sizes,
    )))));
    ui.set_aggressive_categories(ModelRc::new(Rc::new(VecModel::from(category_rows(
        &aggressive_data,
        checked,
        &sizes,
    )))));

    ui.set_total_text(total.clone().into());
    ui.set_files_text(files.clone().into());
    ui.set_result_count(result_count as i32);
    ui.set_has_scanned(true);
    ui.set_busy(false);

    let default_status = if empty {
        "No junk found in the selected categories.".to_string()
    } else {
        format!("Found {files} files using {total}.")
    };
    ui.set_status_text(status.unwrap_or(default_status).into());
    ui.set_selected_count(result_count as i32);
    ui.set_selection_text(format!("{files} files · {total}").into());
}

fn default_checked(
    safe: &[adapters::CategoryData],
    aggressive: &[adapters::CategoryData],
) -> Vec<String> {
    safe.iter()
        .chain(aggressive.iter())
        .filter(|entry| entry.checked)
        .map(|entry| entry.id.clone())
        .collect()
}

fn category_rows(
    data: &[adapters::CategoryData],
    checked: &[String],
    sizes: &HashMap<String, String>,
) -> Vec<CategoryRow> {
    data.iter()
        .map(|entry| CategoryRow {
            id: entry.id.clone().into(),
            label: entry.label.clone().into(),
            detail: entry.detail.clone().into(),
            checked: checked.iter().any(|id| id == &entry.id),
            size_text: sizes
                .get(&entry.id)
                .cloned()
                .unwrap_or_else(|| "—".to_string())
                .into(),
        })
        .collect()
}

fn to_result_rows(summary: &ScanSummary) -> Vec<ResultRow> {
    summary
        .items
        .iter()
        .map(|item| ResultRow {
            selected: true,
            category: item.category.clone().into(),
            path: item.path.to_string_lossy().to_string().into(),
            size_text: crate::core::size_format::format_bytes(item.size_bytes).into(),
            risk: item.risk.label().into(),
        })
        .collect()
}

fn checked_ids(handles: &Handles) -> Vec<String> {
    let mut ids = Vec::new();
    for model in [&handles.safe, &handles.aggressive] {
        for row in model.iter() {
            if row.checked {
                ids.push(row.id.to_string());
            }
        }
    }
    ids
}

fn selected_flags(model: &Rc<VecModel<ResultRow>>) -> Vec<bool> {
    model.iter().map(|row| row.selected).collect()
}

fn flip_checked(model: &Rc<VecModel<CategoryRow>>, index: i32) {
    if let Some(mut row) = model.row_data(index as usize) {
        row.checked = !row.checked;
        model.set_row_data(index as usize, row);
    }
}

fn set_all_selected(model: &Rc<VecModel<ResultRow>>, selected: bool) {
    let rows: Vec<ResultRow> = model
        .iter()
        .map(|mut row| {
            row.selected = selected;
            row
        })
        .collect();
    model.set_vec(rows);
}

fn uncheck_all(model: &Rc<VecModel<CategoryRow>>) {
    for index in 0..model.row_count() {
        if let Some(mut row) = model.row_data(index) {
            row.checked = false;
            model.set_row_data(index, row);
        }
    }
}
