mod prune;
mod nublink;
mod nubunlink;
mod spinup;
mod spindown;
mod constants;
mod help;

use std::env;
use self::constants::{ProgramStatus, health_check};


#[derive(PartialEq)]
enum ActionTypes {
    Nublink,
    Nubunlink,
    Spinup,
    Spindown,
    Help,
    Prune
}

fn parse_args(args: &mut Vec<String>) -> (Option<ActionTypes>, Vec<String>) {
    let mut action: Option<ActionTypes> = Option::None;
    let mut additional_args: Vec<String> = vec![];
    for arg in &mut args[1..] {
        if let Option::None = action {
            match arg.as_str() {
                "help" => action = Some(ActionTypes::Help),
                "prune" => action = Some(ActionTypes::Prune),
                "nublink" => action = Some(ActionTypes::Nublink),
                "nubunlink" => action = Some(ActionTypes::Nubunlink),
                "spinup" => action = Some(ActionTypes::Spinup),
                "spindown" => action = Some(ActionTypes::Spindown),
                &_ => ()
            }
        }
        else {
            additional_args.push(arg.to_string());
        }
    }

    (action, additional_args)
}

fn main() -> ProgramStatus {
    if let None = health_check() {
        return ProgramStatus::FAILED
    }

    let mut args: Vec<String> = env::args().collect();
    let (action, additional_args) = parse_args(&mut args);

    if let Option::None = action {
        println!("Use `imt_cli help` for Available Commands and Usage."); // hardcoding until more features are added

        return ProgramStatus::FAILED;
    }

    match action.unwrap() {
        ActionTypes::Prune => prune::run(&additional_args),
        ActionTypes::Nublink => nublink::run(&additional_args),
        ActionTypes::Nubunlink => nubunlink::run(&additional_args),
        ActionTypes::Spinup => spinup::run(&additional_args),
        ActionTypes::Spindown => spindown::run(&additional_args),
        ActionTypes::Help => help::run(&additional_args)
    }
}

