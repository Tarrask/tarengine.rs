pub mod coord;
pub mod map;

mod layout {
    use std::ops;

    use super::Coord;
    use super::FCoord;

    pub struct Orientation {
        f0: f32,
        f1: f32,
        f2: f32,
        f3: f32,
        b0: f32,
        b1: f32,
        b2: f32,
        b3: f32,
        start_angle: f32
    }

    const SQRT_3: f32 = 1.7320508075688772935274463415059;

    impl Orientation {
        pub const POINTY: Orientation = Orientation {
            f0: SQRT_3,
            f1: SQRT_3 / 2.0,
            f2: 0.0,
            f3: 3.0 / 2.0,
            b0: SQRT_3 / 3.0,
            b1: -1.0 / 3.0,
            b2: 0.0,
            b3: 2.0 / 3.0,
            start_angle: 0.5
        };

        pub const FLAT: Orientation = Orientation {
            f0: 3.0 / 2.0,
            f1: 0.0,
            f2: SQRT_3 / 2.0,
            f3: SQRT_3,
            b0: 2.0 / 3.0,
            b1: 0.0,
            b2: -1.0 / 3.0,
            b3: SQRT_3 / 3.0,
            start_angle: 0.0
        };
    }

    pub struct Point (pub f32, pub f32);

    impl Point {
        pub fn new(x: f32, y: f32) -> Point {
            Point(x, y)
        }
    }
    impl ops::Add for Point {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self(self.0 + other.0, self.1 + other.1)
        }
    }
    impl ops::Add<&Point> for Point {
        type Output = Self;
        fn add(self, other: &Self) -> Self {
            Self(self.0 + other.0, self.1 + other.1)
        }
    }

    impl Copy for Point {}
    impl Clone for Point {
        fn clone(&self) -> Self {
            Self(self.0, self.1)
        }
    }

    pub struct Layout {
        orientation: Orientation,
        size: Point,
        origin: Point
    }

    #[allow(dead_code)]
    impl Layout {
        pub fn new(orientation: Orientation, size: Point, origin: Point) -> Layout {
            Layout { orientation, size, origin }
        }
        pub fn hex_to_pixel(&self, h: &Coord) -> Point {
            let m = &self.orientation;
            Point(
                (m.f0 * h.x() as f32 + m.f1 * h.y() as f32) * self.size.0 + self.origin.0, 
                (m.f2 * h.x() as f32 + m.f3 * h.y() as f32) * self.size.1 + self.origin.1
            )
        }

        pub fn pixel_to_hex(&self, p: &Point) -> FCoord {
            let m = &self.orientation;
            let pt = Point(
                (p.0 - self.origin.0) / self.size.0,
                (p.1 - self.origin.1) / self.size.1
            );
            let x = m.b0 * pt.0 + m.b1 * pt.1;
            let y = m.b2 * pt.0 + m.b3 * pt.1;
            FCoord::new(x, y, -x - y).unwrap()
        }

        pub fn hex_corner_offset(&self, corner: usize) -> Point {
            let size = self.size;
            let angle = 2.0 * std::f32::consts::PI * (self.orientation.start_angle + corner as f32) / 6.0;
            Point(size.0 * angle.cos(), size.1 * angle.sin())
        }

        pub fn polygon_corners(&self, c: &Coord) -> [Point; 6] {
            let mut corners:[Point; 6] = [Point(0.0, 0.0); 6];
            let center = self.hex_to_pixel(c);
            for i in 0 as usize..6 {
                let offset = self.hex_corner_offset(i);
                corners[i] = center + offset
            }

            corners
        }
    }
}

pub use self::coord::Coord;
pub use self::coord::FCoord;
pub use self::map::Map;
pub use self::map::MapShape;
pub use self::layout::Orientation;
pub use self::layout::Layout;
pub use self::layout::Point;