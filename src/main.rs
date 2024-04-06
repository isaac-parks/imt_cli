mod prune;
mod nublink;
mod constants;

use std::env;
use self::constants::{IMT_SERVICES_DIR, ProgramStatus};


#[derive(PartialEq)]
enum ActionTypes {
    Install,
    Nublink,
    Help,
    Prune
}

fn install() -> ProgramStatus {
    // TODO: add to path? Set env variables?
    println!("Jk, this isn't implemented yet. You'll have to add {} to your environment variables.", IMT_SERVICES_DIR);
    ProgramStatus::SUCCESS
}

fn parse_args(args: &mut Vec<String>) -> (Option<ActionTypes>, Vec<String>) {
    let mut action: Option<ActionTypes> = Option::None;
    let mut additional_args: Vec<String> = vec![];
    for arg in &mut args[1..] {
        if let Option::None = action {
            match arg.as_str() {
                "help" => action = Some(ActionTypes::Help),
                "install" => action = Some(ActionTypes::Install),
                "prune" => action = Some(ActionTypes::Prune),
                "nublink" => action = Some(ActionTypes::Nublink),
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
    let mut args: Vec<String> = env::args().collect();
    let (action, additional_args) = parse_args(&mut args);

    if let Option::None = action {
        println!("Available Commands: `imt_cli install`, `imt_cli prune`"); // hardcoding until more features are added

        return ProgramStatus::FAILED;
    }

    match action.unwrap() {
        ActionTypes::Install => install(),
        ActionTypes::Prune => prune::run(additional_args),
        ActionTypes::Nublink => nublink::run(additional_args),
        ActionTypes::Help => ProgramStatus::SUCCESS
    }
}

