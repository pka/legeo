//! tilelive-null, A noop sink

use actix::prelude::*;
use tilesink::*;

// Actor
pub struct NullSink {}

// Declare actor and its context
impl Actor for NullSink {
    type Context = Context<Self>;
}

// Handler for `PutTile` message
impl Handler<PutTile> for NullSink {
    type Result = PutTileResult;

    fn handle(&mut self, _msg: PutTile, _: &mut Context<Self>) -> Self::Result {
        Ok(())
    }
}

// var Null = function(uri, callback) {
//   return setImmediate(callback, null, this);
// };

// Null.registerProtocols = function(tilelive) {
//   tilelive.protocols["null:"] = Null;
// };

// Null.prototype.putTile = function(z, x, y, data, callback) {
//   return setImmediate(callback);
// };

// Null.prototype.putInfo = function(obj, callback) {
//   return setImmediate(callback);
// };
