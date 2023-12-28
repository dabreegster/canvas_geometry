use geojson::{Feature, Geometry};

use crate::{Building, Intersection, Road};

impl Road {
    pub fn to_geojson(&self) -> Feature {
        let mut f = Feature::from(Geometry::from(&self.linestring));
        f.set_property("id", self.id.0);
        f.set_property("way", self.way.to_string());
        f.set_property("node1", self.node1.to_string());
        f.set_property("node2", self.node2.to_string());
        for (k, v) in &self.tags {
            f.set_property(k, v.to_string());
        }

        if let Some(max) = self.max_left_width {
            f.set_property("max_left_width", max);
        }
        if let Some(max) = self.max_right_width {
            f.set_property("max_right_width", max);
        }
        if let Some(ref polygon) = self.polygon {
            // TODO As an object would be better
            f.set_property("polygon", serde_json::to_string(polygon).unwrap());
        }
        f
    }
}

impl Intersection {
    pub fn to_geojson(&self) -> Feature {
        let mut f = Feature::from(Geometry::from(&self.point));
        f.set_property("id", self.id.0);
        f.set_property("node", self.node.to_string());
        f.set_property("roads", self.roads.iter().map(|r| r.0).collect::<Vec<_>>());
        f
    }
}

impl Building {
    pub fn to_geojson(&self) -> Feature {
        let mut f = Feature::from(Geometry::from(&self.polygon));
        f.set_property("id", self.id.to_string());
        for (k, v) in &self.tags {
            f.set_property(k, v.to_string());
        }
        f
    }
}
