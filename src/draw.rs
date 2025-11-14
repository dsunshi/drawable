
use macroquad::prelude::*;
use crate::weighted_voronoi::Polygon;

const POINT_R:   f32 = 2.0;
const LINE_T:    f32 = 1.0;

pub fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

// fn as_vec(p: Point2<f32>) -> Vec2 {
//     Vec2::new(p.x, p.y)
// }

pub fn draw_polygon_lines(polygon: &Polygon, color: Color) {
    if polygon.enclosed {
        let points = &polygon.points;
        let n = points.len() - 1;
        
        for i in 0..n {
            draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, LINE_T, color);
        }
        draw_line(points[n].x, points[n].y, points[0].x, points[0].y, LINE_T, RED);
    }
}
