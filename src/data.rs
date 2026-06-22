// <comment>
// 2026-06-21
// Kaleb Troyer

use std::io;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::background::{Background};
use crate::character::{Character};
use crate::class::{Class};
use crate::feat::{Feat};
use crate::race::{Race};

// ========================================
// Loaded Database and Implementation
// ========================================
// Database is the container for all loaded json data from the data folder.
// Encapsulation of the data provides system cohesion, a means for simpler
// function signatures, and brings data lifetime clarity. The implementation
// provides functions for the collection of data entries which match given field
// specifications.

/// Holds all imported data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {
    pub feats: Vec<Feat>,
    pub races: Vec<Race>,
    pub backgrounds: Vec<Background>,
    pub classes: Vec<Class>,
}

// Database implementation
impl Database {

    /// Creates an empty database struct
    pub fn new() -> Self {
        Database {
            feats: vec![],
            races: vec![],
            backgrounds: vec![],
            classes: vec![]
        }
    }

    /// Collects and unpacks all json objects in the data folder
    pub fn fetch(&mut self, dir: &Path) -> io::Result<()> {

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let epath = entry.path();

                if epath.is_dir() {
                    self.fetch(&epath)?;
                } else {
                    let entry = fs::read_to_string(&epath)?;
                    let evals: serde_json::Value = serde_json::from_str(&entry)?;

                    self.add_entry(&evals, &entry)?;
                }
            }
        }

        Ok(())

    }

    /// Given the raw json and a generic serialized object, unpacks the json
    /// as the appropriate struct and adds it to the database
    fn add_entry(&mut self, evals: &serde_json::Value, entry: &str) -> io::Result<()> {

        match &evals["category"].as_str() {
            Some("background") => {
                self.backgrounds.push(serde_json::from_str(entry)?);
            }
            Some("feature") => {
                println!("Features not yet implemented.");
                // self.features.push(serde_json::from_str(entry)?);
            }
            Some("class") => {
                self.classes.push(serde_json::from_str(entry)?);
            }
            Some("feat") => {
                self.feats.push(serde_json::from_str(entry)?);
            }
            Some("race") => {
                self.races.push(serde_json::from_str(entry)?);
            }
            _ => println!("unknown :("),
        }

        Ok(())

    }

}

// EOF
