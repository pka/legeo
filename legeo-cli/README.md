legeo-cli
=========

Command line interface for [LEGeo](../legeo/) geoprocessing framework.

Currently, a tile copy operation can be used to copy data between tilestores.


Usage examples
--------------

    legeo --maxzoom=4 'mbtiles:///tmp/mvtbench.mbtiles?mode=ro' 'file:///tmp/tiles?filetype=pbf'
