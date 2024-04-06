use std::process::{Termination, ExitCode};

pub const IMT_SERVICES_DIR: &str = "IMT_SERVICES_DIR";

pub enum ProgramStatus {
    SUCCESS,
    FAILED
}

impl Termination for ProgramStatus {
    fn report(self) -> ExitCode {
        match self {
            ProgramStatus::SUCCESS => ExitCode::from(0),
            ProgramStatus::FAILED => ExitCode::from(1)
        }
    }
}
