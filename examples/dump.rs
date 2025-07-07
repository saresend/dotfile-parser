use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use graphviz_parser;

fn attr_map(
    attr_list: &Option<graphviz_parser::ast_nodes::AttributeList>,
) -> HashMap<&str, &String> {
    let mut attrs = HashMap::new();
    if let Some(attribute_list) = attr_list {
        for attr_group in attribute_list {
            for assignment in attr_group {
                attrs.insert(assignment.lhs.as_str(), &assignment.rhs);
            }
        }
    }
    attrs
}

/// Usage: `cargo run --examples dump`
/// By default it uses samples/basic1.dot, but you can also provide a file name.
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "samples/basic1.dot"
    };
    let data = std::fs::read_to_string(filename).expect("Cannot load file");
    let ast = graphviz_parser::DotGraph::from_str(&data).expect("Cannot parse file");
    if let graphviz_parser::DotGraph::Directed(graph) = ast {
        use graphviz_parser::ast_nodes::Statement;
        use graphviz_parser::ast_nodes::{EdgeLHS, EdgeRHS};
        for statement in graph.statements {
            match statement {
                Statement::Node(n) => {
                    let attrs = attr_map(&n.attribute_list);
                    let label = attrs.get("label").unwrap_or(&&n.id).to_string();
                    println!("Node {label} has attributes: {attrs:?}");
                }
                Statement::Edge(e) => {
                    let attr_list = Some(e.attr_list);
                    let attrs = attr_map(&attr_list);
                    let lhs_id = match e.lhs {
                        EdgeLHS::Node(node) => node.id,
                        _ => todo!("unsupported edge node"),
                    };
                    let rhs_id = match *e.rhs {
                        EdgeRHS::Node(node) => node.id,
                        _ => todo!("unsupported edge node"),
                    };
                    println!("Edge {lhs_id} -> {rhs_id} has attributes: {attrs:?}");
                }
                _ => {
                    // Ignore others
                }
            }
        }
    }
}
