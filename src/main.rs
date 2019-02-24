use std::{env, fs, process, path::Path};

use tm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut verbose: bool = false;
    let mut colors_path = Path::new(&"");

    let usage = || {
        println!("usage: tm /path/to/colors/file [-v|-h]");
    };

    if args.len() == 1 {
        println!("error: not enough arguments!");
        usage();
        process::exit(5);
    }

    for i in 1..args.len() {
        if args[i] == "-v" { verbose=true; }
        else if args[i] == "-h" { usage(); process::exit(0); }
        else { colors_path = Path::new(&args[i]); }
    }

    // get the absolue path
    let colors_path = fs::canonicalize(&colors_path)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            usage();
            process::exit(7);
        });

    // convert it to &str again
    let colors_path = colors_path.to_str().unwrap();
    tm::run(colors_path, &verbose);
}
