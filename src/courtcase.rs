use serde::Deserialize;
use toml;
use std::fs;

// Struct for a court case
#[derive(Deserialize, Debug)]
pub struct Case {
    name: String,           // Full name i.e. "Turnabout Joe"
    version: String,        // Version of the case i.e. "1.0"
    foldername: String      // Name of the folder the case is stored in i.e. "turnabout_joe"
}

impl Case {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_foldername(&self) -> &str {
        &self.foldername
    }
}

// Wrapper struct for deserializing the TOML file
#[derive(Deserialize, Debug)]
struct CaseWrapper {
    case: Case,
}

pub fn load_cases() -> Vec<Case> {
    let mut court_cases = Vec::new();
    let cases_dir = "./cases";

    if let Ok(entries) = fs::read_dir(cases_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let case_file = path.join("case.toml");
                    if case_file.exists() {
                        if let Ok(contents) = fs::read_to_string(case_file) {
                            if let Ok(case_wrapper) = toml::from_str::<CaseWrapper>(&contents) {
                                println!("Loaded case {:?}", case_wrapper.case.name);
                                court_cases.push(case_wrapper.case);
                            } else {
                                println!("Failed to parse case.toml in directory {:?}", entry);
                            }
                        }
                    } else {
                        println!("Failed to find case.toml in directory {:?}", entry);
                    }
                }
            }
        }
    } else {
        println!("Failed to read cases directory");
    }

    court_cases
}