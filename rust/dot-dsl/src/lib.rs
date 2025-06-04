pub mod graph {
    pub mod graph_items {
        pub mod edge {
            pub use crate::graph::Edge;
        }
        pub mod node {
            pub use crate::graph::Node;
        }
    }

    use std::collections::HashMap;

    macro_rules! impl_attr {
        () => {
            pub fn with_attrs(mut self, add_attrs: &[(&str, &str)]) -> Self {
                self.attrs.extend(
                    add_attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string())),
                );
                self
            }

            pub fn attr(&self, attr: &str) -> Option<&str> {
                self.attrs.get(attr).map(|a| a.as_str())
            }
        };
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Edge {
        pub start_node: String,
        pub end_node: String,
        attrs: HashMap<String, String>,
    }

    impl Edge {
        pub fn new(start_node: &str, end_node: &str) -> Self {
            Self {
                start_node: start_node.to_string(),
                end_node: end_node.to_string(),
                attrs: HashMap::new(),
            }
        }

        impl_attr!();
    }

    #[derive(Clone, PartialEq, Debug)]
    pub struct Node {
        pub name: String,
        attrs: HashMap<String, String>,
    }

    impl Node {
        pub fn new(node_name: &str) -> Self {
            Self {
                name: node_name.to_string(),
                attrs: HashMap::new(),
            }
        }

        impl_attr!();
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, add_nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(add_nodes);
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node_name)
        }

        pub fn with_edges(mut self, add_edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(add_edges);
            self
        }

        impl_attr!();
    }
}
