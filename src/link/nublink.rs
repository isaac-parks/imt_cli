use std::io::prelude::*;
use crate::constants::{ProgramStatus, Nub, Directory};
use crate::{spinup, vault_db};
use std::fs::OpenOptions;
use super::parse_link_args;

pub fn link_nubs(nubs: &Vec<Nub>, dir: Directory) {
    let token = "#$";
    let mut tokens_assigned: Vec<&Nub> = Vec::new();

    let file_name_to_write = match dir {
        Directory::Frontend => ".env.development",
        Directory::Backend => "local.py"
    };

    for nub_to_write in nubs {
        for nub in nubs { 
            nub.set_as_wd(&dir);

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_name_to_write).unwrap();

            file.write("\n".as_bytes()).unwrap(); // Start write seq with a new line

            if !tokens_assigned.contains(&nub) {
                file.write(token.as_bytes()).unwrap(); // Write a token if we haven't been to this nub yet
                file.write(" <- Leave these tokens and nub configs below at the end of the file for `imt_cli unlinknubs` to work correctly.".as_bytes()).unwrap();
                file.write("\n".as_bytes()).unwrap();
                tokens_assigned.push(nub)
            }
            if dir == Directory::Frontend {
                file.write(nub_to_write.as_local_frontend_url().as_bytes()).unwrap();   
            }
            if dir == Directory::Backend {
                file.write(nub_to_write.as_local_backend_url_internal().as_bytes()).unwrap();                
            }

            println!("Successfully linked nub '{:?}' to {:?} directory.", nub, dir);
        }
    }
}

pub fn run(args: &Vec<String>) -> ProgramStatus {
    let (nubs, flags) = parse_link_args(&args);
    let mut directories_to_run: Vec<Directory> = Vec::new();
    for flag in &flags {
        match &flag.as_str() {
            &"frontend" => {
                directories_to_run.push(Directory::Frontend);
                link_nubs(&nubs, Directory::Frontend);
            },
            &"backend" => {
                directories_to_run.push(Directory::Backend);
                link_nubs(&nubs, Directory::Backend);
            },
            &"spinup" => {
                spinup::run_pre_parsed(&nubs, &directories_to_run);
            },
            &"vault_db" => {
                vault_db::run_pre_parsed(&nubs);
            },
            &_ => ()
        }
    }

    ProgramStatus::SUCCESS
}