extern crate dirs;
extern crate git2;
extern crate users;
extern crate termion;

use std::env;
use std::process;
use git2::{Repository, StatusOptions, StatusShow};
use termion::{color, style};

// Git status strings
const GIT_CLEAN: &'static str = "✓";
const GIT_DIRTY: &'static str = "✗";

// PS1 formatting colors
const NAME_COLOR:         color::Fg<color::LightGreen> = color::Fg(color::LightGreen);
const CWD_COLOR:          color::Fg<color::LightBlue>  = color::Fg(color::LightBlue);
const BRANCH_COLOR:       color::Fg<color::LightRed>   = color::Fg(color::LightRed);
const STATUS_DIRTY_COLOR: color::Fg<color::LightRed>   = color::Fg(color::LightRed);
const STATUS_CLEAN_COLOR: color::Fg<color::LightGreen> = color::Fg(color::LightGreen);

// PS1 formatting styles
const NAME_STYLE:   style::Bold = style::Bold;
const CWD_STYLE:    style::Bold = style::Bold;
const BRANCH_STYLE: style::Bold = style::Bold;
const STATUS_STYLE: style::Bold = style::Bold;

fn get_username() -> Result<String, String> {
    // Get username OsString
    let username = match users::get_current_username() {
        Some(u) => u,
        None => return Err(String::from("Failed to get current user")),
    };

    // Get string from OsString
    if let Some(username_str) = username.to_str() {
        Ok(username_str.to_string())
    } else {
        Err(String::from("Failed to get current user"))
    }
}

fn get_username_backup() -> String {
    match env::var("USER") {
        Ok(name) => name,
        Err(_)           => "unknown".to_string(),
    }
}

fn get_current_dir() -> Result<String, String> {
    // Try to get current dir
    let current_dir = match env::current_dir() {
        Ok(c) => c,
        Err(_) => return Err(String::from("Failed to get current dir")),
    };

    // Try get string from current dir PathBuf
    if let Some(current_dir_str) = current_dir.to_str() {
        Ok(current_dir_str.to_string())
    } else {
        Err(String::from("Failed to get current dir"))
    }
}

fn get_current_dir_backup() -> String {
    match env::var("PWD") {
        Ok(pwd) => pwd,
        Err(_)          => "".to_string(),
    }
}

fn get_home_dir() -> Result<String, String> {
    // Try get current user's home dir
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => return Err(String::from("Failed to get current user's home dir"))
    };

    // Try get string from home dir PathBuf
    if let Some(home_dir_str) = home_dir.to_str() {
        Ok(home_dir_str.to_string())
    } else {
        Err(String::from("Failed to get current user's home dir"))
    }
}

fn get_git_status(cwd_str: &str) -> Option<String> {
    // Try open the repo at current directory
    let repo = match Repository::discover(cwd_str) {
        Ok(r) => r,
        Err(_) => return None,
    };

    // Try get the head from the repo
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return None,
    };

    // Create the git branch name string
    let mut name_str = String::new();

    // If remote, set prefix
    if head.is_remote() {
        name_str.push_str("remote: ");
    }

    // Add the head's name (formatted)
    name_str.push_str(&head.name()?.replace("refs/heads/", ""));

    // Get the current repository state iterator
    let statuses = match repo.statuses(Some(StatusOptions::default().show(StatusShow::Workdir).include_untracked(true).include_ignored(false).no_refresh(true))) {
        Ok(s) => s,
        Err(_) => return None,
    };

    // Create the git status string --> non-zero statuses length means we are ~DIRTY~
    let mut status_str = String::new();
    if statuses.len() > 0 {
        status_str.push_str(&format!("{}{}", STATUS_DIRTY_COLOR, STATUS_STYLE));
        status_str.push_str(&GIT_DIRTY);
    } else {
        status_str.push_str(&format!("{}{}", STATUS_CLEAN_COLOR, STATUS_STYLE));
        status_str.push_str(&GIT_CLEAN);
    }

    // Return formatted git status string
    Some(
        format!(" : {branch_fmt}{branch}{reset_fmt} {status}{reset_fmt}",
            branch_fmt = format!("{}{}", BRANCH_COLOR, BRANCH_STYLE),
            branch     = name_str,
            status     = status_str,
            reset_fmt  = format!("{}{}", color::Fg(color::Reset), style::Reset),
        )
    )
}

fn generate_ps1() -> String {
    // Get the current username
    //let username = get_username()?;
    // Try get current username, else print error and use empty
    let username = match get_username() {
        Ok(name) => name,
        Err(err) => {
            println!("$PS1 ERROR: {}. Using backup method", err);
            get_username_backup()
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
    let mut curdir_str = match get_current_dir() {
        Ok(cwd)  => cwd,
        Err(err) => {
            println!("$PS1 ERROR: {}. Using backup method", err);
            get_current_dir_backup()
        },
    };

    // If the current directory is a git repository, get a git status string
    let mut git_str = String::new();
    if let Some(git_status) = get_git_status(&curdir_str) {
        git_str.push_str(&git_status);
    }

    // Format the current directory to shorten $HOME --> ~
    curdir_str = curdir_str.replace(&homedir_str, "~");

    // Return the formatted PS1 string (remember, git_str could be empty!)
    format!("{name_fmt}{name}{reset_fmt} @ {cwd_fmt}{cwd}{reset_fmt}{git_str}\n--> ",
        name_fmt  = format!("{}{}", NAME_COLOR, NAME_STYLE),
        name      = username,
        cwd       = curdir_str,
        cwd_fmt   = format!("{}{}", CWD_COLOR, CWD_STYLE),
        git_str   = git_str,
        reset_fmt = format!("{}{}", color::Fg(color::Reset), style::Reset),
    )
}

fn main() {
    print!("{}", generate_ps1());
    process::exit(0);
}