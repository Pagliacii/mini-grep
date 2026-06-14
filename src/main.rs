use clap::Parser;
use mini_grep::{SearchOptions, search};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The string to search for
    needle: String,
    /// The path to the file to search in
    path: String,
    /// Case insensitive search
    #[arg(short, long)]
    ignore_case: bool,
    /// Search recursively in directories
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();
    let options = SearchOptions::new(args.ignore_case, args.recursive);

    match search(args.needle.as_str(), args.path, options) {
        Ok(matches) => {
            for m in matches {
                println!(
                    "{}:{}: {}",
                    m.path().display(),
                    m.line_number(),
                    m.content()
                );
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
