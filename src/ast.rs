use std::{collections::HashMap, fs::File, io::Write, mem::take, str::FromStr, vec};

use crate::symbol::{Symbol, SymbolTable, SymbolType, find_symbol_position};

#[derive(Debug, Clone)]
pub enum Expression {
    Num(f64),
    Vector(f64, f64, f64),
    Bool(bool),
    Id(String),
    Call(String, Vec<Expression>),
    Pattern(SigilCall),
    List(Vec<Expression>),
    BinaryOps {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        op: Op,
    },
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
	Power,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone)]
pub enum IotaType {
    Any,
    Num,
    Vector,
    Bool,
    Entity,
    Pattern,
    List,
    Many,
    Void,
}

#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        value: Expression,
        var_type: IotaType,
    },
    FnDecl {
        name: String,
        params: Vec<Parameter>,
        ret_type: IotaType,
        body: Vec<Statement>,
    },
    RawFnDecl {
        name: String,
        params: Vec<Parameter>,
        ret_type: IotaType,
        body: Vec<SigilCall>,
    },
    Expr(Expression),
    Return(Expression),
    SigilDecl(Sigil),
    Conditional {
        condition: Expression,
        then_block: Block,
        else_block: Block,
    },
	Empty
}
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub id: String,
    pub r#type: IotaType,
}

#[derive(Debug, Clone)]
pub struct SigilCall {
    pub id: String,
    pub modifier: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Direction {
    NORTHEAST,
    EAST,
    SOUTHEAST,
    SOUTHWEST,
    WEST,
    NORTHWEST,
    NONE,
}

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Forward,    // w
    Right,      // e
    RightSharp, // d
    Left,       // q
    LeftSharp,  // a
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnglePath(Vec<Angle>);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Sigil {
    pub name: String,
    pub params: Vec<IotaType>,
    pub returns: Vec<IotaType>,
    pub initial_direction: Direction,
    pub angle_path: AnglePath,
}

impl AnglePath {
    pub fn empty() -> Self {
        AnglePath(vec![])
    }
}

impl FromStr for AnglePath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| match c {
                'w' => Ok(Angle::Forward),
                'e' => Ok(Angle::Right),
                'd' => Ok(Angle::RightSharp),
                'q' => Ok(Angle::Left),
                'a' => Ok(Angle::LeftSharp),
                _ => Err(format!("Invalid hex angle: {}", c)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(AnglePath)
    }
}

#[derive(Debug)]
pub struct IR {
    pub instructions: Vec<SigilCall>,
    pub blocks: Vec<HashMap<String, usize>>,
}

impl IR {
    pub fn new() -> Self {
        IR {
            instructions: vec![],
            blocks: vec![],
        }
    }

    pub fn push(&mut self, id: &str, modifier: Option<&str>) {
        self.instructions.push(SigilCall {
            id: id.to_string(),
            modifier: match modifier {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        });
    }

    pub fn reserve_block(&mut self) -> usize {
        let index = self.blocks.len();
        self.blocks.push(HashMap::new());
        index
    }

    pub fn set_block(&mut self, index: usize, symtable: &SymbolTable) {
        let internal_symtable: HashMap<String, usize> = symtable
            .get_last_scope()
            .into_iter()
            .map(|(name, symbol)| (name.clone(), symbol.stack_position))
            .collect();
        self.blocks[index] = internal_symtable;
    }

    pub fn resolve_symbols(&self, sigiltable: &HashMap<String, Sigil>) -> Self {
        let mut ir2 = IR::new();
        let mut stack_bases = vec![0];
        let mut scope_indexes = vec![];
        let special_cases = ["Bookkeeper's Gambit"];
        let mut stack_height = 0;
        for instruction in self.instructions.iter() {
			//print!("{}: {}	", instruction.id, instruction.modifier.clone().unwrap_or_default());
            match sigiltable.get(&instruction.id) {
                Some(s) => {
                    if !special_cases.contains(&s.name.as_str()) {
                        stack_height += s.returns.len();
						stack_height -= s.params.len();
                    } else {
						 match s.name.as_str(){
							 "Bookkeeper's Gambit" => {
								let pop_count = instruction.modifier.clone().expect("Unknown Bookkeeper's Format").matches('v').count();
								stack_height -= pop_count;
							 },
							_ => todo!()
						 }
					}
                }
                None => eprintln!("Unknown Sigil: {}", instruction.id),
            }
			//println!("Stack height: {stack_height}");

            match instruction.id.as_str() {
                "__MARK_BASE" => stack_bases.push(stack_height),
                "__MARK_END" => _ = stack_bases.pop().unwrap() + 1,
                "__START_SCOPE" => {
                    let index: usize = instruction.modifier.as_ref().unwrap().parse().unwrap();
                    scope_indexes.push(index);
                }
                "__END_SCOPE" => {
                    scope_indexes.pop();
                }
                "__LOAD_SYMBOL" => {
                    let symname = instruction.modifier.as_ref().unwrap();
                    let (symbol_position, depth) = find_symbol_position(
                        symname,
                        &self.blocks,
                        &scope_indexes,
                    )
                    .expect(&format!("Unknown Symbol: {}", symname));
					let base_height = stack_bases[depth];
                    let offset = (stack_height - base_height)- symbol_position;
                    ir2.push("Numerical Reflection", Some(&offset.to_string()));
                    ir2.push("Fisherman's Gambit II", None);
                }
                _ => ir2.instructions.push(instruction.clone()),
            }
        }
		ir2
    }

	pub fn generate_hexpattern(&self) -> String{
		let mut output = String::new();
		for instruction in self.instructions.clone(){
			match instruction.modifier{
				Some(s) => {
					output += &format!("{}: {}\n", instruction.id, s);
				},
				None => {
					output += &(instruction.id + "\n");
				},
			}
		}
		output
	}
	
	pub fn write_to(&self, filename: &str) -> std::io::Result<()>{
		let mut file = File::create(filename)?;
		file.write_all(self.generate_hexpattern().as_bytes())?;
		Ok(())
	}
}

impl Expression {
    pub fn evaluate(&self, symtable: &mut SymbolTable, stack_height: &mut usize, ir: &mut IR) {
        match self {
            Expression::Num(n) => {
                ir.push("Numerical Reflection", Some(&n.to_string()));
                *stack_height += 1;
            }
            Expression::Vector(x, y, z) => {
                ir.push("Numerical Reflection", Some(&x.to_string()));
                ir.push("Numerical Reflection", Some(&y.to_string()));
                ir.push("Numerical Reflection", Some(&z.to_string()));
                ir.push("Vector Exaltation", None);
                ir.instructions.push(SigilCall {
                    id: "Vector Exaltation".to_string(),
                    modifier: None,
                });
                *stack_height += 1;
            }
            Expression::Bool(b) => {
                ir.instructions.push(SigilCall {
                    id: (if *b {
                        "True Reflection"
                    } else {
                        "False Reflection"
                    })
                    .to_string(),
                    modifier: None,
                });
                *stack_height += 1;
            }
            Expression::Id(name) => {
                let symbol = symtable.lookup(name);
                match symbol {
                    Some(s) => {
                        Self::load_symbol(s, ir, symtable.get_depth() == 0, symtable.get_depth());
                        *stack_height += 1;
                    }
                    None => eprintln!("Undefined symbol: {}", name),
                }
            }
            Expression::Call(name, args) => {
                let fn_symbol = symtable.lookup(name).cloned();
                ir.push("__MARK_BASE", None);
                for arg in args {
                    arg.evaluate(symtable, &mut *stack_height, ir);
                }
                match fn_symbol {
                    Some(s) => {
                        if s.stack_position != 0 {
                            Self::load_symbol(
                                &s,
                                ir,
                                symtable.get_depth() == 0,
                                symtable.get_depth(),
                            );
                            ir.instructions.push(SigilCall {
                                id: "Hermes' Gambit".to_string(),
                                modifier: None,
                            });
                            *stack_height += match s.iotatype {
                                IotaType::Void => 0,
                                _ => 1,
                            } - args.len();
                        } else {
                            if let SymbolType::Function { body_ir } = s.symtype {
                                ir.instructions.extend(body_ir);
                            }
                        }
                        ir.instructions.push(SigilCall {
                            id: "__MARK_END".to_string(),
                            modifier: None,
                        });
                        match s.iotatype {
                            IotaType::Void => (),
                            _ => *stack_height += 1,
                        }
                    }
                    None => eprintln!("Undefined function: {}()", name),
                }
            }
            Expression::Pattern(sigil) => {
                let modifier = match sigil.modifier.clone() {
                    Some(s) => format!("{}{}", sigil.id, s),
                    None => sigil.id.clone(),
                };
                ir.instructions.push(SigilCall {
                    id: "Consideration".to_string(),
                    modifier: Some(modifier),
                });
                *stack_height += 1;
            }
            Expression::List(expressions) => {
                for expr in expressions {
                    expr.evaluate(symtable, stack_height, ir);
                }

                let item_count: usize = expressions.len();
                ir.instructions.push(SigilCall {
                    id: "Numerical Reflection".to_string(),
                    modifier: Some(item_count.to_string()),
                });
                ir.instructions.push(SigilCall {
                    id: "Flock Gambit".to_string(),
                    modifier: None,
                });
                *stack_height -= item_count - 1;
            }
            Expression::BinaryOps { lhs, rhs, op } => {
                lhs.evaluate(symtable, stack_height, ir);
                rhs.evaluate(symtable, stack_height, ir);
                let instruction = match op {
                    Op::Add => "Additive Distillation",
                    Op::Sub => "Subtractive Distillation",
                    Op::Mul => "Multiplicative Distillation",
                    Op::Div => "Division Distillation",
                    Op::Mod => "Modulus Distillation",
					Op::Power => "POwer Distillation",
                    Op::Equal => "Equality Distillation",
                    Op::NotEqual => "Inequality Distillation",
                    Op::GreaterThan => "Maximus Distillation",
                    Op::LessThan => "Minimus Distillation",
                    Op::GreaterEqual => "Maximus Distillation II",
                    Op::LessEqual => "Minimus Distillation II",
                }
                .to_string();
                ir.instructions.push(SigilCall {
                    id: instruction,
                    modifier: None,
                });
                *stack_height -= 1;
            }
        }
    }

    pub fn simplify(&mut self) {
        match self {
            Expression::BinaryOps { lhs, rhs, op } => {
                lhs.simplify();
                rhs.simplify();

                match (lhs.as_ref(), rhs.as_ref()) {
                    (Expression::Num(a), Expression::Num(b)) => match op {
                        Op::Add => *self = Expression::Num(a + b),
                        Op::Sub => *self = Expression::Num(a - b),
                        Op::Mul => *self = Expression::Num(a * b),
                        Op::Div => *self = Expression::Num(a / b),
                        Op::Mod => *self = Expression::Num(a % b),
						Op::Power => *self = Expression::Num(a.powf(*b)),
                        Op::Equal => *self = Expression::Bool(a == b),
                        Op::NotEqual => *self = Expression::Bool(a != b),
                        Op::GreaterThan => *self = Expression::Bool(a > b),
                        Op::LessThan => *self = Expression::Bool(a < b),
                        Op::GreaterEqual => *self = Expression::Bool(a >= b),
                        Op::LessEqual => *self = Expression::Bool(a <= b),
                    },
                    (Expression::Num(n), Expression::Vector(x, y, z)) => match op {
                        Op::Add => *self = Expression::Vector(n + x, n + y, n + z),
                        Op::Sub => *self = Expression::Vector(n - x, n - y, n - z),
                        Op::Mul => *self = Expression::Vector(n * x, n * y, n * z),
                        Op::Div => *self = Expression::Vector(n / x, n / y, n / z),
                        Op::Mod => *self = Expression::Vector(n % x, n % y, n % z),
						Op::Power => *self = Expression::Vector(n.powf(*x), n.powf(*y), n.powf(*z)),
                        Op::Equal => *self = Expression::Bool(false),
                        Op::NotEqual => *self = Expression::Bool(false),
                        Op::GreaterThan => *self = Expression::Bool(false),
                        Op::LessThan => *self = Expression::Bool(false),
                        Op::GreaterEqual => *self = Expression::Bool(false),
                        Op::LessEqual => *self = Expression::Bool(false),
                    },
                    (Expression::Vector(x, y, z), Expression::Num(n)) => match op {
                        Op::Add => *self = Expression::Vector(x + n, y + n, z + n),
                        Op::Sub => *self = Expression::Vector(x - n, y - n, z - n),
                        Op::Mul => *self = Expression::Vector(x * n, y * n, z * n),
                        Op::Div => *self = Expression::Vector(x / n, y / n, z / n),
                        Op::Mod => *self = Expression::Vector(x % n, y % n, z % n),
						Op::Power => *self = Expression::Vector(x.powf(*n), y.powf(*n), z.powf(*n)),
                        Op::Equal => *self = Expression::Bool(false),
                        Op::NotEqual => *self = Expression::Bool(false),
                        Op::GreaterThan => *self = Expression::Bool(false),
                        Op::LessThan => *self = Expression::Bool(false),
                        Op::GreaterEqual => *self = Expression::Bool(false),
                        Op::LessEqual => *self = Expression::Bool(false),
                    },
                    (Expression::Vector(x1, y1, z1), Expression::Vector(x2, y2, z2)) => match op {
                        Op::Add => *self = Expression::Vector(x1 + x2, y1 + y2, z1 + z2),
                        Op::Sub => *self = Expression::Vector(x1 - x2, y1 - y2, z1 - z2),
                        Op::Mul => *self = Expression::Vector(x1 * x2, y1 * y2, z1 * z2),
                        Op::Div => *self = Expression::Vector(x1 / x2, y1 / y2, z1 / z2),
                        Op::Mod => *self = Expression::Vector(x1 % x2, y1 % y2, z1 % z2),
						Op::Power => (),
                        Op::Equal => *self = Expression::Bool(x1 == x2 && y1 == y2 && z1 == z2),
                        Op::NotEqual => *self = Expression::Bool(x1 != x2 && y1 != y2 && z1 != z2),
                        Op::GreaterThan => *self = Expression::Bool(x1 > x2 && y1 > y2 && z1 > z2),
                        Op::LessThan => *self = Expression::Bool(x1 < x2 && y1 < y2 && z1 < z2),
                        Op::GreaterEqual => {
                            *self = Expression::Bool(x1 >= x2 && y1 >= y2 && z1 >= z2)
                        }
                        Op::LessEqual => *self = Expression::Bool(x1 <= x2 && y1 <= y2 && z1 <= z2),
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn load_symbol(symbol: &Symbol, ir: &mut IR, is_from_global: bool, current_depth: usize) {
        if is_from_global {
            ir.instructions.push(SigilCall {
                id: "__LOAD_SYMBOL".to_string(),
                modifier: Some(symbol.name.clone()),
            });
        } else {
            if symbol.depth != current_depth {
                ir.instructions.push(SigilCall {
                    id: "__LOAD_SYMBOL".to_string(),
                    modifier: Some(symbol.name.clone()),
                });
            } else {
                ir.instructions.push(SigilCall {
                    id: "__LOAD_SYMBOL_REL".to_string(),
                    modifier: Some(symbol.name.clone()),
                });
            }
        }
    }
}

impl Statement {
    pub fn evaluate(
        &self,
        symtable: &mut SymbolTable,
        stack_height: &mut usize,
        sigiltable: &mut HashMap<String, Sigil>,
        ir: &mut IR,
    ) {
        match self {
            Statement::VarDecl {
                name,
                value,
                var_type,
            } => {
                value.evaluate(symtable, stack_height, ir);
                symtable.define(
                    name.to_string(),
                    var_type.clone(),
                    SymbolType::Variable,
                    *stack_height,
                );
            }
            Statement::FnDecl {
                name,
                params,
                ret_type,
                body,
            } => {
                let body_ir =
                    Self::compile_function_body(symtable, params, ret_type, ir, |st, sh, ir| {
                        for statement in body {
                            statement.evaluate(st, sh, sigiltable, ir);
                        }
                    });
                self.register_function(name, ret_type.clone(), body_ir, symtable, stack_height, ir);
            }
            Statement::RawFnDecl {
                name,
                params,
                ret_type,
                body,
            } => {
                let body_ir =
                    Self::compile_function_body(symtable, params, ret_type, ir, |_, sh, ir| {
                        for statement in body {
                            ir.instructions.push(statement.clone());
                            *sh += match sigiltable.get(&statement.id) {
                                Some(s) => s.returns.len() - s.params.len(),
                                None => {
                                    eprint!("Unknown sigil: {}", statement.id);
                                    0
                                }
                            };
                        }
                    });
                self.register_function(name, ret_type.clone(), body_ir, symtable, stack_height, ir);
            }
            Statement::Expr(expression) => expression.evaluate(symtable, stack_height, ir),
            Statement::Return(expression) => expression.evaluate(symtable, stack_height, ir),
            Statement::SigilDecl(sigil) => {
                sigiltable.insert(sigil.name.clone(), sigil.clone());
            }
            Statement::Conditional {
                condition,
                then_block,
                else_block,
            } => {
                condition.evaluate(symtable, stack_height, ir);
                then_block.scope(symtable, sigiltable, ir);
                else_block.scope(symtable, sigiltable, ir);
            }
			Statement::Empty => (),
        }
    }

    fn compile_function_body(
        symtable: &mut SymbolTable,
        params: &[Parameter],
        ret_type: &IotaType,
        global_ir: &mut IR,
        body_compiler: impl FnOnce(&mut SymbolTable, &mut usize, &mut IR),
    ) -> Vec<SigilCall> {
        let mut body_ir = IR {
            instructions: Vec::new(),
            blocks: take(&mut global_ir.blocks),
        };

        let mut internal_stack_height = params.len();
        symtable.push();

        for (i, param) in params.iter().enumerate() {
            symtable.define(
                param.id.clone(),
                param.r#type.clone(),
                SymbolType::Variable,
                i + 1,
            );
        }

        let scope_index = body_ir.reserve_block();

        body_ir.push("__START_SCOPE", Some(&scope_index.to_string()));
        body_compiler(symtable, &mut internal_stack_height, &mut body_ir);
        body_ir.push("__END_SCOPE", None);

        if internal_stack_height != 0 {
            let mask = match ret_type {
                IotaType::Void => "v".repeat(internal_stack_height),
                _ => format!("-{}", "v".repeat(internal_stack_height - 1)),
            };
            if mask != "-" {
                body_ir.instructions.push(SigilCall {
                    id: "Bookkeeper's Gambit".into(),
                    modifier: Some(mask),
                });
            }
        }
        body_ir.set_block(scope_index, symtable);
        symtable.pop();
        global_ir.blocks = body_ir.blocks;
        body_ir.instructions
    }

    fn register_function(
        &self,
        name: &str,
        ret_type: IotaType,
        body: Vec<SigilCall>,
        global_symtable: &mut SymbolTable,
        stack_height: &mut usize,
        global_ir: &mut IR,
    ) {
        let body_size = body
            .iter()
            .filter(|sigil| !sigil.id.starts_with("__"))
            .count();
        let is_inlinable = body_size < 20;

        if is_inlinable {
            global_symtable.define(
                name.to_string(),
                ret_type,
                SymbolType::Function { body_ir: body },
                0,
            );
        } else {
            global_ir.instructions.push(SigilCall {
                id: "Introspection".to_string(),
                modifier: None,
            });
            global_ir.instructions.extend(
                body.iter()
                    .map(|s| match s.id.as_str() {
                        "__LOAD_SYMBOL" => SigilCall {
                            id: "__LOAD_SYMBOL_PIE".to_string(),
                            modifier: s.modifier.clone(),
                        },
                        _ => s.clone(),
                    })
                    .collect::<Vec<SigilCall>>(),
            );
            global_ir.instructions.push(SigilCall {
                id: "Retrospection".to_string(),
                modifier: None,
            });
            *stack_height += 1;
            global_symtable.define(
                name.to_string(),
                ret_type,
                SymbolType::Function { body_ir: vec![] },
                *stack_height,
            );
        }
    }
}

impl Block {
    pub fn walk(
        &self,
        symtable: &mut SymbolTable,
        stack_height: &mut usize,
        sigiltable: &mut HashMap<String, Sigil>,
        ir: &mut IR,
    ) {
        let base_index = ir.reserve_block();
        ir.push("__START_SCOPE", Some(&base_index.to_string()));
        self.evaluate(symtable, stack_height, sigiltable, ir);
        ir.push("__END_SCOPE", None);
        ir.set_block(base_index, symtable);
    }

    pub fn evaluate(
        &self,
        symtable: &mut SymbolTable,
        stack_height: &mut usize,
        sigiltable: &mut HashMap<String, Sigil>,
        ir: &mut IR,
    ) {
        for statement in self.statements.iter() {
            statement.evaluate(symtable, stack_height, sigiltable, ir);
        }
    }
    pub fn optimize_lvl1(&mut self) {
        for statement in self.statements.iter_mut() {
            match statement {
                Statement::VarDecl {
                    name: _,
                    value,
                    var_type: _,
                } => value.simplify(),
                Statement::Expr(expression) => expression.simplify(),
                Statement::Return(expression) => expression.simplify(),
                Statement::Conditional {
                    condition,
                    then_block: _,
                    else_block: _,
                } => condition.simplify(),
                _ => (),
            }
        }
    }

    fn scope(
        &self,
        symtable: &mut SymbolTable,
        sigiltable: &mut HashMap<String, Sigil>,
        ir: &mut IR,
    ) {
        symtable.push();
        let mut local_height = 0;
        let scope_index = ir.reserve_block();
        ir.push("__MARK_BASE", None);
        ir.push("__START_SCOPE", Some(&scope_index.to_string()));
        ir.push("Introspection", None);
        self.evaluate(symtable, &mut local_height, sigiltable, ir);
        if local_height != 0 {
            ir.push("Bookkeeper's Gambit", Some(&"v".repeat(local_height)));
        }
        ir.push("Retrospection", None);
        ir.push("__END_SCOPE", None);
        ir.push("__MARK_END", None);
        ir.set_block(scope_index, symtable);
        symtable.pop();
    }
}
