mod registry;

use ::actix::prelude::*;
use legeo::operation::tile_copy;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    srcuri: String,
    dsturi: String,
}

// Call example: legeo 'file:///tmp/legeo?filetype=pbf' 'file:///tmp/legeoout?filetype=pbf'
fn main() {
    let args = Cli::from_args();
    System::run(|| {
        let src = registry::TileInput::from_uri(args.srcuri);
        let dst = registry::TileOutput::from_uri(args.dsturi);
        tile_copy(src, dst);

        System::current().stop();
    });
}
