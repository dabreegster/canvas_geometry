export function gjToSvg(points) {
  return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
}

export function polygonToSvg(polygon) {
  return polygon.exterior.map((pt) => `${pt.x},${pt.y}`).join(" ");
}

export function lineToSvg(line) {
  return {
    x1: line.start.x,
    y1: line.start.y,
    x2: line.end.x,
    y2: line.end.y,
  };
}
