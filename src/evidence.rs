use serde::Deserialize;
use toml;
use std::fs;

use crate::courtcase::Case;
use promkit::preset::listbox::Listbox;

// Struct for a piece of evidence that is stored in the court record
#[derive(Deserialize, Debug)]
pub struct Evidence {
    fullname: String,       // Full name of the piece of evidence i.e. "Revolver"
    alias: String,          // Shortened name for terminal use i.e. "rev"
    description: String,    // Description of the piece i.e. "The weapon of the crime. Has Joe's fingerprints."
    weight: u8              // Location of the piece in the court record, smaller goes first.
}

impl Evidence {
    pub fn get_fullname(&self) -> &str {
        &self.fullname
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_weight(&self) -> u8 {
        self.weight
    }
    
}

// Wrapper struct for deserializing the TOML file
#[derive(Deserialize, Debug)]
pub struct EvidenceList {
    evidence: Vec<Evidence>,
}

impl EvidenceList {
    pub fn iter(&self) -> std::slice::Iter<Evidence> {
        self.evidence.iter()
    }
    pub fn get_evidence_from_alias(&self, alias: &str) -> Option<&Evidence> {
        self.evidence.iter().find(|e| e.get_alias() == alias)
    }
    pub fn get_evidence_from_fullname(&self, fullname: &str) -> Option<&Evidence> {
        self.evidence.iter().find(|e| e.get_fullname() == fullname)
    }
    pub fn sort_evidence(&mut self) {
        self.evidence.sort_by(|a, b| a.get_weight().cmp(&b.get_weight()));
    }
    pub fn get_evidence_list(&self) -> Vec<String> {
        self.evidence.iter().map(|e| e.get_fullname().to_string()).collect()
    }
}

// Load the evidence from the TOML file
pub fn load_evidence(case: &Case) -> EvidenceList {
    let filename = format!("./cases/{}/evidence.toml", case.get_foldername());
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let evidence_list: EvidenceList = toml::from_str(&contents)
        .expect("Failed to parse TOML");

    evidence_list
}

pub fn select_evidence(evidence_list: &EvidenceList) -> Result<&Evidence, Box<dyn std::error::Error>> {
    let mut p = Listbox::new(evidence_list.get_evidence_list())
        .title("Court record")
        .listbox_lines(10)
        .prompt()?;
    if let Some(e) = evidence_list.get_evidence_from_fullname(&p.run()?) {
        Ok(e)
    } else {
        Err("Failed to find evidence".into())
    }
}