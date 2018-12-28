pub mod file;
pub mod grid_iterator;
pub mod null;
pub mod operation;
pub mod tilesink;
pub mod tilesource;

use crate::operation::{tile_copy, TileInput, TileOutput};
use ::actix::prelude::*;

fn main() {
    System::run(|| {
        let src = TileInput::from_uri("file:///tmp/legeo?filetype=txt".to_string());
        let dst = TileOutput::from_uri("file:///tmp/legeoout?filetype=txt".to_string());
        tile_copy(src, dst);

        System::current().stop();
    });
}
