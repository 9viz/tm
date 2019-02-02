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
    let mut prev_index = 1000;

    for m in matches.iter() {
        // get the index number
        let ind = m.0;

        // get the matching character
        let mut ch = String::from(m.1);

        // get the previous char is an X
        let prev_ch = line.get(ind-1..ind).unwrap_or("");

        // check if the previous char is an X
        // if it is not, skip the iteration
        if prev_ch != "X" {
            continue;
        }

        // get the next char and see if it is numeric
        let next_ch = line.get(ind+1..ind+2).unwrap_or("");
        let next_ch: Vec<_> = next_ch.matches(char::is_numeric).collect();

        // if it is numeric, then set the approriate prev_index and skip
        // the iteration
        if next_ch.len() > 0 {
            prev_index = ind;
            continue;
        }

        // check if the previous index is the one we stored
        // if it is, then it means we have to prepend it
        if ind-1 == prev_index {
            ch = String::from(format!("{}{}", line.get(prev_index..ind).unwrap(), ch));
        }

        // update the prev_index
        prev_index = ind;

        result.push(String::from(ch));
    }

    return result
}
