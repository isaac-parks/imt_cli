use crate::constants::{ProgramStatus, Nub, Directory};
use std::process::{Command, Child};
use std::str;


fn kill_node() {
    let output = Command::new("pgrep")
        .args(["-l", "node"])
        .output()
        .expect("Couldn't get a list of node processes.");

    let output_str = str::from_utf8(&output.stdout)
        .expect("Output was not valid UTF-8");

    let mut pids: Vec<String> = Vec::new();
    for line in output_str.lines() {
        let mut pid = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                pid.push(c);
            } else if !pid.is_empty() {
                pids.push(pid);
                break;
            }
        }
    }
    for id in pids {
        let mut kill = Command::new("kill");
        kill.args(["-TERM", &id]); // sends SIGTERM instead of SIGKILL so they can clean up
        kill.output().expect(&format!("Couldn't kill node process. (pid: {})", id));

        println!("Gracefully killed node process. (pid{})", id);
    }
}

fn shutdown_docker(nub: &Nub) {
    nub.set_as_wd(&Directory::Backend);
    let mut cmd = Command::new("docker-compose");
    cmd.args(["stop"]);
    let child: Child = cmd.spawn().unwrap();
    println!("Backend is being stopped for {}. (pid: {})", nub.as_string(), child.id());
}

pub fn run_pre_parsed(nubs: &Vec<Nub>, dirs: &Vec<Directory>) -> ProgramStatus {
    for directory in dirs {
        if *directory == Directory::Frontend {
            // Since there isn't a good way to find running node processes, just have to remove all node instances.
            kill_node();
        } else {
            for nub in nubs {
                shutdown_docker(nub);
            }
        }
    }
    ProgramStatus::SUCCESS
}

pub fn run(_args: &Vec<String>) -> ProgramStatus {
    println!("Not implemented yet, for now, you must run this command as --spindown with the `imt_cli nubunlink` command.");
    ProgramStatus::SUCCESS
}
