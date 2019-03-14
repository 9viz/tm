use std::{env, fs, process::exit, path::Path};

use tm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut verbose: bool = false;
    let mut colors_path = Path::new(&"");

    let usage = || {
        println!("usage: tm /path/to/colors/file [-v|-h]");
    };

    if args.len() == 1 {
        eprintln!("error: not enough arguments!");
        usage();
        exit(5);
    }

    for i in 1..args.len() {
        match &args[i].as_str() {
            &"-v" => verbose = true,
            &"-h" => { usage(); exit(0); },
            _ => colors_path = Path::new(&args[i]),
        }
    }

    // get the absolue path
    let colors_path = fs::canonicalize(&colors_path)
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            usage();
            exit(7);
        });

    // convert it to &str again
    let colors_path = colors_path.to_str().unwrap();
    tm::run(colors_path, &verbose);
}
