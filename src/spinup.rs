use crate::constants::{ProgramStatus, Nub, Directory};
use std::process::{Command, Child};

fn spawn_server(nub: &Nub, dir: &Directory) -> u32 {
    nub.set_as_wd(dir);
    if *dir == Directory::Frontend {
        let mut cmd = Command::new("yarn");
        cmd.args(["serve"]);
        let child: Child = cmd.spawn().unwrap();
        // this will be the process id of the yarn process zzz not node
        //pgrep -l node | grep -Eo '[0-9]{1,5}' 
        println!("Frontend is starting for {}. (pid: {})", nub.as_local_frontend_url(), child.id());
        return child.id();
    } else {
        let mut cmd = Command::new("docker-compose");
        cmd.args(["up"]);
        let child: Child = cmd.spawn().unwrap();
        println!("Backend is starting for {}. (pid: {})", nub.as_local_frontend_url(), child.id());
        return child.id();
    };
}

// Add arg to run "yarn" first in each directory 
pub fn run_pre_parsed(nubs: &Vec<Nub>, dirs: &Vec<Directory>) -> ProgramStatus {
    println!("spinning up pre-parsed");
    for directory in dirs {
        for nub in nubs {
            let _pid: u32 = spawn_server(nub, directory);

        }
    }
    ProgramStatus::SUCCESS
}

pub fn run(_args: &Vec<String>) -> ProgramStatus {
    println!("Not implemented yet, for now, you must run this command as --spinup with the `imt_cli nublink` command.");
    ProgramStatus::SUCCESS
}
