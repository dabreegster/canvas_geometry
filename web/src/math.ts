export function gjToSvg(points) {
  return points.map((pt) => `${pt[0]},${pt[1]}`).join(" ");
}

export function polygonToSvg(polygon) {
  return polygon.exterior.map((pt) => `${pt.x},${pt.y}`).join(" ");
}
