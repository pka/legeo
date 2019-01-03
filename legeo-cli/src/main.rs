mod registry;

use ::actix::prelude::*;
use legeo::operation::tile_copy;
use legeo_xyz::grid::Extent;
use std::num::ParseFloatError;
use structopt::StructOpt;

/*
Tilelive CLI options:
    --scheme=[scanline,pyramid,list] - Default: scanline.
    --list=[filepath] - Filepath if scheme is list.
    --concurrency=[number] - Control on the number of pending I/O operations with the underlying source during copy. Note: this is not CPU concurrency, which is handled by individual plugins typically by setting UV_THREADPOOL_SIZE=[number] as an environment variable.
    --withoutprogress - Shows progress by default.
    --timeout=[number] - Timeout after n ms of inactivity.
    --slow=[number] - Warn on slow tiles.
    --exit - Exit explicitly when copy is complete.
    --bounds=[w,s,e,n] - as defined by the TileJSON specification
    --minzoom=[number] - as defined by the TileJSON specification
    --maxzoom=[number] - as defined by the TileJSON specification
    --parts=[number] - total number of parts to copy (part splitting is used for processing in parallel, where specific parts only copy specific tiles from the tile pyramid)
    --part=[number] - the specific part to copy
    --retry=[number] - number of retry attempts

Short options and defaults from https://github.com/mojodna/tl/blob/master/lib/commands/copy.js
*/

fn parse_extent(numlist: &str) -> Result<Extent, ParseFloatError> {
    let arr: Vec<&str> = numlist.split(",").collect();
    Ok(Extent {
        minx: arr[0].parse()?,
        miny: arr[1].parse()?,
        maxx: arr[2].parse()?,
        maxy: arr[3].parse()?,
    })
}

#[derive(StructOpt)]
struct Cli {
    /// WGS84 bounding box
    #[structopt(
        long,
        short,
        default_value = "-180,-85.0511,180,85.0511",
        parse(try_from_str = "parse_extent")
    )]
    bounds: Extent,
    /// Min zoom (inclusive)
    #[structopt(long, short = "z", default_value = "0")]
    minzoom: u8,
    /// Max zoom (inclusive)
    #[structopt(long, short = "Z", default_value = "22")]
    maxzoom: u8,
    /// source URI
    srcuri: String,
    /// sink URI
    dsturi: String,
}

// Call example: legeo 'file:///tmp/legeo?filetype=pbf' 'file:///tmp/legeoout?filetype=pbf'
fn main() {
    let args = Cli::from_args();
    System::run(|| {
        let src = registry::TileInput::from_uri(args.srcuri);
        let dst = registry::TileOutput::from_uri(args.dsturi);
        tile_copy(src, dst, args.bounds, args.minzoom, args.maxzoom);

        System::current().stop();
    });
}
