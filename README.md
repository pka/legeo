LEGeo
=====

LEGeo is a geoprocessing framework inspired by [Tilelive](https://github.com/mapbox/tilelive).

Modules:

* [legeo](./legeo): Core framework
* [legeo-xyz](./legeo-xyz): Tile grid library
* [legeo-cli](./legeo-cli): Command line interface

Tilesource and/or Tilesink implementations:

* [legeo-file](./legeo-file): Reads/writes tiles from/to the filesystem
* [legeo-null](./legeo-null): Noop Tilesink implementation
* [legeo-mbtiles](./legeo-mbtiles): Reads tiles from MBTiles
