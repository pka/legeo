//! Reads/writes tiles and grids from/to the filesystem.

use crate::tilesink::*;
use crate::tilesource::*;
use ::actix::prelude::*;
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
    /// Create FileBackend with base path and format information from `uri`
    pub fn new(uri: &str) -> Result<Self, url::ParseError> {
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

    fn get_path(&self, z: usize, x: usize, y: usize, ext: &str) -> PathBuf {
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

impl Actor for FileBackend {
    type Context = Context<Self>;
}

impl Handler<GetTile> for FileBackend {
    type Result = GetTileResult;

    fn handle(&mut self, msg: GetTile, _: &mut Context<Self>) -> Self::Result {
        let path = self.get_path(msg.z, msg.x, msg.y, &self.filetype);
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }
}

impl Handler<PutTile> for FileBackend {
    type Result = PutTileResult;

    fn handle(&mut self, msg: PutTile, _: &mut Context<Self>) -> Self::Result {
        let path = self.get_path(msg.z, msg.x, msg.y, &self.filetype);
        fs::create_dir_all(path.parent().unwrap())?;
        let mut f = File::create(path)?;
        f.write_all(&msg.data)?;
        Ok(())
    }
}

#[test]
fn test_tile() {
    use futures::Future;

    let actor = FileBackend::new("file:///tmp/legeo?filetype=txt").unwrap();
    System::run(move || {
        let addr = Arbiter::start(move |_| actor);
        let tile_data = b"3/7/7";
        let _ = addr
            .send(PutTile {
                z: 3,
                x: 7,
                y: 7,
                data: tile_data.to_vec(),
            })
            .wait()
            .unwrap();
        let mut file = File::open("/tmp/legeo/3/7/7.txt").unwrap();
        let mut content = [0; 5];
        file.read(&mut content).unwrap();
        assert_eq!(&content, tile_data);

        let tile = addr.send(GetTile { z: 3, x: 7, y: 7 }).wait().unwrap();
        assert_eq!(&tile.unwrap(), tile_data);

        System::current().stop();
    });
}

#[test]
fn test_tile_safe() {
    use futures::Future;

    let actor = FileBackend::new("file:///tmp/legeo?safe=true&filetype=txt").unwrap();
    System::run(move || {
        let addr = Arbiter::start(move |_| actor);
        let tile_data = b"3/7/7";
        let _ = addr
            .send(PutTile {
                z: 3,
                x: 7,
                y: 7,
                data: tile_data.to_vec(),
            })
            .wait()
            .unwrap();
        let mut file = File::open("/tmp/legeo/3/000/007/000/007.txt").unwrap();
        let mut content = [0; 5];
        file.read(&mut content).unwrap();
        assert_eq!(&content, tile_data);

        let tile = addr.send(GetTile { z: 3, x: 7, y: 7 }).wait().unwrap();
        assert_eq!(&tile.unwrap(), tile_data);

        System::current().stop();
    });
}
