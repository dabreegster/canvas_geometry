use geo::{Densify, EuclideanLength, Intersects, Line};
use serde::Serialize;

use crate::math::{buffer_linestring, project_away, split_line_by_polygon};
use crate::{MapModel, RoadID};

#[derive(Serialize)]
pub struct Output {
    test_lines: Vec<TestLine>,
    max_left_width: f64,
    max_right_width: f64,
}

#[derive(Serialize)]
pub struct TestLine {
    // Right if false
    left: bool,
    full_line: Line,
    // If the line hits something, what's the shortened line and its length?
    hit: Option<(Line, f64)>,
}

pub fn find_road_width(map: &MapModel, r: RoadID) -> Output {
    let step_size_meters = 1.0;
    // How far away could buildings be from a center?
    let project_away_meters = 25.0;

    // This keeps existing points, which is fine
    let dense_line = map.roads[r.0].linestring.densify(step_size_meters);

    let mut test_lines = Vec::new();
    // Using lines instead of coords so we can get the angle -- but is this hard to reason about?
    // angle_at_point instead?
    for orig_line in dense_line.lines() {
        // TODO For the last line, use the last point too
        let pt = orig_line.start;
        let angle = orig_line.dy().atan2(orig_line.dx()).to_degrees();

        for (angle_offset, left) in [(-90.0, true), (90.0, false)] {
            let projected = project_away(pt, angle + angle_offset, project_away_meters);
            let full_line = Line::new(pt, projected);

            let mut hit = None;
            for b in &map.buildings {
                if b.polygon.intersects(&full_line) {
                    if let Some(shortened_line) = split_line_by_polygon(full_line, &b.polygon) {
                        hit = Some((shortened_line, shortened_line.euclidean_length()));
                        break;
                    }
                }
            }

            test_lines.push(TestLine {
                left,
                full_line,
                hit,
            });
        }
    }

    let mut max_right_width = project_away_meters;
    let mut max_left_width = project_away_meters;
    for test in &test_lines {
        if let Some((_, len)) = test.hit {
            if test.left {
                max_left_width = max_left_width.min(len);
            } else {
                max_right_width = max_right_width.min(len);
            }
        }
    }

    Output {
        test_lines,
        max_left_width,
        max_right_width,
    }
}

pub fn find_all(map: &mut MapModel) {
    let results = map
        .roads
        .iter()
        .map(|r| find_road_width(map, r.id))
        .collect::<Vec<_>>();
    for (road, out) in map.roads.iter_mut().zip(results.into_iter()) {
        road.max_left_width = Some(out.max_left_width);
        road.max_right_width = Some(out.max_right_width);
        // TODO Re-center
        road.polygon =
            buffer_linestring(&road.linestring, out.max_left_width + out.max_right_width);
    }
}
