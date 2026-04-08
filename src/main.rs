use std::{collections::HashMap, env::args, fs::read_to_string};

use winnow::Parser;

use crate::{ast::{IR, Sigil}, parser::block, symbol::SymbolTable};

mod ast;
mod parser;
mod symbol;

fn main() {
	let mut sigiltable: HashMap<String, Sigil> = HashMap::new();
	let mut symtable: SymbolTable = SymbolTable::new();
	let mut stack_height: usize = 0;
	let mut ir: IR = IR{instructions: vec![], blocks: vec![]};

	let mut internal_sigil_file = include_str!("sigil/internal.ooa");
	let p = block.parse_next(&mut internal_sigil_file).unwrap();
	p.evaluate(&mut symtable, &mut stack_height, &mut sigiltable, &mut ir);	
	let mut base_sigil_file = include_str!("sigil/base.ooa");
	let p = block.parse_next(&mut base_sigil_file).unwrap();
	p.evaluate(&mut symtable, &mut stack_height, &mut sigiltable, &mut ir);	


	let args: Vec<String> = args().collect();
	let input = read_to_string(&args[1]).expect("File not found");
	let mut p = block.parse_next(&mut input.as_str()).unwrap();
	p.optimize_lvl1();
	
	p.walk(&mut symtable, &mut stack_height, &mut sigiltable, &mut ir);
	
	_ = ir.resolve_symbols(&sigiltable).write_to(&args[2]);
}
