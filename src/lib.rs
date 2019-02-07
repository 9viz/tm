use std::fs;
use std::process;
use std::path::PathBuf;
use std::env;

fn read_colors(path: &str) -> Vec<String> {
    /*
     * read colors from a raw text file located at path
     * append the colors to a vector of String and return it
     */
    let mut colors: Vec<String> = Vec::new();

    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|err| {
            println!("error: {}", err);
            process::exit(0);
    });

    let contents: Vec<&str> = contents.lines().collect();

    for color in contents {
        colors.push(String::from(color));
    }

    return colors;
}

fn get_color_num(line: &String) -> Vec<String> {
    /*
     * in a template file, Xn indicates the nth color
     * which should be substituted. this function retuns
     * all the n in a line
     */

    // the vector which will be returned
    let mut result: Vec<String> = Vec::new();

    let matches: Vec<_> = line.match_indices(char::is_numeric).collect();

    // find a better to avoid edge cases
    let mut prev_index = 1;

    // check if a &str is numeric
    let is_numeric = |string: &str| {
        let nch: Vec<_> = string.matches(char::is_numeric).collect();

        if nch.len() == 0 { return false; }

        return true;
    };

    for m in matches.iter() {
        // get the index number
        let ind = m.0;

        // get the matching character
        let mut ch = String::from(m.1);

        // get the previous char
        let prev_ch = line.get(ind-1..ind).unwrap_or("");

        // check if the previous char is an X
        // and if the prev_ch is not a number
        // if it is not, skip the iteration
        if prev_ch != "X" && ! is_numeric(prev_ch) {
            continue;
        }

        // get the next char and see if it is numeric
        let next_ch = line.get(ind+1..ind+2).unwrap_or("");

        // if it is numeric, then set the approriate prev_index and skip
        // the iteration
        if  is_numeric(next_ch) {
            prev_index = ind;
            continue;
        }

        // if the ind-1 is prev_index, it means that line[prev_index] is numeric
        // then prepend it to ch
        if ind-1 == prev_index {
            ch = String::from(format!("{}{}", line.get(prev_index..ind).unwrap(), ch));
        }

        // update the prev_index
        prev_index = ind;

        result.push(String::from(ch));
    }

    return result;
}

#[allow(unused_must_use)]
fn write_to_template(output_file_path: &str,
                     template_file_path: &str,
                     colors_path: &str) {
    /*
     * this function takes the templates and subsitutes
     * Xn with the nth color and writes to the file
     */

    // variables needed
    let mut output = String::new();
    let mut color_indices: Vec<String> = Vec::new();

    // colors in a colors file
    let colors: Vec<String> = read_colors(&colors_path);

    // the template itself in a string
    let template = fs::read_to_string(&template_file_path)
        .unwrap_or_else(|err| { println!("error: {}", err); process::exit(6); });

    // split it with newline so it can be looped thru
    let template: Vec<&str> = template.lines().collect();

    for line in template.iter() {
        // change &str to String
        let mut line = String::from(*line);

        // if line contains X, get the color_index
        if line.contains("X") {
            color_indices = get_color_num(&line);
        }

        // iterate through color index and replace the approriate color
        for color_index in color_indices.iter() {
            let re_str = format!("X{}", color_index);
            let color_index: usize = color_index.parse().unwrap();
            let color = colors.get(color_index).unwrap();

            line = line.replace(&re_str, &color);
        }

        output.push_str(&line);
        output.push_str(&"\n");
    }

    // log
    println!("creating {} from {}", output_file_path, template_file_path);

    fs::write(&output_file_path, &output).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });
}

pub fn run(colors_path: &str) {
    /*
     * the heavy lifter. does everything needed to create
     * template files and completes the template after
     * the necessary stuff
     */

    // tm's template directory
    let template_dir = env::var("TM_TEMPLATE_DIR").unwrap_or_else(|_| {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", env::var("HOME").unwrap()));

        return format!("{}/tm/templates", config_dir);
    });

    let ptemplate_dir = PathBuf::from(&template_dir);

    // if the template directory doesn't exist, exit
    if ! ptemplate_dir.is_dir() {
        println!("error: tm template is non-existence! create one");
        process::exit(2);
    }

    // base cache directory
    let cache_dir = env::var("XDG_CACHE_HOME")
        .unwrap_or_else(|_| format!("{}/.cache", env::var("HOME").unwrap()));

    // get cache directory
    let template_cache_dir = format!("{}/tm/colors", cache_dir);
    let ptemplate_cache_dir = PathBuf::from(&template_cache_dir);

    // delete cache dir before proceeding
    if ptemplate_cache_dir.is_dir() {
        fs::remove_dir_all(&ptemplate_cache_dir)
            .unwrap_or_else(|e| {
                println!("error: {}", e);
                process::exit(4);
            });
    }

    // create the template cache dir
    fs::create_dir_all(&ptemplate_cache_dir)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            process::exit(2);
        });

    // can safely use unwrap() because the program exits if
    // this path doesn't exist
    for template in fs::read_dir(&ptemplate_dir).unwrap() {
        // convert DirEntry to Path
        let template = template.unwrap().path();

        // get file name
        let file_name = template.file_name().unwrap();
        // convert osstr to str
        let file_name = file_name.to_str().unwrap();

        // convert Path to &str
        let template = template.to_str().unwrap();

        let output_path = format!("{}/{}", template_cache_dir, file_name);

        write_to_template(&output_path, &template, colors_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testy() {
        run(&"/home/viz/etc/colors/viking");
    }
}
