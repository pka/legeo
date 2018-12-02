//! Reads/writes tiles and grids from/to the filesystem.

use actix::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tilesink::*;
use url::{self, Url};

pub struct FileSink {
    basepath: String,
    filetype: String,
    safe: bool,
}

impl FileSink {
    /// Create FileSink with base path and format information from `uri`
    pub fn new(uri: &str) -> Result<Self, url::ParseError> {
        let uri = Url::parse(uri)?;
        let basepath = uri.path().to_string();
        let params: HashMap<_, _> = uri.query_pairs().collect();
        let filetype = params
            .get("filetype")
            .unwrap_or(&Cow::from("png"))
            .to_string();
        let safe = params.get("safe").map_or(false, |v| v == "true");
        Ok(FileSink {
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

impl Actor for FileSink {
    type Context = Context<Self>;
}

impl Handler<PutTile> for FileSink {
    type Result = usize;

    fn handle(&mut self, msg: PutTile, _: &mut Context<Self>) -> Self::Result {
        let path = self.get_path(msg.z, msg.x, msg.y, &self.filetype);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut f = File::create(path).unwrap();
        f.write_all(&msg.data).unwrap();
        0
    }
}

#[test]
fn test_put_tile() {
    use futures::Future;
    use std::io::Read;

    let actor = FileSink::new("file:///tmp/legeo?filetype=txt").unwrap();
    System::run(move || {
        let addr = Arbiter::start(move |_| actor);
        let tile_data = b"3/7/7";
        addr.send(PutTile {
            z: 3,
            x: 7,
            y: 7,
            data: tile_data.to_vec(),
        }).wait()
        .unwrap();
        let mut file = File::open("/tmp/legeo/3/7/7.txt").unwrap();
        let mut content = [0; 5];
        file.read(&mut content).unwrap();
        assert_eq!(&content, tile_data);
        System::current().stop();
    });
}

#[test]
fn test_put_tile_safe() {
    use futures::Future;
    use std::io::Read;

    let actor = FileSink::new("file:///tmp/legeo?safe=true&filetype=txt").unwrap();
    System::run(move || {
        let addr = Arbiter::start(move |_| actor);
        let tile_data = b"3/7/7";
        addr.send(PutTile {
            z: 3,
            x: 7,
            y: 7,
            data: tile_data.to_vec(),
        }).wait()
        .unwrap();
        let mut file = File::open("/tmp/legeo/3/000/007/000/007.txt").unwrap();
        let mut content = [0; 5];
        file.read(&mut content).unwrap();
        assert_eq!(&content, tile_data);
        System::current().stop();
    });
}
