use std::str::FromStr;

use crate::symbol::{SymbolTable, SymbolType};

#[derive(Debug, Clone)]
pub enum Expression {
    Num(f64),
    Vector(f64, f64, f64),
    Bool(bool),
    Id(String),
    Call(String, Vec<Expression>),
    Pattern(Sigil),
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
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone)]
pub enum IotaType {
    Num,
    Bool,
    Vector,
    Entity,
    Pattern,
    List(Box<IotaType>),
    AnyList,
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

#[derive(Debug)]
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
}

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Forward,    // w
    Right,      // e
    RightSharp, // d
    Left,       // q
    LeftSharp,  // a
}

#[derive(Debug, Clone)]
pub struct AnglePath(Vec<Angle>);

#[derive(Debug, Clone)]
pub struct Sigil {
    pub name: String,
    pub params: Vec<IotaType>,
    pub returns: Vec<IotaType>,
    pub initial_direction: Direction,
    pub angle_path: AnglePath,
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
}

impl Expression {
    pub fn evaluate(&self, symtable: &mut SymbolTable, stack_height: &mut usize, ir: &mut IR) {
        match self {
            Expression::Num(n) => {
                ir.instructions.push(SigilCall {
                    id: "Numerical Reflection".to_string(),
                    modifier: Some(n.to_string()),
                });
                *stack_height += 1;
            }
            Expression::Vector(x, y, z) => {
                ir.instructions.push(SigilCall {
                    id: "Numerical Reflection".to_string(),
                    modifier: Some(x.to_string()),
                });
                ir.instructions.push(SigilCall {
                    id: "Numerical Reflection".to_string(),
                    modifier: Some(y.to_string()),
                });
                ir.instructions.push(SigilCall {
                    id: "Numerical Reflection".to_string(),
                    modifier: Some(z.to_string()),
                });
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
                let symbol = symtable.lookup(name.to_string());
                match symbol {
                    Some(s) => {
                        let offset = *stack_height - s.stack_position;
                        ir.instructions.push(SigilCall {
                            id: "Numerical Reflection".to_string(),
                            modifier: Some(offset.to_string()),
                        });
                        ir.instructions.push(SigilCall {
                            id: "Fisherman's Gambit II".to_string(),
                            modifier: None,
                        });
                        *stack_height += 1;
                    }
                    None => eprintln!("Undefined symbol: {}", name),
                }
            }
            Expression::Call(name, args) => {
                if let Some(param) = symtable.lookup(name.to_string()).cloned() {
                    match param.symtype {
                        SymbolType::Variable => todo!(),
                        SymbolType::Function(parameters) => {
                            for (parameter, arg) in parameters.iter().zip(args) {
                                arg.evaluate(symtable, stack_height, ir);
                                symtable.define(
                                    parameter.id.clone(),
                                    SymbolType::Variable,
                                    *stack_height,
                                );
                            }
                        }
                    }
                }
                symtable.push();
                let symbol = symtable.lookup(name.to_string()).cloned();
                match symbol {
                    Some(s) => {
                        let offset = *stack_height - s.stack_position;
                        ir.instructions.push(SigilCall {
                            id: "Numerical Reflection".to_string(),
                            modifier: Some(offset.to_string()),
                        });
                        ir.instructions.push(SigilCall {
                            id: "Fisherman's Gambit II".to_string(),
                            modifier: None,
                        });
                        *stack_height += 1;
                    }
                    None => eprintln!("Undefined function: {}()", name),
                }
            }
            Expression::Pattern(sigil) => {
                ir.instructions.push(SigilCall {
                    id: "Consideration".to_string(),
                    modifier: Some(sigil.name.clone()),
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
				*stack_height -= item_count-1;
				
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
}
