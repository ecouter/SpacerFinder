use spacer_core::{
    add_element, delete_element, format_length, load_elements, save_elements, update_element,
    validate_length, validate_thread, Element, Thread,
};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::rc::Rc;

slint::include_modules!();

#[derive(Default)]
struct State {
    elements: Vec<Element>,
    search: String,
    sort_mode: i32,
    edit_index: Option<usize>,
    dirty: bool,
}

fn family(element: &Element) -> String {
    Thread::parse(&element.thread_in)
        .map(|thread| format!("M{}", thread.size))
        .unwrap_or_else(|| "Other".to_string())
}

fn family_key(family: &str) -> u64 {
    family
        .strip_prefix('M')
        .and_then(|digits| digits.parse().ok())
        .unwrap_or(u64::MAX)
}

/// Builds a flat list of group-header rows and spacer rows. Each row keeps the
/// original inventory index so edit/delete stay correct even when filtered.
fn visible_rows(state: &State) -> (Vec<UiRow>, usize) {
    let needle = state.search.trim().to_lowercase();
    let mut items: Vec<(usize, &Element)> = state
        .elements
        .iter()
        .enumerate()
        .filter(|(_, element)| {
            needle.is_empty()
                || element.thread_in.to_lowercase().contains(&needle)
                || element.thread_out.to_lowercase().contains(&needle)
                || format_length(element.length).contains(&needle)
        })
        .collect();

    items.sort_by(|(index_a, a), (index_b, b)| {
        family_key(&family(a))
            .cmp(&family_key(&family(b)))
            .then_with(|| match state.sort_mode {
                1 => b.length.partial_cmp(&a.length).unwrap_or(Ordering::Equal),
                2 => a.length.partial_cmp(&b.length).unwrap_or(Ordering::Equal),
                _ => a
                    .thread_in
                    .cmp(&b.thread_in)
                    .then_with(|| a.thread_out.cmp(&b.thread_out))
                    .then_with(|| a.length.partial_cmp(&b.length).unwrap_or(Ordering::Equal)),
            })
            .then_with(|| index_a.cmp(index_b))
    });

    let mut counts = BTreeMap::new();
    for (_, element) in &items {
        *counts.entry(family(element)).or_insert(0) += 1;
    }

    let mut rows = Vec::new();
    let mut current_family: Option<String> = None;
    for (index, element) in items {
        let group = family(element);
        if current_family.as_deref() != Some(group.as_str()) {
            rows.push(UiRow {
                is_header: true,
                family: group.clone().into(),
                count: counts[&group],
                element_index: -1,
                thread_in: "".into(),
                thread_out: "".into(),
                length: 0.0,
            });
            current_family = Some(group);
        }
        rows.push(UiRow {
            is_header: false,
            family: "".into(),
            count: 0,
            element_index: index as i32,
            thread_in: element.thread_in.clone().into(),
            thread_out: element.thread_out.clone().into(),
            length: element.length as f32,
        });
    }

    let visible = rows.iter().filter(|row| !row.is_header).count();
    (rows, visible)
}

fn refresh(window: &MainWindow, state: &State) {
    let (rows, visible) = visible_rows(state);
    window.set_rows(slint::ModelRc::new(slint::VecModel::from(rows)));
    window.set_visible_count(visible as i32);
    window.set_element_count(state.elements.len() as i32);
    window.set_dirty(state.dirty);
}

fn set_status(window: &MainWindow, message: &str, kind: &str) {
    window.set_status_message(message.into());
    window.set_status_kind(kind.into());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Rc::new(RefCell::new(State::default()));
    let window = MainWindow::new()?;

    {
        let mut current = state.borrow_mut();
        current.elements = load_elements("spacers.json").unwrap_or_default();
        window.set_file_path("spacers.json".into());
        refresh(&window, &current);
    }

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_open_add(move || {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().edit_index = None;
            window.set_form_title("Add spacer".into());
            window.set_form_thread_in("".into());
            window.set_form_thread_out("".into());
            window.set_form_length("".into());
            window.set_form_error("".into());
            window.set_duplicate_pending(false);
            window.set_show_form_dialog(true);
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_open_edit(move |index| {
        if let Some(window) = weak.upgrade() {
            let element = shared.borrow().elements.get(index as usize).cloned();
            if let Some(element) = element {
                shared.borrow_mut().edit_index = Some(index as usize);
                window.set_form_title("Edit spacer".into());
                window.set_form_thread_in(element.thread_in.clone().into());
                window.set_form_thread_out(element.thread_out.clone().into());
                window.set_form_length(format_length(element.length).into());
                window.set_form_error("".into());
                window.set_duplicate_pending(false);
                window.set_show_form_dialog(true);
            }
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_save_form(move |force| {
        let Some(window) = weak.upgrade() else { return };
        let thread_in = window.get_form_thread_in().to_string();
        let thread_out = window.get_form_thread_out().to_string();
        let length_text = window.get_form_length().to_string();

        let length = match length_text.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                window.set_form_error("Enter a valid length, for example 5.0.".into());
                window.set_duplicate_pending(false);
                return;
            }
        };
        if let Err(message) = validate_thread(&thread_in)
            .and_then(|_| validate_thread(&thread_out))
            .and_then(|_| validate_length(length))
        {
            window.set_form_error(message.into());
            window.set_duplicate_pending(false);
            return;
        }

        let element = Element::new(thread_in, thread_out, length);
        let mut current = shared.borrow_mut();
        let edit_index = current.edit_index;
        let duplicate = current
            .elements
            .iter()
            .enumerate()
            .any(|(index, existing)| Some(index) != edit_index && existing.matches(&element));

        if duplicate && !force {
            window
                .set_form_error("A spacer with these exact specifications already exists.".into());
            window.set_duplicate_pending(true);
            return;
        }

        current.elements = match edit_index {
            Some(index) => update_element(&current.elements, index, element),
            None => add_element(&current.elements, element),
        };
        current.edit_index = None;
        current.dirty = true;
        refresh(&window, &current);
        window.set_show_form_dialog(false);
        window.set_form_error("".into());
        window.set_duplicate_pending(false);
        set_status(&window, "Spacer saved to the inventory.", "success");
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_cancel_form(move || {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().edit_index = None;
            window.set_show_form_dialog(false);
            window.set_form_error("".into());
            window.set_duplicate_pending(false);
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_confirm_delete(move || {
        let Some(window) = weak.upgrade() else { return };
        let index = window.get_delete_index() as usize;
        let mut current = shared.borrow_mut();
        if index < current.elements.len() {
            current.elements = delete_element(&current.elements, index);
            current.dirty = true;
            refresh(&window, &current);
            set_status(&window, "Spacer deleted.", "success");
        }
        window.set_show_delete_dialog(false);
    });

    let weak = window.as_weak();
    window.on_cancel_delete(move || {
        if let Some(window) = weak.upgrade() {
            window.set_show_delete_dialog(false);
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_save_file(move || {
        let Some(window) = weak.upgrade() else { return };
        let path = window.get_file_path().to_string();
        let current = shared.borrow();
        match save_elements(&path, &current.elements) {
            Ok(()) => {
                drop(current);
                shared.borrow_mut().dirty = false;
                refresh(&window, &shared.borrow());
                set_status(
                    &window,
                    &format!("Saved {} spacers to {}.", window.get_file_path(), path),
                    "success",
                );
            }
            Err(error) => set_status(&window, &error, "error"),
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_load_file(move || {
        let Some(window) = weak.upgrade() else { return };
        let path = window.get_file_path().to_string();
        match load_elements(&path) {
            Ok(elements) => {
                let mut current = shared.borrow_mut();
                current.elements = elements;
                current.dirty = false;
                current.edit_index = None;
                let count = current.elements.len();
                refresh(&window, &current);
                set_status(
                    &window,
                    &format!("Loaded {count} spacers from {path}."),
                    "success",
                );
            }
            Err(error) => set_status(&window, &error, "error"),
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_search_changed(move |text| {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().search = text.to_string();
            let current = shared.borrow();
            refresh(&window, &current);
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_sort_changed(move |mode| {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().sort_mode = mode;
            let current = shared.borrow();
            refresh(&window, &current);
        }
    });

    let weak = window.as_weak();
    window.on_clear_status(move || {
        if let Some(window) = weak.upgrade() {
            window.set_status_message("".into());
        }
    });

    window.run()?;
    Ok(())
}
