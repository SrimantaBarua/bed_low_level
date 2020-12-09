// (C) 2020 Srimanta Barua <srimanta.barua1@gmail.com>

#[macro_use]
extern crate bitflags;

mod cmap;
mod coverage;
mod error;
mod face;
mod featurelist;
mod font_collection;
mod gasp;
mod gsub;
mod head;
mod hhea;
mod hmtx;
mod lookuplist;
mod maxp;
mod rcbuffer;
mod scriptlist;
mod types;

pub use error::*;
pub use face::Face;
pub use font_collection::FontCollection;
