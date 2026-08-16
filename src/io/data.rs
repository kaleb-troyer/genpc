// Contains the core database object and functions for populating the database.
// 2026-06-21
// Kaleb Troyer

use std::io;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::background::{Background};
use crate::character::{Character};
use crate::class::{Class, Subclass};
use crate::feat::{Feat};
use crate::race::{Race};

// simplifies .add_entry(), not strictly necessary
macro_rules! push {
    ($entry:expr, $target:expr, $type:ty) => {
        match serde_json::from_str::<$type>($entry) {
            Ok(obj) => $target.push(obj),
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }
    };
}

// ========================================
// Loaded Database and Implementation
// ========================================
// Database is the container for all loaded json data from the data folder.
// Encapsulation of the data provides system cohesion, a means for simpler
// function signatures, and brings data lifetime clarity. The implementation
// provides functions for the collection of data entries which match given field
// specifications.
//
// Unknown fields in the json are silenty ignored to avoid breaking homebrew on
// updates and generally create a more premissive environment for homebrewers.

/// Holds all imported data
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {
    pub feats: Vec<Feat>,
    pub races: Vec<Race>,
    pub backgrounds: Vec<Background>,
    pub classes: Vec<Class>,
    pub subclasses: Vec<Subclass>,
}

// Database implementation
impl Database {

    /// Creates an empty database struct
    pub fn new() -> Self {
        Database {
            feats: vec![],
            races: vec![],
            backgrounds: vec![],
            classes: vec![],
            subclasses: vec![]
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
                    match serde_json::from_str::<serde_json::Value>(&entry) {
                        Ok(evals) => {
                            if let Err(e) = self.add_entry(&evals, &entry) {
                                eprintln!("[{:?}] {e}", epath);
                            }
                        }
                        Err(e) => eprintln!("[{:?}] Invalid JSON: {e}", epath),
                    }
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
                push!(entry, self.backgrounds, Background)
            }
            Some("feature") => {
                println!("Features not yet implemented.");
                // feature impl goes here
            }
            Some("class") => {
                match &evals["type"].as_str() {
                    Some("primary") => push!(entry, self.classes, Class),
                    Some("subclass") => push!(entry, self.subclasses, Subclass),
                    _ => return Err(
                        io::Error::new(
                            io::ErrorKind::InvalidData, format!("key-value error (\"type\": {})", evals["type"])
                        )
                    ),
                }
            }
            Some("feat") => {
                push!(entry, self.feats, Feat)
            }
            Some("race") => {
                push!(entry, self.races, Race)
            }
            _ => return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, format!("key-value error (\"category\": {})", evals["category"])
                )
            ),
        }

        Ok(())

    }

}



// EOF
