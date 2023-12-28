use geo::{Densify, EuclideanLength, Intersects, Line, LineString, Polygon};
use serde::Serialize;

use crate::math::{buffer_linestring, project_away, split_line_by_polygon};
use crate::{MapModel, Road, RoadID};

#[derive(Serialize)]
pub struct Output {
    test_lines: Vec<TestLine>,
    max_left_width: f64,
    max_right_width: f64,
    buffered_polygon: Option<Polygon>,
    parallel_roads: Vec<(LineString, String)>,
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
    let original_road = &map.roads[r.0];
    let dense_line = original_road.linestring.densify(step_size_meters);

    let mut test_lines = Vec::new();
    // Using lines instead of coords so we can get the angle -- but is this hard to reason about?
    // angle_at_point instead?
    for orig_line in dense_line.lines() {
        // TODO For the last line, use the last point too
        let pt = orig_line.start;
        let angle = line_angle_degrees(orig_line);

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

    let buffered_polygon =
        buffer_linestring(&original_road.linestring, max_left_width, max_right_width);
    let parallel_roads = if let Some(ref poly) = buffered_polygon {
        find_parallel_roads(map, poly, original_road)
    } else {
        Vec::new()
    };

    Output {
        test_lines,
        max_left_width,
        max_right_width,
        buffered_polygon,
        parallel_roads,
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
        road.polygon = buffer_linestring(&road.linestring, out.max_left_width, out.max_right_width);
    }
}

fn find_parallel_roads(
    map: &MapModel,
    within_polygon: &Polygon,
    original_road: &Road,
) -> Vec<(LineString, String)> {
    let mut parallel = Vec::new();
    for road in &map.roads {
        if road.id == original_road.id {
            continue;
        }
        if within_polygon.intersects(&road.linestring) {
            // TODO Connecting lines are included here; we want to exclude them. Check how much of the linestring overlaps the buffer? Or maybe see if the shorter thing (no matter what it is) mostly overlaps?
            if !nearly_parallel(&original_road.linestring, &road.linestring, 10.0) {
                continue;
            }
            //let score = format!("Hausdorff {}", road.linestring.hausdorff_distance(&original_road.linestring));
            let score = format!(
                "Average angle orig {}, this road {}",
                average_angle(&original_road.linestring),
                average_angle(&road.linestring)
            );

            parallel.push((road.linestring.clone(), score));
        }
    }
    parallel
}

// TODO move to math

fn line_angle_degrees(line: Line) -> f64 {
    line.dy().atan2(line.dx()).to_degrees()
}

fn average_angle(linestring: &LineString) -> f64 {
    let angles: Vec<f64> = linestring
        .lines()
        .map(|line| line_angle_degrees(line))
        .collect();
    angles.iter().sum::<f64>() / (angles.len() as f64)
}

/// Degrees for input/output. Returns [-180, 180]. See  //
/// https://math.stackexchange.com/questions/110080/shortest-way-to-achieve-target-angle
fn shortest_rotation(angle1: f64, angle2: f64) -> f64 {
    ((angle1 - angle2 + 540.0) % 360.0) - 180.0
}

fn nearly_parallel(ls1: &LineString, ls2: &LineString, epsilon_degrees: f64) -> bool {
    shortest_rotation(average_angle(ls1), average_angle(ls2)).abs() < epsilon_degrees
}
