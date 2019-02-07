use std::{env, fs, process, path::Path};

use tm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("error: not enough arguments!");
        println!("usage: tm [/path/to/colors/file]");
        process::exit(5)
    }

    // convert the str to a Path
    let colors_path = Path::new(&args[1]);
    // get the absolue path
    let colors_path = fs::canonicalize(&colors_path)
        .unwrap_or_else(|e| { println!("error :{}", e); process::exit(7); });

    // convert it to string again
    let colors_path = colors_path.to_str().unwrap();
    tm::run(colors_path);
}
