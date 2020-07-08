use std::collections::HashMap;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn collect_interface_map(&self, contents: &str) -> HashMap<&str, HashMap<&str, &str>> {
        unimplemented!()
    }
}
