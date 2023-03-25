//! # strkt
//!
//! strkt is a simple utility to transform structured (tabular) data,
//! by applying a global and per-record template to it. Supported
//! input formats are csv and json. Json data must be provided as an
//! array of objects. Templates are expected to be in
//! [Handlebars](https://handlebarsjs.com/) format.


use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use handlebars::Handlebars;
use clap::{App, Arg};
use std::collections::BTreeMap;
use std::io::Write;
use std::ffi::OsStr;
use std::path::Path;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Record(HashMap<String, Value>);

#[derive(Serialize)]
struct RecordBlob {
    data: Vec<String>,
}

/// A function signature to process data
type ProcessFunc = fn(&mut File, &Handlebars) -> Result<Vec<String>, Box<dyn Error>>;


fn parse_args() -> clap::ArgMatches {
    App::new("strkt")
        .version("0.1.0")
        .about("Structured data renderer")
        .arg(
            Arg::with_name("input_file")
                .help("The input data file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("record_template")
                .help("The record template file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("global_template")
                .help("The global template file")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("output_file")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("The output file, or '-' for stdout")
                .takes_value(true),
        )
        .get_matches()
}

/// Process json-formatted data -- data must be an array of json objects
fn process_json_data(input_file: &mut File, handlebars: &Handlebars) -> Result<Vec<String>, Box<dyn Error>> {
    let json_data: Vec<Record> = serde_json::from_reader(input_file)?;
    let mut outvec = Vec::new();

    for record in json_data {
        let record_str: HashMap<String, String> = record
            .0
            .into_iter()
            .map(|(k, v)| match v {
                serde_json::Value::String(s) => (k, s),
                _ => (k, v.to_string()),
            })
            .collect();
        let rendered = handlebars.render("record", &record_str)?;
        outvec.push(rendered);
    }

    Ok(outvec)
}

/// Process csv formatted data
fn process_csv_data(input_file: &mut File, handlebars: &Handlebars) -> Result<Vec<String>, Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_reader(input_file);
    let csv_data: Vec<Record> = csv_reader.deserialize().collect::<Result<_, _>>()?;
    let mut outvec = Vec::new();

    for record in csv_data {
        let record_str: HashMap<String, String> = record
            .0
            .into_iter()
            .map(|(k, v)| match v {
                serde_json::Value::String(s) => (k, s),
                _ => (k, v.to_string()),
            })
            .collect();
        let rendered = handlebars.render("record", &record_str)?;
        outvec.push(rendered);
    }

    Ok(outvec)
}

/// Build a map of file extensions to processing functions
fn build_dispatcher() -> BTreeMap<String, ProcessFunc> {
    let mut dispatcher: BTreeMap<String, ProcessFunc> = BTreeMap::new();
    dispatcher.insert("json".to_string(), process_json_data);
    dispatcher.insert("csv".to_string(), process_csv_data);
    dispatcher
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_args();

    let input_file_path = matches.value_of("input_file").unwrap();
    let record_template_path = matches.value_of("record_template").unwrap();
    let global_template_path = matches.value_of("global_template").unwrap();

    let mut input_file = File::open(input_file_path)?;
    let record_template = std::fs::read_to_string(record_template_path)?;
    let global_template = std::fs::read_to_string(global_template_path)?;

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("record", record_template)?;
    handlebars.register_template_string("global", global_template)?;

    let dispatcher = build_dispatcher();

    let file_ext = Path::new(input_file_path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    let process_fn = dispatcher.get(file_ext).ok_or_else(|| {
        format!(
            "Unsupported file format: {}. Supported formats are: json, csv",
            file_ext
        )
    })?;

    let records = process_fn(&mut input_file, &handlebars)?;

    let recordblob = RecordBlob { data: records };
    let rendered_doc = handlebars.render("global", &recordblob)?;

    match matches.value_of("output_file") {
        Some(path) if path != "-" => {
            let mut output_file = File::create(path)?;
            writeln!(output_file, "{}", rendered_doc)?;
        }
        _ => {
            println!("{}", rendered_doc);
        }
    }

    Ok(())
}
