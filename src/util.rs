use std::fs;
use std::path::PathBuf;
use std::env;
use std::process;

fn copy_template() {
    /*
     * copy files from tm's template dir to
     * $XDG_CACHE_HOME/tm/templates
     * fall back to ~/.cache if $XDG_CACHE_HOME is null
     *
     * tm's template dir can be...
     * TM_TEMPLATE_DIR -> an env var.
     * if that's not set,
     * $XDG_CONFIG_HOME/tm/template
     * if that's not set,
     * $HOME/.config/tm/template
     */

    // tm's template directory
    let template_dir = env::var("TM_TEMPLATE_DIR").unwrap_or_else(|_| {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .unwrap_or(format!("{}/.config", env::var("HOME").unwrap()));

        return format!("{}/tm/templates", config_dir);
    });

    let template_dir = PathBuf::from(&template_dir);

    // if the template directory doesn't exist, exit
    if ! template_dir.is_dir() {
        println!("error: tm template is non-existence! create one");
        process::exit(2);
    }

    // base cache directory
    let cache_dir = env::var("XDG_CACHE_HOME")
        .unwrap_or(format!("{}/.cache", env::var("HOME").unwrap()));

    // get cache directory
    let template_cache_dir = format!("{}/tm/templates", cache_dir);
    let template_cache_dir = PathBuf::from(&template_cache_dir);

    // delete cache dir before proceeding
    if template_cache_dir.is_dir() {
        fs::remove_dir_all(&template_cache_dir)
            .unwrap_or_else(|e| {
                println!("error: {}", e);
                process::exit(4);
            });
    }

    // create the template cache dir
    fs::create_dir_all(&template_cache_dir)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            process::exit(2);
        });


    for template in fs::read_dir(&template_dir).unwrap() {
        // convert DirEntry to PathBuf
        let template = template.unwrap().path();

        // make target directory
        let mut tmplc = PathBuf::from(&template_cache_dir);
        tmplc.push(template.file_name().unwrap());

        // actually copy the file
        fs::copy(&template, &tmplc)
            .unwrap_or_else(|e| {
                println!("error: {}", e);
                process::exit(3);
            });

        // log
        println!("copied {} to {}", template.display(), tmplc.display());
    }
}
