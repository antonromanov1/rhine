mod ir;

use crate::ir::Inst;

fn main() {
    let mut graph = ir::Graph::new();

    let mut block = graph.create_empty_block();
    let add = graph.create_inst_add();

    unsafe {
        (*block).add_inst(add, true);
        (*add).dump();
    }
}
