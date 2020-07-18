use anyhow::*;
use std::collections::HashMap;

const INTERFACE: &'static str = "interface";

pub struct Parser {
    pub exported_interfaces_only: bool,
}

impl Parser {
    pub fn new(exported_interfaces_only: bool) -> Self {
        if exported_interfaces_only {
            unimplemented!()
        }
        Self {
            exported_interfaces_only,
        }
    }
    /// Create hashmap relating interface names -> hashmap of key, value pairs
    pub fn collect_interface_map(
        &self,
        contents: &str,
    ) -> Result<HashMap<String, HashMap<String, String>>> {
        let mut interfaces = HashMap::new();
        let mut line_index = 0;

        for line in contents.lines() {
            match line.find(INTERFACE) {
                Some(index) => {
                    let interface_name: String = line
                        .chars()
                        .skip(index + INTERFACE.len())
                        .take_while(|c| c != &'<' && c != &'{')
                        .collect();

                    interfaces.insert(
                        interface_name.trim().to_string(),
                        contents
                            .lines()
                            .skip(line_index + 1)
                            .take_while(|line| *line != "}")
                            .fold(HashMap::new(), |mut acc, x| {
                                if x.is_empty() {
                                    return acc;
                                }
                                let pair: Vec<String> =
                                    x.split(":").map(|c| c.trim().replace(";", "")).collect();
                                acc.insert(pair[0].to_string(), pair[1].to_string());
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
