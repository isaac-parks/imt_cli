use std::process::{Termination, ExitCode};
use std::path::{PathBuf, Path};
use std::env;

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
#[derive(Debug)]
pub enum Nub {
    APIRouter,
    Behandle,
    Billing,
    BusinessRules,
    CMS,
    Filestore,
    Forms,
    Mapping,
    Paglipat,
    PlatformAdmin,
    PolicyData,
    ServiceTracker,
    Users,
    Vinna,
}

impl Nub {
    pub fn from_string(s: &str) -> Option<Self> {
        let s = s.to_lowercase();
        match s.as_str() {
            "apirouter" | "api" => Some(Nub::APIRouter),
            "billing" => Some(Nub::Billing),
            "behandle" | "bh" => Some(Nub::Behandle),
            "business-rules" | "br" => Some(Nub::BusinessRules),
            "cms" => Some(Nub::CMS),
            "filestore" | "fs" => Some(Nub::Filestore),
            "forms" => Some(Nub::Forms),
            "mapping" => Some(Nub::Mapping),
            "paglipat"=> Some(Nub::Paglipat),
            "platform-admin" | "pa" => Some(Nub::PlatformAdmin),
            "policy-data" | "pd" => Some(Nub::PolicyData),
            "service-tracker" | "st" => Some(Nub::ServiceTracker),
            "users" => Some(Nub::Users),
            "vinna" => Some(Nub::Vinna),
            _ => None,
        }
    }

    fn as_string(self) -> String {
        match self {
            Nub::APIRouter => "api".to_string(),
            Nub::Behandle => "behandle".to_string(),
            Nub::Billing => "billing".to_string(),
            Nub::BusinessRules => "business-rules".to_string(),
            Nub::CMS => "cms".to_string(),
            Nub::Filestore => "filestore".to_string(),
            Nub::Forms => "forms".to_string(),
            Nub::Mapping => "mapping".to_string(),
            Nub::Paglipat => "paglipat".to_string(),
            Nub::PlatformAdmin => "platform-admin".to_string(),
            Nub::PolicyData => "policy-data".to_string(),
            Nub::ServiceTracker => "service-tracker".to_string(),
            Nub::Users => "users".to_string(),
            Nub::Vinna => "vinna".to_string(),
        }
    }

    pub fn as_path_str(self) -> String {
        String::from(env::var(IMT_SERVICES_DIR).unwrap() + "/" + &self.as_string()) // todo need to unwrap imt_services_dir cleaner
    }
}

pub fn set_working_dir(str_path: &String) -> bool {
    let wd = Path::new(str_path);
    let res = env::set_current_dir(&wd);
    res.is_ok()
}