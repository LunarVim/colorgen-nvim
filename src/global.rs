use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Global(LinkedHashMap<String, String>);

impl Display for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- set global variables")?;
        for (key, value) in &self.0 {
            writeln!(f, "vim.g.{key} = \"{value}\"")?;
        }
        Ok(())
    }
}
