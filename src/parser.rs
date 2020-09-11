use std::collections::HashMap;
use anyhow::*;

const INTERFACE: &'static str = "interface";

pub struct Parser {
    pub exported_interfaces_only: bool,
}

impl Parser {
    pub fn new(exported_interfaces_only: bool) -> Self {
        Self {
            exported_interfaces_only,
        }
    }
    /// Create hashmap relating interface names to hashmap of key value pairs
    pub fn collect_interface_map(
        &self,
        contents: &str,
    ) -> Result<HashMap<String, HashMap<String, String>>> {
        let mut interface_match_sequence = String::from(INTERFACE);
        // Alter the match sequence given relevant command line flag
        if self.exported_interfaces_only {
            interface_match_sequence = format!("export {}", INTERFACE);
        }

        let mut interfaces = HashMap::new();
        let mut line_index = 0;
        // Match each line against the determined interface sequence
        for line in contents.lines() {
            match line.find(&interface_match_sequence) {
                Some(index) => {
                    let interface_name: String = line
                        .chars()
                        .skip(index + &interface_match_sequence.len())
                        .take_while(|c| c != &'<' && c != &'{')
                        .collect();
                    // Relate the interface name to key, value pairs that make
                    // up its fields
                    interfaces.insert(
                        interface_name.trim().to_string(),
                        contents
                            .lines()
                            .skip(line_index + 1)
                            .take_while(|line| *line != "}")
                            .fold(HashMap::new(), |mut acc, x| {
                                let pair: Vec<String> =
                                    x.split(":").map(|c| c.trim().replace(";", "")).collect();
                                if pair.len() != 2 {
                                    return acc;
                                }

                                let mut key = pair[0].to_string();
                                let value = pair[1].to_string();
                                // Handle optional fields
                                if key.ends_with("?") {
                                    key = format!("{} (optional)", &key[..key.len() - 1]);
                                }

                                acc.insert(key, value);
                                acc
                            }),
                    );
                }
                None => {}
            }
            line_index += 1;
        }
        Ok(interfaces)
    }
}
