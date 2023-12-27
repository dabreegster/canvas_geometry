use geo::{Coord, Densify, EuclideanLength, Intersects, Line, LineIntersection, Polygon};
use serde::Serialize;

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

fn project_away(pt: Coord, angle_degrees: f64, distance: f64) -> Coord {
    let (sin, cos) = angle_degrees.to_radians().sin_cos();
    Coord {
        x: pt.x + distance * cos,
        y: pt.y + distance * sin,
    }
}

// See also https://github.com/georust/geo/issues/985
fn split_line_by_polygon(line: Line, polygon: &Polygon) -> Option<Line> {
    // The input line could intersect the polygon's exterior at several places. Find the hit
    // closest to line.start.
    let mut shortest: Option<(Line, f64)> = None;
    // Ignore polygon holes
    for polygon_line in polygon.exterior().lines() {
        if let Some(LineIntersection::SinglePoint { intersection, .. }) =
            geo::algorithm::line_intersection::line_intersection(line, polygon_line)
        {
            // Assume line.start is outside the polygon and we're looking for the place it first
            // crosses into the polygon
            let candidate = Line::new(line.start, intersection);
            let candidate_length = candidate.euclidean_length();
            if shortest
                .as_ref()
                .map(|(_, len)| candidate_length < *len)
                .unwrap_or(true)
            {
                shortest = Some((candidate, candidate_length));
            }
        }
    }
    shortest.map(|pair| pair.0)
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
    }
}
