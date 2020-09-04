use super::a_command::ACommand;
use super::c_command::CCommand;
use super::parser::Node;

pub fn generate(nodes: Vec<Node>) -> String {
    nodes
        .iter()
        .flat_map(|node| gen(node))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

fn gen(node: &Node) -> Option<String> {
    match node {
        Node::A(a) => Some(gen_a(a)),
        Node::C(c) => Some(gen_c(c)),
        _ => None,
    }
}

fn gen_a(a: &ACommand) -> String {
    // format!("0{:015b}  : {:?}", a.value, a)
    format!("0{:015b}", a.value)
}

fn gen_c(c: &CCommand) -> String {
    format!(
        "111{:07b}{:03b}{:03b}",
        c.comp.mcode, c.dest as i8, c.jump as i8
    )
}
