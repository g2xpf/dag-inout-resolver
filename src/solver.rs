use super::types::{Dep, Graph, Nodes};

pub struct Solver(Graph);

impl Solver {
    pub fn new(graph: Graph) -> Self {
        Solver(graph)
    }

    pub fn solve(&self) -> Dep {
        let mut deps = Dep::new();
        for sink in self.0.output.iter() {
            let mut set = vec![sink.clone()];
            let mut nodes = Nodes::new();
            while !set.is_empty() {
                let node = set.pop().unwrap();
                for child_node in self
                    .0
                    .dep
                    .get(&node)
                    .unwrap_or_else(|| panic!("no such node: {:?}", node))
                    .iter()
                {
                    if self.0.input.contains(child_node) {
                        nodes.insert(child_node.clone());
                        continue;
                    }
                    set.push(child_node.clone());
                }
            }

            deps.insert(sink.clone(), nodes);
        }
        deps
    }
}
