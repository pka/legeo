//! tilelive Tilesource messages and traits

use actix::Message;

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

pub struct GetTile {
    pub z: usize,
    pub x: usize,
    pub y: usize,
}

pub type GetTileResult = std::io::Result<Vec<u8>>;

impl Message for GetTile {
    type Result = GetTileResult;
}
