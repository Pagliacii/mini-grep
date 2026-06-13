use clap::Parser;
use mini_grep::search;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The string to search for
    needle: String,
    /// The path to the file to search in
    path: String,
}

fn main() {
    let args = Args::parse();

    match search(args.needle.as_str(), args.path) {
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
