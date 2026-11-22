use clap::Parser;
use spacer_core::{find_minimal_chain, load_elements};

#[derive(Parser)]
#[command(
    name = "spacer_finder",
    about = "Find the shortest spacer chain matching an exact total length",
    long_about = "Finds the shortest chain of spacers that connects the start thread to the \
                  end thread, sums to the requested total length (mm), and includes every \
                  required element from must_have.json in order."
)]
struct Args {
    /// JSON file with the available spacers
    json_path: String,
    /// JSON file with the required (must-have) elements
    must_have_json_path: String,
    /// Starting thread specification, e.g. M54M
    start_thread: String,
    /// Ending thread specification, e.g. M54F
    end_thread: String,
    /// Exact target length in millimeters
    total_length: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let spacers = load_elements(&args.json_path)?;
    let must_have = load_elements(&args.must_have_json_path)?;

    match find_minimal_chain(
        &spacers,
        &must_have,
        &args.start_thread,
        &args.end_thread,
        args.total_length,
    ) {
        Some(result) => println!("{}", result.render()),
        None => println!(
            "No chain found from {} to {} that sums to {} mm with the required elements.",
            args.start_thread, args.end_thread, args.total_length
        ),
    }
    Ok(())
}
