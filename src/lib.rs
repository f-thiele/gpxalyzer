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
use chrono::{DateTime, Utc};

pub fn distance(a: &Waypoint, b: &Waypoint) -> f64 {
    // calculate the distance between to GPX waypoints by making use of the
    // haversine formula: distance on two points of a sphere where we know the
    // angular coordinates of the points [lattitude and longitude]

    // get angles and save them as phi and lambda
    let phi1 = a.point().lat();
    let phi2 = b.point().lat();
    let lambda1 = a.point().lng();
    let lambda2 = b.point().lng();

    // calculate the sinus of half the angular differences
    let sphi = ((phi2-phi1)/2.).sin();
    let slambda = ((lambda2-lambda1)/2.).sin();

    // radius of the earth
    let r = 6.3781*1e6; // in meters

    // haversine formula
    return 2.*r*((sphi.powi(2)+phi1.cos()*phi2.cos()*(slambda.powi(2)))).asin();
}

pub fn decorate_speed(ts: &mut TrackSegment) -> Result<&mut TrackSegment, &'static str> {
    // decorate the missing speed information on a whole track segment
    //
    // TODO: this function needs a lot of work still
    // right now we are simply calculating the distance between two (reasonably?) distant
    // waypoints to not trap into precision issues with the GPS information
    //
    // better ideas:
    // a) calculate the speed in every waypoint and average over it
    // b) or interpolate the track (spline?) and calculate the slope at every point

    // use an interpolation of 15 track points (usually ~1 second apart) to calculate the
    // distance
    let d_next_wp = 15;

    for i in 0..ts.points.len()-d_next_wp-1 {
        // loop over all waypoints and use d_next_wp for speed
        // calculation
        let wp = &ts.points[i];
        let nwp = &ts.points[i+d_next_wp];
        let distance = distance(wp, nwp);
        let duration = nwp.time
            .unwrap().
            signed_duration_since(wp.time.unwrap())
            .num_seconds() as f64;
        ts.points[i].speed = Some(distance/duration);
        // TODO: additional error handling of zero divison should be implemented here

    }

    Ok(ts)
}


pub fn get_range_lattitude(ts: &TrackSegment) -> ([f64; 2]) {
    // return the lattitude range of the track segment

    let mut max_lat = ts.points[0].point().lat();
    let mut min_lat = max_lat;

    for n in &ts.points {
        let lat = n.point().lat();
        if max_lat < lat {
            max_lat = lat;
        } else if min_lat > lat {
            min_lat = lat;
        }
    }

    [min_lat, max_lat]
}


pub fn get_range_longitude(ts: &TrackSegment) -> ([f64; 2]) {
    // return the longitude range of the track segment

    let mut max_lng = ts.points[0].point().lng();
    let mut min_lng = max_lng;

    for n in &ts.points {
        let lng = n.point().lng();
        if max_lng < lng {
            max_lng = lng;
        } else if min_lng > lng {
            min_lng = lng;
        }
    }

    [min_lng, max_lng]
}


pub fn get_lattitude(ts: &TrackSegment) -> (std::vec::Vec<f64>) {
    // return a vector of all lattitudes for every waypoint of the track segment

    let mut lat = std::vec::Vec::new();

    for n in &ts.points {
        lat.push(n.point().lat());
    }

    lat
}


pub fn get_longitude(ts: &TrackSegment) -> (std::vec::Vec<f64>) {
    // return a vector of all longitudes for every waypoint of the track segment

    let mut lng = std::vec::Vec::new();

    for n in &ts.points {
        lng.push(n.point().lng());
    }

    lng
}


pub fn get_time(ts: &TrackSegment) -> (std::vec::Vec<DateTime<Utc>>) {
    // return a vector of all timestamps for every waypoint of the track segment

    let mut time = std::vec::Vec::new();

    for n in &ts.points {
        time.push(n.time.unwrap());
    }

    time
}


pub fn get_speed(ts: &TrackSegment) -> (std::vec::Vec<f64>) {
    // return a vector of all speeds for every waypoint of the track segment

    let mut speed = std::vec::Vec::new();

    for n in &ts.points {
        let result = n.speed;
        match result {
            // The division was valid
            Some(x) => speed.push(x),
            // The division was invalid
            None    => speed.push(0.),
        }
        ;
    }

    speed
}


pub fn get_elevation(ts: &TrackSegment) -> (std::vec::Vec<f64>) {
    // return a vector of all elevations for every waypoint of the track segment

    let mut elev = std::vec::Vec::new();

    for n in &ts.points {
        elev.push(n.elevation.unwrap());
    }

    elev
}


// example test to be written
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
