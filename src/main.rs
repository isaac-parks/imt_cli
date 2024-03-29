use std::env;
use std::io::Error;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use env::VarError;

use std::process::Command;
use std::process::Output;
use std::path::Path;
use std::fs::read_dir;
use std::fs::FileType;
use std::str::FromStr;

#[derive(PartialEq)]
enum Action {
    Install,
    Prune
}

fn install() {
 println("Jk, this isn't implemented yet.");
}

fn rm_old_branches_from_cd(branch_data: &Vec<(String, usize)>) -> usize {
    println!("Deleting old branches from {:?}", env::current_dir().unwrap());
    let sys_t = SystemTime::now();
    let now = sys_t.duration_since(UNIX_EPOCH).unwrap();
    let one_twenty_days = Duration::from_secs(10368000);
    let cut_off = now - one_twenty_days;
    let ignore: [&str; 4] = ["staging", "production", "qa", "master"];
    let mut deleted_num = 0;
    for data in branch_data {
        let last_commit_date = Duration::from_secs(data.1.try_into().unwrap());
        if last_commit_date < cut_off {
            let branch_name = &data.0.as_str();
            if !ignore.contains(branch_name) {
                let mut delete_cmd = Command::new("git");
                delete_cmd.args(["branch", "-D", branch_name]);
                dbg!(&delete_cmd);
                match delete_cmd.output() {
                    Ok(output) => {
                        println!("Attempting to delete branch \"{}\".", data.0);
                        let stdout = std::str::from_utf8(&output.stdout).unwrap_or("Could not decode stdout");
                        let stderr = std::str::from_utf8(&output.stderr).unwrap_or("Could not decode stderr");
                        println!("Command output (if any): {}", stdout);
                        println!("Command error (if any): {}", stderr);
                        deleted_num += 1;
                    }
                    Err(_) => {
                        println!("Couldn't delete branch \"{}\".", data.0);
                    }
                }
            }
        }
    }
    println!("Deleted {} branches from {:?}🪦", deleted_num, env::current_dir().unwrap());
    deleted_num
}

fn get_cd_last_commit_dates() -> Vec<(String, usize)> {
    let mut branch_commit_dates: Vec<(String, usize)> = Vec::new();
    let mut branch_cmd = Command::new("git");
    branch_cmd.args(["for-each-ref", "--format=%(refname:short)|%(committerdate:unix)", "refs/heads"]);
    let output = branch_cmd.output();
    match output {
        Ok(out) => {
            let p = "\n";
            let utf8 = String::from_utf8(out.stdout).unwrap();
            let mut split: &Vec<&str> = &utf8.split(&p).collect();
            for branch_split in split {
                let name_date_split: Vec<&str> = branch_split.split('|').collect();
                if name_date_split.len() != 2 {
                    continue;
                }
                let name: &str = name_date_split[0];
                let unix_ts_str: &str = name_date_split[1];
            
                match unix_ts_str.parse::<usize>() {
                    Ok(unix_ts) => {
                        branch_commit_dates.push((String::from(name), unix_ts.try_into().unwrap()));
                    },
                    Err(e) => {
                        eprintln!("Failed to parse timestamp for '{}': {}", name, e);
                    }
                }
            }
        },
        _ => ()
    }
    branch_commit_dates
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

fn prune() -> usize {
    let service_dir: String = make_dir(env::var("IMT_SERVICES_DIR"));
    if !set_working_dir(&service_dir) {
        println!("ERROR: Couldn't set working directory");
    }
    let root_dir = env::current_dir().unwrap();
    let root_dir_path = root_dir.as_path().to_str().unwrap();
    let files = read_dir(&root_dir);
    let mut total_del: usize = 0;
    if let Ok(file_list) = files {
        for file in file_list {
            match &file {
                Ok(dir_entry) => {
                    if !dir_entry.file_type().unwrap().is_dir() {
                        continue;
                    }
                    set_working_dir(&String::from(dir_entry.path().to_str().unwrap()));
                    let branch_tups: Vec<(String, usize)> = get_cd_last_commit_dates();
                    let num_deleted: usize = rm_old_branches_from_cd(&branch_tups);
                    total_del += num_deleted;
                    get_cd_last_commit_dates();
                    set_working_dir(&String::from(root_dir_path));
                },
                _ => ()
            }
        }
    }

    total_del
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
    actions.push(Action::Prune); // gonna make this wonky for now
    actions
}
fn main() {
    let mut args: Vec<String> = env::args().collect();
    let actions: Vec<Action> = parse_args(&mut args);
    for action in actions {
        match action {
            Action::Install => install(),
            Action::Prune => {
                let total_del: usize = prune();
                println!("Done pruning. Deleted a total of {} branches.", total_del);
            }
        }
    }
}

