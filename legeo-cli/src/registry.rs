//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! TileInput/TileOutput registry

use ::actix::prelude::*;
use legeo::message::{GetTile, PutTile};
use legeo::operation::{TileInput as TileInputTrait, TileOutput as TileOutputTrait};
use legeo::tileconnector::Tileconnector;
use legeo_file::file::*;
use legeo_mbtiles::mbtiles::*;
use legeo_null::null::*;
use url::{self, Url};

pub struct TileInput {
    uri: String,
}

pub struct TileOutput {
    uri: String,
}

impl TileInput {
    pub fn from_uri(uri: String) -> TileInput {
        TileInput { uri }
    }
}

impl TileInputTrait for TileInput {
    fn start_actor(&self) -> Recipient<GetTile> {
        let uri = self.uri.clone();
        let url = Url::parse(&self.uri).unwrap();
        // TODO: Replace with a dynamic registry in legeo crate
        match url.scheme() {
            "file" => Arbiter::start(move |_| FileBackend::load(&uri).unwrap()).recipient(),
            "mbtiles" => Arbiter::start(move |_| Mbtiles::load(&uri).unwrap()).recipient(),
            _ => Arbiter::start(move |_| FileBackend::load(&uri).unwrap()).recipient(),
        }
    }
}

impl TileOutput {
    pub fn from_uri(uri: String) -> TileOutput {
        TileOutput { uri }
    }
}

impl TileOutputTrait for TileOutput {
    fn start_actor(&self) -> Recipient<PutTile> {
        let uri = self.uri.clone();
        let url = Url::parse(&self.uri).unwrap();
        // TODO: Replace with a dynamic registry in legeo crate
        match url.scheme() {
            "file" => Arbiter::start(move |_| FileBackend::load(&uri).unwrap()).recipient(),
            "null" => Arbiter::start(move |_| NullSink::load(&uri).unwrap()).recipient(),
            _ => Arbiter::start(move |_| NullSink::load(&uri).unwrap()).recipient(),
        }
    }
}
