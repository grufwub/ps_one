extern crate users;
extern crate termion;

use std::env;
use std::process;
use termion::{color, style};

fn generate_ps1() -> Result<String, String> {
    let username = match users::get_current_username() {
        Some(name) => name,
        None => return Err(String::from("Failed to get current user")),
    };
    let username_str = username.to_str().unwrap();

    let cur_dir = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => return Err(String::from("Failed to get current directory")),
    };
    let cur_dir_str = cur_dir.to_str().unwrap();

    let home_dir = match env::home_dir() {
        Some(hd) => hd,
        None => return Err(String::from("Failed to get user home directory")),
    };
    let home_dir_str = home_dir.to_str().unwrap();

    Ok(format!("{uname_style}{uname}{reset} @ {cwd_style}{cwd}{reset}\n--> ",
                uname_style  = format!("{}{}", style::Bold, color::Fg(color::LightGreen)),
                uname        = username_str,
                cwd_style    = format!("{}{}", style::Bold, color::Fg(color::LightBlue)),
                cwd          = &cur_dir_str.replace(home_dir_str, "~"),
                reset        = format!("{}{}", style::Reset, color::Fg(color::Reset))
            )
    )
}


fn main() {
    match generate_ps1() {
        Ok(ps1_str) => {
            print!("{}", ps1_str);
            process::exit(0);
        },

        Err(err_str) => {
            print!("{}\n", err_str);
            process::exit(1);
        },
    };
}