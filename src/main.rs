use non_cyclic_graph_solver::{
    dep, nodes,
    solver::Solver,
    types::{Dep, Graph, Node, Nodes},
};

fn main() {
    let input = nodes!(a);
    let output = nodes!(b);
    let dep = dep!(b <- a);
    let graph = Graph::new(input, output, dep);
    let solver = Solver::new(graph);
    let result = solver.solve();

    println!("{:#?}", result);
}
