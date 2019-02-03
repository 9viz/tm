use std::fs;
use std::process;

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
pub fn write_to_template(output_file_path: &str,
                         template_file_path: &str,
                         colors_path: &str
) {
    /*
     * this function takes the templates and subsitutes
     * Xn with the nth color and writes to the file
     */

    let mut output = String::new();
    let mut color_indices: Vec<String> = Vec::new();
    let colors: Vec<String> = read_colors(&colors_path);

    let template = fs::read_to_string(&template_file_path)
        .unwrap_or_else(|err| { println!("error: {}", err); process::exit(1) });

    let template: Vec<&str> = template.lines().collect();

    for line in template.iter() {
        let mut line = String::from(*line);

        if line.contains("X") {
            color_indices = get_color_num(&line);
        }

        if color_indices.len() == 0 {
            output.push_str(&line); output.push_str(&"\n");
            continue;
        }

        for color_index in color_indices.iter() {
            let re_str = format!("X{}", color_index);
            let color_index: usize = color_index.parse().unwrap();
            let color = colors.get(color_index).unwrap();

            line = line.replace(&re_str, &color);
        }

        output.push_str(&line);
        output.push_str(&"\n");
    }

    fs::write(&output_file_path, &output).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let colors = "/home/viz/etc/colors/viking";
        write_to_template(&"/home/viz/test", &"/home/viz/etc/tm/templates/dunstrc", &colors);
    }
}
