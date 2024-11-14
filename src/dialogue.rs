use console::style;
use serde::Deserialize;
use toml;
use std::fs;

use crate::courtcase::Case;
use regex::Regex;

#[derive(Deserialize, Debug)]
pub struct Statement {          // Struct for testimony statements (not for dialogue)
    text: String,               // The text of the statement
    objection: Option<String>   // The evidence that can be used to object to the statement
}

impl Statement {
    pub fn play(&self) {
        println!("{}", self.format_text());
    }

    pub fn format_text(&self) -> String {
        // Regex to find text to put in red
        let re = Regex::new(r"!\[(.*?)\]").unwrap();
        let mut formatted_text = self.text.clone();

        for cap in re.captures_iter(&self.text) {
            let matched_text = &cap[0];
            let inner_text = &cap[1];
            // Make the remaining text green
            // If we don't do this, what comes after the red text will be white instead of green
            let remaining_text = formatted_text.split_at(formatted_text.find(matched_text).unwrap() + matched_text.len()).1;
            let green_text = style(remaining_text).green().to_string();
            formatted_text = formatted_text.replace(remaining_text, &green_text);
            // Then we can make the highlited text red
            let styled_text = style(inner_text).red().to_string();
            formatted_text = formatted_text.replace(matched_text, &styled_text);
        }

        style(formatted_text).green().to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct TestimonyData {      // Struct for testimony data
    title: String,              // The title of the testimony
    character: String,          // The character that is giving the testimony
    alias: String,              // The alias of the testimony
    statements: Vec<Statement>  // The statements that the character will make
}

impl TestimonyData {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_character(&self) -> &str {
        &self.character
    }

    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn get_statement(&self, index: usize) -> Option<&Statement> {
        self.statements.get(index)
    }

    pub fn length(&self) -> usize {
        self.statements.len()
    }

    pub fn play(&self) {
        println!("{}: {}", style("Title").bold(), self.title);
        println!("{}: {}", style("Character").bold(), self.character);
        for statement in &self.statements {
            statement.play();
        }
    }
}

pub fn load_testimony(case: &Case, testimony_alias: &str) -> TestimonyData {
    let testimony_file = format!("./cases/{}/testimonies/{}.toml", case.get_foldername(), testimony_alias);
    let contents = fs::read_to_string(testimony_file).expect("Failed to read testimony file");
    let testimony_data: TestimonyData = toml::from_str(&contents).expect("Failed to parse testimony file");
    testimony_data
}

pub fn get_all_testimonies(case: &Case) -> Vec<TestimonyData> {
    let mut testimonies = Vec::new();
    let testimony_dir = format!("./cases/{}/testimonies", case.get_foldername());
    if let Ok(entries) = fs::read_dir(testimony_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_stem() {
                        let testimony_name = file_name.to_string_lossy();
                        let testimony_data = load_testimony(case, &testimony_name);
                        testimonies.push(testimony_data);
                    }
                }
            }
        }
    }
    testimonies
}

pub fn print_testimony(testimony_data: &TestimonyData) {
    println!("{}: {}", style("Title").bold(), testimony_data.title);
    println!("{}: {}", style("Character").bold(), testimony_data.character);
    for statement in &testimony_data.statements {
        println!("{}: {}", style("Statement").bold(), statement.text);
        println!("{}: {:?}", style("Evidence").bold(), statement.objection);
    }
}
