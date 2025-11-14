
use macroquad::prelude::*;
use macroquad::rand;

use spade::{DelaunayTriangulation, Point2, Triangulation};

use image::*;

const LERP_RATE: f32 = 0.01;
const LUM_LIMIT: f32 = 100.0;

const NUM_POINTS: u32 = 10000;

pub const IMG_IN: &str = "bird.jpg";

pub struct Polygon {
    pub points:   Vec<Point2<f32>>,
    pub seed:     Option<Point2<f32>>,
    pub enclosed: bool,
}

pub fn load_points(filename: &str) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = Vec::new();

    let image = image::open(filename).unwrap(); // TODO
    let (width, height) = image.dimensions();

    loop {
        let x = rand::gen_range(0.0, width as f32);
        let y = rand::gen_range(0.0, height as f32);

        let pixel = image.get_pixel(x as u32, y as u32);
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
        // let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        // let lum = (r + g + b) / 3;
        
        if rand::gen_range(0.0, LUM_LIMIT) > lum {
            points.push(Point2::new(x, y));
        }

        if points.len() >= NUM_POINTS as usize { break; }
    }

    points
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


pub fn update_points(polygons: &Vec<Polygon>, points: &mut Vec<Point2<f32>>) {
    points.clear();
    for polygon in polygons {
        if let Some(seed) = polygon.seed {
            points.push(seed);
        }
    }
}

pub fn voronoi_polygons(triangulation: &DelaunayTriangulation<Point2<f32>>, seed_points: &Vec<Point2<f32>>) -> Vec<Polygon> {
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

        polygon.seed = crate::math::nearest_point(polygon.centroid(), seed_points);
        if enclosed && polygon.points.len() > 0 {
            polygon.enclosed = true;
        }

        polygons.push(polygon);
    }

    polygons
}
