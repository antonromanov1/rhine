use rhine::ir::*;

#[test]
fn common() {
    let mut graph = Graph::new();

    let block1 = graph.create_empty_block();
    let add1 = graph.create_inst_add();
    let add2 = graph.create_inst_add();
    let add3 = graph.create_inst_add();

    unsafe {
        (*block1).add_inst(add1, false);
        (*block1).add_inst(add2, false);
        (*block1).add_inst(add2, true);
        assert_eq!((*add1).get_opcode(), Opcode::Add);
        assert_eq!((*add1).get_block(), block1);
        (*add1).dump();
    }

    let block2 = graph.create_empty_block();
    let not = graph.create_inst_not();

    unsafe {
        (*block2).add_inst(not, false);
        (*block1).add_succ(block2, false);
        (*block1).add_succ(block2, true);
        assert_eq!((*block2).get_pred_block_index(block1), 1);
        assert_eq!((*block1).get_succ_block_index(block2), 1);
    }
}

#[test]
fn ir_constructor() {
    let mut z = IrConstructor::new();
    z.basic_block(2, &[3]);
}
