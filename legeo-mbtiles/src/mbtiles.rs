//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! MBTiles backend

use ::actix::prelude::*;
use legeo::message::{GetTile, GetTileResult};
use legeo::tilesource::Tilesource;
use log::error;
use rusqlite::types::ToSql;
use rusqlite::{Connection, OpenFlags};
use std::borrow::Cow;
use std::collections::HashMap;
use url::{self, Url};

pub struct Mbtiles {
    conn: Connection,
}

impl Mbtiles {
    /// Create Mbtiles backend
    pub fn new(uri: &str) -> Result<Self, url::ParseError> {
        let uri = Url::parse(uri)?;
        // if (uri.hostname === '.' || uri.hostname == '..') {
        //     uri.pathname = uri.hostname + uri.pathname;
        // }
        let params: HashMap<_, _> = uri.query_pairs().collect();
        let default_mode = Cow::from("rwc");
        let mode: &str = params.get("mode").unwrap_or(&default_mode);
        let flags = match mode {
            "ro" => OpenFlags::SQLITE_OPEN_READ_ONLY,
            "rw" => OpenFlags::SQLITE_OPEN_READ_WRITE,
            "rwc" => OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
            _ => {
                error!(r#"Only supports "ro", "rw", or "rwc" mode."#);
                return Err(url::ParseError::Overflow);
            }
        };
        let conn = Connection::open_with_flags(uri.path(), flags).map_err(|e| {
            error!("Connection error: {}", e);
            url::ParseError::Overflow
        })?;
        Ok(Mbtiles { conn })
    }
}

impl Tilesource for Mbtiles {
    fn get_tile(&self, z: u8, x: u32, y: u32) -> std::io::Result<Vec<u8>> {
        // Flip Y coordinate because MBTiles files are TMS.
        let y = (1 << z) - 1 - y;

        let mut stmt = self.conn
            .prepare_cached("SELECT tile_data FROM tiles WHERE zoom_level = ?1 AND tile_column = ?2 AND tile_row = ?3")
            .unwrap();
        let tile: rusqlite::Result<Vec<u8>> =
            stmt.query_row(&[&z as &ToSql, &x, &y], |rec| rec.get(0));
        //TODO: Handle "Query returned no rows"
        let tile =
            tile.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        // var headers = tiletype.headers(row.tile_data);
        // headers['Last-Modified'] = new Date(mbtiles._stat.mtime).toUTCString();
        // headers['ETag'] = mbtiles._stat.size + '-' + Number(mbtiles._stat.mtime);

        Ok(tile)
    }
}

impl Actor for Mbtiles {
    type Context = Context<Self>;
}

impl Handler<GetTile> for Mbtiles {
    type Result = GetTileResult;

    fn handle(&mut self, msg: GetTile, _: &mut Context<Self>) -> Self::Result {
        self.get_tile(msg.z, msg.x, msg.y)
    }
}
