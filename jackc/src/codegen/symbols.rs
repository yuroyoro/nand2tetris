use super::typedef::{ClassDef, Type};
use super::*;
use crate::token::Location;

use std::collections::HashMap;

use anyhow::anyhow;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub typ: Type,
    pub kind: Kind,
    pub index: usize,
}

#[derive(Debug)]
pub struct Symbols<'a> {
    types: &'a Types,
    parent: Option<&'a Symbols<'a>>,
    table: HashMap<String, Symbol>,
    counts: HashMap<Kind, usize>,
    labels: HashMap<String, usize>,
}

impl Symbols<'_> {
    fn new<'a>(types: &'a Types) -> Symbols<'a> {
        Symbols {
            types: types,
            parent: None,
            table: HashMap::new(),
            counts: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.table
            .get(name)
            .or_else(|| self.parent.and_then(|parent| parent.lookup(name)))
    }

    pub fn lookup_or_die(&self, name: &str, loc: &Location) -> Result<&Symbol> {
        self.lookup(name)
            .ok_or_else(|| anyhow!("{:?}: undefined symbol: {}", loc, name))
    }
    pub fn lookup_type(&self, name: &str) -> Option<&ClassDef> {
        self.types.get(name)
    }
    pub fn lookup_type_or_die(&self, name: &str, loc: &Location) -> Result<&ClassDef> {
        self.lookup_type(name)
            .ok_or_else(|| anyhow!("{:?}: undefined type: {}", loc, name))
    }

    pub fn class<'a>(types: &'a &Types, cls: &Class) -> Symbols<'a> {
        let mut scope = Self::new(types);

        // set only static vars to class scope
        cls.static_vars()
            .iter()
            .for_each(|var| scope.define_class_var(var));

        scope
    }

    pub fn method(&self, cls: &Class, sub: &SubroutineDec) -> Symbols {
        let mut scope = Self::new(self.types);
        scope.parent = Some(&self);

        // "this" is first argument
        scope.define("this", Type::Class(cls.name.clone()), Kind::Arg);

        // set class fields
        cls.field_vars()
            .iter()
            .for_each(|var| scope.define_class_var(var));

        // "this" is pop from argument 0 and set pointer 0
        scope.define_this(Type::Class(cls.name.clone()));

        scope.setup_paramters_and_vars(sub);

        scope
    }

    pub fn constructor(&self, cls: &Class, sub: &SubroutineDec) -> Symbols {
        let mut scope = Self::new(self.types);
        scope.parent = Some(&self);

        // set class fields
        cls.field_vars()
            .iter()
            .for_each(|var| scope.define_class_var(var));

        // "this" is allocated by Memory.alloc
        scope.define_this(Type::Class(cls.name.clone()));

        scope.setup_paramters_and_vars(sub);

        scope
    }

    pub fn function(&self, _cls: &Class, sub: &SubroutineDec) -> Symbols {
        let mut scope = Self::new(self.types);

        scope.parent = Some(&self);

        scope.setup_paramters_and_vars(sub);

        scope
    }

    fn setup_paramters_and_vars(&mut self, sub: &SubroutineDec) {
        // arguments
        for param in sub.parameters.iter() {
            self.define(&param.name, Type::from_ast_type(&param.typ), Kind::Arg);
        }

        // local vars
        for var in sub.body.vars.iter() {
            for name in var.names.iter() {
                self.define(name, Type::from_ast_type(&var.typ), Kind::Var);
            }
        }
    }

    fn define(&mut self, name: &str, typ: Type, kind: Kind) {
        let index = self.counts.get(&kind).map(|n| *n).unwrap_or(0);
        self.counts.insert(kind, index + 1);

        self.table.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                typ: typ,
                kind: kind,
                index: index,
            },
        );
    }

    fn define_this(&mut self, typ: Type) {
        self.table.insert(
            "this".to_string(),
            Symbol {
                name: "this".to_string(),
                typ: typ,
                kind: Kind::This,
                index: 0,
            },
        );
    }

    fn define_class_var(&mut self, var: &ClassVarDec) {
        let kind = Kind::from_class_var_modifier(&var.modifier);

        var.names
            .iter()
            .for_each(|name| self.define(name, Type::from_ast_type(&var.typ), kind.clone()))
    }

    pub fn label(&mut self, name: &str) -> String {
        let index = self.labels.get(name).map(|n| *n).unwrap_or(0);
        self.labels.insert(name.to_string(), index + 1);
        format!("{}_{}", name, index)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
    This,
}

impl Kind {
    fn from_class_var_modifier(modifier: &ClassVarModifier) -> Kind {
        match modifier {
            ClassVarModifier::Static => Kind::Static,
            ClassVarModifier::Field => Kind::Field,
        }
    }

    pub fn segment(&self) -> Segment {
        match self {
            Kind::Static => Segment::Static,
            Kind::Field => Segment::This,
            Kind::Arg => Segment::Arg,
            Kind::Var => Segment::Local,
            Kind::This => Segment::Pointer,
        }
    }
}
