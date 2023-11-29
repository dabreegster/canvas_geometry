export function gjToSvg(points) {
  return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
}

// TODO Hacks to use turf, which doesn't handle euclidean.
export function mercatorToSpherical(gj, f) {
  let fix = (pt) => {
    let m = gj.mercator;
    let x = m.x1 + (pt[0] / m.width) * (m.x2 - m.x1);
    let y = m.y1 + ((m.y2 - m.y1) * (m.height - pt[1])) / m.height;
    pt[0] = x;
    pt[1] = y;
  };

  let copy = JSON.parse(JSON.stringify(f));

  if (copy.geometry.type == "Point") {
    fix(copy.geometry.coordinates);
  } else if (copy.geometry.type == "LineString") {
    for (let pt of copy.geometry.coordinates) {
      fix(pt);
    }
  } else if (copy.geometry.type == "Polygon") {
    for (let pt of f.geometry.coordinates[0]) {
      fix(pt);
    }
  }

  return copy;
}

export function sphericalToMercator(gj, f) {
  let fix = (pt) => {
    let m = gj.mercator;
    let x = (m.width * (pt[0] - m.x1)) / (m.x2 - m.x1);
    let y = m.height - (m.height * (pt[1] - m.y1)) / (m.y2 - m.y1);
    pt[0] = x;
    pt[1] = y;
  };

  let copy = JSON.parse(JSON.stringify(f));

  if (copy.geometry.type == "Point") {
    fix(copy.geometry.coordinates);
  } else if (copy.geometry.type == "LineString") {
    for (let pt of copy.geometry.coordinates) {
      fix(pt);
    }
  } else if (copy.geometry.type == "Polygon") {
    for (let pt of copy.geometry.coordinates[0]) {
      fix(pt);
    }
  }
  return copy;
}
