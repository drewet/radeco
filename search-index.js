var searchIndex = {};
searchIndex['radeco'] = {"items":[[0,"","radeco","",null,null],[0,"frontend","","",null,null],[0,"esil","radeco::frontend","Module to parse ESIL strings and convert them into the IR.",null,null],[3,"Operator","radeco::frontend::esil","",null,null],[3,"Value","","Value is used to represent operands to an operator in a statement.",null,null],[3,"Instruction","","",null,null],[3,"Parser","","",null,null],[4,"Arity","","",null,null],[13,"Zero","","",0,null],[13,"Unary","","",0,null],[13,"Binary","","",0,null],[13,"Ternary","","",0,null],[4,"Location","","",null,null],[13,"Memory","","",1,null],[13,"Register","","",1,null],[13,"Constant","","",1,null],[13,"Temporary","","",1,null],[13,"Unknown","","",1,null],[13,"Null","","",1,null],[11,"clone","","",0,{"inputs":[{"name":"arity"}],"output":{"name":"arity"}}],[11,"fmt","","",0,{"inputs":[{"name":"arity"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"n","","",0,{"inputs":[{"name":"arity"}],"output":{"name":"u8"}}],[11,"clone","","",2,{"inputs":[{"name":"operator"}],"output":{"name":"operator"}}],[11,"fmt","","",2,{"inputs":[{"name":"operator"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",2,{"inputs":[{"name":"operator"},{"name":"str"},{"name":"arity"}],"output":{"name":"operator"}}],[11,"nop","","",2,{"inputs":[{"name":"operator"}],"output":{"name":"operator"}}],[11,"clone","","",1,{"inputs":[{"name":"location"}],"output":{"name":"location"}}],[11,"fmt","","",1,{"inputs":[{"name":"location"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"value"}],"output":{"name":"value"}}],[11,"fmt","","",3,{"inputs":[{"name":"value"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",3,{"inputs":[{"name":"value"},{"name":"string"},{"name":"u8"},{"name":"location"},{"name":"i64"},{"name":"u32"}],"output":{"name":"value"}}],[11,"null","","",3,{"inputs":[{"name":"value"}],"output":{"name":"value"}}],[11,"tmp","","",3,{"inputs":[{"name":"value"},{"name":"u64"}],"output":{"name":"value"}}],[11,"clone","","",4,{"inputs":[{"name":"instruction"}],"output":{"name":"instruction"}}],[11,"fmt","","",4,{"inputs":[{"name":"instruction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",4,{"inputs":[{"name":"instruction"},{"name":"operator"},{"name":"value"},{"name":"value"},{"name":"value"}],"output":{"name":"instruction"}}],[11,"to_string","","",4,{"inputs":[{"name":"instruction"}],"output":{"name":"string"}}],[11,"new","","",5,{"inputs":[{"name":"parser"}],"output":{"name":"parser"}}],[11,"parse","","",5,{"inputs":[{"name":"parser"},{"name":"str"}],"output":null}],[11,"emit_insts","","",5,{"inputs":[{"name":"parser"}],"output":{"name":"vec"}}],[0,"backend","radeco","",null,null],[3,"D","radeco::backend","",null,null],[0,"lang_c","","",null,null],[5,"serialize","radeco::backend::lang_c","Serializes SCFNodes for debugging purposes.\nTODO: Newlines and indentation, also cover all constructs",null,{"inputs":[{"name":"scfnode"}],"output":{"name":"string"}}],[0,"scf","radeco::backend","",null,null],[3,"MutRefDomain","radeco::backend::scf","Tells SCFNode to refer to children via &SCFNode",null,null],[3,"BoxDomain","","Tells SCFNode to refer to children via Box<SCFNode> (for testing only)",null,null],[4,"ForInitClause","","This enum distinguishes between\nfor(x=1;;) and\nfor(int x=1;;)",null,null],[13,"InitDeclaration","","",6,null],[13,"InitExpression","","",6,null],[4,"SCFNode","","Enum to represent *syntactic* flow structures in C.",null,null],[13,"Empty","","",7,null],[13,"Statement","","",7,null],[13,"Seq","","",7,null],[12,"body","radeco::backend::scf::SCFNode","",7,null],[12,"noreturn","","",7,null],[13,"Cond","radeco::backend::scf","",7,null],[12,"cond","radeco::backend::scf::SCFNode","",7,null],[12,"body","","",7,null],[12,"alt","","",7,null],[13,"Switch","radeco::backend::scf","",7,null],[12,"selector","radeco::backend::scf::SCFNode","",7,null],[12,"cases","","",7,null],[12,"default","","",7,null],[13,"While","radeco::backend::scf","",7,null],[12,"cond","radeco::backend::scf::SCFNode","",7,null],[12,"body","","",7,null],[13,"DoWhile","radeco::backend::scf","",7,null],[12,"cond","radeco::backend::scf::SCFNode","",7,null],[12,"body","","",7,null],[13,"For","radeco::backend::scf","",7,null],[12,"init","radeco::backend::scf::SCFNode","",7,null],[12,"cond","","",7,null],[12,"step","","",7,null],[12,"body","","",7,null],[8,"SCFDomain","radeco::backend::scf","A trait to pass to SCFNode to control the types for declarations,\nexpressions, statements, and most importantly to child SCFNodes.\nThe choice of the name 'Domain' was arbitrary.",null,null],[16,"Declaration","radeco::backend::scf::SCFDomain","",null,null],[16,"Expression","","",null,null],[16,"Statement","","",null,null],[16,"Node","","",null,null],[6,"Declaration","radeco::backend::scf","",null,null],[6,"Expression","","",null,null],[6,"Statement","","",null,null],[6,"Node","","",null,null],[6,"Declaration","","",null,null],[6,"Expression","","",null,null],[6,"Statement","","",null,null],[6,"Node","","",null,null],[11,"fmt","","",6,{"inputs":[{"name":"forinitclause"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",7,{"inputs":[{"name":"scfnode"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","radeco::backend","",8,{"inputs":[{"name":"d"},{"name":"formatter"}],"output":{"name":"result"}}],[6,"Declaration","","",null,null],[6,"Expression","","",null,null],[6,"Statement","","",null,null],[6,"Node","","",null,null]],"paths":[[4,"Arity"],[4,"Location"],[3,"Operator"],[3,"Value"],[3,"Instruction"],[3,"Parser"],[4,"ForInitClause"],[4,"SCFNode"],[3,"D"]]};
searchIndex['rustc_data_structures'] = {"items":[[0,"","rustc_data_structures","Various data structures used by the Rust compiler. The intention\nis that code in here should be not be *specific* to rustc, so that\nit can be easily unit tested and so forth.",null,null],[0,"snapshot_vec","","A utility class for implementing \"snapshottable\" things; a snapshottable data structure permits\nyou to take a snapshot (via `start_snapshot`) and then, after making some changes, elect either\nto rollback to the start of the snapshot or commit those changes.",null,null],[3,"SnapshotVec","rustc_data_structures::snapshot_vec","",null,null],[3,"Snapshot","","",null,null],[4,"UndoLog","","",null,null],[13,"OpenSnapshot","","Indicates where a snapshot started.",0,null],[13,"CommittedSnapshot","","Indicates a snapshot that has been committed.",0,null],[13,"NewElem","","New variable with given index was created.",0,null],[13,"SetElem","","Variable with given index was changed *from* the given value.",0,null],[13,"Other","","Extensible set of actions",0,null],[8,"SnapshotVecDelegate","","",null,null],[16,"Value","rustc_data_structures::snapshot_vec::SnapshotVecDelegate","",null,null],[16,"Undo","","",null,null],[10,"reverse","rustc_data_structures::snapshot_vec","",1,{"inputs":[{"name":"snapshotvecdelegate"},{"name":"vec"},{"name":"undo"}],"output":null}],[11,"new","","",2,{"inputs":[{"name":"snapshotvec"}],"output":{"name":"snapshotvec"}}],[11,"record","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"d"}],"output":null}],[11,"len","","",2,{"inputs":[{"name":"snapshotvec"}],"output":{"name":"usize"}}],[11,"push","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"d"}],"output":{"name":"usize"}}],[11,"get","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"usize"}],"output":{"name":"d"}}],[11,"get_mut","","Returns a mutable pointer into the vec; whatever changes you make here cannot be undone\nautomatically, so you should be sure call `record()` with some sort of suitable undo\naction.",2,{"inputs":[{"name":"snapshotvec"},{"name":"usize"}],"output":{"name":"d"}}],[11,"set","","Updates the element at the given index. The old value will saved (and perhaps restored) if\na snapshot is active.",2,{"inputs":[{"name":"snapshotvec"},{"name":"usize"},{"name":"d"}],"output":null}],[11,"start_snapshot","","",2,{"inputs":[{"name":"snapshotvec"}],"output":{"name":"snapshot"}}],[11,"actions_since_snapshot","","",2,null],[11,"rollback_to","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"snapshot"}],"output":null}],[11,"commit","","Commits all changes since the last snapshot. Of course, they\ncan still be undone if there is a snapshot further out.",2,{"inputs":[{"name":"snapshotvec"},{"name":"snapshot"}],"output":null}],[6,"Target","","",null,null],[11,"deref","","",2,null],[11,"deref_mut","","",2,null],[6,"Output","","",null,null],[11,"index","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"usize"}],"output":{"name":"d"}}],[11,"index_mut","","",2,{"inputs":[{"name":"snapshotvec"},{"name":"usize"}],"output":{"name":"d"}}],[0,"graph","rustc_data_structures","A graph module for use in dataflow, region resolution, and elsewhere.",null,null],[3,"Graph","rustc_data_structures::graph","",null,null],[3,"Node","","",null,null],[12,"data","","",3,null],[3,"Edge","","",null,null],[12,"data","","",4,null],[3,"NodeIndex","","",null,null],[3,"EdgeIndex","","",null,null],[3,"Direction","","",null,null],[3,"AdjacentEdges","","",null,null],[3,"AdjacentTargets","","",null,null],[3,"AdjacentSources","","",null,null],[3,"DepthFirstTraversal","","",null,null],[5,"each_edge_index","","",null,{"inputs":[{"name":"edgeindex"},{"name":"f"}],"output":null}],[17,"INVALID_EDGE_INDEX","","",null,null],[17,"OUTGOING","","",null,null],[17,"INCOMING","","",null,null],[6,"Value","","",null,null],[6,"Undo","","",null,null],[11,"reverse","","",3,null],[6,"Value","","",null,null],[6,"Undo","","",null,null],[11,"reverse","","",4,null],[11,"fmt","","",4,{"inputs":[{"name":"edge"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",5,{"inputs":[{"name":"nodeindex"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",5,{"inputs":[{"name":"nodeindex"},{"name":"nodeindex"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"nodeindex"},{"name":"nodeindex"}],"output":{"name":"bool"}}],[11,"clone","","",5,{"inputs":[{"name":"nodeindex"}],"output":{"name":"nodeindex"}}],[11,"fmt","","",6,{"inputs":[{"name":"edgeindex"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",6,{"inputs":[{"name":"edgeindex"},{"name":"edgeindex"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"edgeindex"},{"name":"edgeindex"}],"output":{"name":"bool"}}],[11,"clone","","",6,{"inputs":[{"name":"edgeindex"}],"output":{"name":"edgeindex"}}],[11,"fmt","","",7,{"inputs":[{"name":"direction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"direction"}],"output":{"name":"direction"}}],[11,"node_id","","Returns unique id (unique with respect to the graph holding associated node).",5,{"inputs":[{"name":"nodeindex"}],"output":{"name":"usize"}}],[11,"edge_id","","Returns unique id (unique with respect to the graph holding associated edge).",6,{"inputs":[{"name":"edgeindex"}],"output":{"name":"usize"}}],[11,"new","","",8,{"inputs":[{"name":"graph"}],"output":{"name":"graph"}}],[11,"all_nodes","","",8,null],[11,"all_edges","","",8,null],[11,"next_node_index","","",8,{"inputs":[{"name":"graph"}],"output":{"name":"nodeindex"}}],[11,"add_node","","",8,{"inputs":[{"name":"graph"},{"name":"n"}],"output":{"name":"nodeindex"}}],[11,"mut_node_data","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"n"}}],[11,"node_data","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"n"}}],[11,"node","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"node"}}],[11,"next_edge_index","","",8,{"inputs":[{"name":"graph"}],"output":{"name":"edgeindex"}}],[11,"add_edge","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"},{"name":"nodeindex"},{"name":"e"}],"output":{"name":"edgeindex"}}],[11,"mut_edge_data","","",8,{"inputs":[{"name":"graph"},{"name":"edgeindex"}],"output":{"name":"e"}}],[11,"edge_data","","",8,{"inputs":[{"name":"graph"},{"name":"edgeindex"}],"output":{"name":"e"}}],[11,"edge","","",8,{"inputs":[{"name":"graph"},{"name":"edgeindex"}],"output":{"name":"edge"}}],[11,"first_adjacent","","Accesses the index of the first edge adjacent to `node`.\nThis is useful if you wish to modify the graph while walking\nthe linked list of edges.",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"},{"name":"direction"}],"output":{"name":"edgeindex"}}],[11,"next_adjacent","","Accesses the next edge in a given direction.\nThis is useful if you wish to modify the graph while walking\nthe linked list of edges.",8,{"inputs":[{"name":"graph"},{"name":"edgeindex"},{"name":"direction"}],"output":{"name":"edgeindex"}}],[11,"each_node","","Iterates over all edges defined in the graph.",8,{"inputs":[{"name":"graph"},{"name":"f"}],"output":{"name":"bool"}}],[11,"each_edge","","Iterates over all edges defined in the graph",8,{"inputs":[{"name":"graph"},{"name":"f"}],"output":{"name":"bool"}}],[11,"outgoing_edges","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"adjacentedges"}}],[11,"incoming_edges","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"adjacentedges"}}],[11,"adjacent_edges","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"},{"name":"direction"}],"output":{"name":"adjacentedges"}}],[11,"successor_nodes","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"adjacenttargets"}}],[11,"predecessor_nodes","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"adjacentsources"}}],[11,"iterate_until_fixed_point","","",8,{"inputs":[{"name":"graph"},{"name":"f"}],"output":null}],[11,"depth_traverse","","",8,{"inputs":[{"name":"graph"},{"name":"nodeindex"}],"output":{"name":"depthfirsttraversal"}}],[6,"Item","","",null,null],[11,"next","","",9,{"inputs":[{"name":"adjacentedges"}],"output":{"name":"option"}}],[6,"Item","","",null,null],[11,"next","","",10,{"inputs":[{"name":"adjacenttargets"}],"output":{"name":"option"}}],[6,"Item","","",null,null],[11,"next","","",11,{"inputs":[{"name":"adjacentsources"}],"output":{"name":"option"}}],[6,"Item","","",null,null],[11,"next","","",12,{"inputs":[{"name":"depthfirsttraversal"}],"output":{"name":"option"}}],[11,"source","","",4,{"inputs":[{"name":"edge"}],"output":{"name":"nodeindex"}}],[11,"target","","",4,{"inputs":[{"name":"edge"}],"output":{"name":"nodeindex"}}],[0,"bitvec","rustc_data_structures","",null,null],[3,"BitVector","rustc_data_structures::bitvec","A very simple BitVector type.",null,null],[11,"new","","",13,{"inputs":[{"name":"bitvector"},{"name":"usize"}],"output":{"name":"bitvector"}}],[11,"contains","","",13,{"inputs":[{"name":"bitvector"},{"name":"usize"}],"output":{"name":"bool"}}],[11,"insert","","",13,{"inputs":[{"name":"bitvector"},{"name":"usize"}],"output":{"name":"bool"}}],[0,"unify","rustc_data_structures","",null,null],[3,"VarValue","rustc_data_structures::unify","Value of a unification key. We implement Tarjan's union-find\nalgorithm: when two keys are unified, one of them is converted\ninto a \"redirect\" pointing at the other. These redirects form a\nDAG: the roots of the DAG (nodes that are not redirected) are each\nassociated with a value of type `V` and a rank. The rank is used\nto keep the DAG relatively balanced, which helps keep the running\ntime of the algorithm under control. For more information, see\n<http://en.wikipedia.org/wiki/Disjoint-set_data_structure>.",null,null],[3,"UnificationTable","","Table of unification keys and their values.",null,null],[3,"Snapshot","","At any time, users may snapshot a unification table.  The changes\nmade during the snapshot may either be *committed* or *rolled back*.",null,null],[8,"UnifyKey","","This trait is implemented by any type that can serve as a type\nvariable. We call such variables *unification keys*. For example,\nthis trait is implemented by `IntVid`, which represents integral\nvariables.",null,null],[16,"Value","rustc_data_structures::unify::UnifyKey","",null,null],[10,"index","rustc_data_structures::unify","",14,{"inputs":[{"name":"unifykey"}],"output":{"name":"u32"}}],[10,"from_index","","",14,{"inputs":[{"name":"unifykey"},{"name":"u32"}],"output":{"name":"self"}}],[10,"tag","","",14,{"inputs":[{"name":"unifykey"},{"name":"option"}],"output":{"name":"str"}}],[11,"fmt","","",15,{"inputs":[{"name":"varvalue"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",15,{"inputs":[{"name":"varvalue"}],"output":{"name":"varvalue"}}],[11,"eq","","",15,{"inputs":[{"name":"varvalue"},{"name":"varvalue"}],"output":{"name":"bool"}}],[11,"ne","","",15,{"inputs":[{"name":"varvalue"},{"name":"varvalue"}],"output":{"name":"bool"}}],[11,"new","","",16,{"inputs":[{"name":"unificationtable"}],"output":{"name":"unificationtable"}}],[11,"snapshot","","Starts a new snapshot. Each snapshot must be either\nrolled back or committed in a \"LIFO\" (stack) order.",16,{"inputs":[{"name":"unificationtable"}],"output":{"name":"snapshot"}}],[11,"rollback_to","","Reverses all changes since the last snapshot. Also\nremoves any keys that have been created since then.",16,{"inputs":[{"name":"unificationtable"},{"name":"snapshot"}],"output":null}],[11,"commit","","Commits all changes since the last snapshot. Of course, they\ncan still be undone if there is a snapshot further out.",16,{"inputs":[{"name":"unificationtable"},{"name":"snapshot"}],"output":null}],[11,"new_key","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"}],"output":{"name":"k"}}],[11,"union","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"},{"name":"k"}],"output":null}],[11,"find","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"}],"output":{"name":"k"}}],[11,"unioned","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"},{"name":"k"}],"output":{"name":"bool"}}],[11,"unify_var_var","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"},{"name":"k"}],"output":{"name":"result"}}],[11,"unify_var_value","","Sets the value of the key `a_id` to `b`. Because simple keys do not have any subtyping\nrelationships, if `a_id` already has a value, it must be the same as `b`.",16,{"inputs":[{"name":"unificationtable"},{"name":"k"},{"name":"v"}],"output":{"name":"result"}}],[11,"has_value","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"}],"output":{"name":"bool"}}],[11,"probe","","",16,{"inputs":[{"name":"unificationtable"},{"name":"k"}],"output":{"name":"option"}}]],"paths":[[4,"UndoLog"],[8,"SnapshotVecDelegate"],[3,"SnapshotVec"],[3,"Node"],[3,"Edge"],[3,"NodeIndex"],[3,"EdgeIndex"],[3,"Direction"],[3,"Graph"],[3,"AdjacentEdges"],[3,"AdjacentTargets"],[3,"AdjacentSources"],[3,"DepthFirstTraversal"],[3,"BitVector"],[8,"UnifyKey"],[3,"VarValue"],[3,"UnificationTable"]]};
initSearch(searchIndex);