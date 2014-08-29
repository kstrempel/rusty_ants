use std::num;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};

struct Point {
  x: uint,
  y: uint
}

fn generate_world(height: uint, width: uint, count: uint) -> Vec<Point> {
  let range_x = Range::new(0, height);
  let range_y = Range::new(0, width);
  let mut rng = rand::task_rng();
  let counter = range(0u, count);
  counter.map(|_| Point{x: range_x.ind_sample(&mut rng),
                        y: range_y.ind_sample(&mut rng)}).collect()
}

fn distance(a: Point, b: Point) -> f64 {
  let result = (num::pow(2i, (a.x - b.y)) + num::pow(2i, (a.y-b.x)));
  (result as f64).sqrt()
}

fn main(){
  let mut world = generate_world(100, 200, 10);

  let a = world.pop().unwrap();
  let b = world.pop().unwrap();

  println!("Result {}", distance(a, b));
}
