//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//!Tile grids

use std::f64::consts;

/// Geographic extent
#[derive(PartialEq, Clone, Debug)]
pub struct Extent {
    pub minx: f64,
    pub miny: f64,
    pub maxx: f64,
    pub maxy: f64,
}

/// Min and max grid cell numbers
#[derive(PartialEq, Debug)]
pub struct ExtentInt {
    pub minx: u32,
    pub miny: u32,
    pub maxx: u32,
    pub maxy: u32,
}

// Max grid cell numbers
type CellIndex = (u32, u32);

/// Grid origin
#[derive(PartialEq, Debug)]
pub enum Origin {
    TopLeft,
    BottomLeft, //TopRight, BottomRight
}

/// Grid units
#[derive(PartialEq, Debug)]
pub enum Unit {
    Meters,
    Degrees,
    Feet,
}

/// Tile grid
// Credits: MapCache by Thomas Bonfort (http://mapserver.org/mapcache/)
#[derive(Debug)]
pub struct Grid {
    /// The width of an individual tile, in pixels.
    width: u16,
    /// The height of an individual tile, in pixels.
    height: u16,
    /// The geographical extent covered by the grid, in ground units (e.g. meters, degrees, feet, etc.).
    /// The (minx,miny) point defines the origin of the grid, i.e. the pixel at the bottom left of the
    /// bottom-left most tile is always placed on the (minx,miny) geographical point.
    /// The (maxx,maxy) point is used to determine how many tiles there are for each zoom level.
    pub extent: Extent,
    /// Spatial reference system (PostGIS SRID).
    pub srid: i32,
    /// Grid units
    pub units: Unit,
    /// This is a list of resolutions for each of the zoom levels defined by the grid.
    /// This must be supplied as a list of positive floating point values, ordered from largest to smallest.
    /// The largest value will correspond to the grid’s zoom level 0. Resolutions
    /// are expressed in “units-per-pixel”,
    /// depending on the unit used by the grid (e.g. resolutions are in meters per
    /// pixel for most grids used in webmapping).
    resolutions: Vec<f64>,
    /// maxx/maxy for each resolution
    level_max: Vec<CellIndex>,
    /// Grid origin
    pub origin: Origin,
}

impl Grid {
    /// WGS84 grid
    pub fn wgs84() -> Grid {
        let mut grid = Grid {
            width: 256,
            height: 256,
            extent: Extent {
                minx: -180.0,
                miny: -90.0,
                maxx: 180.0,
                maxy: 90.0,
            },
            srid: 4326,
            units: Unit::Degrees,
            resolutions: vec![
                0.703125000000000,
                0.351562500000000,
                0.175781250000000,
                8.78906250000000e-2,
                4.39453125000000e-2,
                2.19726562500000e-2,
                1.09863281250000e-2,
                5.49316406250000e-3,
                2.74658203125000e-3,
                1.37329101562500e-3,
                6.86645507812500e-4,
                3.43322753906250e-4,
                1.71661376953125e-4,
                8.58306884765625e-5,
                4.29153442382812e-5,
                2.14576721191406e-5,
                1.07288360595703e-5,
                5.36441802978516e-6,
            ],
            level_max: Vec::new(),
            origin: Origin::BottomLeft,
        };
        grid.level_max = grid.level_max();
        grid
    }

    /// Web Mercator grid (Google maps compatible)
    pub fn web_mercator() -> Grid {
        let mut grid = Grid {
            width: 256,
            height: 256,
            extent: Extent {
                minx: -20037508.3427892480,
                miny: -20037508.3427892480,
                maxx: 20037508.3427892480,
                maxy: 20037508.3427892480,
            },
            srid: 3857,
            units: Unit::Meters,
            // Formula: http://wiki.openstreetmap.org/wiki/Slippy_map_tilenames#Resolution_and_Scale
            resolutions: vec![
                156543.0339280410,
                78271.51696402048,
                39135.75848201023,
                19567.87924100512,
                9783.939620502561,
                4891.969810251280,
                2445.984905125640,
                1222.992452562820,
                611.4962262814100,
                305.7481131407048,
                152.8740565703525,
                76.43702828517624,
                38.21851414258813,
                19.10925707129406,
                9.554628535647032,
                4.777314267823516,
                2.388657133911758,
                1.194328566955879,
                0.5971642834779395,
                0.2985821417389700,
                0.1492910708694850,
                0.0746455354347424,
                0.0373227677173712,
            ],
            level_max: Vec::new(),
            origin: Origin::BottomLeft,
        };
        grid.level_max = grid.level_max();
        grid
    }

    pub fn nlevels(&self) -> u8 {
        self.resolutions.len() as u8
    }
    pub fn maxzoom(&self) -> u8 {
        self.nlevels() - 1
    }
    pub fn pixel_width(&self, zoom: u8) -> f64 {
        const METERS_PER_DEGREE: f64 = 6378137.0 * 2.0 * consts::PI / 360.0;
        match self.units {
            Unit::Meters => self.resolutions[zoom as usize],
            Unit::Degrees => self.resolutions[zoom as usize] * METERS_PER_DEGREE,
            Unit::Feet => self.resolutions[zoom as usize] * 0.3048,
        }
    }
    pub fn scale_denominator(&self, zoom: u8) -> f64 {
        const PIXEL_SCREEN_WIDTH: f64 = 0.00028;
        // https://github.com/mapnik/mapnik/wiki/ScaleAndPpi#scale-denominator
        // Mapnik calculates it's default at about 90.7 PPI, which originates from an assumed standard pixel size
        // of 0.28 millimeters as defined by the OGC (Open Geospatial Consortium) SLD (Styled Layer Descriptor) Specification.
        self.pixel_width(zoom) / PIXEL_SCREEN_WIDTH
    }
    /// Extent of a given tile in the grid given its x, y, and z in TMS adressing scheme
    pub fn tile_extent(&self, xtile: u32, ytile: u32, zoom: u8) -> Extent {
        // based on mapcache_grid_get_tile_extent
        let res = self.resolutions[zoom as usize];
        let tile_sx = self.width as f64;
        let tile_sy = self.height as f64;
        match self.origin {
            Origin::BottomLeft => Extent {
                minx: self.extent.minx + (res * xtile as f64 * tile_sx),
                miny: self.extent.miny + (res * ytile as f64 * tile_sy),
                maxx: self.extent.minx + (res * (xtile + 1) as f64 * tile_sx),
                maxy: self.extent.miny + (res * (ytile + 1) as f64 * tile_sy),
            },
            Origin::TopLeft => Extent {
                minx: self.extent.minx + (res * xtile as f64 * tile_sx),
                miny: self.extent.maxy - (res * (ytile + 1) as f64 * tile_sy),
                maxx: self.extent.minx + (res * (xtile + 1) as f64 * tile_sx),
                maxy: self.extent.maxy - (res * ytile as f64 * tile_sy),
            },
        }
    }
    /// reverse y tile for XYZ adressing scheme
    pub fn ytile_from_xyz(&self, ytile: u32, zoom: u8) -> u32 {
        // y = maxy-ytile-1
        let maxy = self.level_max[zoom as usize].1;
        let y = maxy.saturating_sub(ytile).saturating_sub(1);
        y
    }
    /// Extent of a given tile in XYZ adressing scheme
    pub fn tile_extent_xyz(&self, xtile: u32, ytile: u32, zoom: u8) -> Extent {
        let y = self.ytile_from_xyz(ytile, zoom);
        self.tile_extent(xtile, y, zoom)
    }
    /// (maxx, maxy) of grid level
    pub(crate) fn level_limit(&self, zoom: u8) -> CellIndex {
        let res = self.resolutions[zoom as usize];
        let unitheight = self.height as f64 * res;
        let unitwidth = self.width as f64 * res;

        let maxy =
            ((self.extent.maxy - self.extent.miny - 0.01 * unitheight) / unitheight).ceil() as u32;
        let maxx =
            ((self.extent.maxx - self.extent.minx - 0.01 * unitwidth) / unitwidth).ceil() as u32;
        (maxx, maxy)
    }
    /// (maxx, maxy) of all grid levels
    fn level_max(&self) -> Vec<CellIndex> {
        (0..self.nlevels())
            .map(|zoom| self.level_limit(zoom))
            .collect()
    }
    /// Tile index limits covering extent
    pub fn tile_limits(&self, extent: Extent, tolerance: i32) -> Vec<ExtentInt> {
        // Based on mapcache_grid_compute_limits
        const EPSILON: f64 = 0.0000001;
        (0..self.nlevels())
            .map(|i| {
                let res = self.resolutions[i as usize];
                let unitheight = self.height as f64 * res;
                let unitwidth = self.width as f64 * res;
                let (level_maxx, level_maxy) = self.level_max[i as usize];

                let (mut minx, mut maxx, mut miny, mut maxy) = match self.origin {
                    Origin::BottomLeft => (
                        (((extent.minx - self.extent.minx) / unitwidth + EPSILON).floor() as i32)
                            - tolerance,
                        (((extent.maxx - self.extent.minx) / unitwidth - EPSILON).ceil() as i32)
                            + tolerance,
                        (((extent.miny - self.extent.miny) / unitheight + EPSILON).floor() as i32)
                            - tolerance,
                        (((extent.maxy - self.extent.miny) / unitheight - EPSILON).ceil() as i32)
                            + tolerance,
                    ),
                    Origin::TopLeft => (
                        (((extent.minx - self.extent.minx) / unitwidth + EPSILON).floor() as i32)
                            - tolerance,
                        (((extent.maxx - self.extent.minx) / unitwidth - EPSILON).ceil() as i32)
                            + tolerance,
                        (((self.extent.maxy - extent.maxy) / unitheight + EPSILON).floor() as i32)
                            - tolerance,
                        (((self.extent.maxy - extent.miny) / unitheight - EPSILON).ceil() as i32)
                            + tolerance,
                    ),
                };

                // to avoid requesting out-of-range tiles
                if minx < 0 {
                    minx = 0;
                }
                if maxx > level_maxx as i32 {
                    maxx = level_maxx as i32
                };
                if miny < 0 {
                    miny = 0
                };
                if maxy > level_maxy as i32 {
                    maxy = level_maxy as i32
                };

                ExtentInt {
                    minx: minx as u32,
                    maxx: maxx as u32,
                    miny: miny as u32,
                    maxy: maxy as u32,
                }
            })
            .collect()
    }
}

/// Returns the Spherical Mercator (x, y) in meters
fn lonlat_to_merc(lon: f64, lat: f64) -> (f64, f64) {
    // from mod web_mercator in grid_test
    //lng, lat = truncate_lnglat(lng, lat)
    let x = 6378137.0 * lon.to_radians();
    let y = 6378137.0 * ((consts::PI * 0.25) + (0.5 * lat.to_radians())).tan().ln();
    (x, y)
}

/// Projected extent
pub fn extent_to_merc(extent: &Extent) -> Extent {
    let (minx, miny) = lonlat_to_merc(extent.minx, extent.miny);
    let (maxx, maxy) = lonlat_to_merc(extent.maxx, extent.maxy);
    Extent {
        minx,
        miny,
        maxx,
        maxy,
    }
}
