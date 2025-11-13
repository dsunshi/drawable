use macroquad::prelude::*;
use macroquad::rand;

use spade::{DelaunayTriangulation, Triangulation, Point2};

const POINT_R: f32 = 2.0;
const LINE_T:  f32 = 1.0;

const NUM_POINTS: u32 = 100;

fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

fn as_vec(p: Point2<f32>) -> Vec2 {
    Vec2::new(p.x, p.y)
}

#[macroquad::main("drawable")]
async fn main() {
    let width  = screen_width()  as f32;
    let height = screen_height() as f32;

    // Setup
    // Optional random seed
    // rand::srand(macroquad::miniquad::date::now() as _);
    let mut points:Vec<Point2<f32>> = Vec::new();
    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

    for _i in 0..NUM_POINTS {
        points.push(Point2::new(rand::gen_range(0.0, width),
                                rand::gen_range(0.0, height)));
    }

    for point in &points {
        _ = triangulation.insert(*point).map_err(|_err| {
            eprintln!("Failed to insert point into triangulation!");
        });
    }

    loop {
        clear_background(WHITE);

        for point in &points {
            draw_point(point.x, point.y, BLACK);
        }

        for face in triangulation.inner_faces() {
          let vertices = face.vertices();
          let a = as_vec(vertices[0].position());
          let b = as_vec(vertices[1].position());
          let c = as_vec(vertices[2].position());
          draw_triangle_lines(a, b, c, LINE_T, BLACK);
        }

        next_frame().await
    }
}
