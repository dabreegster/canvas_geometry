use geo::{LineString, OffsetCurve, Polygon};
use serde::Serialize;

use crate::{IntersectionID, MapModel};

#[derive(Serialize)]
pub struct Output {
    thick_roads: Vec<Polygon>,
}

pub fn find_intersection_geometry(map: &MapModel, i: IntersectionID) -> Output {
    let buffer_meters = 1.0;

    let mut thick_roads = Vec::new();
    for r in &map.intersections[i.0].roads {
        if let Some(polygon) = buffer_linestring(&map.roads[r.0].linestring, buffer_meters) {
            thick_roads.push(polygon);
        } else {
            warn!("Couldn't buffer a road");
        }
    }

    Output { thick_roads }
}

fn buffer_linestring(linestring: &LineString, buffer_meters: f64) -> Option<Polygon> {
    let left = linestring.offset_curve(-buffer_meters)?;
    let right = linestring.offset_curve(buffer_meters)?;
    // Make a polygon by gluing these points together
    let mut pts = left.0;
    pts.reverse();
    pts.extend(right.0);
    Some(Polygon::new(LineString(pts), Vec::new()))
}
