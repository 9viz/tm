mod template;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use template::*;

    #[test]
    fn write() {
        let colors = "/home/viz/etc/colors/viking";
        write_to_template(&"/home/viz/test", &"/home/viz/etc/tm/templates/dunstrc", &colors);
    }
}
