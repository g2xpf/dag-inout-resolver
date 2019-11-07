use non_cyclic_graph_solver::*;

fn main() {
    let input = nodes!(a);
    let output = nodes!(b);
    let dep = dep!(b <- a);
    let graph = Graph::new(input, output, dep);
    let solver = Solver::new(graph);
    let result = solver.solve();

    println!("{:#?}", result);
}
