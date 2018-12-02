//! tilelive-null, A noop sink

use actix::prelude::*;
use tilesink::*;

// Actor
pub struct NullSink {
    pub count: usize,
}

// Declare actor and its context
impl Actor for NullSink {
    type Context = Context<Self>;
}

// Handler for `PutTile` message
impl Handler<PutTile> for NullSink {
    type Result = usize;

    fn handle(&mut self, msg: PutTile, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.x;
        self.count
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
