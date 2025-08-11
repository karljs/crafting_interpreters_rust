use std::path::PathBuf;

use bytecode::{run_from_source, run_repl};
use clap::Parser;
use clio::Input;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(filename) = &args.file {
        let reader = Box::new(Input::new(filename)?);
        run_from_source(reader)
    } else {
        run_repl()
    }
}
