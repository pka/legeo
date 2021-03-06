//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Tile operations

use crate::message::{GetTile, PutTile};
use ::actix::prelude::*;
use futures::Future;
use legeo_xyz::grid::{extent_to_merc, Extent, Grid};
use legeo_xyz::grid_iterator::GridIterator;
use log::error;

//  From https://github.com/mapbox/tilelive/blob/master/lib/tilelive.js

// ## Read/write streams
//
// Tilelive provides an implementation of node object streams for copying tiles from one source to another.
//
// ```javascript
// // Copy all tiles and metadata from source A to source B.
// var get = tilelive.createReadStream(sourceA);
// var put = tilelive.createWriteStream(sourceB);
// get.pipe(put);
// put.on('finish', function() {
//     console.log('done!');
// });
// ```
// ## Parallel read streams
//
// Tilelive can split a read operation into an arbitrary number of jobs. Pass a `job` parameter to options when using `tilelive.createReadStream` or `tilelive.deserialize`:
//
// ```javascript
// var readable = tilelive.createReadStream(src, { type: 'scanline', job: { total: 4, num: 1 } });
// ```
//
// This instructs tilelive to only read tiles that would fall into job `1` of `4`. A complete read would mean four calls each with a different `num`.
//
// ## bin/tilelive-copy
//
// tilelive can be used to copy data between tilestores. The CLI tool uses tilelive.auto() to register plugins by filename. For example, file.mbtiles will result in using the `mbtiles:` protocol and the `@mapbox/mbtiles` module.
//
// ```shell
// # usage
// tilelive-copy <src> <dst>
//
// # example
// tilelive-copy orig.mbtiles copy.mbtiles
// ```
//
// Options:
//
// * **--scheme**=[scanline,pyramid,list] - Default: scanline.
// * **--list**=[filepath] - Filepath if scheme is list.
// * **--concurrency**=[number] - Control on the number of pending I/O operations with the underlying source during copy. Note: this is not CPU concurrency, which is handled by individual plugins typically by setting UV_THREADPOOL_SIZE=[number] as an environment variable.
// * **--withoutprogress** - Shows progress by default.
// * **--timeout**=[number] - Timeout after `n` ms of inactivity.
// * **--slow**=[number] - Warn on slow tiles.
// * **--exit** - Exit explicitly when copy is complete.
// * **--bounds**=[w,s,e,n] - as defined by the [TileJSON specification](https://github.com/mapbox/tilejson-spec)
// * **--minzoom**=[number] - as defined by the [TileJSON specification](https://github.com/mapbox/tilejson-spec)
// * **--maxzoom**=[number] - as defined by the [TileJSON specification](https://github.com/mapbox/tilejson-spec)
// * **--parts**=[number] - total number of parts to copy (part splitting is used for processing in parallel, where specific parts only copy specific tiles from the tile pyramid)
// * **--part**=[number] - the specific part to copy
// * **--retry**=[number] - number of retry attempts

pub trait TileInput {
    fn start_actor(&self) -> Recipient<GetTile>;
}

pub trait TileOutput {
    fn start_actor(&self) -> Recipient<PutTile>;
}

pub fn tile_copy(
    src: impl TileInput,
    dst: impl TileOutput,
    bounds: Extent,
    minzoom: u8,
    maxzoom: u8,
) {
    let srcaddr = src.start_actor();
    let dstaddr = dst.start_actor();

    let grid = Grid::web_mercator();
    let tile_limits = grid.tile_limits(extent_to_merc(&bounds), 0);
    let griditer = GridIterator::new(minzoom, maxzoom, tile_limits);
    for (z, x, y) in griditer {
        let res = srcaddr
            .send(GetTile { z, x, y })
            .and_then(|tile| {
                dstaddr.send(PutTile {
                    z,
                    x,
                    y,
                    data: tile.unwrap(),
                })
            })
            .map_err(|err| error!("{:?}", err));
        let _ = res.wait();
    }
}
