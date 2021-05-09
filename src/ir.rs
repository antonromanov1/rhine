use std::collections::HashMap;
use std::convert::TryInto;
use std::ptr::NonNull;

pub enum DataType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Bool,
    F32,
    F64,
    Void,
    NoType,
}

// enum Opcode:
include!(concat!(env!("OUT_DIR"), "/opcode.rs"));

pub trait Inst {
    fn get_opcode(&self) -> Opcode;
    fn get_block(&self) -> *mut BasicBlock;
    fn is_phi(&self) -> bool;

    fn set_id(&mut self, id: u16);
    fn set_opcode(&mut self, opcode: Opcode);
    fn set_type(&mut self, type_: DataType);
    fn set_block(&mut self, block: *mut BasicBlock);
    fn set_prev(&mut self, prev: *mut dyn Inst);
    fn set_next(&mut self, next: *mut dyn Inst);

    fn dump(&self);
}

pub struct InstData {
    id: u16,

    opcode: Opcode,
    type_: DataType,

    // Basic block this instruction belongs to
    block: *mut BasicBlock,

    // Next instruction within basic block
    next: Option<NonNull<dyn Inst>>,

    // Previous instruction within basic block
    prev: Option<NonNull<dyn Inst>>,
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

    fn set_type(&mut self, type_: DataType) {
        self.type_ = type_;
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

macro_rules! gen_new {
    ( $t: ident) => {
        fn new() -> $t {
            $t {
                inst: InstData {
                    id: u16::MAX,
                    opcode: Opcode::Nop,
                    type_: DataType::NoType,
                    block: 0 as *mut BasicBlock,
                    next: None,
                    prev: None,
                },
            }
        }
    };
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

        fn set_type(&mut self, type_: DataType) {
            self.inst.set_type(type_);
        }

        fn set_block(&mut self, block: *mut BasicBlock) {
            self.inst.set_block(block);
        }

        fn set_prev(&mut self, prev: *mut dyn Inst) {
            self.inst.set_prev(prev);
        }

        fn set_next(&mut self, next: *mut dyn Inst) {
            self.inst.set_next(next);
        }

        fn dump(&self) {
            self.inst.dump();
        }
    };
}

pub struct UnaryOperation {
    pub inst: InstData,
}

impl UnaryOperation {
    gen_new! {UnaryOperation}
}

impl Inst for UnaryOperation {
    impl_inst!();
}

pub struct BinaryOperation {
    pub inst: InstData,
}

impl BinaryOperation {
    gen_new! {BinaryOperation}
}

impl Inst for BinaryOperation {
    impl_inst!();
}

pub struct AllocInst {
    pub inst: InstData,
}

impl AllocInst {
    gen_new! {AllocInst}
}

impl Inst for AllocInst {
    impl_inst!();
}

pub struct PhiInst {
    pub inst: InstData,
}

impl PhiInst {
    gen_new! {PhiInst}
}

impl Inst for PhiInst {
    impl_inst!();
}

pub struct ReturnVoidInst {
    pub inst: InstData,
}

impl ReturnVoidInst {
    gen_new! {ReturnVoidInst}
}

impl Inst for ReturnVoidInst {
    impl_inst!();
}

pub struct NopInst {
    pub inst: InstData,
}

impl NopInst {
    gen_new! {NopInst}
}

impl Inst for NopInst {
    impl_inst! {}
}

pub struct BasicBlock {
    graph: *mut Graph,

    // Sequence of successor blocks
    succs: Vec<*mut BasicBlock>,

    // Sequence of predecessor blocks
    preds: Vec<*mut BasicBlock>,

    // first_phi: Option<NonNull<dyn Inst>>,
    first_inst: Option<NonNull<dyn Inst>>,
    last_inst: Option<NonNull<dyn Inst>>,

    id: u8,
}

impl BasicBlock {
    pub fn new(graph: *mut Graph) -> BasicBlock {
        BasicBlock {
            graph: graph,
            succs: Vec::new(),
            preds: Vec::new(),
            // first_phi: None,
            first_inst: None,
            last_inst: None,
            id: 0,
        }
    }

    /*
    pub fn has_phi(&self) -> bool {
        self.first_phi != None
    }
    */

    pub fn set_graph(&mut self, graph: *mut Graph) {
        self.graph = graph;
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    // FIXME: DRY
    pub fn get_pred_block_index(&self, block: *const BasicBlock) -> usize {
        let pos = self
            .preds
            .iter()
            .position(|&pred| pred == block as *mut BasicBlock)
            .unwrap();
        if pos == 0 && self.preds.len() == 2 {
            assert!(self.preds[1] != block as *mut BasicBlock);
        }
        pos
    }

    // FIXME: DRY
    pub fn get_succ_block_index(&self, block: *const BasicBlock) -> usize {
        let pos = self
            .succs
            .iter()
            .position(|&succ| succ == block as *mut BasicBlock)
            .unwrap();
        if pos == 0 && self.succs.len() == 2 {
            assert!(self.succs[1] != block as *mut BasicBlock);
        }
        pos
    }

    pub fn replace_pred(&mut self, prev_pred: *mut BasicBlock, new_pred: *mut BasicBlock) {
        let index = self.get_pred_block_index(prev_pred);
        self.preds[index] = new_pred;
        unsafe {
            (*new_pred).succs.push(self as *mut BasicBlock);
        }
    }

    // FIXME: DRY
    pub fn replace_succ(
        &mut self,
        prev_succ: *const BasicBlock,
        new_succ: *mut BasicBlock,
        can_add_empty_block: bool,
    ) {
        let contains = self.succs.contains(&new_succ);
        assert!(
            !contains || can_add_empty_block,
            "Uncovered case where empty block needed to fix CFG"
        );

        if contains && can_add_empty_block {
            // If edge already exists we create empty block on it
            let empty_block: *mut BasicBlock;
            unsafe {
                empty_block = (*self.graph).create_empty_block();
            }
            self.replace_succ(new_succ, empty_block, false);
            unsafe {
                (*new_succ).replace_pred(self as *mut BasicBlock, empty_block);
            }
        }

        let index = self.get_succ_block_index(prev_succ);
        self.succs[index] = new_succ;
        unsafe {
            (*new_succ).preds.push(self as *mut BasicBlock);
        }
    }

    // FIXME: DRY
    pub fn add_succ(&mut self, succ: *mut BasicBlock, can_add_empty_block: bool) {
        let contains = self.succs.contains(&succ);
        assert!(
            !contains || can_add_empty_block,
            "Uncovered case where empty block needed to fix CFG"
        );

        if contains && can_add_empty_block {
            // If edge already exists we create empty block on it
            let empty_block: *mut BasicBlock;
            unsafe {
                empty_block = (*self.graph).create_empty_block();
            }
            self.replace_succ(succ, empty_block, false);
            unsafe {
                (*succ).replace_pred(self as *mut BasicBlock, empty_block);
            }
        }

        self.succs.push(succ);
        unsafe {
            (*succ).preds.push(self as *mut BasicBlock);
        }
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

    pub fn get_start_block(&self) -> *mut BasicBlock {
        self.start_block
    }

    pub fn get_end_block(&self) -> *mut BasicBlock {
        self.end_block
    }

    pub fn get_blocks(&self) -> &Vec<*mut BasicBlock> {
        &self.blocks
    }

    fn add_block(&mut self, block: *mut BasicBlock) {
        unsafe {
            (*block).set_id(self.blocks.len().try_into().unwrap());
            (*block).set_graph(self as *mut Graph);
        }
        self.blocks.push(block);
    }

    pub fn create_empty_block(&mut self) -> *mut BasicBlock {
        let block = Box::into_raw(Box::new(BasicBlock::new(self as *mut Graph)));
        self.add_block(block);
        block
    }

    pub fn create_start_block(&mut self) -> *const BasicBlock {
        let block = self.create_empty_block();
        self.start_block = block;
        block
    }

    pub fn create_end_block(&mut self) -> *const BasicBlock {
        let block = self.create_empty_block();
        self.end_block = block;
        block
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        while !self.instructions.is_empty() {
            unsafe {
                Box::from_raw(self.instructions.pop().unwrap());
            }
        }

        while !self.blocks.is_empty() {
            unsafe {
                Box::from_raw(self.blocks.pop().unwrap());
            }
        }
    }
}

// Graph methods create_inst_<opcode>
include!(concat!(env!("OUT_DIR"), "/graph_create_inst.rs"));

pub struct IrConstructor {
    graph: *mut Graph,
    current_block: (i16, *mut BasicBlock),
    current_inst: (i32, Option<NonNull<dyn Inst>>),
    block_map: HashMap<u8, *mut BasicBlock>,
    block_succs_map: Vec<(i32, Vec<i8>)>,
    inst_map: HashMap<u16, *mut dyn Inst>,
    inst_inputs_map: HashMap<u16, Vec<u16>>,
    // phi_inputs_map: HashMap<u16, Vec<(u16, u16)>>,
}

const ID_ENTRY_BB: u8 = 0;
const ID_EXIT_BB: u8 = 1;

macro_rules! fn_type {
    ( $type_: ident ) => {
        #[allow(non_snake_case)]
        pub fn $type_(&mut self) -> &mut Self {
            unsafe {
                (*self.get_current_inst()).set_type(DataType::$type_);
            }
            self
        }
    };
}

impl IrConstructor {
    pub fn new() -> Self {
        let graph = Box::into_raw(Box::new(Graph::new()));

        unsafe {
            if (*graph).get_start_block() == 0 as *mut BasicBlock {
                (*graph).create_start_block();
            }
            if (*graph).get_end_block() == 0 as *mut BasicBlock {
                (*graph).create_end_block();
            }

            assert!((*graph).get_blocks().len() == 2);
        }

        let mut block_map = HashMap::new();
        unsafe {
            block_map.insert(ID_ENTRY_BB, (*graph).get_start_block());
            block_map.insert(ID_EXIT_BB, (*graph).get_end_block());
        }

        IrConstructor {
            graph: graph,
            current_block: (-1, 0 as *mut BasicBlock),
            current_inst: (-1, None),
            block_map: block_map,
            block_succs_map: Vec::new(),
            inst_map: HashMap::new(),
            inst_inputs_map: HashMap::new(),
        }
    }

    pub fn new_block(&mut self, id: u8) -> &mut Self {
        assert!(id != ID_ENTRY_BB && id != ID_EXIT_BB);
        assert!(!self.block_map.contains_key(&id));
        assert!(self.get_current_bb() == 0 as *mut BasicBlock);

        let block = Box::into_raw(Box::new(BasicBlock::new(self.graph)));

        unsafe {
            (*self.graph).add_block(block);
        }

        self.current_block = (id.into(), block);
        self.block_map.insert(id, block);

        if self.block_succs_map.is_empty() {
            unsafe {
                (*(*self.graph).get_start_block()).add_succ(block, false);
            }
        }

        self
    }

    pub fn inst(&mut self, id: u16, opc: Opcode) -> &mut Self {
        assert!(
            !self.inst_map.contains_key(&id),
            "Instruction with same ID already exists"
        );

        let inst: *mut dyn Inst;
        unsafe {
            inst = (*self.graph).create_inst(opc);
            (*inst).set_id(id);
        }
        self.current_inst = (id.into(), NonNull::new(inst));
        self.inst_map.insert(id, inst);

        assert!(self.get_current_bb() != 0 as *mut BasicBlock);
        unsafe {
            if (*inst).is_phi() {
            } else {
                (*self.get_current_bb()).add_inst(inst, true);
            }
        }

        self
    }

    pub fn succs(&mut self, succs: &[i8]) -> &mut Self {
        self.block_succs_map
            .push((self.get_current_bb_id().into(), succs.to_vec()));
        self
    }

    pub fn basic_block(&mut self, id: u8, succs: &[i8]) {
        self.new_block(id).succs(succs);
    }

    pub fn get_current_bb_id(&self) -> i16 {
        self.current_block.0
    }

    pub fn get_current_bb(&self) -> *mut BasicBlock {
        self.current_block.1
    }

    // Reset current basic block
    pub fn b(&mut self) {
        self.current_block = (-1, 0 as *mut BasicBlock);
    }

    // Reset current instruction
    pub fn i(&mut self) {
        self.current_inst = (-1, None);
    }

    fn_type! {U8}
    fn_type! {U16}
    fn_type! {U32}
    fn_type! {U64}
    fn_type! {I8}
    fn_type! {I16}
    fn_type! {I32}
    fn_type! {I64}
    fn_type! {Bool}
    fn_type! {F32}
    fn_type! {F64}
    fn_type! {Void}

    fn get_current_inst(&self) -> *mut dyn Inst {
        self.current_inst.1.unwrap().as_ptr()
    }
}
