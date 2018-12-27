pub mod file;
pub mod null;
pub mod tilesink;
pub mod tilesource;

use crate::null::NullSink;
use crate::tilesink::PutTile;
use ::actix::prelude::*;
use futures::Future;

fn main() {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let addr = NullSink {}.start();

        // send message and get future for result
        let res = addr.send(PutTile {
            x: 10,
            y: 0,
            z: 0,
            data: Vec::new(),
        });

        // handle() returns tokio handle
        tokio::spawn(
            res.map(|_res| {
                println!("PutTile to NullSink successful");

                // stop system and exit
                System::current().stop();
            })
            .map_err(|_| ()),
        );
    });
}
