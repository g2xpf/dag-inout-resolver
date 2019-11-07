use non_cyclic_graph_solver::*;

#[allow(dead_code)]
// #[test]
fn fan_controller() {
    let input = nodes!(tmp, hmd);
    let output = nodes!(di, fan);
    let dep = dep!(
        di <- tmp, hmd;
        fan <- di, th;
        th <- fan;
    );
    let graph = Graph::new(input, output, dep);
    let solver = Solver::new(graph);
    let result = solver.solve();

    println!("{:?}", result);
}
