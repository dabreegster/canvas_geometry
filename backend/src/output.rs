use geojson::{Feature, Geometry};

use crate::{Building, Road};

impl Road {
    pub fn to_geojson(&self) -> Feature {
        let mut f = Feature::from(Geometry::from(&self.linestring));
        f.set_property("way", self.way.to_string());
        f.set_property("node1", self.node1.to_string());
        f.set_property("node2", self.node2.to_string());
        for (k, v) in &self.tags {
            f.set_property(k, v.to_string());
        }
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
