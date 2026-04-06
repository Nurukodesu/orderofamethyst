use winnow::Parser;

use crate::{ast::IR, parser::{block, expression::expression}, symbol::SymbolTable};

mod ast;
mod parser;
mod symbol;

fn main() {
	let mut test_file = include_str!("test.ooa");
	let p = block.parse_next(&mut test_file);
	println!("{:#?}", p);

    let mut input = "(1, 1, 1)";
	let mut symtable: SymbolTable = SymbolTable::new();
	let mut stack_height: usize = 0;
	let mut ir: IR = IR{instructions: vec![]};
    let p = expression.parse_next(&mut input).unwrap();
	p.evaluate(&mut symtable, &mut stack_height, &mut ir);
    println!("{:#?}", p);
	println!("{:#?}", ir);
	println!("{}", stack_height);
}
