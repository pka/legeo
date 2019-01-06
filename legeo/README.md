LEGeo
=====

LEGeo is a geoprocessing framework inspired by [Tilelive](https://github.com/mapbox/tilelive).

From the Tilelive documentation:

Tilelive is designed for streaming map tiles from _sources_ (like custom geographic data formats) to _sinks_ (destinations, like file systems) by providing a consistent API.

LEGeo is written in [Rust](https://www.rust-lang.org/), using the asynchronous high-performance framework [Actix](https://actix.rs/).

From the Actix [user guide](https://actix.rs/book/actix/):

Actix is built on the [Actor Model](https://en.wikipedia.org/wiki/Actor_model) which allows applications to be written as a group of independently executing but cooperating "Actors" which communicate via messages. Actors are objects which encapsulate state and behavior and run within the Actor System provided by the actix library.
