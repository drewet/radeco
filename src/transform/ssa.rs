//! Implements the SSA construction algorithm described in
//! "Simple and Efficient Construction of Static Single Assignment Form"

use petgraph::EdgeDirection;
use petgraph::graph::NodeIndex;
use frontend::structs::LRegInfo;
use middle::cfg::NodeData as CFGNodeData;
use middle::cfg::EdgeType as CFGEdgeType;
use middle::cfg::{CFG, BasicBlock};
use middle::ssa::NodeData as SSANodeData;
use middle::ssa::EdgeData as SSAEdgeData;
use middle::ssa::{SSAStorage, NodeData, ValueType};
use middle::ir::{MVal, MOpcode, MValType};
use std::collections::{HashSet, HashMap};

pub type VarId = String; // consider transitioning to &str
pub type Block = NodeIndex;
pub type Node = NodeIndex;

pub struct SSAConstruction<'a> {
	pub ssa:          &'a mut SSAStorage,
	variables:        Vec<VarId>, // assume consequtive integers?
	sealed_blocks:    HashSet<Block>, // replace with bitfield
	current_def:      HashMap<VarId, HashMap<Block, Node>>,
	incomplete_phis:  HashMap<Block, HashMap<VarId, Node>>,
	global_variables: HashMap<VarId, Node>
}

impl<'a> SSAConstruction<'a> {
	pub fn new(ssa: &'a mut SSAStorage, reg_info: &LRegInfo) -> SSAConstruction<'a> {
		let mut s = SSAConstruction {
			ssa:              ssa,
			variables:        Vec::new(),
			sealed_blocks:    HashSet::new(),
			current_def:      HashMap::new(),
			incomplete_phis:  HashMap::new(),
			global_variables: HashMap::new()
		};
		for reg in &reg_info.reg_info {
			s.variables.push(reg.name.clone());
		}
		for var in &s.variables {
			s.current_def.insert(var.clone(), HashMap::new());
		}
		return s
	}

	pub fn write_variable(&mut self, block: Block, variable: VarId, value: Node) {
		if let Option::Some(vd) = self.current_def.get_mut(&variable) {
			vd.insert(block, value);
		} else {
			self.global_variables.insert(variable, value);
		}
	}

	fn replaced_by(&self, node: Node) -> Node {
		let mut walk = self.ssa.g.walk_edges_directed(node, EdgeDirection::Outgoing);
		while let Some((edge, othernode)) = walk.next_neighbor(&self.ssa.g) {
			if let SSAEdgeData::ReplacedBy = self.ssa.g[edge] {
				return othernode
			}
		}
		return NodeIndex::end()
	}

	pub fn read_variable(&mut self, block: Block, variable: VarId) -> Node {
		let mut n = match {
			if let Option::Some(vd) = self.current_def.get(&variable) {
				vd.get(&block)
			} else {
				self.global_variables.get(&variable)
			}
		}.map(|r|*r) {
			Option::Some(r) => r,
			Option::None => self.read_variable_recursive(variable, block)
		};
		while let NodeData::Removed = self.ssa.g[n] {
			// Ask authors of this algorithm if they had to loop here too
			n = self.replaced_by(n);
		}
		assert!(if let NodeData::Removed = self.ssa.g[n] { false } else { true });
		return n
	}

	pub fn seal_block(&mut self, block: Block) {
		let inc = self.incomplete_phis[&block].clone(); // TODO: remove clone

		for (variable, node) in inc {
			self.add_phi_operands(block, variable.clone(), node.clone());
		}
		self.sealed_blocks.insert(block);
	}
	/*
	pub fn run(&mut self, cfg: &CFG) {
		let mut blocks = Vec::<Block>::new();
		for i in 0..cfg.g.node_count() {
		 	let block = self.ssa.add_block();
			self.incomplete_phis.insert(block, HashMap::new());
			blocks.push(block);

			match cfg.g[NodeIndex::new(i)] {
		 		CFGNodeData::Block(ref srcbb) => {
		 			self.process_block(block, srcbb);
				},
				CFGNodeData::Entry => {
					for (ref name, ref mut vd) in &mut self.current_def {
						let mut msg = "initial_".to_string();
						msg.push_str(name);
						vd.insert(block, self.ssa.add_comment(block, &msg));
					}
				},
				_ => {}
			}
		}
		for edge in cfg.g.raw_edges() {
			let i = match edge.weight.edge_type {
				CFGEdgeType::True => 1,
				CFGEdgeType::False => 0,
				CFGEdgeType::Unconditional => 0,
			};
			self.ssa.g.add_edge(
				blocks[edge.source().index()],
				blocks[edge.target().index()],
				SSAEdgeData::Control(i));
		}
		for block in blocks {
			self.seal_block(block);
		}
		self.ssa.cleanup();
	}
	*/

	fn read_variable_recursive(&mut self, variable: VarId, block: Block) -> Node {
		let mut val;

		if !self.sealed_blocks.contains(&block) {
			// Incomplete CFG
			val = self.ssa.add_phi_comment(block, &variable);
			let oldval = self.incomplete_phis.get_mut(&block).unwrap().insert(variable.clone(), val);
			assert!(oldval.is_none());
		} else {
			let pred = self.ssa.preds_of(block);
			if pred.len() == 1 {
				// Optimize the common case of one predecessor: No phi needed
				val = self.read_variable(pred[0], variable.clone())
			} else {
				// Break potential cycles with operandless phi
				val = self.ssa.add_phi_comment(block, &variable);
				// TODO: only mark (see paper)
				self.write_variable(block, variable.clone(), val);
				val = self.add_phi_operands(block, variable.clone(), val)
			}
		}
		self.write_variable(block, variable, val);
		return val
	}

	fn add_phi_operands(&mut self, block: Block, variable: VarId, phi: Node) -> Node {
		assert!(block == self.ssa.block_of(phi));
		// Determine operands from predecessors
		for pred in self.ssa.preds_of(block) {
			let datasource = self.read_variable(pred, variable.clone());
			self.ssa.phi_use(phi, datasource)
		}
		return self.try_remove_trivial_phi(phi)
	}

	fn try_remove_trivial_phi(&mut self, phi: Node) -> Node {
		let undef = NodeIndex::end();
		let mut same: Node = undef; // The phi is unreachable or in the start block
		for op in self.ssa.args_of(phi) {
			if op == same || op == phi {
				continue // Unique value or self−reference
			}
			if same != undef {
				return phi // The phi merges at least two values: not trivial
			}
			same = op
		}

		if same == undef {
			same = self.ssa.g.add_node(SSANodeData::Undefined);
		}

		let users = self.ssa.uses_of(phi);
		self.ssa.replace(phi, same); // Reroute all uses of phi to same and remove phi

		// Try to recursively remove all phi users, which might have become trivial
		for use_ in users {
			if use_ == phi { continue; }
			if let NodeData::Phi(_) = self.ssa.g[use_] {
				//println!("After replacing {:?} by {:?}, proceeding to simplify user {:?}",
				//	self.ssa.g[phi],
				//	self.ssa.g[same],
				//	self.ssa.g[use_]);
				self.try_remove_trivial_phi(use_);
				//println!("done");
			}
		}
		return same
	}

	/*fn remove_redundant_phis(&self, phi_functions: Vec<Node>) {
		// should phi_functions be `Vec` or something else?
		let sccs = compute_phi_sccs(induced_subgraph(phi_functions));
		for scc in topological_sort(sccs) {
			processSCC(scc)
		}
	}*/

	/*fn processSCC(&self, scc) {
		if len(scc) = 1 { return } // we already handled trivial φ functions

		let inner = HashSet::new();
		let outerOps = HashSet::new();

		for phi in scc:
			isInner = True
			for operand in phi.getOperands() {
				if operand not in scc {
					outerOps.add(operand)
					isInner = False
				}
			}
			if isInner {
				inner.add(phi)
			}

		if len(outerOps) == 1 {
			replaceSCCByValue(scc, outerOps.pop())
		} else if len(outerOps) > 1 {
			remove_redundant_phis(inner)
		}
	}*/
}
