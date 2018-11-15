extern crate actix;
extern crate futures;
extern crate tokio;

mod null;

use actix::prelude::*;
use futures::Future;
use null::{NullSink, PutTile};


fn main() {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let addr = NullSink { count: 10 }.start();

        // send message and get future for result
        let res = addr.send(PutTile{ x: 10, y: 0, z: 0, data: Vec::new() });

        // handle() returns tokio handle
        tokio::spawn(
            res.map(|res| {
                println!("RESULT: {}", res == 20);

                // stop system and exit
                System::current().stop();
            }).map_err(|_| ()),
        );
    });
}
