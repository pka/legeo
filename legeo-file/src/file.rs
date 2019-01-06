//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Reads/writes tiles from/to the filesystem.

use ::actix::prelude::*;
use legeo::message::{GetTile, GetTileResult, PutTile, PutTileResult};
use legeo::tileconnector::Tileconnector;
use legeo::tilesink::Tilesink;
use legeo::tilesource::Tilesource;
use log::debug;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use url::{self, Url};

pub struct FileBackend {
    basepath: String,
    filetype: String,
    safe: bool,
}

impl FileBackend {
    fn get_path(&self, z: u8, x: u32, y: u32, ext: &str) -> PathBuf {
        let mut path = if self.safe {
            Path::new(&self.basepath)
                .join(z.to_string())
                .join(format!("{:03}", x / 1000))
                .join(format!("{:03}", x % 1000))
                .join(format!("{:03}", y / 1000))
                .join(format!("{:03}", y % 1000))
        } else {
            Path::new(&self.basepath)
                .join(z.to_string())
                .join(x.to_string())
                .join(y.to_string())
        };
        path.set_extension(ext);
        path
    }
}

impl Tileconnector for FileBackend {
    /// Create FileBackend with base path and format information from `uri`
    fn load(uri: &str) -> Result<Self, url::ParseError> {
        let uri = Url::parse(uri)?;
        let basepath = uri.path().to_string();
        let params: HashMap<_, _> = uri.query_pairs().collect();
        let filetype = params
            .get("filetype")
            .unwrap_or(&Cow::from("png"))
            .to_string();
        let safe = params.get("safe").map_or(false, |v| v == "true");
        Ok(FileBackend {
            basepath,
            filetype,
            safe,
        })
    }
}

impl Tilesource for FileBackend {
    fn get_tile(&self, z: u8, x: u32, y: u32) -> std::io::Result<Vec<u8>> {
        let path = self.get_path(z, x, y, &self.filetype);
        debug!("GetTile {:?}", path);
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }
}

impl Tilesink for FileBackend {
    fn put_tile(&self, z: u8, x: u32, y: u32, data: Vec<u8>) -> std::io::Result<()> {
        let path = self.get_path(z, x, y, &self.filetype);
        debug!("PutTile {:?}", path);
        fs::create_dir_all(path.parent().unwrap())?;
        let mut f = File::create(path)?;
        f.write_all(&data)?;
        Ok(())
    }
}

impl Actor for FileBackend {
    type Context = Context<Self>;
}

impl Handler<GetTile> for FileBackend {
    type Result = GetTileResult;

    fn handle(&mut self, msg: GetTile, _: &mut Context<Self>) -> Self::Result {
        self.get_tile(msg.z, msg.x, msg.y)
    }
}

impl Handler<PutTile> for FileBackend {
    type Result = PutTileResult;

    fn handle(&mut self, msg: PutTile, _: &mut Context<Self>) -> Self::Result {
        self.put_tile(msg.z, msg.x, msg.y, msg.data)
    }
}

#[test]
fn test_tile() {
    let backend = FileBackend::load("file:///tmp/legeo?filetype=txt").unwrap();
    let tile_data = b"3/7/7";
    let _ = backend.put_tile(3, 7, 7, tile_data.to_vec());
    let mut file = File::open("/tmp/legeo/3/7/7.txt").unwrap();
    let mut content = [0; 5];
    file.read(&mut content).unwrap();
    assert_eq!(&content, tile_data);

    let tile = backend.get_tile(3, 7, 7).unwrap();
    assert_eq!(&tile, tile_data);
}

#[test]
fn test_tile_safe() {
    let backend = FileBackend::load("file:///tmp/legeo?safe=true&filetype=txt").unwrap();
    let tile_data = b"3/7/7";
    let _ = backend.put_tile(3, 7, 7, tile_data.to_vec());
    let mut file = File::open("/tmp/legeo/3/000/007/000/007.txt").unwrap();
    let mut content = [0; 5];
    file.read(&mut content).unwrap();
    assert_eq!(&content, tile_data);

    let tile = backend.get_tile(3, 7, 7).unwrap();
    assert_eq!(&tile, tile_data);
}
