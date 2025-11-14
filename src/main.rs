
use macroquad::prelude::*;
use spade::{DelaunayTriangulation, Triangulation};
use drawable::gcode::*;
use image::*;
use drawable::weighted_voronoi::*;
use drawable::draw::*;

#[macroquad::main("drawable")]
async fn main() {
    let image = image::open(IMG_IN).unwrap(); // TODO
    let (width, height) = image.dimensions();

    request_new_screen_size(width as f32, height as f32);
    let width  = screen_width()  as f32;
    let height = screen_height() as f32;

    let mut gcode = Printer::new((50.0, 35.0), (254.0, 212.0), PrintMode::DOTS);
    gcode.set_scale(width, height);

    // Setup
    // Optional random seed
    // rand::srand(macroquad::miniquad::date::now() as _);
    // let mut points:Vec<Point2<f32>> = Vec::new();
    //
    // for _i in 0..NUM_POINTS {
    //     points.push(Point2::new(rand::gen_range(0.0, width),
    //                             rand::gen_range(0.0, height)));
    // }
    let mut points = load_points(IMG_IN);

    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
    for point in &points {
        _ = triangulation.insert(*point).map_err(|_err| {
            eprintln!("Failed to insert point into triangulation!");
        });

        // gcode test
        gcode.goto(point.x, point.y);
    }

    // let mut polygons = voronoi_polygons(&triangulation, &points);

    gcode.save("drawing.gcode");

    loop {
        let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();

        for point in &points {
            _ = triangulation.insert(*point).map_err(|_err| {
                eprintln!("Failed to insert point into triangulation!");
            });
        }

        let mut polygons = voronoi_polygons(&triangulation, &points);

        clear_background(WHITE);

        // for point in &points {
        //     draw_point(point.x, point.y, BLACK);
        // }

        for polygon in &polygons {
            // draw_polygon_lines(polygon, BLACK);
            let p = polygon.centroid();
            if let Some(seed) = polygon.seed {
                if polygon.enclosed {
                    draw_point(seed.x, seed.y, BLACK);
                } else {
                    draw_point(p.x, p.y, RED);
                    draw_point(seed.x, seed.y, BLUE);
                }
            }
        }

        for polygon in &mut polygons {
            polygon.relax();
        }

        update_points(&polygons, &mut points);

        // for face in triangulation.inner_faces() {
        //   let vertices = face.vertices();
        //   let a = as_vec(vertices[0].position());
        //   let b = as_vec(vertices[1].position());
        //   let c = as_vec(vertices[2].position());
        //   draw_triangle_lines(a, b, c, LINE_T, BLACK);
        // }

        next_frame().await
    }

}
