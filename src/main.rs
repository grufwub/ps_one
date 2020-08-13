extern crate dirs;
extern crate git2;
extern crate users;
extern crate termion;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::process;
use git2::{Repository, StatusOptions, StatusShow};
use termion::{color, style};

// Git status strings
const GIT_CLEAN: &'static str = "✓";
const GIT_DIRTY: &'static str = "✗";

// PS1 formatting colors + styles
lazy_static! {
    static ref RESET_FMT:        String = format!("{}{}", color::Fg(color::Reset), style::Reset);
    static ref NAME_FMT:         String = format!("{}{}", color::Fg(color::LightGreen), style::Bold);
    static ref CWD_FMT:          String = format!("{}{}", color::Fg(color::LightBlue), style::Bold);
    static ref BRANCH_FMT:       String = format!("{}{}", color::Fg(color::LightRed), style::Bold);
    static ref STATUS_CLEAN_FMT: String = format!("{}{}", color::Fg(color::LightGreen), style::Bold);
    static ref STATUS_DIRTY_FMT: String = format!("{}{}", color::Fg(color::LightRed), style::Bold);
}

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
    let repo = match Repository::open(cwd_str) {
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

    // Create the git status string, check if we are ~DIRTY~
    let mut status_str = String::new();
    if statuses.len() > 0 {
        status_str.push_str(&STATUS_DIRTY_FMT);
        status_str.push_str(&GIT_DIRTY);
    } else {
        status_str.push_str(&STATUS_CLEAN_FMT);
        status_str.push_str(&GIT_CLEAN);
    }

    // Return head name without the preceding 'refs/heads/'
    Some(
        format!(" : {branch_fmt}{branch}{reset_fmt} {status}{reset_fmt}",
            branch_fmt = BRANCH_FMT.to_string(),
            branch     = name_str,
            status     = status_str,
            reset_fmt  = format!("{}{}", color::Fg(color::Reset), style::Reset),
        )
    )
}

fn generate_ps1() -> Result<String, String> {
    // The terminal reset style ANSI code
    let reset_fmt = format!("{}{}", style::Reset, color::Fg(color::Reset));

    // Get the current username
    let username = get_username()?;

    // Get user's home directory (for setting tilde in cur_dir string)
    let home_dir = get_home_dir()?;

    // Get the current working directory
    let mut cur_dir = get_current_dir()?;

    // If current directory is a 
    let mut git_str = String::new();
    if let Some(git_status) = get_git_status(&cur_dir) {
        git_str.push_str(&git_status);
    }

    // Format the current directory to shorten $HOME --> ~
    cur_dir = cur_dir.replace(&home_dir, "~");

    // Return the formatted PS1 string (remember, git_str could be empty!)
    Ok(
        format!("{name_fmt}{name}{reset_fmt} @ {cwd_fmt}{cwd}{reset_fmt}{git_str}\n--> ",
            name_fmt  = NAME_FMT.to_string(),
            name      = username,
            cwd       = cur_dir,
            cwd_fmt   = CWD_FMT.to_string(),
            git_str   = git_str,
            reset_fmt = reset_fmt,
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