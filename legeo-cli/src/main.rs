mod registry;

use ::actix::prelude::*;
use legeo::operation::tile_copy;
use std::env;

// Call example: legeo 'file:///tmp/legeo?filetype=pbf' 'file:///tmp/legeoout?filetype=pbf'
fn main() {
    let args: Vec<String> = env::args().collect();
    let srcuri = args.get(1).expect("source-url missing").clone();
    let dsturi = args.get(2).expect("dest-url missing").clone();
    System::run(|| {
        let src = registry::TileInput::from_uri(srcuri);
        let dst = registry::TileOutput::from_uri(dsturi);
        tile_copy(src, dst);

        System::current().stop();
    });
}
