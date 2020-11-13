use crate::parser::ast::*;

use crate::parser::ast;
use crate::to_lowercase_first_char;

use std::collections::HashMap;

macro_rules! args {
    ( $name: literal, $typ: ident$(($cls: literal))* )  => {
        vec![ ($name.to_string(), Type::$typ$(($cls.to_string()))*) ]
    };

    ( $( ($name: literal, $typ: ident$(($cls: literal))*) ), * ) => {
        vec![ $( ($name.to_string(), Type::$typ$(($cls.to_string()))*) ), *]
    };
}

#[derive(Debug)]
pub struct Types {
    classes: HashMap<String, ClassDef>,
}

impl Types {
    pub fn new() -> Types {
        let mut classes = HashMap::new();

        classes.insert("Math".to_string(), Types::built_in_math());
        classes.insert("String".to_string(), Types::built_in_string());
        classes.insert("Array".to_string(), Types::built_in_array());
        classes.insert("Output".to_string(), Types::built_in_output());
        classes.insert("Screen".to_string(), Types::built_in_screen());
        classes.insert("Keyboard".to_string(), Types::built_in_keyborad());
        classes.insert("Memory".to_string(), Types::built_in_memory());
        classes.insert("Sys".to_string(), Types::built_in_sys());

        Types { classes }
    }

    fn built_in_math() -> ClassDef {
        let mut cls = ClassDef::new("Math");

        cls.add_function("init", Type::Void, Vec::new());
        cls.add_function("abs", Type::Int, args!["x", Int]);
        cls.add_function("multiply", Type::Int, args![("x", Int), ("y", Int)]);
        cls.add_function("divide", Type::Int, args![("x", Int), ("y", Int)]);
        cls.add_function("min", Type::Int, args![("x", Int), ("y", Int)]);
        cls.add_function("max", Type::Int, args![("x", Int), ("y", Int)]);
        cls.add_function("sqrt", Type::Int, args!["x", Int]);

        cls
    }

    fn built_in_string() -> ClassDef {
        let mut cls = ClassDef::new("String");

        cls.add_constructor("new", Type::class("String"), args!["max_length", Int]);

        cls.add_method("dispose", Type::Void, args![]);
        cls.add_method("length", Type::Int, args![]);
        cls.add_method("charAt", Type::Char, args!["i", Int]);
        cls.add_method("setCharAt", Type::Char, args![("i", Int), ("c", Char)]);
        cls.add_method("appendChar", Type::class("String"), args!["c", Char]);
        cls.add_method("eraseLastChar", Type::Void, args![]);
        cls.add_method("intValue", Type::Int, args![]);
        cls.add_method("setInt", Type::Void, args!["i", Int]);

        cls.add_function("backSpace", Type::Char, args![]);
        cls.add_function("doubleQuote", Type::Char, args![]);
        cls.add_function("newLine", Type::Char, args![]);

        cls
    }

    fn built_in_array() -> ClassDef {
        let mut cls = ClassDef::new("Array");

        cls.add_constructor("new", Type::class("Array"), args!["size", Int]);
        cls.add_method("dispose", Type::Void, args![]);

        cls
    }

    fn built_in_output() -> ClassDef {
        let mut cls = ClassDef::new("Output");

        cls.add_function("init", Type::Void, args![]);
        cls.add_function("moveCursor", Type::Void, args![("i", Int), ("j", Int)]);
        cls.add_function("printChar", Type::Void, args!["c", Char]);
        cls.add_function("printString", Type::Void, args!["s", Class("String")]);
        cls.add_function("printInt", Type::Void, args!["i", Int]);
        cls.add_function("println", Type::Void, args![]);
        cls.add_function("backSpace", Type::Void, args![]);

        cls
    }

    fn built_in_screen() -> ClassDef {
        let mut cls = ClassDef::new("Output");

        cls.add_function("init", Type::Void, args![]);
        cls.add_function("clearScreen", Type::Void, args![]);
        cls.add_function("setColor", Type::Void, args!["b", Boolean]);
        cls.add_function("drawPixel", Type::Void, args![("x", Int), ("y", Int)]);
        cls.add_function(
            "drawLine",
            Type::Void,
            args![("x1", Int), ("y1", Int), ("x2", Int), ("y2", Int)],
        );
        cls.add_function(
            "drawRectangle",
            Type::Void,
            args![("x1", Int), ("y1", Int), ("x2", Int), ("y2", Int)],
        );
        cls.add_function("drawCirle", Type::Void, args![("x", Int), ("y", Int)]);

        cls
    }

    fn built_in_keyborad() -> ClassDef {
        let mut cls = ClassDef::new("Keyboard");

        cls.add_function("init", Type::Void, args![]);
        cls.add_function("keyPressed", Type::Char, args![]);
        cls.add_function("readChar", Type::Char, args![]);
        cls.add_function(
            "readLine",
            Type::class("String"),
            args!["message", Class("String")],
        );
        cls.add_function("readInt", Type::Int, args!["message", Class("String")]);

        cls
    }

    fn built_in_memory() -> ClassDef {
        let mut cls = ClassDef::new("Memory");

        cls.add_function("init", Type::Void, args![]);
        cls.add_function("peek", Type::Int, args!["address", Int]);
        cls.add_function("poke", Type::Void, args![("address", Int), ("value", Int)]);
        cls.add_function("alloc", Type::class("Array"), args!["size", Int]);
        cls.add_function("deAlloc", Type::Void, args!["o", Class("Array")]);

        cls
    }

    fn built_in_sys() -> ClassDef {
        let mut cls = ClassDef::new("Sys");

        cls.add_function("init", Type::Void, args![]);
        cls.add_function("halt", Type::Void, args![]);

        cls.add_function("error", Type::Void, args!["errorCode", Int]);
        cls.add_function("wait", Type::Void, args!["duration", Int]);

        cls
    }

    pub fn define_classes(&mut self, asts_list: &Vec<ASTs>) {
        asts_list.into_iter().for_each(|asts| {
            self.define_class(&asts.class);
        });
    }

    pub fn get(&self, name: &str) -> Option<&ClassDef> {
        self.classes.get(name)
    }

    pub fn define_class(&mut self, cls: &ast::Class) {
        self.classes
            .insert(cls.name.to_string(), ClassDef::from_class(cls));
    }
}

#[derive(Debug)]
pub enum Type {
    Void,
    Int,
    Char,
    Boolean,
    Class(String),
}

impl Type {
    pub fn from_ast_type(typ: &ast::Type) -> Type {
        match typ {
            ast::Type::Int => Type::Int,
            ast::Type::Char => Type::Char,
            ast::Type::Boolean => Type::Boolean,
            ast::Type::Class(name) => Type::Class(name.to_string()),
        }
    }

    pub fn from_ast_return_type(typ: &ast::ReturnType) -> Type {
        match typ {
            ast::ReturnType::Void => Type::Void,
            ast::ReturnType::Type(typ) => Self::from_ast_type(typ),
        }
    }

    pub fn is_class(&self) -> bool {
        match self {
            Type::Class(_) => true,
            _ => false,
        }
    }

    fn class(name: &str) -> Type {
        Type::Class(name.to_string())
    }

    pub fn extract_class(&self) -> Option<String> {
        match self {
            Type::Class(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn display(&self) -> String {
        to_lowercase_first_char(format!("{:?}", self).as_str())
    }
}

#[derive(Debug)]
pub struct ClassDef {
    pub name: String,
    pub static_vars: HashMap<String, Type>,
    pub fields: HashMap<String, Type>,
    pub constructors: HashMap<String, FuncDef>,
    pub functions: HashMap<String, FuncDef>,
    pub methods: HashMap<String, FuncDef>,
}

impl ClassDef {
    fn new(name: &str) -> ClassDef {
        ClassDef {
            name: name.to_string(),
            static_vars: HashMap::new(),
            fields: HashMap::new(),
            constructors: HashMap::new(),
            functions: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    fn add_constructor(&mut self, name: &str, typ: Type, args: Vec<(String, Type)>) {
        self.constructors
            .insert(name.to_string(), FuncDef::new(name, typ, args));
    }

    pub fn constructor(&self, name: &str) -> Option<&FuncDef> {
        self.constructors.get(name)
    }

    fn add_function(&mut self, name: &str, typ: Type, args: Vec<(String, Type)>) {
        self.functions
            .insert(name.to_string(), FuncDef::new(name, typ, args));
    }

    pub fn function(&self, name: &str) -> Option<&FuncDef> {
        self.functions.get(name)
    }

    fn add_method(&mut self, name: &str, typ: Type, args: Vec<(String, Type)>) {
        self.methods
            .insert(name.to_string(), FuncDef::new(name, typ, args));
    }

    pub fn method(&self, name: &str) -> Option<&FuncDef> {
        self.methods.get(name)
    }

    pub fn from_class(cls: &ast::Class) -> ClassDef {
        let name = cls.name.clone();
        let static_vars = Self::build_vars_map(cls, ClassVarModifier::Static);
        let fields = Self::build_vars_map(cls, ClassVarModifier::Field);
        let constructors = Self::build_func_map(cls, SubroutineModifier::Constructor);
        let functions = Self::build_func_map(cls, SubroutineModifier::Function);
        let methods = Self::build_func_map(cls, SubroutineModifier::Method);

        ClassDef {
            name,
            static_vars,
            fields,
            constructors,
            functions,
            methods,
        }
    }

    fn build_vars_map(cls: &ast::Class, modifier: ClassVarModifier) -> HashMap<String, Type> {
        let mut map = HashMap::new();
        cls.vars
            .iter()
            .filter(|var| var.modifier == modifier)
            .for_each(|var| {
                var.names.iter().for_each(|name| {
                    map.insert(name.to_string(), Type::from_ast_type(&var.typ));
                });
            });

        map
    }

    fn build_func_map(cls: &ast::Class, modifier: SubroutineModifier) -> HashMap<String, FuncDef> {
        let mut map = HashMap::new();
        cls.subroutines
            .iter()
            .filter(|sub| sub.modifier == modifier)
            .for_each(|sub| {
                map.insert(sub.name.clone(), FuncDef::from_subroutine(sub));
            });

        map
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub name: String,
    pub typ: Type,
    pub args: Vec<(String, Type)>,
}

impl FuncDef {
    fn new(name: &str, typ: Type, args: Vec<(String, Type)>) -> FuncDef {
        FuncDef {
            name: name.to_string(),
            typ: typ,
            args: args,
        }
    }

    pub fn from_subroutine(sub: &ast::SubroutineDec) -> FuncDef {
        let typ = Type::from_ast_return_type(&sub.typ);
        let args = sub
            .parameters
            .iter()
            .map(|param| (param.name.clone(), Type::from_ast_type(&param.typ)))
            .collect();

        FuncDef {
            name: sub.name.clone(),
            typ: typ,
            args: args,
        }
    }
}
