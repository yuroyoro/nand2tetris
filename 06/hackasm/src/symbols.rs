use super::a_command::*;
use super::parser::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub addr: usize,
}

impl Symbol {
    pub fn new(name: String, addr: usize) -> Symbol {
        Symbol {
            name: name,
            addr: addr,
        }
    }
}

pub struct Symbols {
    table:  HashMap<String, Symbol>,
    offset: usize,
}

const DEFINED_SYMBOLS: &[(&str, usize)] = &[
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("SCREEN", 16384),
    ("KBD", 24576),
];

impl Symbols {
    pub fn new() -> Symbols {
        let mut table: HashMap<String, Symbol> = HashMap::new();

        // defined symbols
        DEFINED_SYMBOLS.into_iter().for_each(|(name, addr)| {
            let name = String::from(*name);
            table.insert(
                name.clone(),
                Symbol {
                    name: name,
                    addr: *addr,
                },
            );
        });

        // R0-R15
        for n in 0..16 {
            let name = format!("R{}", n);
            table.insert(
                name.clone(),
                Symbol {
                    name: name,
                    addr: n,
                },
            );
        }

        Symbols {
            table:  table,
            offset: 16,
        }
    }

    pub fn get_or_assign(&mut self, name: &str) -> &Symbol {
        let name = String::from(name);
        let addr = self.offset;
        let mut assigned = false;

        self.table.entry(name.clone()).or_insert_with(|| {
            assigned = true;
            Symbol {
                name: name.clone(),
                addr: addr,
            }
        });

        if assigned {
            self.offset += 1;
        }

        self.get(name).unwrap()
    }

    pub fn add(&mut self, sym: Symbol) -> &Symbol {
        self.table.insert(sym.name.clone(), sym.clone());
        self.get(sym.name.clone()).unwrap()
    }

    pub fn get(&self, name: String) -> Option<&Symbol> {
        self.table.get(&name)
    }

    pub fn resolve(&mut self, nodes: Vec<Node>) -> Vec<Node> {
        // collect symbols
        self.collect_symbols(&nodes);

        // assign address to node
        nodes
            .into_iter()
            .map(|node| match node {
                Node::A(a) => Node::A(a.assign(&self)),
                _ => node,
            })
            .collect()
    }

    fn collect_symbols(&mut self, nodes: &Vec<Node>) {
        nodes.into_iter().for_each(|node| {
            match node {
                Node::A(ACommand {
                    symbol_name: Some(name),
                    ..
                }) => {
                    self.get_or_assign(&name);
                }
                _ => {}
            };
        });
    }
}
