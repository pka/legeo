//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Tilesource trait API

use crate::tileconnector::Tileconnector;

//  https://github.com/mapbox/tilelive/blob/master/API.md
//
// ```javascript
// function Tilesource(options, callback) {
//     // call callback when done.
// }
// ```
//
// ```javascript
// // z, x, y is XYZ
// Tilesource.prototype.getTile = function(z, x, y, callback) {
//     // when initialization is incomplete, this will fail always.
//
//     // obtains tile and calls callback:
//     function(err, tile, options) {
//         // err is set when the tile does not exist or when retrieval failed.
//         // If the tile does not exist and that's OK, the error message should
//         // explicitly read 'Tile does not exist' in order to be handled correctly
//         // by tilelive.copy.
//         // otherwise, tile is a buffer containing the compressed image data
//     }
// };
// ```
//
// Get a UTFGrid tile from store
// ```javascript
// // z, x, y is XYZ
// Tilesource.prototype.getGrid = function(z, x, y, callback) {
//     // when initialization is incomplete, this will fail always.
//
//     // obtains tile and calls callback:
//     function(err, tile, options) {
//         // err is set when the tile does not exist or when retrieval failed.
//         // otherwise, tile is a buffer containing the compressed image data
//     }
// };
// ```
//
// ```javascript
// Tilesource.prototype.getInfo = function(callback) {
//     // when initialization is incomplete, this will fail always.
//
//     // obtains tile and calls callback:
//     function(err, data) {
//         // err is set when information retrieval failed.
//         // otherwise, data is a hash containing all the information.
//     }
// };
// ```

/// Map tile source
pub trait Tilesource: Tileconnector {
    fn get_tile(&self, z: u8, x: u32, y: u32) -> std::io::Result<Vec<u8>>;
}
