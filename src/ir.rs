extern crate libc;

use std::convert::TryInto;
use std::ptr::NonNull;

// enum Opcode:
include!(concat!(env!("OUT_DIR"), "/opcode.rs"));

pub trait Inst {
    fn get_opcode(&self) -> Opcode;
    fn get_block(&self) -> *mut BasicBlock;
    fn is_phi(&self) -> bool;

    fn set_id(&mut self, id: u16);
    fn set_opcode(&mut self, opcode: Opcode);
    fn set_block(&mut self, block: *mut BasicBlock);
    fn set_prev(&mut self, prev: *mut dyn Inst);
    fn set_next(&mut self, next: *mut dyn Inst);

    fn dump(&self);
}

pub struct InstData {
    id: u16,

    // Basic block this instruction belongs to
    block: *mut BasicBlock,

    // Next instruction within basic block
    next: Option<NonNull<dyn Inst>>,

    // Previous instruction within basic block
    prev: Option<NonNull<dyn Inst>>,

    opcode: Opcode,
}

impl Inst for InstData {
    fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    fn get_block(&self) -> *mut BasicBlock {
        self.block
    }

    fn is_phi(&self) -> bool {
        self.opcode == Opcode::Phi
    }

    fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    fn set_opcode(&mut self, opcode: Opcode) {
        self.opcode = opcode;
    }

    fn set_block(&mut self, block: *mut BasicBlock) {
        self.block = block;
    }

    fn set_prev(&mut self, prev: *mut dyn Inst) {
        self.prev = NonNull::new(prev);
    }

    fn set_next(&mut self, next: *mut dyn Inst) {
        self.next = NonNull::new(next);
    }

    fn dump(&self) {
        println!(
            "ID: {}, Opcode: {}",
            self.id,
            get_opcode_string(self.opcode)
        );
    }
}

macro_rules! impl_inst {
    () => {
        fn get_opcode(&self) -> Opcode {
            self.inst.get_opcode()
        }

        fn get_block(&self) -> *mut BasicBlock {
            self.inst.get_block()
        }

        fn is_phi(&self) -> bool {
            self.inst.is_phi()
        }

        fn set_id(&mut self, id: u16) {
            self.inst.set_id(id);
        }

        fn set_opcode(&mut self, opcode: Opcode) {
            self.inst.set_opcode(opcode);
        }

        fn set_block(&mut self, block: *mut BasicBlock) {
            self.inst.set_block(block);
        }

        fn set_prev(&mut self, prev: *mut dyn Inst) {
            self.set_prev(prev);
        }

        fn set_next(&mut self, next: *mut dyn Inst) {
            self.set_next(next);
        }

        fn dump(&self) {
            self.inst.dump();
        }
    };
}

pub struct UnaryOperation {
    pub inst: InstData,
}

impl Inst for UnaryOperation {
    impl_inst!();
}

pub struct BinaryOperation {
    pub inst: InstData,
}

impl Inst for BinaryOperation {
    impl_inst!();
}

pub struct PhiInst {
    pub inst: InstData,
}

impl Inst for PhiInst {
    impl_inst!();
}

pub struct BasicBlock {
    graph: *mut Graph,

    /*

    // Sequence of predecessor blocks
    preds: Vec<*mut BasicBlock>,

    // Sequence of successor blocks
    succs: Vec<*mut BasicBlock>,

    // Sequence of dominated blocks
    dom_blocks: Vec<*mut BasicBlock>,

    // Dominator block
    dominator: *mut BasicBlock,

    */
    first_phi: Option<NonNull<dyn Inst>>,
    first_inst: Option<NonNull<dyn Inst>>,
    last_inst: Option<NonNull<dyn Inst>>,

    id: u8,
}

impl BasicBlock {
    pub fn new(graph: *mut Graph) -> BasicBlock {
        BasicBlock {
            graph: graph,
            first_phi: None,
            first_inst: None,
            last_inst: None,
            id: 0,
        }
    }

    pub fn has_phi(&self) -> bool {
        self.first_phi != None
    }

    pub fn set_graph(&mut self, graph: *mut Graph) {
        self.graph = graph;
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn add_inst(&mut self, inst: *mut dyn Inst, to_end: bool) {
        unsafe {
            assert_eq!((*inst).is_phi(), false);
            (*inst).set_block(self as *mut BasicBlock);
        }

        if self.last_inst == None {
            assert!(self.first_inst == self.last_inst);

            self.first_inst = NonNull::new(inst);
            self.last_inst = self.first_inst.clone();
            return;
        }

        let first_or_last = match &self.last_inst {
            Some(non_null) => non_null.as_ptr(),
            _ => unreachable!(),
        };
        if to_end {
            unsafe {
                (*first_or_last).set_next(inst);
            }
        } else {
            unsafe {
                (*first_or_last).set_prev(inst);
            }
        }
        let new_inst = NonNull::new(inst);
        if to_end {
            self.last_inst = new_inst;
        } else {
            self.first_inst = new_inst;
        }
    }
}

pub struct Graph {
    // Sequence of blocks in the insertion order
    blocks: Vec<*mut BasicBlock>,

    start_block: *mut BasicBlock,
    end_block: *mut BasicBlock,

    inst_cur_id: u16,
    instructions: Vec<*mut dyn Inst>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            blocks: Vec::new(),
            start_block: 0 as *mut BasicBlock,
            end_block: 0 as *mut BasicBlock,
            inst_cur_id: 0,
            instructions: Vec::new(),
        }
    }

    fn add_block(&mut self, block: *mut BasicBlock) {
        unsafe {
            (*block).set_id(self.blocks.len().try_into().unwrap());
            (*block).set_graph(self as *mut Graph);
        }
        self.blocks.push(block);
    }

    pub fn create_empty_block(&mut self) -> *mut BasicBlock {
        let block: *mut BasicBlock;
        unsafe {
            block = libc::malloc(std::mem::size_of::<BasicBlock>()) as *mut BasicBlock;
        }
        self.add_block(block);
        block
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        while !self.instructions.is_empty() {
            unsafe {
                libc::free(self.instructions.pop().unwrap() as *mut libc::c_void);
            }
        }
    }
}

// Graph methods create_inst_<opcode>
include!(concat!(env!("OUT_DIR"), "/graph_create_inst.rs"));
