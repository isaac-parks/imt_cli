use std::io::prelude::*;
use crate::constants::{Nub, ProgramStatus};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::env;


pub fn unset_vault_db(nub: &Nub) {
    let file_path = format!("{}/docker-compose.yml", nub.as_path_str());
    let file = fs::File::open(file_path.clone()).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut env_index: Option<usize> = None;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.trim() == "environment:" {
            env_index = Some(index + 1);
        }
        lines.push(line);
    }

    let vault_token_db = env::var("VAULT_TOKEN_DB").unwrap();
    let new_line = format!("      - VAULT_TOKEN_DB={}", vault_token_db);
    if let Some(index) = env_index {
        lines.remove(index);
    }

    let temp_file_path = format!("{}.tmp", file_path);
    let temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_file_path)
        .unwrap();

    let mut writer = BufWriter::new(temp_file);
    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }

    fs::rename(&temp_file_path, &file_path).unwrap();

}

pub fn run_pre_parsed(nubs: &Vec<Nub>) -> ProgramStatus {
    for nub in nubs {
        set_vault_db(nub);
    }

    ProgramStatus::SUCCESS
}


pub fn run(_args: &Vec<String>) -> ProgramStatus {
    ProgramStatus::SUCCESS
}