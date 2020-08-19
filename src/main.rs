extern crate dirs;
extern crate git2;

use std::env;
use std::process;
use git2::{Repository, StatusOptions, StatusShow};

// Git status strings
const GIT_CLEAN: &'static str = "✓";
const GIT_DIRTY: &'static str = "✗";

// Terminal color + style constants
//const COLOR_BLACK:   &str = "\x1b[30m";
//const COLOR_RED:     &str = "\x1b[31m";
//const COLOR_GREEN:   &str = "\x1b[32m";
//const COLOR_YELLOW:  &str = "\x1b[33m";
//const COLOR_BLUE:    &str = "\x1b[34m";
//const COLOR_MAGENTA: &str = "\x1b[35m";
//const COLOR_CYAN:    &str = "\x1b[36m";
//const COLOR_WHITE:   &str = "\x1b[37m";
//const COLOR_BRIGHT_BLACK:   &str = "\x1b[30;1m";
//const COLOR_BRIGHT_YELLOW:  &str = "\x1b[33;1m";
//const COLOR_BRIGHT_MAGENTA: &str = "\x1b[35;1m";
//const COLOR_BRIGHT_CYAN:    &str = "\x1b[36;1m";
//const COLOR_BRIGHT_WHITE:   &str = "\x1b[37;1m";
//const STYLE_UNDERLINED: &str = "\x1b[4m";
//const STYLE_REVERSED:   &str = "\x1b[7m";

const COLOR_BRIGHT_RED:   &str = "\x1b[31;1m";
const COLOR_BRIGHT_GREEN: &str = "\x1b[32;1m";
const COLOR_BRIGHT_BLUE:  &str = "\x1b[34;1m";
const STYLE_BOLD:         &str = "\x1b[1m";
const RESET:              &str = "\x1b[0m";

// PS1 format username: {color}{style}{name}{reset}
macro_rules! format_name {
    ($name:expr) => {
        [COLOR_BRIGHT_GREEN, STYLE_BOLD, $name, RESET].join("")
    };
}

// PS1 format current working directory: {color}{style}{directory}{reset}
macro_rules! format_cwd {
    ($cwd:expr) => {
        [COLOR_BRIGHT_BLUE, STYLE_BOLD, $cwd, RESET].join("")
    };
}

// PS1 format git branch name: {color}{style}{[remote: ]?name}{reset}
macro_rules! format_branch {
    ($is_remote:expr, $name:expr) => {
        if $is_remote {
            [COLOR_BRIGHT_RED, STYLE_BOLD, "remote: ", $name, RESET].join("")
        } else {
            [COLOR_BRIGHT_RED, STYLE_BOLD, $name, RESET].join("")
        }
    };
}

// PS1 format git status string: {color}{style}{status_symbol}{reset}
macro_rules! format_status {
    ($dirty:expr) => {
        if $dirty == true {
            [COLOR_BRIGHT_RED, STYLE_BOLD, GIT_DIRTY, RESET].join("")
        } else {
            [COLOR_BRIGHT_GREEN, STYLE_BOLD, GIT_CLEAN, RESET].join("")
        }
    };
}

fn get_username<'a>() -> Result<String, &'a str> {
    // Get current user from $LOGNAME var
    match env::var("LOGNAME") {
        Ok(u) => Ok(u),
        Err(_) => return Err("Failed to get current user"),
    }
}

fn get_current_dir<'a>() -> Result<String, &'a str> {
    // Try to get current dir
    let current_dir = match env::current_dir() {
        Ok(c) => c,
        Err(_) => return Err("Failed to get current dir"),
    };

    // Try get string from current dir PathBuf
    if let Some(current_dir_str) = current_dir.to_str() {
        Ok(current_dir_str.to_string())
    } else {
        Err("Failed to get current dir")
    }
}

fn get_home_dir<'a>() -> Result<String, &'a str> {
    // Try get current user's home dir
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => return Err("Failed to get current user's home dir")
    };

    // Try get string from home dir PathBuf
    if let Some(home_dir_str) = home_dir.to_str() {
        Ok(home_dir_str.to_string())
    } else {
        Err("Failed to get current user's home dir")
    }
}

fn get_git_status(cwd_str: &str) -> String {
    // Try open the repo at current directory
    let repo = match Repository::discover(cwd_str) {
        Ok(r) => r,
        Err(_) => return "".to_string(),
    };

    // Try get the head from the repo
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return "".to_string(),
    };

    // Try get head name, else return empty
    let name_str = if let Some(name) = head.name() {
        name.replace("refs/heads/", "")
    } else {
        return "".to_string()
    };

    // Get the current repository state iterator
    let statuses = match repo.statuses(Some(StatusOptions::default().show(StatusShow::Workdir).include_untracked(true).include_ignored(false).no_refresh(true))) {
        Ok(s) => s,
        Err(_) => return "".to_string(),
    };

    // Are we clean? ... or diwrty OwO 
    let is_dirty = statuses.len() > 0;

    // Return formatted git status string
    format!(" : {branch} {status}",
        branch = format_branch!(head.is_remote(), &name_str),
        status = format_status!(is_dirty),
    )
}

fn print_ps1() {
    // Get the current username
    //let username = get_username()?;
    // Try get current username, else print error and use empty
    let username = match get_username() {
        Ok(name) => name,
        Err(err) => {
            println!("$PS1 ERROR: {}", err);
            "unknown_user".to_string()
        },
    };

    // Try get current directory, else print error and use empty
    let homedir_str = match get_home_dir() {
        Ok(home) => home,
        Err(err) => {
            println!("$PS1 ERROR: {}", err);
            "".to_string()
        }
    };

    // Try get current directory, else print the error and use backup method
    let curdir_str = match get_current_dir() {
        Ok(cwd)  => cwd,
        Err(err) => {
            println!("$PS1 ERROR: {}", err);
            "".to_string()
        },
    };

    // Try get the git status string (if PWD != git repo then will be empty)
    let git_str = get_git_status(&curdir_str);

    // Return the formatted PS1 string (remember, git_str could be empty!)
    print!("{name} @ {cwd}{git}\n--> ",
        name = format_name!(&username),
        cwd  = format_cwd!(&curdir_str.replace(&homedir_str, "~")),
        git  = git_str,
    )
}

fn main() {
    print_ps1();
    process::exit(0);
}