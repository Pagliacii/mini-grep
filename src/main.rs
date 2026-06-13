use mini_grep::search;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <needle> <path>", args[0]);
        std::process::exit(1);
    }

    let needle = &args[1];
    let path = &args[2];

    match search(needle, path) {
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
