mod ir;

use crate::ir::Inst;

fn main() {
    let mut graph = ir::Graph::new();

    let add = graph.create_inst_add();
    let _not = graph.create_inst_not();

    unsafe {
        (*add).dump();
    }
}
