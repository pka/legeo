//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Noop sink

use ::actix::prelude::*;
use legeo::message::{PutTile, PutTileResult};
use legeo::tileconnector::Tileconnector;
use legeo::tilesink::Tilesink;
use url;

pub struct NullSink {}

impl Tileconnector for NullSink {
    fn load(_uri: &str) -> Result<Self, url::ParseError> {
        Ok(NullSink {})
    }
}

impl Tilesink for NullSink {
    fn put_tile(&self, _z: u8, _x: u32, _y: u32, _data: Vec<u8>) -> std::io::Result<()> {
        Ok(())
    }
}

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
