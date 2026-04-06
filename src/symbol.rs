use std::collections::HashMap;

use crate::ast::Parameter;

#[derive(Debug, Clone)]
pub struct SymbolTable(Vec<HashMap<String, Symbol>>);

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symtype: SymbolType,
    pub depth: usize,
    pub stack_position: usize,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Function(Vec<Parameter>),
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

    pub fn define(&mut self, name: String, symtype: SymbolType, stack_position: usize) {
        let depth = self.0.len() - 1;
        match self.0.last_mut() {
            Some(s) => {
                s.insert(
                    name.clone(),
                    Symbol {
                        name,
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

    pub fn lookup(&self, name: String) -> Option<&Symbol> {
        for scope in self.0.iter().rev() {
            if let Some(symbol) = scope.get(&name) {
                return Some(symbol);
            }
        }
        None
    }
}
