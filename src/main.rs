use std::{
    io::Write,
    process::Command,
};

use clap::{App, Arg};

mod parser;
mod model;
mod gviz;
mod trie;

use parser::{parse_turns};
use model::{PlayerMove};
use trie::{Trie};

fn main() {
    let matches = App::new("chessgraph")
        .about("Graph chess games into a trie")
        .version("0.0.1")
        .arg(
            Arg::new("output")
            .short('o')
            .long("output")
            .help("The filename for the graph image")
            .takes_value(true)
            .default_value("chessgraph.png")
        )
        .arg(
            Arg::new("format")
            .short('f')
            .long("format")
            .help("Image format for the output graph")
            .takes_value(true)
            .default_value("png")
        )
        .arg(
            Arg::new("files")
            .help("Input files in PGN format")
            .required(true)
            .takes_value(true)
            .multiple_values(true)
        )
        .get_matches();

    let mut graph = Trie::<PlayerMove>::new();

    let files: Vec<&str> = matches.values_of("files").unwrap().collect();
    let intermidiate_file = format!("{}{}chessgraph_temp.dot",
        std::env::temp_dir().display(),
        std::path::MAIN_SEPARATOR
    );

    for file in files {
        match std::fs::read_to_string(file) {
            Ok(contents) => match parse_turns(&contents) {
                Ok((_, turns)) => graph.add_path(turns),
                Err(e) => println!("{}", e),
            },
            Err(e) => println!("{}", e),
        }
    }

    match std::fs::File::create(&intermidiate_file) {
        Ok(mut f) => match write!(&mut f, "digraph{{\n{}\n}}", graph) {
            Ok(_) => {
                Command::new("dot")
                .arg(format!("-T{}", matches.value_of("format").unwrap()))
                .arg(&intermidiate_file)
                .arg("-o")
                .arg(matches.value_of("output").unwrap())
                .output()
                .expect("Could not compile dot file");
            },
            Err(e) => println!("{}", e),
        },
        Err(e) => println!("{}", e),
    }
}
