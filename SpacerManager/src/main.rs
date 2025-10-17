use serde::{Deserialize, Serialize};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

slint::slint! {
    import { Button, ScrollView, ListView, LineEdit, GridBox } from "std-widgets.slint";

    struct UiSpacer {
        thread_in: string,
        thread_out: string,
        length: float,
    }

    export component MainWindow inherits Window {
        title: "Spacer Manager";
        width: 800px;
        height: 600px;

        in-out property <[UiSpacer]> spacers: [];
        callback add-spacer(UiSpacer);
        callback update-spacer(int, UiSpacer);
        callback delete-spacer(int);
        callback save-spacers();
        callback load-spacers();

        property <bool> show-add-dialog: false;
        property <bool> show-edit-dialog: false;

        VerticalLayout {
            HorizontalLayout {
                Button {
                    text: "Add Spacer";
                    clicked => {
                        show-add-dialog = true;
                    }
                }
                Button {
                    text: "Save";
                    clicked => {
                        save-spacers();
                    }
                }
                Button {
                    text: "Load";
                    clicked => {
                        load-spacers();
                    }
                }
            }

            ScrollView {
                ListView {
                    for spacer[index] in spacers: Rectangle {
                        height: 40px;
                        HorizontalLayout {
                            Text {
                                text: spacer.thread_in;
                                width: 80px;
                            }
                            Text {
                                text: "→";
                                width: 20px;
                            }
                            Text {
                                text: spacer.thread_out;
                                width: 80px;
                            }
                            Text {
                                text: @tr("{} mm", spacer.length);
                                width: 80px;
                            }
                            Button {
                                text: "Edit";
                                clicked => {
                                    edit-index = index;
                                    show-edit-dialog = true;
                                }
                            }
                            Button {
                                text: "Delete";
                                clicked => {
                                    delete-spacer(index);
                                }
                            }
                        }
                    }
                }
            }
        }

        property <int> edit-index: -1;

        Rectangle {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            width: 300px;
            height: 200px;
            background: #4d4d4d;
            border-radius: 8px;
            border-width: 1px;
            border-color: #cccccc;
            visible: show-add-dialog;

            VerticalLayout {
                padding: 20px;
                Text { text: "Add Spacer"; }
                GridBox {
                    Row {
                        Text { text: "Thread In:"; }
                        thread-in := LineEdit {}
                    }
                    Row {
                        Text { text: "Thread Out:"; }
                        thread-out := LineEdit {}
                    }
                    Row {
                        Text { text: "Length:"; }
                        length := LineEdit {
                            input-type: decimal;
                        }
                    }
                }
                HorizontalLayout {
                    Button {
                        text: "Cancel";
                        clicked => {
                            show-add-dialog = false;
                        }
                    }
                    Button {
                        text: "Add";
                        clicked => {
                            if thread-in.text != "" && thread-out.text != "" && length.text != "" {
                                add-spacer({
                                    thread_in: thread-in.text,
                                    thread_out: thread-out.text,
                                    length: length.text.to-float(),
                                });
                                thread-in.text = "";
                                thread-out.text = "";
                                length.text = "";
                                show-add-dialog = false;
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            width: 300px;
            height: 200px;
            background: #f0f0f0;
            border-radius: 8px;
            border-width: 1px;
            border-color: #cccccc;
            visible: show-edit-dialog;

            VerticalLayout {
                padding: 20px;
                Text { text: "Edit Spacer"; }
                GridBox {
                    Row {
                        Text { text: "Thread In:"; }
                        edit-thread-in := LineEdit {}
                    }
                    Row {
                        Text { text: "Thread Out:"; }
                        edit-thread-out := LineEdit {}
                    }
                    Row {
                        Text { text: "Length:"; }
                        edit-length := LineEdit {
                            input-type: decimal;
                        }
                    }
                }
                HorizontalLayout {
                    Button {
                        text: "Cancel";
                        clicked => {
                            show-edit-dialog = false;
                        }
                    }
                    Button {
                        text: "Update";
                        clicked => {
                            if edit-thread-in.text != "" && edit-thread-out.text != "" && edit-length.text != "" && edit-index >= 0 {
                                update-spacer(edit-index, {
                                    thread_in: edit-thread-in.text,
                                    thread_out: edit-thread-out.text,
                                    length: edit-length.text.to-float(),
                                });
                                show-edit-dialog = false;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Spacer {
    thread_in: String,
    thread_out: String,
    length: f64,
}

// Pure functions for spacer operations
fn add_spacer(spacers: &[Spacer], new_spacer: Spacer) -> Vec<Spacer> {
    let mut result = spacers.to_vec();
    result.push(new_spacer);
    result
}

fn update_spacer(spacers: &[Spacer], index: usize, updated_spacer: Spacer) -> Vec<Spacer> {
    spacers.iter().enumerate().map(|(i, spacer)| {
        if i == index {
            updated_spacer.clone()
        } else {
            spacer.clone()
        }
    }).collect()
}

fn delete_spacer(spacers: &[Spacer], index: usize) -> Vec<Spacer> {
    spacers.iter().enumerate().filter_map(|(i, spacer)| {
        if i != index {
            Some(spacer.clone())
        } else {
            None
        }
    }).collect()
}

fn load_spacers_from_file() -> Result<Vec<Spacer>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("spacers.json")?;
    let spacers: Vec<Spacer> = serde_json::from_str(&data)?;
    Ok(spacers)
}

fn save_spacers_to_file(spacers: &[Spacer]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(spacers)?;
    fs::write("spacers.json", data)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spacers = Rc::new(RefCell::new(load_spacers_from_file().unwrap_or_else(|_| Vec::new())));

    let window = MainWindow::new().unwrap();

    // Set initial spacers
    {
        let current_spacers = spacers.borrow().clone();
        let slint_spacers: Vec<UiSpacer> = current_spacers.iter().map(|s| {
            UiSpacer {
                thread_in: s.thread_in.clone().into(),
                thread_out: s.thread_out.clone().into(),
                length: s.length as f32,
            }
        }).collect();
        window.set_spacers(slint::ModelRc::new(slint::VecModel::from(slint_spacers)));
    }

    // Add spacer callback
    let spacers_clone = spacers.clone();
    let window_weak = window.as_weak();
    window.on_add_spacer(move |spacer| {
        let new_spacer = Spacer {
            thread_in: spacer.thread_in.to_string(),
            thread_out: spacer.thread_out.to_string(),
            length: spacer.length as f64,
        };
        let updated_spacers = add_spacer(&spacers_clone.borrow(), new_spacer);
        *spacers_clone.borrow_mut() = updated_spacers.clone();

        let slint_spacers: Vec<UiSpacer> = updated_spacers.iter().map(|s| {
            UiSpacer {
                thread_in: s.thread_in.clone().into(),
                thread_out: s.thread_out.clone().into(),
                length: s.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak.upgrade() {
            window.set_spacers(slint::ModelRc::new(slint::VecModel::from(slint_spacers)));
        }
    });

    // Update spacer callback
    let spacers_clone = spacers.clone();
    let window_weak_update = window.as_weak();
    window.on_update_spacer(move |index, spacer| {
        let updated_spacer = Spacer {
            thread_in: spacer.thread_in.to_string(),
            thread_out: spacer.thread_out.to_string(),
            length: spacer.length as f64,
        };
        let updated_spacers = update_spacer(&spacers_clone.borrow(), index as usize, updated_spacer);
        *spacers_clone.borrow_mut() = updated_spacers.clone();

        let slint_spacers: Vec<UiSpacer> = updated_spacers.iter().map(|s| {
            UiSpacer {
                thread_in: s.thread_in.clone().into(),
                thread_out: s.thread_out.clone().into(),
                length: s.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_update.upgrade() {
            window.set_spacers(slint::ModelRc::new(slint::VecModel::from(slint_spacers)));
        }
    });

    // Delete spacer callback
    let spacers_clone = spacers.clone();
    let window_weak_delete = window.as_weak();
    window.on_delete_spacer(move |index| {
        let updated_spacers = delete_spacer(&spacers_clone.borrow(), index as usize);
        *spacers_clone.borrow_mut() = updated_spacers.clone();

        let slint_spacers: Vec<UiSpacer> = updated_spacers.iter().map(|s| {
            UiSpacer {
                thread_in: s.thread_in.clone().into(),
                thread_out: s.thread_out.clone().into(),
                length: s.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_delete.upgrade() {
            window.set_spacers(slint::ModelRc::new(slint::VecModel::from(slint_spacers)));
        }
    });

    // Save spacers callback
    let spacers_clone = spacers.clone();
    window.on_save_spacers(move || {
        let _ = save_spacers_to_file(&spacers_clone.borrow());
    });

    // Load spacers callback
    let spacers_clone = spacers.clone();
    let window_weak_load = window.as_weak();
    window.on_load_spacers(move || {
        if let Ok(loaded_spacers) = load_spacers_from_file() {
            *spacers_clone.borrow_mut() = loaded_spacers.clone();

            let slint_spacers: Vec<UiSpacer> = loaded_spacers.iter().map(|s| {
                UiSpacer {
                    thread_in: s.thread_in.clone().into(),
                    thread_out: s.thread_out.clone().into(),
                    length: s.length as f32,
                }
            }).collect();

            if let Some(window) = window_weak_load.upgrade() {
                window.set_spacers(slint::ModelRc::new(slint::VecModel::from(slint_spacers)));
            }
        }
    });

    window.run().unwrap();

    Ok(())
}
