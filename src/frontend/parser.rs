//! Implements parser to convert from ESIL to RadecoIR.

use num::traits::Num;

use std::collections::Bound;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp;
use std::slice;
use regex::Regex;
use petgraph::graph::NodeIndex;

use super::{MInst, MVal, MOpcode, MValType, Address, MArity, MRegInfo, MAddr};
use super::structs::{LOpInfo, LAliasInfo, LRegInfo, LRegProfile, LFlagInfo};
use super::super::middle::ssa::{SSAStorage, ValueType, BBInfo};
use super::super::middle::ssa::EdgeData as SSAEdgeData;
use super::super::transform::ssa::{SSAConstruction, Node, Block};

// Macro to return a new hash given (key, value) tuples.
// Example: hash![("foo", "bar"), ("bar", "baz")]
macro_rules! hash {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut temp_hash = HashMap::new();
            $(
                temp_hash.insert($x, $y);
             )*
                temp_hash
        }
    };
}

// Convert hex string ("0xbadc0de") to decimal representation.
macro_rules! hex_to_i {
    ( $x:expr ) => {
        Num::from_str_radix($x.trim_left_matches("0x"), 16)
    };
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEsil,
    InvalidMOperator,
    InsufficientOperands,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    stack:        Vec<MVal>,
    insts:        Vec<MInst>,
    allinsts:     Vec<MInst>,
    opset:        HashMap<&'a str, MOpcode>,
    regset:       HashMap<String, LRegProfile>,
    alias_info:   HashMap<String, LAliasInfo>,
    flags:        HashMap<u64, LFlagInfo>,
    default_size: u8,
    tmp_prefix:   String,
    arch:         String,
    addr:         Address,
    opinfo:       Option<LOpInfo>,
    tmp_index:    u64,
    last_assgn:   MVal,
    ssac:         Option<SSAConstruction<'a>>, // TODO: Remove the Option<> here
    block:        NodeIndex,
    bbs:          BTreeMap<Address, (NodeIndex, Address)>,
    bbworklist:   Vec<Address>
}

// Struct used to configure the Parser. If `None` is passed to any of the fields, then the default
// values are set.
pub struct ParserConfig<'a> {
    arch:         Option<String>,
    default_size: Option<u8>,
    tmp_prefix:   Option<String>,
    init_opset:   Option<fn() -> HashMap<&'a str, MOpcode>>,
    regset:       Option<HashMap<String, LRegProfile>>,
    alias_info:   Option<HashMap<String, LAliasInfo>>,
    flags:        Option<HashMap<u64, LFlagInfo>>,
}

impl<'a> Default for ParserConfig<'a> {
    fn default() -> Self {
        ParserConfig {
            arch:         Some("x86_64".to_string()),
            default_size: Some(64),
            tmp_prefix:   Some("tmp".to_string()),
            init_opset:   Some(map_esil_to_opset),
            regset:       Some(HashMap::new()),
            alias_info:   Some(HashMap::new()),
            flags:        Some(HashMap::new()),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(config: Option<ParserConfig<'a>>) -> Parser<'a> {
        let config = config.unwrap_or_default();
        let arch = config.arch.unwrap_or("x86_64".to_string());
        let default_size = config.default_size.unwrap_or(64);
        let tmp_prefix = config.tmp_prefix.unwrap_or("tmp".to_string());
        let init_opset = config.init_opset.unwrap_or(map_esil_to_opset);
        let regset = config.regset.unwrap_or(HashMap::new());
        let alias_info = config.alias_info.unwrap_or(HashMap::new());
        let flags = config.flags.unwrap_or(HashMap::new());
        let val = MVal::null();

        Parser { 
            stack:        Vec::new(),
            insts:        Vec::new(),
            allinsts:     Vec::new(),
            opset:        init_opset(),
            regset:       regset,
            default_size: default_size,
            arch:         arch,
            tmp_prefix:   tmp_prefix,
            addr:         0,
            opinfo:       None,
            alias_info:   alias_info,
            flags:        flags,
            tmp_index:    0,
            last_assgn:   val,
            ssac:         None,
            block:        Block::end(),
            bbs:          BTreeMap::new(),
            bbworklist:   Vec::new()
        }
    }

    pub fn set_register_profile(&mut self, reg_info: &LRegInfo, ssa: &'a mut SSAStorage) {
        // TODO: use SSA methods instead of SSAStorage methods
        self.ssac = Some(SSAConstruction::new(ssa, reg_info));

        //if let Some(ref mut ssac) = self.ssac { self.block = ssac.add_block(); } else { unreachable!(); } // TODO: Remove this

        self.regset = HashMap::new();
        let mut tmp: HashMap<String, LAliasInfo> = HashMap::new();
        for alias in reg_info.alias_info.iter() {
            let a = alias.clone();
            tmp.insert(a.reg.clone(), a.clone());
        }

        for reg in reg_info.reg_info.iter() {
            let r = reg.clone();
            self.regset.insert(r.name.clone(), r.clone());
        }

        self.alias_info = tmp.clone();
    }

    pub fn run(&mut self, mut ops: Vec<LOpInfo>, start: Address) -> Result<(), ParseError> {
        // TODO: Accept hints on bb beginnings (with corresponding entries in self.bbworklist and self.bbs)

        let mut edges: Vec<(Address, Address, u8)> = Vec::new();

        ops.sort_by(|a, b| a.offset.unwrap().cmp(&b.offset.unwrap()));

        self.bbworklist.push(start);
        while let Some(top) = self.bbworklist.pop() {

            // check for existing blocks
            if let Some((&addr, &(ni, end))) = self.bbs.range(Bound::Unbounded, Bound::Included(&top)).next_back() {
                if addr == top && ni != NodeIndex::end() {
                    // this block already exists
                    continue;
                } else if top < end {
                    // we overlap with a previous block
                    self.bbworklist.push(addr); // reenqueue overlapping block
                    // TODO: Correct disposal of old node
                    self.bbs.insert(addr, (NodeIndex::end(), addr));
                }
            }

            // determine where we have to stop picking up instructions
            let mut nbound = if let Some((&naddr, _)) = self.bbs.range(Bound::Excluded(&top), Bound::Unbounded).next() {
                // at the next bb
                Bound::Excluded(naddr)
            } else {
                // not at all
                Bound::Unbounded
            };

            // TODO: consider .size of LOpInfo so we don't get confused by overlapping instructions

            // get index of first lexed instruction
            let mut i = match ops.binary_search_by(|probe| probe.offset.unwrap().cmp(&top)) {
                Ok(i) => i,
                Err(i) => i,
            };

            // process from here on
            let mut bbstart = top;
            let mut pc = bbstart;
            let mut prevpc = pc; // what a mess
            let mut newbb: bool = true;
            while i < ops.len() {

                // check if we've reached the end
                pc = ops[i].offset.unwrap();
                if match nbound {
                    Bound::Excluded(end) => pc >= end,
                    Bound::Included(end) => pc > end,
                    _ => false,
                } { break }

                if newbb {
                    newbb = true;
                    self.bbs.insert(bbstart, (self.block, pc));
                    bbstart = pc;
                    if prevpc != bbstart {edges.push((prevpc, bbstart, 0));}
                    // add the new block to the ssa graph and remember it
                    self.block = {
                        let ssac = self.ssac.as_mut().unwrap();
                        ssac.add_block(BBInfo{addr: pc})
                    };
                }

                prevpc = pc;

                // okay
                try!(self.parse_opinfo(&ops[i]));

                // process jumps
                for inst in &self.insts {
                    if inst.opcode == MOpcode::OpJmp {
                        // end is earlier
                        nbound = Bound::Included(pc);

                        // if we can determine the target of the jump investigate target
                        if let Some(target) = self.read_const(&inst.operand_1) {
                            self.bbworklist.push(target);
                            edges.push((bbstart, target, 9));
                        }
                    } else if inst.opcode == MOpcode::OpCJmp {
                        newbb = true;

                        // see above
                        if let Some(target) = self.read_const(&inst.operand_2) {
                            self.bbworklist.push(target);
                            edges.push((bbstart, target, 1));
                        } else {
                            println!("Can't tell target of {:?}", inst);
                        }
                    }
                }

                // keep insts for cfg generation
                self.allinsts.extend(self.insts.clone()); // TODO: no unneccesary copy
                self.insts = Vec::new();

                // next
                i+=1;
            }

            self.bbs.insert(bbstart, (self.block, pc));
        }
        {
            let ssa = &mut self.ssac.as_mut().unwrap().ssa;
            for (ref source, ref target, i) in edges {
                ssa.g.add_edge(
                    self.bbs[source].0,
                    self.bbs[target].0,
                    SSAEdgeData::Control(i));
            }
        }
        for (_, &(ref ni, _)) in &self.bbs {
            self.ssac.as_mut().unwrap().seal_block(*ni);
        }
        Ok(())
    }

    pub fn read_const(&self, val: &MVal) -> Option<u64> {
        if let &MVal { val_type: MValType::Temporary, node: Some(ni), .. } = val {
            self.ssac.as_ref().unwrap().ssa.read_const(ni)
        } else {
            None
        }
    }

    pub fn set_flags(&mut self, flags: &Vec<LFlagInfo>) {
        for f in flags.iter() {
            self.flags.insert(f.offset.clone(), f.clone());
        }
    }

    pub fn new_mreg_info(&self, info: LRegProfile) -> MRegInfo {
        MRegInfo {
            reg_type: info.type_str,
            offset:   info.offset,
            reg:      info.name,
            size:     info.size,
            alias:    String::new(),
        }
    }

    pub fn parse_opinfo(&mut self, opinfo: &LOpInfo) -> Result<(), ParseError> {
        let opinfo = opinfo.clone();
        let esil = opinfo.esil.clone().unwrap_or("".to_string());

        self.addr = match opinfo.offset {
            Some(s) => s,
            None    => self.addr + 1,
        };

        self.opinfo = Some(opinfo);
        self.parse_str(&*esil)
    }

    pub fn parse_str(&mut self, esil: &str) -> Result<(), ParseError> {
        if esil.len() == 0 {
            return Err(ParseError::InvalidEsil);
        }

        let esil: Vec<String> = esil.split(',')
            .map(|x| x.to_string()).collect();
        for token in esil {
            let op = match self.opset.get(&*token) {
                Some(op) => op.clone(),
                None     => MOpcode::OpInvalid,
            };

            if op != MOpcode::OpInvalid {
                try!(self.add_inst(op));
                continue;
            }

            // If it contains atleast one alpha, it cannot be an operator.
            let re = Regex::new("[a-zA-Z]").unwrap();
            if re.is_match(&*token) {
                let mut val_type = MValType::Unknown;
                let mut val: Option<u64> = None;
                let mut size: u8 = self.default_size;
                let mut reg_info: Option<MRegInfo> = None;
                if let Some(r) = self.regset.get(&*token) {
                    val_type = MValType::Register;
                    let mut reg_info_ = self.new_mreg_info(r.clone());
                    let alias = self.alias_info.get(&reg_info_.reg)
                        .map(|x| x.role_str.clone())
                        .unwrap_or_default();
                    reg_info_.alias = alias;
                    reg_info = Some(reg_info_.clone());
                    size = r.size; 
                } else if let Ok::<i64, _>(v) = hex_to_i!(token) { // <u64>? will it be able to deal with negative numbers?
                    val = Some(v as u64);
                } else if let Some('%') = token.chars().nth(0) {
                    val_type = MValType::Internal
                } else {
                    panic!("Proper error handling here");
                }
                let v = if let Some(cv) = val {
                    self.constant_value(cv)
                } else {
                    MVal::new(String::from(token), size, val_type, 0, reg_info)
                };
                self.stack.push(v);
                continue;
            }

            // Handle constants.
            if let Ok(num) = token.parse::<i64>() { // <u64>? will it be able to deal with negative numbers?
                let v = self.constant_value(num as u64);
                self.stack.push(v);
                continue;
            }

            // Deal with normal 'composite' instructions.
            if token.char_indices().last().unwrap().1 != ']' {
                let mut dst: MVal;
                if let Some(x) = self.stack.last() {
                    dst = x.clone();
                } else {
                    return Err(ParseError::InsufficientOperands);
                }
                let re = Regex::new(r"^(.|..)=$").unwrap();
                let t = re.captures(&*token).unwrap().at(1).unwrap_or("");
                if t.len() == 0 {
                    return Err(ParseError::InvalidMOperator);
                }
                let op = match self.opset.get(t) {
                    Some(op) => op.clone(),
                    None     => return Err(ParseError::InvalidMOperator),
                };

                try!(self.add_inst(op));
                self.stack.push(dst);
                try!(self.add_inst(MOpcode::OpEq));
                continue;
            }

            // Deal with memaccess 'composite' instructions.
            let re = Regex::new(r"^(.|..)?(=)?\[([1248]?)\]$").unwrap();
            let tokens = re.captures(&*token).unwrap();
            let eq = tokens.at(2).unwrap_or("");
            let has_op = tokens.at(1).unwrap_or("");
            let access_size = tokens.at(3).unwrap_or("");
            let access_size = match access_size {
                "" => self.default_size,
                _  => access_size.parse::<u8>().unwrap() * 8,
            };

            try!(self.add_inst(MOpcode::OpLoad));
            // Set the correct size.
            let mut x = self.stack.pop().unwrap();
            self.add_narrow_inst(&mut x, access_size);
            let tmp_dst1 = x.clone();
            self.stack.push(x);

            // Simple 'peek' ([n])
            if eq.is_empty() {
                continue;
            }

            // Simple 'poke' (=[n])
            if has_op.is_empty() {
                try!(self.add_inst(MOpcode::OpEq));
                continue;
            }

            // 'poke' with another operation. (<op>=[n])
            let o = match self.opset.get(has_op) {
                Some(x) => x.clone(),
                // Return with error
                None => return Err(ParseError::InvalidMOperator),
            };
            try!(self.add_inst(o));
            // Reassignment.
            self.stack.push(tmp_dst1);
            try!(self.add_inst(MOpcode::OpEq));
        }
        Ok(())
    }

    /// Emit-instructions converts certain compound instructions to simpler instructions.
    /// For example, the sequence of instructions,
    /// ```none
    /// if zf            (OpIf)
    ///   jump addr      (OpJmp)
    /// ```
    /// is converted to a single conditional jump as,
    /// ```none
    /// if zf jump addr  (OpCJmp)
    /// ```

    pub fn emit_insts(&mut self) -> Vec<MInst> {
        let mut res: Vec<MInst> = Vec::new();
        {
            let mut insts_iter = self.allinsts.iter();
            while let Some(inst) = insts_iter.next() {
                Self::emit_inst(&inst, &mut res, &mut insts_iter);
            }
        }

        self.allinsts = res;
        return (self).allinsts.clone();
    }

    pub fn emit_inst(inst: &MInst, res: &mut Vec<MInst>, insts_iter: &mut slice::Iter<MInst>) {
        match inst.opcode {

            MOpcode::OpJmp => {
                while let Some(_inst) = insts_iter.next() {
                    if _inst.addr.val != inst.addr.val {
                        res.push(inst.clone());
                        res.push(_inst.clone());
                        break;
                    }
                    res.push(_inst.clone());
                }
            },

            MOpcode::OpIf => {
                let mut jmp_inst = None;
                while let Some(_inst) = insts_iter.next() {
                    if _inst.opcode == MOpcode::OpCl {
                        break;
                    }
                    if _inst.opcode != MOpcode::OpJmp {
                        res.push(_inst.clone());
                        continue;
                    }

                    jmp_inst = Some(
                        MInst::new(MOpcode::OpCJmp, MVal::null(),
                        inst.operand_1.clone(),
                        _inst.clone().operand_1, Some(inst.addr.clone()))
                    );
                }

                res.push(jmp_inst.unwrap());
            },

            _ => {
                res.push(inst.clone());
            }
        };
    }

    fn get_tmp_register(&mut self, mut size: u8) -> MVal {
        self.tmp_index += 1;
        if size == 0 {
            size = self.default_size;
        }
        MVal::tmp(self.tmp_index, size)
    }

    // Convert a lower field width to higher field width.
    fn add_widen_inst(&mut self, op: &mut MVal, size: u8) {
        if op.size >= size {
            return;
        }
        let mut dst = self.get_tmp_register(size);
        let operator = MOpcode::OpWiden(size);
        let addr = MAddr::new(self.addr);
        let inst = MInst::new(operator, dst.clone(), op.clone(), MVal::null(), Some(addr));
        dst = self.push_inst(inst.clone());
        *op = dst;
    }

    // Convert a higher field width to lower field width.
    fn add_narrow_inst(&mut self, op: &mut MVal, size: u8) {
        if op.size <= size {
            return;
        }
        let mut dst = self.get_tmp_register(size);
        let operator = MOpcode::OpNarrow(size);
        let addr = MAddr::new(self.addr);
        let inst = MInst::new(operator, dst.clone(), op.clone(), MVal::null(), Some(addr));
        dst = self.push_inst(inst.clone());
        *op = dst;
    }

    fn add_assign_inst(&mut self, op: MOpcode) -> Result<(), ParseError> {
        let     dst = try!(self.stack.pop().ok_or(ParseError::InsufficientOperands));
        let mut op1 = try!(self.get_param().ok_or(ParseError::InsufficientOperands));

        // Check the alias of dst. If it is the instruction pointer, the assignment should be a
        // OpJmp rather than a OpEq.
        if dst.reg_info.clone().unwrap_or_default().alias == "pc" {
            let mut op = MOpcode::OpJmp;
            if let Some(ref info) = self.opinfo {
                let optype = info.clone().optype.unwrap_or("".to_string());
                if optype == "call" {
                    op = MOpcode::OpCall;
                }
            }

            let addr = MAddr::new(self.addr);
            let inst = MInst::new(op, MVal::null(), op1, MVal::null(), Some(addr));
            self.push_inst(inst.clone());
            return Ok(());
        }

        // If the dst is a register. Update the last_assgn information.
        if dst.val_type == MValType::Register {
            self.last_assgn = dst.clone();
        }

        if dst.size == op1.size {
            let addr = MAddr::new(self.addr);
            let inst = MInst::new(op, dst.clone(), op1, MVal::null(), Some(addr));
            self.push_inst(inst.clone());
            return Ok(());
        }

        if dst.size > op1.size {
            self.add_widen_inst(&mut op1, dst.size);
        } else {
            self.add_narrow_inst(&mut op1, dst.size);
        }

        // We don't need to use another instruction for assignment. Just replace the dst of the
        // narrow/widen instruction generated.
        self.insts.last_mut().unwrap().dst = dst.clone();
        Ok(())
    }

    fn add_inst(&mut self, op: MOpcode) -> Result<(), ParseError> {
        // Handle all the special cases.
        match op {
            MOpcode::OpCl => {
                let null = MVal::null();
                let addr = MAddr::new(self.addr);
                let inst = MInst::new(op, null.clone(), null.clone(), null.clone(), Some(addr));
                self.push_inst(inst.clone());
                return Ok(());
            },
            MOpcode::OpInc |
            MOpcode::OpDec => {
                let _op = match op {
                    MOpcode::OpInc => MOpcode::OpAdd,
                    MOpcode::OpDec => MOpcode::OpSub,
                    _              => unreachable!(),
                };
                let mut top = self.stack.len();
                top -= 2;
                let v = self.constant_value(1);
                self.stack.insert(top, v);
                try!(self.add_inst(_op));
                return Ok(());
            },
            MOpcode::OpEq => {
                return self.add_assign_inst(op);
            },
            _ => { },
        }

        let mut op2 = match self.get_param() {
            Some(ele) => ele,
            None      => return Err(ParseError::InsufficientOperands),
        };

        let mut op1 = MVal::null();
        if op.arity() == MArity::Binary {
            op1 = match self.get_param() {
                Some(ele) => ele,
                None      => return Err(ParseError::InsufficientOperands),
            };
        }

        if op == MOpcode::OpIf {
            let addr = MAddr::new(self.addr);
            let inst = MInst::new(op, MVal::null(), op2, op1, Some(addr));
            self.push_inst(inst);
            return Ok(());
        }

        let mut dst_size: u8;
        let mut dst: MVal;
        dst_size = cmp::max(op1.size, op2.size);
        dst = self.get_tmp_register(dst_size);

        // Add a check to see if dst, op1 and op2 have the same size.
        // If they do not, cast it. op2 is never 'Null'.
        assert!(op2.val_type != MValType::Null);

        if op.arity() == MArity::Binary {
            if op1.size > op2.size {
                dst_size = op1.size;
                self.add_widen_inst(&mut op2, op1.size);
            } else if op2.size > op1.size {
                dst_size = op2.size;
                self.add_widen_inst(&mut op1, op2.size);
            }
        }

        dst.size = dst_size;


        let addr = MAddr::new(self.addr);
        let inst = MInst::new(op, dst.clone(), op2, op1, Some(addr));
        let dst_n = self.push_inst(inst);
        self.stack.push(dst_n.clone());
        
        // If it is a compare instruction, then the flags must be updated.
        if op == MOpcode::OpCmp {
            self.last_assgn = dst_n;
        }

        Ok(())
    }

    // correspons to r_anal_esil_get_parm
    fn get_param(&mut self) -> Option<MVal> {
        if let Some(mv) = self.stack.pop() {
            Some(match mv.val_type {
                MValType::Internal => {
                    let addr = MAddr::new(self.addr);
                    match mv.name.chars().nth(1).unwrap_or('\0') {
                        '%' => {
                            let addr = self.addr;
                            self.constant_value(addr)
                        },
                        'z' => {
                            let dst = self.get_tmp_register(1);
                            let inst = MInst::new(MOpcode::OpCmp, dst.clone(), self.last_assgn.clone(), self.constant_value(0), Some(addr));
                            self.push_inst(inst)
                        },
                        //'b' => _ // OpIFBorrow(u8),
                        //'c' => _ // OpIFCarry(u8),
                        //'o' => _ // OpIFOverflow,
                        //'p' => _ // OpIFParity,
                        //'r' => _ // OpIFRegSize,
                        //'s' => _ // OpIFSign,
                        _ => mv
                    }
                },
                _ => mv
            })
        } else {
            None
        }
    }

    fn constant_value(&mut self, num: u64) -> MVal {
        let op = MOpcode::OpConst(num);
        let size = self.default_size;
        let dst = self.get_tmp_register(size);
        let addr = MAddr::new(self.addr);
        let inst = MInst::new(op, dst.clone(), MVal::null(), MVal::null(), Some(addr));
        self.push_inst(inst)
    }

    fn push_inst(&mut self, instruction: MInst) -> MVal {
        self.insts.push(instruction.clone());

        let block = self.block;
        let n0 = self.process_in(block, &instruction.operand_1);
        let n1 = self.process_in(block, &instruction.operand_2);
        let nn = self.process_op(block, instruction.opcode, n0, n1);
        self.process_out(block, &instruction.dst, nn);

        MVal { node: Some(nn), val_type: MValType::Temporary, .. instruction.dst }
    }

    fn process_in(&mut self, block: Block, mval: &MVal) -> Node {
        let ssac = self.ssac.as_mut().unwrap();

        match mval.val_type {
            MValType::Register  => ssac.read_variable(block, mval.name.clone()),
            MValType::Temporary => mval.node.unwrap(), //ssac.read_variable(block, mval.name.clone()),
            MValType::Unknown   => ssac.ssa.add_comment(block, &"Unknown".to_string()), // unimplemented!()
            MValType::Internal  => ssac.ssa.add_comment(block, &mval.name), // unimplemented!()
            MValType::Null      => NodeIndex::end(),
        }
    }

    fn process_out(&mut self, block: Block, mval: &MVal, value: Node) {
        let ssac = self.ssac.as_mut().unwrap();
        match mval.val_type {
            MValType::Register  => ssac.write_variable(block, mval.name.clone(), value),
            MValType::Temporary => {}, // ssac.write_variable(block, mval.name.clone(), value),
            MValType::Unknown   => {}, // unimplemented!(),
            MValType::Internal  => {}, // unimplemented!()
            MValType::Null      => {},
        }
    }

    fn process_op(&mut self, block: Block, opc: MOpcode, n0: Node, n1: Node) -> Node {
        let ssac = self.ssac.as_mut().unwrap();
        if opc == MOpcode::OpEq {
            return n0
        }
        // TODO: give correct integer type here
        let nn = ssac.ssa.add_op(block, opc, ValueType::Integer{width: 64});
        ssac.ssa.op_use(nn, 0, n0);
        ssac.ssa.op_use(nn, 1, n1);
        return nn
    }
}

fn map_esil_to_opset() -> HashMap<&'static str, MOpcode> {
    // Make a map from esil string to struct MOperator.
    // (operator: &str, op: MOperator).
    // Possible Optimization:  Move to compile-time generation ?
    hash![
        ("==" , MOpcode::OpCmp),
        ("<"  , MOpcode::OpLt),
        (">"  , MOpcode::OpGt),
        ("<=" , MOpcode::OpGteq),
        (">=" , MOpcode::OpLteq),
        ("<<" , MOpcode::OpLsl),
        (">>" , MOpcode::OpLsr),
        ("&"  , MOpcode::OpAnd),
        ("|"  , MOpcode::OpOr),
        ("="  , MOpcode::OpEq),
        ("*"  , MOpcode::OpMul),
        ("^"  , MOpcode::OpXor),
        ("+"  , MOpcode::OpAdd),
        ("-"  , MOpcode::OpSub),
        ("/"  , MOpcode::OpDiv),
        ("%"  , MOpcode::OpMod),
        ("?{" , MOpcode::OpIf),
        ("!"  , MOpcode::OpNot),
        ("--" , MOpcode::OpDec),
        ("++" , MOpcode::OpInc),
        ("}"  , MOpcode::OpCl)
     ]
}
