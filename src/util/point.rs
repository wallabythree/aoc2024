#![allow(dead_code)]

use std::ops::{ Add, Div, Mul, Neg, Sub };
use std::hash::Hash;
use std::collections::{ HashMap, HashSet };
use num::{
    CheckedAdd,
    CheckedSub,
    Integer,
    NumCast,
    Signed,
    ToPrimitive,
    Unsigned
};
use Direction::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Point<T: Integer> {
    pub x: T,
    pub y: T,
}

impl<T: Integer> Add<Point<T>> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Integer + Signed> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl<T: Integer> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Integer + Copy> Mul<T> for Point<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T: Integer + Copy> Div<T> for Point<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T: Integer + Copy> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl<T: Integer + Copy> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Integer + Copy + CheckedAdd> Point<T> {
    pub fn checked_add<U>(&self, rhs: Point<U>) -> Option<Self>
    where U: Integer + Copy + CheckedAdd + TryFrom<T> + TryInto<T>
    {
        let u_x = U::try_from(self.x).ok()?.checked_add(&rhs.x)?;
        let u_y = U::try_from(self.y).ok()?.checked_add(&rhs.y)?;

        let x = U::try_into(u_x).ok()?;
        let y = U::try_into(u_y).ok()?;

        Some((x, y).into())
    }
}

impl<T: Integer + Copy + CheckedSub> Point<T> {
    pub fn checked_sub<U>(&self, rhs: Point<U>) -> Option<Self>
    where U: Integer + Copy + CheckedSub + TryFrom<T> + TryInto<T>
    {
        let u_x = U::try_from(self.x).ok()?.checked_sub(&rhs.x)?;
        let u_y = U::try_from(self.y).ok()?.checked_sub(&rhs.y)?;

        let x = U::try_into(u_x).ok()?;
        let y = U::try_into(u_y).ok()?;

        Some((x, y).into())
    }
}

impl<T: Integer + Copy + TryInto<usize> + CheckedAdd> Point<T> {
    pub fn usized(&self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        Ok((
            self.x.try_into().map_err(|_| "Could not convert to usize")?,
            self.y.try_into().map_err(|_| "Could not convert to usize")?
        ))
    }
}

impl<T: Integer + Signed + Copy> From<Direction> for Point<T> {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => (T::zero(), -T::one()).into(),
            Direction::East => (T::one(), T::zero()).into(),
            Direction::South => (T::zero(), T::one()).into(),
            Direction::West => (-T::one(), T::zero()).into(),
            Direction::Northeast => {
                <Point<T>>::from(Direction::North) +
                <Point<T>>::from(Direction::East)
            },
            Direction::Southeast => {
                <Point<T>>::from(Direction::East) +
                <Point<T>>::from(Direction::South)
            },
            Direction::Southwest => {
                <Point<T>>::from(Direction::South) +
                <Point<T>>::from(Direction::West)
            },
            Direction::Northwest=> {
                <Point<T>>::from(Direction::West) +
                <Point<T>>::from(Direction::North)
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

#[derive(Debug)]
pub struct Grid<T> {
    pub tiles: Vec<Vec<T>>,
}

impl<E> Grid<E> {
    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        if self.tiles.is_empty() {
            0
        } else {
            self.tiles[0].len()
        }
    }

    pub fn in_bounds<T>(&self, p: Point<T>) -> bool
    where T: Integer + Copy + TryInto<usize> + CheckedAdd {
        let p: Result<(usize, usize), _> = p.usized();

        p.is_ok_and(|(x, y)| x < self.width() && y < self.height())
    }
}

impl<E: TryFrom<char>> TryFrom<&str> for Grid<E> {
    type Error = <E as std::convert::TryFrom<char>>::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let tiles = input
            .lines()
            .map(|row| row
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<_>, _>>()
            )
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        Ok(Self { tiles })
    }
}

#[derive(Debug)]
pub struct PointMap<T: Integer + Hash, V> {
    pub tiles: HashMap<Point<T>, V>,
    min: Point<T>,
    max: Point<T>,
}

impl<T: Integer + ToPrimitive + Ord + Hash + Copy, V> PointMap<T, V> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            min: (T::zero(), T::zero()).into(),
            max: (T::zero(), T::zero()).into()
        }
    }

    pub fn width<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.x.sub(self.min.x)).expect("Conversion error")
    }

    pub fn height<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.y.sub(self.min.y)).expect("Conversion error")
    }

    pub fn in_bounds(&self, p: Point<T>) -> bool {
        p >= self.min && p <= self.max
    }

    pub fn insert(&mut self, p: Point<T>, e: V) -> Option<V> {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);

        self.tiles.insert(p, e)
    }

    pub fn remove(&mut self, p: Point<T>) -> Option<V> {
        self.tiles.remove(&p)
    }
}

#[derive(Debug)]
pub struct PointSet<T: Integer + Hash> {
    pub tiles: HashSet<Point<T>>,
    min: Point<T>,
    max: Point<T>,
}

impl<T> PointSet<T>
where T: Integer + ToPrimitive + Ord + Hash + Copy + CheckedAdd + TryFrom<i64>, i64: TryFrom<T>
{
    pub fn new() -> Self {
        Self {
            tiles: HashSet::new(),
            min: (T::zero(), T::zero()).into(),
            max: (T::zero(), T::zero()).into()
        }
    }

    pub fn width<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.x.sub(self.min.x)).expect("Conversion error")
    }

    pub fn height<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.y.sub(self.min.y)).expect("Conversion error")
    }

    pub fn in_bounds(&self, p: Point<T>) -> bool {
        p >= self.min && p <= self.max
    }

    pub fn insert(&mut self, p: Point<T>) -> bool {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);

        self.tiles.insert(p)
    }

    pub fn remove(&mut self, p: Point<T>) -> bool {
        self.tiles.remove(&p)
    }

    pub fn neighbours(&mut self, p: Point<T>) -> Vec<Point<T>> {
        [North, East, South, West]
            .iter()
            .filter_map(move |&d| p.checked_add(<Point<i64>>::from(d)))
            .filter(|p| self.tiles.contains(p))
            .collect()
    }
}
