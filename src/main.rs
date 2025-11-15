use macroquad::prelude::*;

use d3_delaunay_rs::delaunay::Delaunay;
use geo_types::Coord;

use std::collections::HashMap;

use ::rand::prelude::*;

use image::*;

const IMG_IN: &str = "bird.jpg";

const POINT_R:   f32 = 2.0;
const LINE_T:    f32 = 1.0;

pub fn draw_point(x: f32, y: f32, color: Color) {
    draw_circle(x, y, POINT_R, color);
}

pub struct Stipple {
    width: usize,
    height: usize,
    n: usize,
    data: Vec<f64>,
    pub delaunay: Delaunay<f64>,
}

impl Stipple {
    pub(crate) fn new( width: usize, height: usize,
        data: Vec<f64>, // gray scale - image data
        n: usize,
    ) -> Stipple {
        let mut h_points: HashMap<usize, Coord<f64>> = HashMap::with_capacity(n);
        // Initialize the points using rejection sampling.
        let mut rng = ::rand::rng();
        for i in 0..n {
            '_30Loop: for _ in 0..30 {
                let x = rng.random_range(0.0..(width as f64));
                let y = rng.random_range(0.0..(height as f64));
                let index = y as usize * width + x as usize;
                h_points.insert(i, Coord { x, y });

                if rng.random_range(0.0..1.0) < data[index] {
                    break '_30Loop;
                }
            }
        }

        let points   = h_points.into_values().collect::<Vec<_>>();
        let delaunay = Delaunay::new(&points);

        Stipple {
            width,
            height,
            n,
            data,
            delaunay,
        }
    }

    pub fn next(&mut self, k: usize) {
        // Compute the weighted centroid for each Voronoi cell.
        let mut c: Vec<Coord<f64>> = Vec::with_capacity(self.n);
        let mut s: Vec<f64> = Vec::with_capacity(self.n);
        for _i in 0..self.n {
            c.push(Coord { x: 0_f64, y: 0_f64 });
            s.push(0_f64);
        }

        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let w = self.data[y * self.width + x];
                i = self.delaunay.find(
                    &Coord {
                        x: x as f64 + 0.5_f64,
                        y: y as f64 + 0.5_f64,
                    },
                    Some(i),
                );
                s[i] += w;
                c[i].x += w * (x as f64 + 0.5_f64);
                c[i].y += w * (y as f64 + 0.5_f64);
            }
        }

        // Relax the diagram by moving points to the weighted centroid.
        // Wiggle the points a little bit so they don’t get stuck.
        let w = (k as f64 + 1_f64).powf(-0.8) * 10_f64;
        for i in 0..self.n {
            let x0 = self.delaunay.points[i].x;
            let y0 = self.delaunay.points[i].y;
            let x1 = if s[i] == 0_f64 { x0 } else { c[i].x / s[i] };
            let y1 = if s[i] == 0_f64 { y0 } else { c[i].y / s[i] };

            self.delaunay.points[i].x =
                x0 + (x1 - x0) * 1.8 + (::rand::rng().random_range(0.0..1.0) - 0.5) * w;
            self.delaunay.points[i].y =
                y0 + (y1 - y0) * 1.8 + (::rand::rng().random_range(0.0..1.0) - 0.5) * w;
        }

        // // TODO: doing a update() the hard way...
        // // What can I refactor here.
        self.delaunay = Delaunay::new(&self.delaunay.points);

    }
}

pub fn as_bytes(img: &DynamicImage) -> Vec<f64> {
    let (width, height)     = img.dimensions();
    let mut bytes: Vec<f64> = vec![0.0; (width * height) as usize];

    for x in 0..width {
        for y in 0..height {

            let pixel = img.get_pixel(x as u32, y as u32);
            let index = (y * width + x) as usize;

            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            let mut lum = 0.299 * r + 0.587 * g + 0.114 * b;
            // let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            // let lum = (r + g + b) / 3;
            
            lum /= 255.0;
            lum = 1.0 - lum;

            bytes[index] = lum as f64;
        }
    }

    bytes
}

#[macroquad::main("drawable")]
async fn main() {
    let image = image::open(IMG_IN).unwrap(); // TODO
    let (width, height) = image.dimensions();
    println!("Image size: {} x {}", width, height);
    
    let mut stipple = Stipple::new(width as usize, height as usize, as_bytes(&image), 20000);

    request_new_screen_size(width as f32, height as f32);

    loop {
        clear_background(WHITE);

        for point in &stipple.delaunay.points {
            draw_point(point.x as f32, point.y as f32, BLACK);
        }

        stipple.next(10);

        next_frame().await
    }

}
