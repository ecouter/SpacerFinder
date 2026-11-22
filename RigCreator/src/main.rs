use spacer_core::{
    add_element, delete_element, find_minimal_chain, format_length, load_elements, move_down,
    move_up, save_elements, update_element, validate_length, validate_thread, Element,
};
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

#[derive(Default)]
struct State {
    rigs: Vec<Element>,
    edit_index: Option<usize>,
    dirty: bool,
}

fn refresh(window: &MainWindow, state: &State) {
    let rigs: Vec<UiElement> = state
        .rigs
        .iter()
        .map(|element| UiElement {
            thread_in: element.thread_in.clone().into(),
            thread_out: element.thread_out.clone().into(),
            length: element.length as f32,
        })
        .collect();
    window.set_rigs(slint::ModelRc::new(slint::VecModel::from(rigs)));
    window.set_rig_count(state.rigs.len() as i32);
    window.set_dirty(state.dirty);
}

fn set_status(window: &MainWindow, message: &str, kind: &str) {
    window.set_status_message(message.into());
    window.set_status_kind(kind.into());
}

fn clear_result(window: &MainWindow) {
    window.set_result_visible(false);
    window.set_result_steps(slint::ModelRc::new(slint::VecModel::from(
        Vec::<UiStep>::new(),
    )));
}

fn wire_callbacks(window: &MainWindow, state: Rc<RefCell<State>>) {
    let weak = window.as_weak();
    let shared = state.clone();
    window.on_open_add(move || {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().edit_index = None;
            window.set_form_title("Add required element".into());
            window.set_form_thread_in("".into());
            window.set_form_thread_out("".into());
            window.set_form_length("".into());
            window.set_form_error("".into());
            window.set_show_form_dialog(true);
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_open_edit(move |index| {
        if let Some(window) = weak.upgrade() {
            let element = shared.borrow().rigs.get(index as usize).cloned();
            if let Some(element) = element {
                shared.borrow_mut().edit_index = Some(index as usize);
                window.set_form_title("Edit required element".into());
                window.set_form_thread_in(element.thread_in.clone().into());
                window.set_form_thread_out(element.thread_out.clone().into());
                window.set_form_length(format_length(element.length).into());
                window.set_form_error("".into());
                window.set_show_form_dialog(true);
            }
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_save_form(move |_force| {
        let Some(window) = weak.upgrade() else { return };
        let thread_in = window.get_form_thread_in().to_string();
        let thread_out = window.get_form_thread_out().to_string();
        let length_text = window.get_form_length().to_string();

        let length = match length_text.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                window.set_form_error("Enter a valid length, for example 5.0.".into());
                return;
            }
        };
        if let Err(message) = validate_thread(&thread_in)
            .and_then(|_| validate_thread(&thread_out))
            .and_then(|_| validate_length(length))
        {
            window.set_form_error(message.into());
            return;
        }

        let element = Element::new(thread_in, thread_out, length);
        let mut current = shared.borrow_mut();
        current.rigs = match current.edit_index {
            Some(index) => update_element(&current.rigs, index, element),
            None => add_element(&current.rigs, element),
        };
        current.edit_index = None;
        current.dirty = true;
        refresh(&window, &current);
        window.set_show_form_dialog(false);
        window.set_form_error("".into());
        set_status(&window, "Required element saved.", "success");
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_cancel_form(move || {
        if let Some(window) = weak.upgrade() {
            shared.borrow_mut().edit_index = None;
            window.set_show_form_dialog(false);
            window.set_form_error("".into());
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_confirm_delete(move || {
        let Some(window) = weak.upgrade() else { return };
        let index = window.get_delete_index() as usize;
        let mut current = shared.borrow_mut();
        if index < current.rigs.len() {
            current.rigs = delete_element(&current.rigs, index);
            current.dirty = true;
            refresh(&window, &current);
            set_status(&window, "Required element deleted.", "success");
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
    window.on_move_up(move |index| {
        let Some(window) = weak.upgrade() else { return };
        let mut current = shared.borrow_mut();
        current.rigs = move_up(&current.rigs, index as usize);
        current.dirty = true;
        refresh(&window, &current);
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_move_down(move |index| {
        let Some(window) = weak.upgrade() else { return };
        let mut current = shared.borrow_mut();
        current.rigs = move_down(&current.rigs, index as usize);
        current.dirty = true;
        refresh(&window, &current);
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_save_file(move || {
        let Some(window) = weak.upgrade() else { return };
        let path = window.get_file_path().to_string();
        let current = shared.borrow();
        match save_elements(&path, &current.rigs) {
            Ok(()) => {
                drop(current);
                shared.borrow_mut().dirty = false;
                refresh(&window, &shared.borrow());
                set_status(
                    &window,
                    &format!("Saved {} elements to {}.", window.get_file_path(), path),
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
            Ok(rigs) => {
                let mut current = shared.borrow_mut();
                current.rigs = rigs;
                current.dirty = false;
                let count = current.rigs.len();
                refresh(&window, &current);
                set_status(
                    &window,
                    &format!("Loaded {count} elements from {path}."),
                    "success",
                );
            }
            Err(error) => set_status(&window, &error, "error"),
        }
    });

    let weak = window.as_weak();
    let shared = state.clone();
    window.on_find_chain(move || {
        let Some(window) = weak.upgrade() else { return };
        let spacers_path = window.get_spacers_path().to_string();
        let start = window.get_start_thread().to_string();
        let end = window.get_end_thread().to_string();
        let target_text = window.get_target_length().to_string();

        let result_message = match validate_thread(&start).and_then(|_| validate_thread(&end)) {
            Err(message) => {
                clear_result(&window);
                format!("Invalid thread: {message}")
            }
            Ok(()) => match target_text.parse::<f64>() {
                Err(_) => {
                    clear_result(&window);
                    "Enter a valid target length, for example 8.0.".to_string()
                }
                Ok(target) => match validate_length(target) {
                    Err(message) => {
                        clear_result(&window);
                        message
                    }
                    Ok(()) => match load_elements(&spacers_path) {
                        Err(message) => {
                            clear_result(&window);
                            message
                        }
                        Ok(spacers) => {
                            let current = shared.borrow();
                            match find_minimal_chain(&spacers, &current.rigs, &start, &end, target) {
                                Some(chain) => {
                                    let steps: Vec<UiStep> = chain
                                        .steps
                                        .iter()
                                        .enumerate()
                                        .map(|(index, step)| UiStep {
                                            number: (index + 1) as i32,
                                            thread_in: step.element.thread_in.clone().into(),
                                            thread_out: step.element.thread_out.clone().into(),
                                            length: step.element.length as f32,
                                            cumulative: step.cumulative_length as f32,
                                            required: step.required,
                                        })
                                        .collect();
                                    let summary = format!(
                                        "Found {} steps · total {} mm · {} required",
                                        steps.len(),
                                        format_length(chain.total_length),
                                        chain.required_count()
                                    );
                                    window.set_result_steps(slint::ModelRc::new(slint::VecModel::from(steps)));
                                    window.set_result_visible(true);
                                    window.set_result_kind("success".into());
                                    summary
                                }
                                None => {
                                    clear_result(&window);
                                    window.set_result_visible(true);
                                    window.set_result_kind("error".into());
                                    format!(
                                        "No chain from {} to {} totaling {} mm with all required elements. Threads connect when sizes match and genders are opposite; try flipping an endpoint.",
                                        start,
                                        end,
                                        format_length(target)
                                    )
                                }
                            }
                        }
                    },
                },
            },
        };
        window.set_result_summary(result_message.into());
    });

    let weak = window.as_weak();
    window.on_clear_status(move || {
        if let Some(window) = weak.upgrade() {
            window.set_status_message("".into());
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Rc::new(RefCell::new(State::default()));
    let window = MainWindow::new()?;

    {
        let mut current = state.borrow_mut();
        current.rigs = load_elements("must_have.json").unwrap_or_default();
        window.set_file_path("must_have.json".into());
        refresh(&window, &current);
    }
    wire_callbacks(&window, state);

    window.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::Model;

    #[test]
    fn find_chain_callback_reports_a_match() {
        // A GUI test needs a platform window; skip when running headless.
        let Ok(window) = MainWindow::new() else {
            return;
        };
        wire_callbacks(&window, Rc::new(RefCell::new(State::default())));
        let path = std::env::temp_dir().join("spacerfinder-test-spacers.json");
        std::fs::write(
            &path,
            r#"[{"thread_in":"M42M","thread_out":"M42F","length":2.0}]"#,
        )
        .unwrap();
        window.set_spacers_path(path.to_string_lossy().to_string().into());
        window.set_start_thread("M42F".into());
        window.set_end_thread("M42F".into());
        window.set_target_length("2.0".into());
        window.invoke_find_chain();
        assert!(
            window.get_result_summary().contains("Found 1 steps"),
            "summary: {}",
            window.get_result_summary()
        );
        assert_eq!(window.get_result_steps().row_count(), 1);

        // Also accept the common convention where start/end name the
        // first/last spacer's own threads.
        window.set_start_thread("M42M".into());
        window.set_end_thread("M42F".into());
        window.invoke_find_chain();
        assert!(
            window.get_result_summary().contains("Found 1 steps"),
            "summary: {}",
            window.get_result_summary()
        );
    }
}
