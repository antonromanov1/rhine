extern crate libc;

include!(concat!(env!("OUT_DIR"), "/opcode.rs"));

pub trait Inst {
    fn set_id(&mut self, id: u16);
    fn set_opcode(&mut self, opcode: Opcode);
}

pub struct InstData {
    id: u16,
    opcode: Opcode,
}

impl Inst for InstData {
    fn set_id(&mut self, id: u16) {
        self.id = id;
    }

    fn set_opcode(&mut self, opcode: Opcode) {
        self.opcode = opcode;
    }
}

macro_rules! impl_inst {
    () => {
        fn set_id(&mut self, id: u16) {
            self.inst.set_id(id);
        }

        fn set_opcode(&mut self, opcode: Opcode) {
            self.inst.set_opcode(opcode);
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

pub struct Graph {
    cur_id: u16,
    instructions: Vec<*mut dyn Inst>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            cur_id: 0,
            instructions: Vec::new(),
        }
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

include!(concat!(env!("OUT_DIR"), "/graph_create_inst.rs"));
