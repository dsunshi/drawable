
use spade::{Point2};

fn point_dist(a: Point2<f32>, b: &Point2<f32>) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn nearest_point(c: Point2<f32>, points: &Vec<Point2<f32>>) -> Option<Point2<f32>> {
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
