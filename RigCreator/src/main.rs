use serde::{Deserialize, Serialize};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

slint::slint! {
    import { Button, ScrollView, ListView, LineEdit, GridBox } from "std-widgets.slint";

    struct UiRig {
        thread_in: string,
        thread_out: string,
        length: float,
    }

    export component MainWindow inherits Window {
        title: "Rig Creator";
        width: 1000px;
        height: 700px;

        in-out property <[UiRig]> rigs: [];
        callback add-rig(UiRig);
        callback update-rig(int, UiRig);
        callback delete-rig(int);
        callback move-rig-up(int);
        callback move-rig-down(int);
        callback save-rigs();
        callback load-rigs();
        callback find-chain();

        property <bool> show-add-dialog: false;
        property <bool> show-edit-dialog: false;

        // Chain finding properties
        in-out property <string> spacers-path: "spacers.json";
        in-out property <string> start-thread: "";
        in-out property <string> end-thread: "";
        in-out property <string> target-length: "";
        in-out property <string> chain-result: "Enter parameters and click 'Find Chain' to test your rig configuration.";

        VerticalLayout {
            // Rig management section
            Text { text: "Rig Management"; font-weight: 600; }
            HorizontalLayout {
                Button {
                    text: "Add Rig";
                    clicked => {
                        show-add-dialog = true;
                    }
                }
                Button {
                    text: "Save";
                    clicked => {
                        save-rigs();
                    }
                }
                Button {
                    text: "Load";
                    clicked => {
                        load-rigs();
                    }
                }
            }

            ScrollView {
                min-height: 200px;
                ListView {
                    for rig[index] in rigs: Rectangle {
                        height: 40px;
                        HorizontalLayout {
                            Text {
                                text: @tr("{}.", index + 1);
                                width: 30px;
                                font-weight: 600;
                            }
                            Text {
                                text: rig.thread_in;
                                width: 80px;
                            }
                            Text {
                                text: "→";
                                width: 20px;
                            }
                            Text {
                                text: rig.thread_out;
                                width: 80px;
                            }
                            Text {
                                text: @tr("{} mm", rig.length);
                                width: 80px;
                            }
                            Button {
                                text: "↑";
                                enabled: index > 0;
                                clicked => {
                                    move-rig-up(index);
                                }
                            }
                            Button {
                                text: "↓";
                                enabled: index < rigs.length - 1;
                                clicked => {
                                    move-rig-down(index);
                                }
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
                                    delete-rig(index);
                                }
                            }
                        }
                    }
                }
            }

            // Chain finding section
            Rectangle {
                border-width: 1px;
                border-color: #cccccc;
                border-radius: 4px;

                VerticalLayout {
                    padding: 10px;
                    Text { text: "Test Chain Finding"; font-weight: 600; }

                    GridBox {
                        Row {
                            Text { text: "Spacers File:"; }
                            spacers-file := LineEdit {
                                text: spacers-path;
                                edited => { spacers-path = spacers-file.text; }
                            }
                        }
                        Row {
                            Text { text: "Start Thread:"; }
                            start-thread-input := LineEdit {
                                text: start-thread;
                                edited => { start-thread = start-thread-input.text; }
                            }
                        }
                        Row {
                            Text { text: "End Thread:"; }
                            end-thread-input := LineEdit {
                                text: end-thread;
                                edited => { end-thread = end-thread-input.text; }
                            }
                        }
                        Row {
                            Text { text: "Target Length:"; }
                            target-length-input := LineEdit {
                                text: target-length;
                                input-type: decimal;
                                edited => { target-length = target-length-input.text; }
                            }
                        }
                    }

                    Button {
                        text: "Find Chain";
                        clicked => {
                            find-chain();
                        }
                    }

                    ScrollView {
                        min-height: 100px;
                        Text {
                            text: chain-result;
                            wrap: word-wrap;
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
                Text { text: "Add Rig"; }
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
                                add-rig({
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
            background: #4d4d4d;
            border-radius: 8px;
            border-width: 1px;
            border-color: #cccccc;
            visible: show-edit-dialog;

            VerticalLayout {
                padding: 20px;
                Text { text: "Edit Rig"; }
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
                                update-rig(edit-index, {
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
struct Rig {
    thread_in: String,
    thread_out: String,
    length: f64,
}

// Pure functions for rig operations
fn add_rig(rigs: &[Rig], new_rig: Rig) -> Vec<Rig> {
    let mut result = rigs.to_vec();
    result.push(new_rig);
    result
}

fn update_rig(rigs: &[Rig], index: usize, updated_rig: Rig) -> Vec<Rig> {
    rigs.iter().enumerate().map(|(i, rig)| {
        if i == index {
            updated_rig.clone()
        } else {
            rig.clone()
        }
    }).collect()
}

fn delete_rig(rigs: &[Rig], index: usize) -> Vec<Rig> {
    rigs.iter().enumerate().filter_map(|(i, rig)| {
        if i != index {
            Some(rig.clone())
        } else {
            None
        }
    }).collect()
}

fn move_rig_up(rigs: &[Rig], index: usize) -> Vec<Rig> {
    if index == 0 || index >= rigs.len() {
        return rigs.to_vec();
    }
    let mut result = rigs.to_vec();
    result.swap(index - 1, index);
    result
}

fn move_rig_down(rigs: &[Rig], index: usize) -> Vec<Rig> {
    if index >= rigs.len() - 1 {
        return rigs.to_vec();
    }
    let mut result = rigs.to_vec();
    result.swap(index, index + 1);
    result
}

fn load_rigs_from_file() -> Result<Vec<Rig>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("must_have.json")?;
    let rigs: Vec<Rig> = serde_json::from_str(&data)?;
    Ok(rigs)
}

fn save_rigs_to_file(rigs: &[Rig]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(rigs)?;
    fs::write("must_have.json", data)?;
    Ok(())
}

// Spacer chain finding functions
fn can_connect(out_thread: &str, in_thread: &str) -> bool {
    if out_thread.len() < 2 || in_thread.len() < 2 {
        return false;
    }
    let out_gender = out_thread.chars().last().unwrap();
    let in_gender = in_thread.chars().last().unwrap();
    let out_size = &out_thread[..out_thread.len() - 1];
    let in_size = &in_thread[..in_thread.len() - 1];
    out_size == in_size && out_gender != in_gender
}

fn spacer_eq(a: &Rig, b: &Rig) -> bool {
    a.thread_in == b.thread_in && a.thread_out == b.thread_out && (a.length - b.length).abs() < 1e-6
}

fn find_minimal_chain<'a>(spacers: &'a [Rig], must_have: &'a [Rig], start: &str, end: &str, target: f64) -> Option<Vec<&'a Rig>> {
    let mut valid_chains = Vec::new();
    find_chains(spacers, must_have, 0, start, end, 0.0, Vec::new(), target, &mut valid_chains);
    valid_chains.into_iter().min_by_key(|chain| chain.len())
}

fn find_chains<'a>(
    spacers: &'a [Rig],
    must_have: &'a [Rig],
    must_index: usize,
    current_thread: &str,
    end: &str,
    current_sum: f64,
    current_chain: Vec<&'a Rig>,
    target: f64,
    valid_chains: &mut Vec<Vec<&'a Rig>>,
) {
    if current_chain.len() >= 10 {
        return;
    }
    for spacer in spacers {
        if can_connect(current_thread, &spacer.thread_in) {
            let new_sum = current_sum + spacer.length;
            if new_sum > target + 1e-6 {
                continue;
            }
            let mut new_chain = current_chain.clone();
            new_chain.push(spacer);
            let new_index = if must_index < must_have.len() && spacer_eq(spacer, &must_have[must_index]) {
                must_index + 1
            } else {
                must_index
            };
            if (new_sum - target).abs() < 1e-6 && spacer.thread_out == end && new_index == must_have.len() {
                valid_chains.push(new_chain.clone());
            }
            find_chains(spacers, must_have, new_index, &spacer.thread_out, end, new_sum, new_chain, target, valid_chains);
        }
    }
}

fn load_spacers_from_file(path: &str) -> Result<Vec<Rig>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let spacers: Vec<Rig> = serde_json::from_str(&data)?;
    Ok(spacers)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rigs = Rc::new(RefCell::new(load_rigs_from_file().unwrap_or_else(|_| Vec::new())));

    let window = MainWindow::new().unwrap();

    // Set initial rigs
    {
        let current_rigs = rigs.borrow().clone();
        let slint_rigs: Vec<UiRig> = current_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();
        window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
    }

    // Add rig callback
    let rigs_clone = rigs.clone();
    let window_weak = window.as_weak();
    window.on_add_rig(move |rig| {
        let new_rig = Rig {
            thread_in: rig.thread_in.to_string(),
            thread_out: rig.thread_out.to_string(),
            length: rig.length as f64,
        };
        let updated_rigs = add_rig(&rigs_clone.borrow(), new_rig);
        *rigs_clone.borrow_mut() = updated_rigs.clone();

        let slint_rigs: Vec<UiRig> = updated_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak.upgrade() {
            window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
        }
    });

    // Update rig callback
    let rigs_clone = rigs.clone();
    let window_weak_update = window.as_weak();
    window.on_update_rig(move |index, rig| {
        let updated_rig = Rig {
            thread_in: rig.thread_in.to_string(),
            thread_out: rig.thread_out.to_string(),
            length: rig.length as f64,
        };
        let updated_rigs = update_rig(&rigs_clone.borrow(), index as usize, updated_rig);
        *rigs_clone.borrow_mut() = updated_rigs.clone();

        let slint_rigs: Vec<UiRig> = updated_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_update.upgrade() {
            window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
        }
    });

    // Delete rig callback
    let rigs_clone = rigs.clone();
    let window_weak_delete = window.as_weak();
    window.on_delete_rig(move |index| {
        let updated_rigs = delete_rig(&rigs_clone.borrow(), index as usize);
        *rigs_clone.borrow_mut() = updated_rigs.clone();

        let slint_rigs: Vec<UiRig> = updated_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_delete.upgrade() {
            window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
        }
    });

    // Move rig up callback
    let rigs_clone = rigs.clone();
    let window_weak_up = window.as_weak();
    window.on_move_rig_up(move |index| {
        let updated_rigs = move_rig_up(&rigs_clone.borrow(), index as usize);
        *rigs_clone.borrow_mut() = updated_rigs.clone();

        let slint_rigs: Vec<UiRig> = updated_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_up.upgrade() {
            window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
        }
    });

    // Move rig down callback
    let rigs_clone = rigs.clone();
    let window_weak_down = window.as_weak();
    window.on_move_rig_down(move |index| {
        let updated_rigs = move_rig_down(&rigs_clone.borrow(), index as usize);
        *rigs_clone.borrow_mut() = updated_rigs.clone();

        let slint_rigs: Vec<UiRig> = updated_rigs.iter().map(|r| {
            UiRig {
                thread_in: r.thread_in.clone().into(),
                thread_out: r.thread_out.clone().into(),
                length: r.length as f32,
            }
        }).collect();

        if let Some(window) = window_weak_down.upgrade() {
            window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
        }
    });

    // Save rigs callback
    let rigs_clone = rigs.clone();
    window.on_save_rigs(move || {
        let _ = save_rigs_to_file(&rigs_clone.borrow());
    });

    // Load rigs callback
    let rigs_clone = rigs.clone();
    let window_weak_load = window.as_weak();
    window.on_load_rigs(move || {
        if let Ok(loaded_rigs) = load_rigs_from_file() {
            *rigs_clone.borrow_mut() = loaded_rigs.clone();

            let slint_rigs: Vec<UiRig> = loaded_rigs.iter().map(|r| {
                UiRig {
                    thread_in: r.thread_in.clone().into(),
                    thread_out: r.thread_out.clone().into(),
                    length: r.length as f32,
                }
            }).collect();

            if let Some(window) = window_weak_load.upgrade() {
                window.set_rigs(slint::ModelRc::new(slint::VecModel::from(slint_rigs)));
            }
        }
    });

    // Find chain callback
    let rigs_clone = rigs.clone();
    let window_weak_chain = window.as_weak();
    window.on_find_chain(move || {
        let spacers_path = window_weak_chain.upgrade().unwrap().get_spacers_path().to_string();
        let start_thread = window_weak_chain.upgrade().unwrap().get_start_thread().to_string();
        let end_thread = window_weak_chain.upgrade().unwrap().get_end_thread().to_string();
        let target_length_str = window_weak_chain.upgrade().unwrap().get_target_length().to_string();

        let result = match (load_spacers_from_file(&spacers_path), target_length_str.parse::<f64>()) {
            (Ok(spacers), Ok(target_length)) => {
                match find_minimal_chain(&spacers, &rigs_clone.borrow(), &start_thread, &end_thread, target_length) {
                    Some(chain) => {
                        let mut result = format!("Found chain with {} elements:\n", chain.len());
                        for element in chain {
                            result.push_str(&format!("Thread in: {}, Thread out: {}, Length: {}\n",
                                                   element.thread_in, element.thread_out, element.length));
                        }
                        result
                    }
                    None => format!("No chain found from {} to {} that sums to {} with required elements",
                                  start_thread, end_thread, target_length)
                }
            }
            (Err(e), _) => format!("Error loading spacers file '{}': {}", spacers_path, e),
            (_, Err(_)) => "Error: Invalid target length. Please enter a valid number.".to_string()
        };

        if let Some(window) = window_weak_chain.upgrade() {
            window.set_chain_result(result.into());
        }
    });

    window.run().unwrap();

    Ok(())
}
