use std::collections::HashMap;

use crate::ast::{IotaType, SigilCall};

#[derive(Debug, Clone)]
pub struct SymbolTable(Vec<HashMap<String, Symbol>>);

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub iotatype: IotaType,
    pub symtype: SymbolType,
    pub depth: usize,
    pub stack_position: usize,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Function {
        body_ir: Vec<SigilCall>,
    },
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable(vec![HashMap::new()])
    }

    pub fn push(&mut self) {
        self.0.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        if self.0.len() > 0 {
            self.0.pop();
        }
    }

    pub fn define(
        &mut self,
        name: String,
        iotatype: IotaType,
        symtype: SymbolType,
        stack_position: usize,
    ) {
        let depth = self.0.len() - 1;
        match self.0.last_mut() {
            Some(s) => {
                s.insert(
                    name.clone(),
                    Symbol {
                        name,
                        iotatype,
                        symtype,
                        depth,
                        stack_position,
                    },
                );
            }
            None => {
                eprintln!("No scope in Symbol Table");
            }
        };
    }

    pub fn lookup(&self, name: &String) -> Option<&Symbol> {
        for scope in self.0.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn get_last_scope(&self) -> &HashMap<String, Symbol> {
        match self.0.last() {
            Some(s) => s,
            None => todo!(),
        }
    }
	
	pub fn get_depth(&self) -> usize {
		self.0.len() - 1
	}
}

pub fn find_symbol_position(name: &str, tables: &Vec<HashMap<String, usize>>, scope_indexes: &Vec<usize>) -> Option<(usize, usize)> {
	for (depth,index) in scope_indexes.iter().enumerate().rev(){
		match tables[*index].get(name) {
			Some(s) => return Some((*s, depth)),
			None => (),
		};
	}
	None
}
