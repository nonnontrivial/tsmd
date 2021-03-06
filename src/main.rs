//! CLI tool that generates markdown tables from interfaces in TypeScript files.

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

mod parser;

use crate::parser::Parser;

use anyhow::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;
use tokio::fs;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tsmd",
    version = env!("CARGO_PKG_VERSION"),
    about = "
        generate markdown tables from type script interfaces
    "
)]
struct Opt {
    /// Filepath to .ts source
    #[structopt(parse(from_os_str), short, long, required = true)]
    source_filepath: PathBuf,
    /// Characters that should should prefix interface names in generated markdown
    #[structopt(short, long, default_value = "##")]
    interface_prefix: String,
    /// Exclusively include exported interfaces
    #[structopt(short, long)]
    exported_only: bool,
}

/// Transforms interface hashmap to string of actual table contents
fn transform_interfaces(
    interfaces: HashMap<String, HashMap<String, String>>,
    interface_prefix: &str,
) -> Result<String> {
    let mut output = String::new();
    // Iterate over borrow of hash map in order to build a string containing
    // formatted table
    for (interface, contents) in &interfaces {
        output.push_str(&format!("{} {}\n\n", interface_prefix, interface));
        output.push_str(
            &contents.iter().fold(
                String::from("| Field | Type |\n| --- | --- |\n"),
                |acc, (key, value)| {
                    let line = format!("| {} | `{}` |\n", key, value);
                    let string = &line[..];
                    acc + string
                },
            )[..],
        );
    }
    Ok(output)
}

/// Reads .ts input from options and writes .md output to file of same name.
async fn handle_file_input(opt: &Opt) -> Result<(), Error> {
    if opt.source_filepath.extension() != Some(OsStr::new("ts")) {
        return Err(anyhow!("source_filepath must have .ts extension"));
    }

    let contents = fs::read_to_string(&opt.source_filepath).await?;
    let parser: Parser = Parser::new(opt.exported_only);
    let interfaces = parser.collect_interface_map(&contents)?;
    let md_content = transform_interfaces(interfaces, &opt.interface_prefix)?;
    let md_filepath = opt.source_filepath.to_str().unwrap().replace(".ts", ".md");
    if Path::exists(Path::new(&md_filepath)) {
        fs::remove_file(&md_filepath).await?;
    }

    fs::write(&md_filepath, md_content.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    if let Err(err) = handle_file_input(&opt).await {
        eprintln!("{}", err);
        process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_of_fields() {
        let parser = Parser::new(false);
        let file_contents = String::from("interface A {\n\ta: 0;\n\tb: 0;\n\tc: 0;\n}");
        let interfaces = parser.collect_interface_map(&file_contents).unwrap();
        assert_eq!(interfaces["A"].len(), 3);
    }
}
