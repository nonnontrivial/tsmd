#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! generate markdown tables from type script interfaces
//!

use anyhow::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use tokio::fs;
use tokio::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tmg",
    version = env!("CARGO_PKG_VERSION"),
    about = "
        generate markdown tables from type script interfaces
    "
)]
struct Opt {
    /// source_filepath to .ts source.
    #[structopt(parse(from_os_str), short, long, required = true)]
    source_filepath: PathBuf,
}

const INTERFACE: &'static str = "interface";

/// Collects parsed contents of all interfaces in the source file.
///
/// Returns HashMap which associates interfaceName -> HashMap<keyInInterface, valueInInterface>
fn collect_interfaces(contents: String) -> Result<HashMap<String, HashMap<String, String>>> {
    let mut interfaces = HashMap::new();
    let mut line_index = 0;
    for line in contents.lines() {
        match line.find(INTERFACE) {
            Some(index) => {
                let interface_name: String = line
                    .chars()
                    .skip(index + INTERFACE.len())
                    .take_while(|c| c != &'<' && c != &'{')
                    .collect(); // let content_map: HashMap<String, String> = HashMap::new();
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

/// Unpacks hashmap describing interfaces to actual table contents.
fn transform_interfaces_to_md_content(
    interfaces: HashMap<String, HashMap<String, String>>,
) -> Result<String> {
    let mut output = String::new();
    for (interface, contents) in interfaces {
        output.push_str(&format!("## {}\n\n", interface));
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
    let str = String::from(contents);
    let interfaces = collect_interfaces(str)?;
    let md_content = transform_interfaces_to_md_content(interfaces)?;

    let md_filepath = opt.source_filepath.to_str().unwrap().replace(".ts", ".md");
    let mut file = fs::File::create(md_filepath).await?;
    file.write_all(md_content.as_bytes()).await?;
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
    // use super::*;
    #[test]
    fn interface_collection() {
        unimplemented!();
    }

    #[test]
    fn md_content_transformation() {
        unimplemented!();
    }
}
