use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Spacer {
    thread_in: String,
    thread_out: String,
    length: f64,
}

#[derive(Parser)]
struct Args {
    json_path: String,
    must_have_json_path: String,
    start_thread: String,
    end_thread: String,
    total_length: f64,
}

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

fn spacer_eq(a: &Spacer, b: &Spacer) -> bool {
    a.thread_in == b.thread_in && a.thread_out == b.thread_out && (a.length - b.length).abs() < 1e-6
}

fn find_minimal_chain<'a>(spacers: &'a [Spacer], must_have: &'a [Spacer], start: &str, end: &str, target: f64) -> Option<Vec<&'a Spacer>> {
    let mut valid_chains = Vec::new();
    find_chains(spacers, must_have, 0, start, end, 0.0, Vec::new(), target, &mut valid_chains);
    valid_chains.into_iter().min_by_key(|chain| chain.len())
}

fn find_chains<'a>(
    spacers: &'a [Spacer],
    must_have: &'a [Spacer],
    must_index: usize,
    current_thread: &str,
    end: &str,
    current_sum: f64,
    current_chain: Vec<&'a Spacer>,
    target: f64,
    valid_chains: &mut Vec<Vec<&'a Spacer>>,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let data = fs::read_to_string(&args.json_path)?;
    let spacers: Vec<Spacer> = serde_json::from_str(&data)?;
    let must_data = fs::read_to_string(&args.must_have_json_path)?;
    let must_have: Vec<Spacer> = serde_json::from_str(&must_data)?;
    match find_minimal_chain(&spacers, &must_have, &args.start_thread, &args.end_thread, args.total_length) {
        Some(chain) => {
            println!("Found chain with {} elements:", chain.len());
            for element in chain {
                println!("Thread in: {}, Thread out: {}, Length: {}", element.thread_in, element.thread_out, element.length);
            }
        }
        None => println!("No chain found from {} to {} that sums to {} with required elements", args.start_thread, args.end_thread, args.total_length),
    }
    Ok(())
}
