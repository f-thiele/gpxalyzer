//  gpxalyzer  --  GPX data modification and extraction library
//  Copyright (C) 2019 - Fabian A.J. Thiele, <fabian.thiele@posteo.de>
//
//  This file is part of gpxalyzer.
//
//  gpxalyzer is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  gpxalyzer is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.
extern crate gpx;

use gpx::{Waypoint, TrackSegment};

pub fn decorate_speed(ts: &TrackSegment) -> Result<&TrackSegment, &'static str> {
    for wp in &ts.points {
        // println!("{:?}", wp);
    }
    return Ok(ts);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}