use serde::Deserialize;
use toml;
use std::fs;

// Struct for a piece of evidence that is stored in the court record
#[derive(Deserialize, Debug)]
pub struct Evidence {
    fullname: String,       // Full name of the piece of evidence i.e. "Revolver"
    alias: String,          // Shortened name for terminal use i.e. "rev"
    description: String,    // Description of the piece i.e. "The weapon of the crime. Has Joe's fingerprints."
    weight: u8              // Location of the piece in the court record, smaller goes first.
}

// Wrapper struct for deserializing the TOML file
#[derive(Deserialize, Debug)]
struct EvidenceList {
    evidence: Vec<Evidence>,
}

pub fn load_evidence(filename: String) -> Vec<Evidence> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let evidence_list: EvidenceList = toml::from_str(&contents)
        .expect("Failed to parse TOML");

    evidence_list.evidence
}