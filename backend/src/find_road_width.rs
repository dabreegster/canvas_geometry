use geo::{Coord, Densify, Line};
use serde::Serialize;

use crate::{MapModel, RoadID};

#[derive(Serialize)]
pub struct Output {
    test_points: Vec<TestPoint>,
}

#[derive(Serialize)]
pub struct TestPoint {
    pt: Coord,
    line_left: Line,
    line_right: Line,
}

pub fn find_road_width(map: &MapModel, r: RoadID) -> Output {
    let step_size_meters = 1.0;
    let project_away_meters = 5.0;

    // This keeps existing points, which is fine
    let dense_line = map.roads[r.0].linestring.densify(step_size_meters);

    let mut test_points = Vec::new();
    // Using lines instead of coords so we can get the angle -- but is this hard to reason about?
    // angle_at_point instead?
    for orig_line in dense_line.lines() {
        // TODO For the last line, use the last point too
        let pt = orig_line.start;
        let angle = orig_line.dy().atan2(orig_line.dx()).to_degrees();
        let projected_left = project_away(pt, angle - 90.0, project_away_meters);
        let projected_right = project_away(pt, angle + 90.0, project_away_meters);
        test_points.push(TestPoint {
            pt,
            line_left: Line::new(pt, projected_left),
            line_right: Line::new(pt, projected_right),
        });
    }

    Output { test_points }
}

fn project_away(pt: Coord, angle_degrees: f64, distance: f64) -> Coord {
    let (sin, cos) = angle_degrees.to_radians().sin_cos();
    Coord {
        x: pt.x + distance * cos,
        y: pt.y + distance * sin,
    }
}
