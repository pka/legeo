//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

//! Grid iterators

use crate::grid::ExtentInt;

/// Level-by-level iterator
pub struct GridIterator {
    z: u8,
    x: u32,
    y: u32,
    maxz: u8,
    limits: Vec<ExtentInt>,
}

impl GridIterator {
    pub fn new(minz: u8, maxz: u8, limits: Vec<ExtentInt>) -> GridIterator {
        if minz <= maxz && limits.len() >= maxz as usize {
            let limit = &limits[minz as usize];
            GridIterator {
                z: minz,
                x: limit.minx,
                y: limit.miny,
                maxz,
                limits: limits,
            }
        } else {
            // Return "empty" iterator for invalid parameters
            GridIterator {
                z: 0,
                x: 0,
                y: 0,
                maxz: 0,
                limits: vec![ExtentInt {
                    minx: 0,
                    miny: 0,
                    maxx: 1,
                    maxy: 1,
                }],
            }
        }
    }
}

impl Iterator for GridIterator {
    /// Current cell index `(z, y, x)`
    type Item = (u8, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let current = (self.z, self.x, self.y);
        let limit = &self.limits[self.z as usize];
        if self.y < limit.maxy - 1 {
            self.y += 1;
        } else if self.x < limit.maxx - 1 {
            self.x += 1;
            self.y = limit.miny;
        } else if self.z < self.maxz {
            self.z += 1;
            let limit = &self.limits[self.z as usize];
            self.x = limit.minx;
            self.y = limit.miny;
        } else {
            return None;
        }
        Some(current)
    }
}

#[test]
fn test_mercator_iter() {
    use crate::grid::Grid;
    let grid = Grid::web_mercator();
    let tile_limits = grid.tile_limits(grid.extent.clone(), 0);
    let griditer = GridIterator::new(0, 2, tile_limits);
    let cells = griditer.collect::<Vec<_>>();
    assert_eq!(
        cells,
        vec![
            (0, 0, 0),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1),
            (2, 0, 0),
            (2, 0, 1),
            (2, 0, 2),
            (2, 0, 3),
            (2, 1, 0),
            (2, 1, 1),
            (2, 1, 2),
            (2, 1, 3),
            (2, 2, 0),
            (2, 2, 1),
            (2, 2, 2),
            (2, 2, 3),
            (2, 3, 0),
            (2, 3, 1),
            (2, 3, 2)
        ]
    );
}
