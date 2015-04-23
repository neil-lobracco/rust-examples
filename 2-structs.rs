// Declare a Point - a struct with two fields
#[derive(Debug,Clone,Copy)]
struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    pub fn distance_from_origin(&self) -> f32 {
        self.distance_from_point(Point::new())
    }
    pub fn distance_from_point(&self,other: Point) -> f32 {
        let (dx,dy) = (self.x - other.x, self.y - other.y);
        (dy.powi(2) + dx.powi(2)).sqrt()
    }
}

fn main(){
    let p1 = Point { x: -5.0, y: 3.0 };
    let mut p2 = Point::new();
    p2.x = 7.0;
    println!("p1 ({:?}) is {} units from the origin",p1,p1.distance_from_origin());
    println!("p2 ({:?}) is {} units from p1 ({:?})",p2,p2.distance_from_point(p1),p1);
}
