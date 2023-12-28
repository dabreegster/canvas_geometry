use geo::{BooleanOps, MultiPolygon, Polygon};
use serde::Serialize;

use crate::math::{buffer_linestring, union_all};
use crate::{IntersectionID, MapModel};

#[derive(Serialize)]
pub struct Output {
    thick_roads: Vec<Polygon>,
    overlaps: Vec<MultiPolygon>,
    unioned: MultiPolygon,
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

    // Take every pair of roads (TODO just adjacent), find their intersection ("overlap" for
    // sanity), then union all of that.
    let mut overlaps = Vec::new();
    for idx1 in 0..thick_roads.len() {
        for idx2 in 0..thick_roads.len() {
            if idx1 != idx2 {
                let overlap = thick_roads[idx1].intersection(&thick_roads[idx2]);
                overlaps.push(overlap);
            }
        }
    }
    let unioned = union_all(overlaps.clone());

    Output {
        thick_roads,
        overlaps,
        unioned,
    }
}
