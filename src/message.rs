//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Actor message and result types

use actix::Message;

pub struct GetTile {
    pub z: u8,
    pub x: u32,
    pub y: u32,
}

pub type GetTileResult = std::io::Result<Vec<u8>>;

impl Message for GetTile {
    type Result = GetTileResult;
}

/// Stores a tile into the data store. Parameters are in XYZ format.
/// `tile` must contain the compressed image.
pub struct PutTile {
    pub z: u8,
    pub x: u32,
    pub y: u32,
    pub data: Vec<u8>,
}

pub type PutTileResult = std::io::Result<()>;

impl Message for PutTile {
    type Result = PutTileResult;
}

/* Generic implementation seems not possible

use ::actix::prelude::*;
use crate::tilesource::Tilesource;

impl<T: Tilesource> Actor for T {
    type Context = Context<Self>;
}

impl<T: Tilesource> Handler<GetTile> for T {
    type Result = GetTileResult;

    fn handle(&mut self, msg: GetTile, _: &mut Context<Self>) -> Self::Result {
        self.get_tile(msg.z, msg.x, msg.y)
    }
}
*/
