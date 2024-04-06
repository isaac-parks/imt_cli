use std::env;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use env::VarError;

use std::process::Command;
use std::fs::read_dir;

use crate::constants::{IMT_SERVICES_DIR, ProgramStatus, set_working_dir};


fn rm_old_branches_from_cd(branch_data: &Vec<(String, usize)>) -> usize {
    println!("Deleting old branches from {:?}", env::current_dir().unwrap());
    let sys_t = SystemTime::now();
    let now = sys_t.duration_since(UNIX_EPOCH).unwrap();
    let one_twenty_days = Duration::from_secs(10368000);
    let cut_off = now - one_twenty_days;
    let ignore: [&str; 4] = ["staging", "production", "qa", "master"];
    let mut deleted_num = 0;
    for data in branch_data {
        let branch_name = &data.0.as_str();
        let last_commit_date = Duration::from_secs(data.1.try_into().unwrap());
        if last_commit_date < cut_off {
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
    println!("Tried deleted {} branches from {:?}🪦", deleted_num, env::current_dir().unwrap());
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
            let split: &Vec<&str> = &utf8.split(&p).collect();
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

fn make_dir(service_dir: Result<String,VarError>) -> String{
    if !service_dir.is_ok() {
        eprintln!("ERROR: You are trying to prune, but don't have the {} environment variable set. 
            (Hint: try running this command with install OR setting the environment variable {}
            to the directory containing IMT services.)",
            IMT_SERVICES_DIR,
            IMT_SERVICES_DIR
        );
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

pub fn run(_args: Vec<String>) -> ProgramStatus { // Args unused for now
    let service_dir: String = make_dir(env::var(IMT_SERVICES_DIR));
    // TODO support vue & pip packages
    if !set_working_dir(&service_dir) {
        println!("ERROR: Couldn't set working directory");

        return ProgramStatus::FAILED;
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
                    set_working_dir(&String::from(root_dir_path));
                },
                _ => ()
            }
        }
    }
    println!("Done pruning. Tried deleted a total of {} branches. If errors were found, they will be printed above.", total_del);
    ProgramStatus::SUCCESS
}
