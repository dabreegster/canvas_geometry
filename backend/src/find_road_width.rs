use geo::{Coord, Densify};
use serde::Serialize;

use crate::{MapModel, RoadID};

#[derive(Serialize)]
pub struct Output {
    test_points: Vec<Coord>,
}

pub fn find_road_width(map: &MapModel, r: RoadID) -> Output {
    let step_size_meters = 1.0;
    // This keeps existing points, which is fine
    let dense_line = map.roads[r.0].linestring.densify(step_size_meters);

    let mut test_points = Vec::new();
    for pt in dense_line.coords() {
        test_points.push(*pt);
    }

    Output { test_points }
}
