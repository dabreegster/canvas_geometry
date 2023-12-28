/// Things that could go in geo eventually
use geo::{
    BooleanOps, Coord, EuclideanLength, Line, LineIntersection, LineString, MultiPolygon,
    OffsetCurve, Polygon,
};

pub fn buffer_linestring(
    linestring: &LineString,
    left_meters: f64,
    right_meters: f64,
) -> Option<Polygon> {
    assert!(left_meters >= 0.0);
    assert!(right_meters >= 0.0);
    let left = linestring.offset_curve(-left_meters)?;
    let right = linestring.offset_curve(right_meters)?;
    // Make a polygon by gluing these points together
    let mut pts = left.0;
    pts.reverse();
    pts.extend(right.0);
    Some(Polygon::new(LineString(pts), Vec::new()))
}

pub fn union_all(mut list: Vec<MultiPolygon>) -> MultiPolygon {
    let mut result = list.pop().unwrap();
    while let Some(next) = list.pop() {
        result = result.union(&next);
    }
    result
}

pub fn project_away(pt: Coord, angle_degrees: f64, distance: f64) -> Coord {
    let (sin, cos) = angle_degrees.to_radians().sin_cos();
    Coord {
        x: pt.x + distance * cos,
        y: pt.y + distance * sin,
    }
}

// See also https://github.com/georust/geo/issues/985
pub fn split_line_by_polygon(line: Line, polygon: &Polygon) -> Option<Line> {
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
