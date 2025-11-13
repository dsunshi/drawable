use macroquad::prelude::*;
use macroquad::rand;

use spade::Point2;

const POINT_R: f32 = 2.0;

fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

#[macroquad::main("drawable")]
async fn main() {
    let width  = screen_width() as f32;
    let height = screen_height() as f32;

    // Setup
    // Optional random seed
    // rand::srand(macroquad::miniquad::date::now() as _);
    let mut points:Vec<Point2<f32>> = Vec::new();
    for _i in 0..100 {
        points.push(Point2::new(rand::gen_range(0.0, width),
                                rand::gen_range(0.0, height)));
    }

    loop {
        clear_background(WHITE);

        for point in &points {
            draw_point(point.x, point.y, BLACK);
        }

        next_frame().await
    }
}
