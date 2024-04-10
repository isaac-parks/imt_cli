use crate::constants::ProgramStatus;

pub fn run(_args: &Vec<String>) -> ProgramStatus {
   println!("{}", format!("
    Available Commands: 
        - `imt_cli prune`
        - `imt_cli nublink`
        - `imt_cli nubunlink`
        - `imt_cli help`
        
    Usage/Description (<> denotes optional arguments):
        - `imt_cli prune` Removes all local git branches that haven't been committed to in over 3 months.

        - `imt_cli nublink <--frontend> <--backend> <--spinup> <nubs_to_link>` Links all nubs listed in <nubs_to_link> together. 
           The --frontend and/or --backend flags must be specified. If --frontend is specified, the .env.development files of each
           nub will be set to point to each other. Same with backend. Both flags can be used at once. Specifying the --spinup command
           will start the --frontend and/or --backends depending on what flag was passed.

        - `imt_cli nubunlink <--frontend> <--backend> <--spindown> <nubs_to_unlink>` Does the reverse of nublink.

        - `imt_cli help` Displays this command.
    "
    ));

    ProgramStatus::SUCCESS
}