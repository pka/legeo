//! Tile operations

use crate::file::*;
use crate::grid_iterator::GridIterator;
use crate::null::NullSink;
use crate::tilesink::PutTile;
use crate::tilesource::GetTile;
use ::actix::prelude::*;
use futures::Future;

//  Methods from https://github.com/mapbox/tilelive/blob/master/lib/tilelive.js

// * `tilelive.list(source, callback)`: Lists all tilesets in a directory. `source` is a folder that is used by registered implementations to search for individual tilesets. `callback` receives an error object (or `null`) and a hash with keys being Tilestore IDs and values being Tilestore URIs. Example:
//
//       {
//           "world-light": "mbtiles:///path/to/file/world-light.mbtiles",
//           "mapquest": "tilejson:///path/to/file/mapquest.tilejson"
//       }
//
// * `tilelive.findID(source, id, callback)`: Looks for a particular tileset ID in a directory. `callback` receives an error object (or `null`) and the URI of the tileset.
//
// * `tilelive.load(uri, callback)`: Loads the Tilestore object associated with the specified `uri`. `callback` receives an error object (or `null`) and the [Tilestore object](API.md).
//
// * `tilelive.info(uri, callback)`: Loads the Tilestore object associated with the specified `uri` and retrieves its metadata in a [TileJSON](http://github.com/mapbox/tilejson-spec) compliant format. `callback` receives an error object (or `null`), the metadata hash and the Tilestore object.
//
// * `tilelive.all(source, callback)`: Loads metadata in a [TileJSON](http://github.com/mapbox/tilejson-spec) compliant format for all tilesets in the `source` directory. `callback` receives an error object (or `null`) and an array with TileJSON metadata about each tileset in that directory.
//
// * `tilelive.verify(tilejson)`: Validates a TileJSON object and returns error objects for invalid entries.
//
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

pub struct TileInput {
    uri: String,
}

impl TileInput {
    pub fn from_uri(uri: String) -> TileInput {
        TileInput { uri }
    }
    fn start_actor(&self) -> Addr<FileBackend> {
        // FIXME: return Addr<impl Actor + Handler<GetTile>>
        let actor = FileBackend::new(&self.uri).unwrap();
        Arbiter::start(move |_| actor)
    }
}

pub struct TileOutput {
    uri: String,
}

impl TileOutput {
    pub fn from_uri(uri: String) -> TileOutput {
        TileOutput { uri }
    }
    fn start_actor(&self) -> Addr<NullSink> {
        let actor = NullSink {}; // FIXME
        Arbiter::start(move |_| actor)
    }
}

pub fn tile_copy(src: TileInput, dst: TileOutput) {
    let srcaddr = src.start_actor();
    let dstaddr = dst.start_actor();

    let griditer = GridIterator::new();
    for (z, x, y) in griditer {
        println!("x={}", x);
        //TODO: use ActorStream API
        let res = srcaddr
            .send(GetTile { z, x, y })
            .and_then(|tile| {
                dstaddr.send(PutTile {
                    x,
                    y,
                    z,
                    data: tile.unwrap(),
                })
            })
            .map_err(|err| println!("{:?}", err));
        println!("res={:?}", res.wait()); // FIXME: without wait() tiles are skipped!?
    }
}
