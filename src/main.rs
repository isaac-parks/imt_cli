use chrono::Local;
use chrono::Datelike;

use std::env;
use std::io::Error;
use env::VarError;

use std::process::Command;
use std::process::Output;
use std::path::Path;
use std::fs::read_dir;
use std::fs::FileType;

#[derive(PartialEq)]
enum Action {
    Install,
    Prune
}

fn parse_args(args: &mut Vec<String>) -> Vec<Action> {
    let mut actions = vec![];
    for arg in &mut args[1..] {
        match arg.as_str() {
            "--install" => actions.push(Action::Install),
            &_ => ()
        }
    }
    if actions.len() > 0  {
        return actions
    }
    actions.push(Action::Prune);
    actions
}

fn install() {
// TOOD
}

fn prune() {
    let service_dir: String = make_dir(env::var("IMT_SERVICES_DIR"));
    if !set_working_dir(&service_dir) {
        println!("ERROR: Couldn't set working directory");
    }
    let root_dir = env::current_dir().unwrap();
    let root_dir_path = root_dir.as_path().to_str().unwrap();
    let files = read_dir(&root_dir);
    if let Ok(file_list) = files {
        for file in file_list {
            match &file {
                Ok(dir_entry) => {
                    if !dir_entry.file_type().unwrap().is_dir() {
                        continue; // skip non dir
                    }
                    set_working_dir(&String::from(dir_entry.path().to_str().unwrap()));
                    // todo get all branches
                    // loop through all branches and run git log
                    // compare date to current date
                    // if too old delete
                    remove_origins();
                    set_working_dir(&String::from(root_dir_path));
                },
                _ => ()
            }
        }
    }
}

fn remove_origins() {
    let mut git_cmd = Command::new("git");
    git_cmd.args(["fetch", "--all", "--prune"]);
    let out: Result<Output, Error> = git_cmd.output();
    match &out {
        Output => {
            dbg!(env::current_dir().unwrap());
            dbg!(&out);
            println!("{:?}", &out.unwrap().stdout);
        }
        Error => {
            dbg!(out);
        }
    }
}

fn set_working_dir(service_dir: &String) -> bool {
    let root = Path::new(&service_dir);
    let res = env::set_current_dir(&root);
    res.is_ok()
}

fn make_dir(service_dir: Result<String,VarError>) -> String{
    if !service_dir.is_ok() {
        eprintln!("ERROR: You are trying to prune, but don't have the IMT_SERVICES_DIR environment variable set. (Hint: try running this command with --install OR setting the environment variable IMT_SERVICES_DIR to the directory containing IMT services.)");
        return String::new()
    }
    let home_dir: String = env::var("HOME").unwrap();
    if let Result::Ok(dir) = service_dir {
        if dir.chars().nth(0) == Some('~') {
            return String::from(format!("{}{}", &home_dir, &dir[1..]))
        }
    }
    String::new()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let actions: Vec<Action> = parse_args(&mut args);
    for action in actions {
        match action {
            Action::Install => install(),
            Action::Prune => prune()
        }
    }
}

