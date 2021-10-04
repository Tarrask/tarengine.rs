use std::collections::HashMap;

use super::Coord;

pub enum MapShape<T> {
    Parallelogram { q: usize, r: usize, direction: usize, f: fn(Coord) -> T },
    Triangle { size: usize, pointy: bool },
    Hexagon { 
        radius: i32,
        generator: fn(Coord) -> T
    },
    Rectangle
}

pub struct Map<T> {
    pub ground: HashMap<Coord, T>
}

impl<T> Map<T> {
    pub fn new(shape: MapShape<T>) -> Self {
        let mut ground = HashMap::<Coord, T>::new();

        match shape {
            MapShape::Parallelogram { q, r, direction, f } => {
                for q in -(q as i32)..(q as i32) {
                    for r in -(r as i32)..(r as i32) {
                        match direction {
                            0 => { 
                                let c = Coord::new(q, r, -q - r).unwrap();
                                ground.insert(c, f(c)); 
                            }
                            1 => { 
                                let c = Coord::new(q, -q - r, r).unwrap();
                                ground.insert(c, f(c)); 
                            }
                            _ => { 
                                let c = Coord::new(-q - r, q, r).unwrap();
                                ground.insert(c, f(c)); 
                            }
                        }
                    }
                }
            },
            // MapShape::Triangle { size, pointy } => {
            //     for q in 0..size as i32 {
            //         for r in 0..size as i32 - q {
            //             match pointy {
            //                 true => { ground.insert(Coord::new(q, -q - r, r).unwrap(), 0); }
            //                 false => { ground.insert(Coord::new(q, r, -q - r).unwrap(), 0); }
            //             }
            //         }
            //     }
            // },
            MapShape::Hexagon { radius, generator} => {
                for q in -radius..=radius {
                    let r1 = i32::max(-radius, -q - radius);
                    let r2 = i32::min(radius, -q + radius);
                    for r in r1..=r2 {
                        let red = q as f32 / (2.0 * radius as f32) + 1.0;
                        let green = r as f32 / (2.0 * radius as f32) + 1.0;
                        let blue = (-q - r) as f32 / (2.0 * radius as f32) + 1.0;
                        let c = Coord::new(q, r, -q - r).unwrap();
                        ground.insert(c, generator(c));
                    }
                }
            }
            _ => {
            }
        }
        // ground.insert(Coord::new(0, 0, 0).unwrap(), 0);

        Map {
            ground
        }
    }
}