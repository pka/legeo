pub mod file;
pub mod grid;
pub mod grid_iterator;
#[cfg(test)]
mod grid_test;
pub mod null;
pub mod operation;
pub mod tilesink;
pub mod tilesource;

use crate::operation::{tile_copy, TileInput, TileOutput};
use ::actix::prelude::*;
use std::env;

// Call example: legeo 'file:///tmp/legeo?filetype=pbf' 'file:///tmp/legeoout?filetype=pbf'
fn main() {
    let args: Vec<String> = env::args().collect();
    let srcuri = args.get(1).expect("source-url missing").clone();
    let dsturi = args.get(2).expect("dest-url missing").clone();
    System::run(|| {
        let src = TileInput::from_uri(srcuri);
        let dst = TileOutput::from_uri(dsturi);
        tile_copy(src, dst);

        System::current().stop();
    });
}
