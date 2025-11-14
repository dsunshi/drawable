use macroquad::prelude::*;
use macroquad::rand;

// use spade::handles::{VoronoiVertex::Inner, VoronoiVertex::Outer};
use spade::{DelaunayTriangulation, Point2, Triangulation};

use drawable::gcode::*;

const POINT_R: f32 = 2.0;
const LINE_T:  f32 = 1.0;

const NUM_POINTS: u32 = 100;

struct Polygon {
    pub points: Vec<Point2<f32>>
}

impl Polygon {
    pub fn new() -> Self {
        Polygon {
            points: Vec::new(),
        }
    }
}

fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

// fn as_vec(p: Point2<f32>) -> Vec2 {
//     Vec2::new(p.x, p.y)
// }

fn draw_polygon_lines(polygon: &Polygon, color: Color) {
    let points = &polygon.points;
    let n = points.len() - 1;
    
    for i in 0..n {
        draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, LINE_T, color);
    }
    draw_line(points[n].x, points[n].y, points[0].x, points[0].y, LINE_T, RED);
}

fn voronoi_polygons(triangulation: &DelaunayTriangulation<Point2<f32>>) -> Vec<Polygon> {
    let mut polygons: Vec<Polygon> = Vec::new();

    for face in triangulation.voronoi_faces() {
        let mut polygon = Polygon::new();
        let edges = face.adjacent_edges();

        let mut enclosed: bool = true;
        for edge in edges {
            let from = edge.from().position();
            let to   = edge.to().position();

            if let Some(start) = from {
                if let Some(end) = to {
                    polygon.points.push(end);
                    polygon.points.push(start);
                } else {
                    enclosed = false;
                }
            } else if let Some(_end) = to {
                enclosed = false;
            }
        }

        if enclosed {
            polygons.push(polygon);
        }
    }

    polygons
}


#[macroquad::main("drawable")]
async fn main() {
    let width  = screen_width()  as f32;
    let height = screen_height() as f32;

    let mut gcode = Printer::new((50.0, 35.0), (254.0, 212.0), PrintMode::DOTS);
    gcode.set_scale(width, height);

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

        // gcode test
        gcode.goto(point.x, point.y);
    }

    let polygons = voronoi_polygons(&triangulation);

    gcode.save("drawing.gcode");

    loop {
        clear_background(WHITE);

        for point in &points {
            draw_point(point.x, point.y, BLACK);
        }

        for polygon in &polygons {
            draw_polygon_lines(polygon, BLACK);
        }

        // for face in triangulation.inner_faces() {
        //   let vertices = face.vertices();
        //   let a = as_vec(vertices[0].position());
        //   let b = as_vec(vertices[1].position());
        //   let c = as_vec(vertices[2].position());
        //   draw_triangle_lines(a, b, c, LINE_T, BLACK);
        // }

        // for face in triangulation.voronoi_faces() {
        //     // println!("found a face!");
        //     // let mut shape: Vec<Vec2> = Vec::new(); 
        //     let edges = face.adjacent_edges();
        //     for edge in edges {
        //         let from = edge.from().position();
        //         let to = edge.to().position();
        //         if let Some(start) = from {
        //             if let Some(end) = to {
        //                 draw_line(start.x, start.y, end.x, end.y, LINE_T, BLACK);
        //             } else {
        //                 let direction = edge.direction_vector();
        //                 // println!("direction: {:?}", direction);
        //                 draw_point(start.x, start.y, RED);
        //                 draw_point(direction.x, direction.y, BLUE);
        //             }
        //         } else if let Some(end) = to {
        //             let direction = edge.direction_vector();
        //             // println!("direction: {:?}", direction);
        //             draw_point(end.x, end.y, RED);
        //             draw_point(direction.x, direction.y, BLUE);
        //             // draw_line(end.x, end.y, direction.x, direction.y, LINE_T, BLUE);
        //         }
        //     }
        // }

        next_frame().await
    }

}
