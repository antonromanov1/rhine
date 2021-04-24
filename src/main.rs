mod ir;

fn main() {
    let mut graph = ir::Graph::new();
    let _add = graph.create_inst_add();
    let _not = graph.create_inst_not();
}
