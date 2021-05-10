pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes<'a>(mut self, nodes: &'a [graph_items::node::Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }

        pub fn with_edges<'a>(mut self, edges: &'a [graph_items::edge::Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }

        // TODO: Factor this into a trait????
        pub fn with_attrs<'a>(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for &attr in attrs {
                self.attrs.insert(attr.0.into(), attr.1.into());
            }
            self
        }

        pub fn get_node<'a>(&self, node: &'a str) -> Option<graph_items::node::Node> {
            self.nodes
                .iter()
                .find(|n| n.name == node)
                .map(|n| n.clone())
        }
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new<'a>(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from: from.into(),
                        to: to.into(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<'a>(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for &attr in attrs {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new<'a>(name: &'a str) -> Self {
                    Node {
                        name: name.into(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<'a>(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for &attr in attrs {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    }
                    self
                }

                pub fn get_attr<'a>(&self, attr: &'a str) -> Option<&str> {
                    self.attrs.get(attr.into()).map(|av| av.as_str())
                }
            }
        }
    }
}
