use std::collections::HashMap;
use std::hash::Hash;
use std::process::{Termination, ExitCode};
use std::path::Path;
use std::env::{self, current_dir};

pub const IMT_SERVICES_DIR: &str = "IMT_SERVICES_DIR";

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Directory {
    Frontend,
    Backend
}
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
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
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

    pub fn as_string(&self) -> String {
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

    pub fn as_path_str(&self) -> String {
        parse_service_dir().unwrap()  + "/" + &self.as_string()
    }

    pub fn as_local_frontend_url(&self) -> String {
        let mut mappings = HashMap::new();
        mappings.insert(Nub::BusinessRules, "VUE_APP_ADMIN_URL_BUSINESS_RULES=https://localhost.imtins.com:8013".to_string());
        mappings.insert(Nub::CMS, "VUE_APP_ADMIN_URL_CMS=https://localhost.imtins.com:8015".to_string());
        mappings.insert(Nub::Filestore, "VUE_APP_ADMIN_URL_FILESTORE=https://localhost.imtins.com:8019".to_string());
        mappings.insert(Nub::Forms, "VUE_APP_ADMIN_URL_FORMS=https://localhost.imtins.com:8014".to_string());
        mappings.insert(Nub::Users, "VUE_APP_ADMIN_URL_USERS=https://localhost.imtins.com:8018".to_string());
        mappings.insert(Nub::Mapping, "VUE_APP_ADMIN_URL_MAPPING=https://localhost.imtins.com:8010".to_string());
        mappings.insert(Nub::PolicyData, "VUE_APP_POLICY_DATA_URL=https://localhost.imtins.com:8012".to_string());
        mappings.insert(Nub::PlatformAdmin, "VUE_APP_ADMIN_URL_PLATFORM_ADMIN=https://localhost.imtins.com:8016".to_string());
        mappings.insert(Nub::Vinna, "VUE_APP_ADMIN_URL_VINNA=https://localhost.imtins.com:8009".to_string());
        mappings.insert(Nub::Behandle, "VUE_APP_BEHANDLE_URL=https://localhost.imtins.com:8020".to_string());
        mappings.insert(Nub::Billing, "VUE_APP_BILLING_URL=https://localhost.imtins.com:8022".to_string());

        String::from(mappings.get(self).unwrap())
    }

    pub fn as_local_backend_url_internal(&self) -> String {
        let mut mappings = HashMap::new();
        mappings.insert(Nub::BusinessRules, "INTERNAL_NUBS['business-rules']=\"http://br:7004/\"".to_string());
        mappings.insert(Nub::CMS, "INTERNAL_NUBS['cms']=\"http://cms:7001/\"".to_string());
        mappings.insert(Nub::Filestore, "INTERNAL_NUBS['filestore']=\"http://fs:7005\"".to_string());
        mappings.insert(Nub::Forms, "INTERNAL_NUBS['forms']=\"http://forms:7002/\"".to_string());
        mappings.insert(Nub::Users, "INTERNAL_NUBS['users']=\"http://users:7007/\"".to_string());
        mappings.insert(Nub::Mapping, "INTERNAL_NUBS['mapping']=\"http://mapping:7015/\"".to_string());
        mappings.insert(Nub::PolicyData, "INTERNAL_NUBS['policy-data']=\"http://pd:7003/\"".to_string());
        mappings.insert(Nub::PlatformAdmin, "INTERNAL_NUBS['platform-admin']=\"http://pa:7000/\"".to_string());
        mappings.insert(Nub::Vinna, "INTERNAL_NUBS['vinna']=\"http://vinna:7016/\"".to_string());
        mappings.insert(Nub::Behandle, "INTERNAL_NUBS['behandle']=\"http://behandle:7010/\"".to_string());
        mappings.insert(Nub::Billing, "INTERNAL_NUBS['billing']=\"http://billing:7009/\"".to_string());
        mappings.insert(Nub::ServiceTracker, "INTERNAL_NUBS['service-tracker']=\"http://st:7006/\"".to_string());
        mappings.insert(Nub::APIRouter, "INTERNAL_NUBS['api-router']=\"http://api:7013/\"".to_string());
        mappings.insert(Nub::Paglipat, "INTERNAL_NUBS['paglipat']=\"http://paglipat:7017/\"".to_string());

        mappings.get(self).unwrap().clone()
    }

    pub fn set_as_wd(&self, dir_type: &Directory) -> bool {
        if *dir_type == Directory::Frontend {
            return set_working_dir(&format!("{}{}", self.as_path_str(), "/frontend"));
        }
        else {
            return set_working_dir(&format!("{}{}", self.as_path_str(), "/project/settings"));
        }
    }
}

pub fn set_working_dir(str_path: &String) -> bool {
    let wd = Path::new(str_path);
    let res = env::set_current_dir(&wd);
    res.is_ok()
}

fn parse_service_dir() -> Option<String> {
    let service_dir = env::var(IMT_SERVICES_DIR);
    if !service_dir.is_ok() {
        return None
    } else {
        if let Result::Ok(s_dir) = service_dir {
            let s_dir = String::from(s_dir);
            let home_dir: String = env::var("HOME").unwrap();
            if s_dir.chars().nth(0) == Some('~') {
                return Some(String::from(format!("{}{}", &home_dir, &s_dir[1..])))
            } else {
                return Some(String::from(&s_dir))
            }
        }
        return None
    }
}

pub fn get_service_dir_string() -> Option<String> {
    let result = parse_service_dir();
    match result {
        Some(rs) => Some(rs),
        None => None
    }
}

pub const NUB_LIST: [Nub; 14] = [
    Nub::APIRouter,
    Nub::Behandle,
    Nub::Billing,
    Nub::BusinessRules,
    Nub::CMS,
    Nub::Filestore,
    Nub::Forms,
    Nub::Mapping,
    Nub::Paglipat,
    Nub::PlatformAdmin,
    Nub::PolicyData,
    Nub::ServiceTracker,
    Nub::Users,
    Nub::Vinna
];

pub fn health_check() -> Option<ProgramStatus> {
    // Verify can find the IMT_SERVICES dir
    let dir_check = get_service_dir_string();
    if let None = dir_check {
        eprintln!("
            ERROR: The {} environment variable is not set/properly configured. 
            (Hint: try setting the environment variable {} to the directory containing IMT services.)",
            IMT_SERVICES_DIR,
            IMT_SERVICES_DIR
        );
        return None
    }

    // Make sure directories for nubs exist
    let root_dir = current_dir().unwrap();
    for nub in NUB_LIST {
        if !nub.set_as_wd(&Directory::Frontend) {
            eprintln!("ERROR during health check prevented command from running: Couldn't find frontend directory for {}. Make sure it is cloned, and make sure your IMT_SERVICES_DIR variable points to the correct directory containing services", nub.as_string());
            return None
        }

        if !nub.set_as_wd(&Directory::Backend) {
            eprintln!("ERROR during health check prevented command from running: Couldn't find backend directory for {}. Make sure it is cloned, and make sure your IMT_SERVICES_DIR variable points to the correct directory containing services", nub.as_string());
            return None
        }
    }
    set_working_dir(&String::from(root_dir.to_str().unwrap()));

    Some(ProgramStatus::SUCCESS)
}