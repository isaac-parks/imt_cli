use crate::constants::{ProgramStatus, Nub, Directory};
use crate::vault_db;
use std::process::{Command, Child};
use std::{thread, time};

fn parse_args(args: &Vec<String>) -> (Vec<Nub>, bool, bool, bool) {
    let mut backend = false;
    let mut frontend = false;
    let mut vault = false;
    let mut nubs: Vec<Nub> = Vec::new();
    for arg in args {
        let nub: Option<Nub> = Nub::from_string(&arg);
        match nub {
            Option::Some(n) => nubs.push(n),
            Option::None => {
                match arg.as_str() {
                    "--frontend" => frontend = true,
                    "--backend" => backend = true,
                    "--vault" => vault = true,
                    &_ => ()
                };
            }
        }
    }

    (nubs, frontend, backend, vault)
}

fn spawn_server(nub: &Nub, dir: &Directory) -> u32 {
    nub.set_as_wd(dir);
    if *dir == Directory::Frontend {
        let mut cmd = Command::new("yarn");
        cmd.args(["serve"]);
        let child: Child = cmd.spawn().unwrap();
        println!("Frontend is starting for {}. (pid: {})", nub.as_local_frontend_url(), child.id());
        return child.id();
    } else {
        let mut cmd = Command::new("docker-compose");
        cmd.args(["up"]);
        let child: Child = cmd.spawn().unwrap();
        println!("Backend is starting for {}. (pid: {})", nub.as_string(), child.id());
        return child.id();
    };
}
 
pub fn run_pre_parsed(nubs: &Vec<Nub>, dirs: &Vec<Directory>) -> ProgramStatus {
    for directory in dirs {
        for nub in nubs {
            let _pid: u32 = spawn_server(nub, directory);
            // when multiple containers start at once, docker gets confused. Give it some time to start each container
            thread::sleep(time::Duration::from_secs(1)); 

        }
    }
    ProgramStatus::SUCCESS
}

pub fn run(args: &Vec<String>) -> ProgramStatus {
    let (nubs, spin_up_front, spin_up_back, add_vault) = parse_args(args);

    if add_vault {
        vault_db::run_pre_parsed(&nubs);
    }

    for nub in nubs {
        if spin_up_front {
            spawn_server(&nub, &Directory::Frontend);
        }
    
        if spin_up_back {
            spawn_server(&nub, &Directory::Backend);
        }
    }

    ProgramStatus::SUCCESS
}
