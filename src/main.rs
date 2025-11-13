use macroquad::prelude::*;
use macroquad::rand;

use spade::handles::{VoronoiVertex::Inner, VoronoiVertex::Outer};
use spade::{DelaunayTriangulation, Point2, Triangulation};

use drawable::gcode::*;

const POINT_R: f32 = 2.0;
const LINE_T:  f32 = 1.0;

const NUM_POINTS: u32 = 10;

fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

fn draw_shape_lines(points: &Vec<Vec2>, color: Color) {
    let n = points.len() - 1;
    
    for i in 0..n {
        draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, LINE_T, color);
    }
    draw_line(points[n].x, points[n].y, points[0].x, points[0].y, LINE_T, color);
}

fn as_vec(p: Point2<f32>) -> Vec2 {
    Vec2::new(p.x, p.y)
}

// fn voronoi_shapes(triangulation: &DelaunayTriangulation<Point2<f32>>) -> Vec<Vec<Vec2>> {
//     let mut shapes:  Vec<Vec<Vec2>> = Vec::new();
//     for face in triangulation.voronoi_faces() {
//         // println!("found a face!");
//         let mut shape: Vec<Vec2> = Vec::new(); 
//         let edges = face.adjacent_edges();
//         for edge in edges {
//             let from = edge.from().position();
//             let to = edge.to().position();
//             if let Some(start) = from {
//                 shape.push(as_vec(start));
//             }
//             if let Some(end) = to {
//                 shape.push(as_vec(end));
//             }
//
//             // from and to are vertex handles
//             // println!("found an edge: {:?} -> {:?}", from.position(), to.position());
//         }
//         shapes.push(shape);
//     }
//     shapes
// }

fn voronoi_shapes(triangulation: &DelaunayTriangulation<Point2<f32>>) -> Vec<Vec<Vec2>> {
    let mut shapes:  Vec<Vec<Vec2>> = Vec::new();
    let mut shape: Vec<Vec2> = Vec::new();
    for edge in triangulation.undirected_voronoi_edges() {
        match edge.vertices() {
            [Inner(from), Inner(to)] => {
                // "from" and "to" are inner faces of the Delaunay triangulation
                println!(
                    "Found voronoi edge between {:?} and {:?}",
                    from.circumcenter(),
                    to.circumcenter()
                );
                shape.push(as_vec(from.circumcenter()));
                shape.push(as_vec(to.circumcenter()));

            }
            [Inner(from), Outer(edge)] | [Outer(edge), Inner(from)] => {
                // Some lines don't have a finite end and extend into infinity.
                println!(
                    "Found infinite voronoi edge going out of {:?} into the direction {:?}",
                    from.circumcenter(),
                    edge.direction_vector()
                );
            }
            [Outer(_), Outer(_)] => {
                // This case only happens if all vertices of the triangulation lie on the
                // same line and can probably be ignored.
            }
        }
    }
    shapes.push(shape);
    shapes
}

// Prints out the location of all voronoi edges in a triangulation
// fn log_voronoi_diagram(triangulation: &DelaunayTriangulation<Point2<f32>>) {
//     for face in triangulation.voronoi_faces() {
//         for edges in face.adjacent_edges() {
//               for edge in &edges {
//                 let from = edge.from();
//                 let to = edge.to();
//                 // from and to are vertex handles
//                 println!("found an edge: {:?} -> {:?}", from, to);
//               }
//             // match edge.vertices() {
//             //     [Inner(from), Inner(to)] => {
//             //         // "from" and "to" are inner faces of the Delaunay triangulation
//             //         println!(
//             //             "Found voronoi edge between {:?} and {:?}",
//             //             from.circumcenter(),
//             //             to.circumcenter()
//             //         );
//             //     }
//             //     [Inner(from), Outer(edge)] | [Outer(edge), Inner(from)] => {
//             //         // Some lines don't have a finite end and extend into infinity.
//             //         println!(
//             //             "Found infinite voronoi edge going out of {:?} into the direction {:?}",
//             //             from.circumcenter(),
//             //             edge.direction_vector()
//             //         );
//             //     }
//             //     [Outer(_), Outer(_)] => {
//             //         // This case only happens if all vertices of the triangulation lie on the
//             //         // same line and can probably be ignored.
//             //     }
//             // }
//         }
//     }
// }
//
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

    // for face in triangulation.voronoi_faces() {
    //     println!("found a face!");
    //     let edges = face.adjacent_edges();
    //     for edge in edges {
    //         let from = edge.from();
    //         let to = edge.to();
    //         // from and to are vertex handles
    //         println!("found an edge: {:?} -> {:?}", from.position(), to.position());
    //     }
    // }
    // let voronoi_faces = voronoi_shapes(&triangulation);

    gcode.save("drawing.gcode");

    loop {
        clear_background(WHITE);

        for point in &points {
            draw_point(point.x, point.y, BLACK);
        }

        // for face in triangulation.inner_faces() {
        //   let vertices = face.vertices();
        //   let a = as_vec(vertices[0].position());
        //   let b = as_vec(vertices[1].position());
        //   let c = as_vec(vertices[2].position());
        //   draw_triangle_lines(a, b, c, LINE_T, BLACK);
        // }
        

        for face in triangulation.voronoi_faces() {
            // println!("found a face!");
            // let mut shape: Vec<Vec2> = Vec::new(); 
            let edges = face.adjacent_edges();
            for edge in edges {
                let from = edge.from().position();
                let to = edge.to().position();
                if let Some(start) = from {
                    if let Some(end) = to {
                        draw_line(start.x, start.y, end.x, end.y, LINE_T, BLACK);
                    }
                }
            }
        }
        // for face in &voronoi_faces {
        //     draw_shape_lines(face, BLACK);
        // }
        // for edge in triangulation.undirected_voronoi_edges() {
        //     match edge.vertices() {
        //         [Inner(from), Inner(to)] => {
        //             // "from" and "to" are inner faces of the Delaunay triangulation
        //             println!(
        //                 "Found voronoi edge between {:?} and {:?}",
        //                 from.circumcenter(),
        //                 to.circumcenter()
        //             );
        //             let start = as_vec(from.circumcenter());
        //             let end   = as_vec(to.circumcenter());
        //             draw_line(start.x, start.y, end.x, end.y, LINE_T, BLACK);
        //
        //         }
        //         [Inner(from), Outer(edge)] | [Outer(edge), Inner(from)] => {
        //             // Some lines don't have a finite end and extend into infinity.
        //             println!(
        //                 "Found infinite voronoi edge going out of {:?} into the direction {:?}",
        //                 from.circumcenter(),
        //                 edge.direction_vector()
        //             );
        //         }
        //         [Outer(_), Outer(_)] => {
        //             // This case only happens if all vertices of the triangulation lie on the
        //             // same line and can probably be ignored.
        //         }
        //     }
        // }

        next_frame().await
    }

}
