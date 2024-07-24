use crate::constants::{ProgramStatus, Nub, Directory};
use crate::spindown;
use super::parse_link_args;
use crate::unset_vault_db;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};


fn get_lines_to_rewrite(fname: &str) -> Vec<String> {
    let token = "#$";
    let file = OpenOptions::new()
    .read(true)
    .open(fname).unwrap();

    let buf = BufReader::new(file);
    let mut lines_to_rewrite: Vec<String> = Vec::new();
    for line in buf.lines().map(|l| l.unwrap()) {
        if line.contains(token) {
            return lines_to_rewrite;
        }
        lines_to_rewrite.push(line);
    }

    lines_to_rewrite
}

pub fn unlink_nubs(nubs: &Vec<Nub>, dir: Directory) {
    let file_name_to_write = match dir {
        Directory::Frontend => ".env.development",
        Directory::Backend => "local.py"
    };

    for nub in nubs { 
        nub.set_as_wd(&dir);

        let lines_to_rewrite = get_lines_to_rewrite(file_name_to_write);

        if lines_to_rewrite.len() == 0 {
            return;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_name_to_write).unwrap();

        let mut bytes = Vec::new();
        for line in lines_to_rewrite {
            bytes.extend(line.into_bytes());
            bytes.extend("\n".as_bytes());
        }

        bytes.pop(); // to perserve single empty new line 

        file.write_all(&bytes).unwrap();
        println!("Successfully unlinked nub '{:?}' from {:?} directory.", nub, dir);
    }

}

pub fn run(args: &Vec<String>) -> ProgramStatus {
    let (nubs, flags) = parse_link_args(&args);
    let mut directories_to_run: Vec<Directory> = Vec::new();
    for flag in &flags {
        match &flag.as_str() {
            &"frontend" => {
                directories_to_run.push(Directory::Frontend);
                unlink_nubs(&nubs, Directory::Frontend);
            },
            &"backend" => {
                directories_to_run.push(Directory::Backend);
                unlink_nubs(&nubs, Directory::Backend);
            },
            &"spindown" => {
                spindown::run_pre_parsed(&nubs, &directories_to_run);
            },
            &"vault_db" => {
                unset_vault_db::run_pre_parsed(&nubs);
            },
            &_ => ()
        }
    }

    ProgramStatus::SUCCESS
}