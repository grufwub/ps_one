extern crate dirs;
extern crate git2;
extern crate users;
extern crate termion;

use std::env;
use std::process;
use git2::{Repository, StatusOptions, StatusShow};
use termion::{color, style};

const PROMPT: &str = "\n--> ";

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

    // Create the git status string
    let mut git_str = String::new();

    // If remote, set prefix
    if head.is_remote() {
        git_str.push_str("remote: ");
    }

    // Add the head's name (formatted)
    git_str.push_str(&head.name()?.replace("refs/heads/", ""));

    // Get the current repository state iterator
    let statuses = match repo.statuses(Some(StatusOptions::default().show(StatusShow::Workdir).include_untracked(true).include_ignored(false).no_refresh(true))) {
        Ok(s) => s,
        Err(_) => return None,
    };

    // If there are statuses in the iterator, means we are ~DIRTY~
    if statuses.len() > 0 {
        git_str.push_str(" ✗");
    } else {
        git_str.push_str(" ✓");
    }

    // Return head name without the preceding 'refs/heads/'
    Some(git_str)
}

fn generate_ps1() -> Result<String, String> {
    // The PS1 string to later return
    let mut ps1_str = String::new();

    // The terminal reset style ANSI code
    let reset_style = format!("{}{}", style::Reset, color::Fg(color::Reset));

    // Get the current username
    let username = get_username()?;

    // Add to PS1:
    // - username style string
    // - username string
    // - reset style string
    // - intermediate ' @ ' between username & directory
    ps1_str.push_str(&format!("{}{}", style::Bold, color::Fg(color::LightGreen)));
    ps1_str.push_str(&username);
    ps1_str.push_str(&reset_style);
    ps1_str.push_str(" @ ");

    // Get the current working directory
    let cur_dir = get_current_dir()?;

    // Get user's home directory (for setting tilde in cur_dir string)
    let home_dir = get_home_dir()?;

    // Add to PS1:
    // - current dir style string
    // - current dir formatted to truncate home -> '~'
    // - reset style string
    ps1_str.push_str(&format!("{}{}", style::Bold, color::Fg(color::LightBlue)));
    ps1_str.push_str(&cur_dir.replace(&home_dir, "~"));
    ps1_str.push_str(&reset_style);

    // Check for git details, if so add:
    // - intermediate ' : ' between directory & git status
    // - git style string
    // - git status string
    // - reset style string
    if let Some(git_status) = get_git_status(&cur_dir) {
        ps1_str.push_str(" : ");
        ps1_str.push_str(&format!("{}{}", style::Bold, color::Fg(color::LightRed)));
        ps1_str.push_str(&git_status);
        ps1_str.push_str(&reset_style);
    }

    // Add the final touch (the prompt)
    ps1_str.push_str(&PROMPT);

    // Return the formatted PS1 string
    Ok(ps1_str)
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