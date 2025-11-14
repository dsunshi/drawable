use macroquad::prelude::*;
use macroquad::rand;

use spade::{DelaunayTriangulation, Point2, Triangulation};

use drawable::gcode::*;

const POINT_R:   f32 = 2.0;
const LINE_T:    f32 = 1.0;
const LERP_RATE: f32 = 0.1;

const NUM_POINTS: u32 = 1000;

struct Polygon {
    pub points:   Vec<Point2<f32>>,
    pub seed:     Option<Point2<f32>>,
    pub enclosed: bool,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon {
            points:   Vec::new(),
            seed:     None,
            enclosed: false,
        }
    }

    fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
      (1.0 - t) * v0 + t * v1
    }

    pub fn relax(&mut self) {
        if self.enclosed {
            if let Some(seed) = self.seed {
                let c = self.centroid();
                let x = Self::lerp(seed.x, c.x, LERP_RATE);
                let y = Self::lerp(seed.y, c.y, LERP_RATE);

                self.seed = Some(Point2::new(x, y));
            }
        }
    }

    pub fn centroid(&self) -> Point2<f32> {
        let mut c: Point2<f32> = Point2::new(0.0, 0.0);

        let n = self.points.len();
        let mut area = 0.0;
        
        for i in 0..n {
            let v0 = self.points[i];
            let v1 = self.points[(i + 1) % n];
            let cross_p = v0.x * v1.y - v1.x * v0.y;

            area += cross_p;
            c.x += (v0.x + v1.x) * cross_p;
            c.y += (v0.y + v1.y) * cross_p;
        }

        area /= 2.0;
        c.x  /= 6.0 * area;
        c.y  /= 6.0 * area;

        c
    }
}

fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

// fn as_vec(p: Point2<f32>) -> Vec2 {
//     Vec2::new(p.x, p.y)
// }

fn draw_polygon_lines(polygon: &Polygon, color: Color) {
    if polygon.enclosed {
        let points = &polygon.points;
        let n = points.len() - 1;
        
        for i in 0..n {
            draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, LINE_T, color);
        }
        draw_line(points[n].x, points[n].y, points[0].x, points[0].y, LINE_T, RED);
    }
}

fn point_dist(a: Point2<f32>, b: &Point2<f32>) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn nearest_point(c: Point2<f32>, points: &Vec<Point2<f32>>) -> Option<Point2<f32>> {
    let mut min_dist: f32 = f32::MAX;
    let mut nearest: Option<Point2<f32>> = None;

    for point in points {
        let dist = point_dist(c, point);
        if dist < min_dist {
            min_dist = dist;
            nearest  = Some(*point);
        }
    }

    nearest
}

fn update_points(polygons: &Vec<Polygon>, points: &mut Vec<Point2<f32>>) {
    points.clear();
    for polygon in polygons {
        if let Some(seed) = polygon.seed {
            points.push(seed);
        }
    }
}

fn voronoi_polygons(triangulation: &DelaunayTriangulation<Point2<f32>>, seed_points: &Vec<Point2<f32>>) -> Vec<Polygon> {
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

        polygon.seed = nearest_point(polygon.centroid(), seed_points);
        if enclosed && polygon.points.len() > 0 {
            polygon.enclosed = true;
        }

        polygons.push(polygon);
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

    for _i in 0..NUM_POINTS {
        points.push(Point2::new(rand::gen_range(0.0, width),
                                rand::gen_range(0.0, height)));
    }

    let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
    for point in &points {
        _ = triangulation.insert(*point).map_err(|_err| {
            eprintln!("Failed to insert point into triangulation!");
        });

        // gcode test
        gcode.goto(point.x, point.y);
    }

    let mut polygons = voronoi_polygons(&triangulation, &points);

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
            draw_polygon_lines(polygon, BLACK);
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
