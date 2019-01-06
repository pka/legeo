//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Tile connector API

use url;

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

pub trait Tileconnector {
    /// Loads the Tileconnector object associated with the specified `uri`
    fn load(uri: &str) -> Result<Self, url::ParseError>
    where
        Self: std::marker::Sized;
}
