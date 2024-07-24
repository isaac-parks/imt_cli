use crate::constants::Nub;


pub mod nublink;
pub mod nubunlink;


pub fn parse_link_args(args: &Vec<String>) -> (Vec<Nub>, Vec<String>) {
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
                    "--spinup" => String::from("spinup"),
                    "--spindown" => String::from("spindown"),
                    "--vault" => String::from("vault_db"),
                    &_ => String::new()
                };

                flags.push(flag);
            }
        }
    }

    (nubs, flags)
}