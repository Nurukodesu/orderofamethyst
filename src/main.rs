use std::{collections::HashMap, fs::read_to_string};

use winnow::Parser;

use crate::{
    ast::{IR, Sigil},
    parser::block,
    symbol::SymbolTable,
};

use clap::Parser as CLIParser;

mod ast;
mod parser;
mod symbol;

macro_rules! include_files {
    ($($file:expr),* $(,)?) => {
        concat!(
            $(include_str!($file), "\n"),*
        )
    };
}

#[derive(CLIParser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    #[arg(short, long, default_value = "output.hexpattern")]
    output: String,
    #[arg(short = 'O', default_value = "1")]
    optimization_level: u8,
    #[arg[short, long]]
    print: bool,
}

fn main() {
    let args = Args::parse();

    let mut sigiltable: HashMap<String, Sigil> = HashMap::new();
    let mut symtable: SymbolTable = SymbolTable::new();
    let mut stack_height: usize = 0;
    let mut ir: IR = IR {
        instructions: vec![],
        blocks: vec![],
    };

    let mut sigil_files = include_files!("sigil/base.ooa", "sigil/internal.ooa");
    let p = block.parse_next(&mut sigil_files).unwrap();
    p.evaluate(&mut symtable, &mut stack_height, &mut sigiltable, &mut ir);

    let input = read_to_string(args.input).expect("File not found");
    match block.parse(&mut input.as_str()) {
        Ok(mut p) => {
            if args.optimization_level == 1 {
                p.optimize_lvl1();
            }

            p.walk(&mut symtable, &mut stack_height, &mut sigiltable, &mut ir);

            let ir2 = ir.resolve_symbols(&sigiltable);

            if args.print {
                println!("{}", ir2.generate_hexpattern())
            } else {
                _ = ir2.write_to(&args.output);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
