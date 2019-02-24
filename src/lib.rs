use std::fs;
use std::process;
use std::path::PathBuf;
use std::env;

fn read_colors(path: &str) -> Vec<String> {
    /*
     * read colors from a raw text file located at path
     * append the colors to a vector of String and return it
     *
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

fn get_color_num(lin: &String) -> Vec<String> {
    /*
     * in a template file, Xn indicates the nth color
     * which should be substituted. this function retuns
     * all the n in a lin
     *
     */


    // the vector which will be returned
    let mut result: Vec<String> = Vec::new();

    let matches: Vec<_> = lin.match_indices("X").collect();

    // check if a &str is numeric
    let is_numeric = |string: &str| {
        let nch: Vec<_> = string.matches(char::is_numeric).collect();

        if nch.len() == 0 { return false; }

        return true;
    };

    for m in matches.iter() {
        // the matching index of the char next to an X
        let ind = m.0 + 1;

        // the char next to X
        let ch = lin.get(ind..ind+1).unwrap();
        let mut ch = String::from(ch);

        // if the char next to X is not a number,
        // skip the current iteration
        if ! is_numeric(&ch) {
            continue;
        }

        // the char next to the char next to X
        let next_ch = lin.get(ind+1..ind+2).unwrap();

        if is_numeric(next_ch) {
            ch = format!("{}{}", ch, next_ch);
        }

        result.push(ch);
    }

    return result;
}

#[allow(unused_must_use)]
fn write_to_template(output_file_path: &str,
                     template_file_path: &str,
                     colors_path: &str,
                     verbose: &bool) {
    /*
     * this function takes the templates and subsitutes
     * Xn with the nth color and writes to the file
     *
     */

    // variables needed
    let mut output = String::new();
    let mut color_indices: Vec<String> = Vec::new();

    // colors in a colors file
    let colors: Vec<String> = read_colors(&colors_path);

    // the template itself in a string
    let template = fs::read_to_string(&template_file_path)
        .unwrap_or_else(|err| { println!("error: {}", err); process::exit(6); });

    // split it with newlin so it can be looped thru
    let template: Vec<&str> = template.lines().collect();

    for lin in template.iter() {
        // change &str to String
        let mut lin = String::from(*lin);

        // if lin contains X, get the color_index
        if lin.contains("X") {
            color_indices = get_color_num(&lin);
        }

        // iterate through color index and replace the approriate color
        for color_index in color_indices.iter() {
            let re_str = format!("X{}", color_index);
            let color_index: usize = color_index.parse().unwrap();

            // only support 15 colors
            if color_index > 15 { continue; }

            let color = colors.get(color_index).unwrap();

            lin = lin.replace(&re_str, &color);
        }

        output.push_str(&lin);
        output.push_str(&"\n");
    }

    // log

    if *verbose {
        println!("creating {} from {}", output_file_path, template_file_path);
    }

    fs::write(&output_file_path, &output).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });
}

pub fn run(colors_path: &str, verbose: &bool) {
    /*
     * the heavy lifter. does everything needed to create
     * template files and completes the template after
     * the necessary stuff
     *
     */

    // tm's template directory
    let template_dir = env::var("TM_TEMPLATE_DIR").unwrap_or_else(|_| {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", env::var("HOME").unwrap()));

        return format!("{}/tm/", config_dir);
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
    let template_cache_dir = format!("{}/tm", cache_dir);
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

        write_to_template(&output_path, &template, colors_path, verbose);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testy() {
        run(&"/home/viz/etc/colors/viking");
    }

    #[test]
    fn write_test() {
        write_to_template(&"./test", &"/home/viz/etc/tm/templates/colors_st.h", &"/home/viz/etc/colors/viking", &true);
    }
}
