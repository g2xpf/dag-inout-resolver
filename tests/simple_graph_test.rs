use non_cyclic_graph_solver::{
    dep, nodes,
    solver::Solver,
    types::{Dep, Graph, Node, Nodes},
};

#[test]
fn simple_graph() {
    let input = nodes!(a, b, c, d, e);
    let output = nodes!(m, n, o);
    let dep = dep!(
    f <- a, b;
    g <- f;
    h <- c, g;
    i <- d;
    j <- e, i, h;
    l <- f, g;
    k <- g, h, j;
    m <- f;
    n <- l;
    o <- k;
    );
    let graph = Graph::new(input, output, dep);
    let solver = Solver::new(graph);
    let result = solver.solve();

    println!("{:?}", result);
}
