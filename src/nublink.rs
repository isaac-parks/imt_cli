use crate::constants::{ProgramStatus, Nub, set_working_dir};

// link 
// usage imt_cli nub_link br pd cms (would take each nub, find .env.development, 
// append token (to id when removed), append corresponding nub urls
// options, --frontend(default if not specified) --backend(set internal nubs in local.py)

// figure out which nubs need linked based on args
// navigate to IMTServices/nub.as_path()
// find .env.development file
// append -> new line + token
// append each nub that needs linked
// append end token
// spin up each nub with yarn && yarn serve

fn parse_args(args: Vec<String>) -> (Vec<Nub>, Vec<String>) {
    // Could use an iterator to see flag arguments if needed
    let mut flags: Vec<String> = Vec::new();
    let mut nubs: Vec<Nub> = Vec::new();
    for arg in args {
        let nub: Option<Nub> = Nub::from_string(&arg);
        match nub {
            Option::Some(n) => nubs.push(n),
            Option::None => {
                let flag = match arg.as_str() {
                    "--frontend" => String::from("frontend"),
                    "--backend" => String::from("backend"),
                    &_ => String::new()
                };

                flags.push(flag);
            }
        }
    }

    (nubs, flags)
}

pub fn link_nubs(nubs: Vec<Nub>) {
    for nub in nubs {
        set_working_dir(&nub.as_path_str());
    }
}

pub fn run(args: Vec<String>) -> ProgramStatus {


    println!("Hello nublink args! {:?}", args);
    let (nubs, _flags) = parse_args(args);
    link_nubs(nubs);

    ProgramStatus::SUCCESS
}