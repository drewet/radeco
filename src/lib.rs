#![doc(html_root_url = "https://radare.github.io/radeco/")]
#![doc(html_logo_url = "http://rada.re/r/img/r2logo3.png")]
#![feature(collections_bound)]
#![feature(btree_range)]
extern crate regex;
extern crate petgraph;
extern crate rustc_serialize;
extern crate num;

#[macro_use]
extern crate r2pipe;

pub mod middle;
pub mod analysis;
pub mod transform;
pub mod frontend;
pub mod backend;
pub mod util;
