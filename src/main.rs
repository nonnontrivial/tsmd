#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! generate markdown tables from type script interfaces
//!
//! ## example
//! ```shell
//! cargo r -- -s ./input.ts -i "#"
//! ```
//!

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
    /// Filepath to .ts source.
    #[structopt(parse(from_os_str), short, long, required = true)]
    source_filepath: PathBuf,
    /// Characters that should should prefix interface names in markdown
    #[structopt(short, long, default_value = "##")]
    interface_prefix: String,
    /// Whether only exported interfaces should be parsed
    #[structopt(short, long)]
    exported_only: bool,
}

/// Unpacks hashmap describing interfaces to actual table contents.
fn transform_interfaces_to_md_content(
    interfaces: HashMap<String, HashMap<String, String>>,
    interface_prefix: &str,
) -> Result<String> {
    let mut output = String::new();

    for (interface, contents) in interfaces {
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

/// Reads .ts input and writes .md output.
async fn handle_file_input(opt: &Opt) -> Result<(), Error> {
    if opt.source_filepath.extension() != Some(OsStr::new("ts")) {
        return Err(anyhow!("source_filepath must have .ts extension"));
    }

    let contents = fs::read_to_string(&opt.source_filepath).await?;
    let parser: Parser = Parser::new(opt.exported_only);
    let interfaces = parser.collect_interface_map(&contents)?;

    let md_content = transform_interfaces_to_md_content(interfaces, &opt.interface_prefix)?;
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
mod tests {
    use super::*;

    #[test]
    fn interface_collection() {
        let parser = Parser::new(false);
        let mut file_contents = String::from("");
        file_contents.push_str("export interface {");
        file_contents.push_str("  thing: 42;");
        file_contents.push_str("}");
        let interfaces = parser.collect_interface_map(&file_contents).unwrap();
        assert_eq!(interfaces.keys().len(), 1);
    }

    #[test]
    fn md_content_transformation() {
        let interfaces = HashMap::new();
        let md_content = transform_interfaces_to_md_content(interfaces, "##").unwrap();
        assert_eq!(md_content, "");
    }
}
