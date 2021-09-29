use clap::{App, Arg, SubCommand};
use petgraph::graph::Graph;
use std::io::prelude::*;

mod lex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Dotfile Parser")
        .version("1.0")
        .author("Samuel Resendez") .arg(
            Arg::with_name("INPUT_FILE")
                .short("f")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let filename = matches.value_of("INPUT_FILE").unwrap();
    let mut file = std::fs::File::open(filename)?;

    let mut dotfile_text = String::new();
    file.read_to_string(&mut dotfile_text);
    let petgraph = parse_to_graph(dotfile_text);
    Ok(())
}


fn parse_to_graph(dotfile: String) -> Graph<String, String> {
    todo!()
}
