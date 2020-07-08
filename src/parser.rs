use std::collections::HashMap;

pub struct Parser {
    pub exported_interfaces_only: bool,
}

impl Parser {
    pub fn new(exported_interfaces_only: bool) -> Self {
        Self {
            exported_interfaces_only,
        }
    }
    pub fn collect_interface_map(&self, contents: &str) -> HashMap<&str, HashMap<&str, &str>> {
        unimplemented!()
    }
}
