# Dot Parser - An AST & Parser for Graphviz files

# Installation 

Add the following to your cargo.toml file: 

`dot_parser = 0.1`

# Example Usage

At its core, this crate exposes a `GraphViz` struct, which represents 
the root of an AST describing a given graphviz file. As an example of how 
to use this, we could write the following:

```rust

use std::str::FromStr;
use graphviz_dot_parser::DotGraph;

let graph_text = "digraph G { A -> B }";
let result = DotGraph::from_str(graph_text).unwrap();

```

# Using the AST 

Once parsed, the ast provides information about the original underlying graphviz graph - for example,
the following code reads in the node ids presented the toplevel lines:

```rust
use graphviz_dot_parser::DotGraph;
use graphviz_dot_parser::ast_nodes::Statement::Node;
use std::str::FromStr;

let dot_graph = DotGraph::from_str("graph G { a; b; c; }").unwrap();
let mut node_ids = vec![];
if let DotGraph::Directed(graph) = dot_graph {
   for statement in graph.statements {
      if let Node(n) = statement {
            node_ids.push(n.id);
      }
   }
   assert_eq!(node_ids, vec!["a", "b", "c"]);
}
```

# Filing Bugs 

Since this is still very much in development, if you encounter 
an issue, please file a bug! Please include the original graphviz string you were trying to 
parse to help us debug the issue faster
