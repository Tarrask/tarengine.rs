use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, Hash)]
pub struct BaseCoord<T> (T, T, T);

pub type Coord = BaseCoord<i32>;
pub type FCoord = BaseCoord<f32>;

#[allow(dead_code)]
impl Coord {
    pub const ZERO: Coord = BaseCoord::<i32>(0, 0, 0);

    pub const NEIGHBOURS : [Coord; 6] = [
        BaseCoord::<i32>(1, 0, -1),
        BaseCoord::<i32>(1, -1, 0),
        BaseCoord::<i32>(0, -1, 1),
        BaseCoord::<i32>(-1, 0, 1),
        BaseCoord::<i32>(-1, 1, 0),
        BaseCoord::<i32>(0, 1, -1)
    ];

    pub fn new(x: i32, y: i32, z: i32) -> Result<Self, String> {
        let c = Self(x, y, z);
        if x + y + z != 0 {
            Err(format!("{} is not on the hexa plane", c))
        }
        else {
            Ok(c)
        }
    }

    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
    pub fn z(&self) -> i32 {
        self.2
    }

    pub fn length(&self) -> i32 {
        (self.0.abs() + self.1.abs() + self.2.abs()) / 2
    }

    pub fn distance(self, other: Coord) -> i32 {
        (self - other).length()
    }

    pub fn direction(direction: usize) -> Coord {
        Self::NEIGHBOURS[direction % 6]
    }

    pub fn neighbour(self, direction: usize) -> Coord {
        self + Self::direction(direction)
    }

    pub fn ring(&self, radius: usize) -> Vec<Coord> {
        let mut v = Vec::<Coord>::new();

        v
    }
}

impl FCoord {
    pub fn new(x: f32, y: f32, z: f32) -> Result<Self, String> {
        let c = Self(x, y, z);
        if x + y + z != 0.0 {
            Err(format!("{} is not on the hexa plane", c))
        }
        else {
            Ok(c)
        }
    }
}

impl From<FCoord> for Coord {
    fn from(fc: FCoord) -> Coord {
        let mut q: i32 = f32::round(fc.0) as i32;
        let mut r: i32 = f32::round(fc.1) as i32;
        let mut s = f32::round(fc.2) as i32;
        let q_diff = f32::abs(q as f32 - fc.0);
        let r_diff = f32::abs(r as f32 - fc.1);
        let s_diff = f32::abs(s as f32 - fc.2);
        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        }
        else if r_diff > s_diff {
            r = -q - s;
        }
        else {
            s = -q - r;
        }

        BaseCoord::<i32>(q, r, s)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}
impl Eq for Coord {}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2 )
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2 )
    }
}

impl ops::Mul<i32> for Coord {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl<T: fmt::Display> fmt::Display for BaseCoord<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::Coord;

    #[test]
    fn can_be_constructed() {
        let coord = Coord::new(0, 0, 0).unwrap();
        assert!(coord.0 == 0);
        assert!(coord.1 == 0);
        assert!(coord.2 == 0);
    
        let coord = Coord::new(1, 2, -3).unwrap();
        assert!(coord.0 == 1);
        assert!(coord.1 == 2);
        assert!(coord.2 == -3);
    }
    
    #[test]
    fn prevent_construction_out_of_plane() {
        assert_eq!(Coord::new(1, 2, 3), Err(String::from("(1,2,3) is not on the hexa plane")))
    }
    
    #[test]
    fn equality() {
        let c1 = Coord::new(1, 2, -3).unwrap();
        let c2 = Coord::new(1, 2, -3).unwrap();
        assert_eq!(c1, c2);
    
        let c3 = Coord::new(0, 0, 0).unwrap();
        assert_ne!(c1, c3);
    }
    
    #[test]
    fn addition() -> Result<(), String> {
        let c0 = Coord::ZERO;
        assert_eq!(c0 + c0, c0);
        let c1 = Coord::new(1, 2, -3)?;
        let c2 = Coord::new(-4, 5, -1)?;
        let sum = Coord::new(-3, 7, -4)?;
        assert_eq!(c1 + c2, sum);
    
        // associativity
        let c3 = Coord::new(2, 3, -5)?;
        assert_eq!((c1 + c2) + c3, c1 + (c2 + c3));
    
        // commutativity
        assert_eq!(c1 + c2, c2 + c1);
    
        Ok(())
    }
    
    #[test]
    fn substraction() -> Result<(), String> {
        let c0 = Coord::ZERO;
        assert_eq!(c0 - c0, c0);
        let c1 = Coord::new(1, 2, -3)?;
        let c2 = Coord::new(-4, 5, -1)?;
        let sum = Coord::new(5, -3, -2)?;
        assert_eq!(c1 - c2, sum);
    
        // !associativity
        let c3 = Coord::new(2, 3, -5)?;
        assert_ne!((c1 - c2) - c3, c1 - (c2 - c3));
    
        // !commutativity
        assert_ne!(c1 - c2, c2 - c1);
    
        Ok(())
    }
    
    #[test]
    fn multiplication() -> Result<(), String> {
        let c0 = Coord::ZERO;
        assert_eq!(c0 * 5, c0);
    
        let c1 = Coord::new(1, 2, -3)?;
        let rhs = 5;
        let mul = Coord::new(1 * rhs, 2 * rhs, -3 * rhs)?;
        assert_eq!(c1 * rhs, mul);
    
        Ok(())
    }
        #[test]
    fn distance() -> Result<(), String> {
        let c0 = Coord::ZERO;
        assert_eq!(c0.distance(c0), 0);
    
        let c1 = Coord::new(1, 2, -3)?;
        assert_eq!(c1.distance(c0), c1.length());
    
        let c2 = Coord::new(4, 5, -9)?;
        assert_eq!(c1.distance(c2), 6);
    
        // commutativity
        assert_eq!(c1.distance(c2), c2.distance(c1));
    
        Ok(())
    }
    
    #[test]
    fn directions() -> Result<(), String> {
        let c0 = Coord::ZERO;
        let c1 = Coord::new(1, 0, -1)?;
        let c2 = Coord::new(2, 0, -2)?;
    
        assert_eq!(c0.neighbour(0), c1);
        assert_eq!(c1.neighbour(0), c2);
        assert_eq!(c2.neighbour(3), c1);
        assert_eq!(c1.neighbour(3), c0);
    
        let c3 = Coord::direction(1);
        assert_eq!(c0.neighbour(1), c3);
        assert_eq!(c3.neighbour(4), c0);
    
        let c4 = Coord::direction(2);
        assert_eq!(c0.neighbour(2), c4);
        assert_eq!(c4.neighbour(5), c0);
    
        let c5 = Coord::new(4, 5, -9)?;
        assert_eq!(c5.neighbour(0).neighbour(3), c5);
        assert_eq!(c5.neighbour(1).neighbour(4), c5);
        assert_eq!(c5.neighbour(2).neighbour(5), c5);
    
        Ok(())
    }
}