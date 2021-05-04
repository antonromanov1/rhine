mod ir;

use crate::ir::Inst;

fn main() {
    let mut graph = ir::Graph::new();

    let block = graph.create_empty_block();
    let add = graph.create_inst_add();
    let _not = graph.create_inst_not();
    let _phi = graph.create_inst_phi();

    unsafe {
        (*block).add_inst(add, true);
        (*add).dump();
    }
}
