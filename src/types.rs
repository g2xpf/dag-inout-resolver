use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub struct Node(pub &'static str);
impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

pub type Nodes = HashSet<Node>;
pub type Input = Nodes;
pub type Output = Nodes;
pub type Dep = HashMap<Node, Nodes>;
#[derive(Clone, Debug)]
pub struct Graph {
    pub(super) input: Input,
    pub(super) output: Output,
    pub(super) dep: Dep,
}

impl Graph {
    pub fn new(input: Input, output: Output, dep: Dep) -> Self {
        Graph { input, output, dep }
    }
}

#[macro_export]
macro_rules! nodes {
    ($($name:ident),*) => {{
        let mut nodes = Nodes::new();
        $(
            nodes.insert(Node(stringify!($name)));
        )*
        nodes
    }}
}

#[macro_export]
macro_rules! dep {
    ($($ident:ident <- $($dep:ident),*);*) => {{
        let mut dep = Dep::new();
        $(
            let key = Node(stringify!($ident));
            let nodes = nodes!($($dep),*);
            dep.insert(key, nodes);
        )*
        dep
    }};

    ($($ident:ident <- $($dep:ident),*);*;) => {{
        dep!($($ident <- $($dep),*);*)
    }};
}
