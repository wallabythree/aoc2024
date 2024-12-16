#![allow(dead_code)]

use std::ops::{ Add, Div, Mul, Neg, Sub };
use std::hash::Hash;
use std::collections::HashMap;
use num::{ Integer, NumCast, Signed, ToPrimitive, Unsigned };

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Point<T: Integer> {
    pub x: T,
    pub y: T,
}

impl<T: Integer> Add for Point<T> {
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

impl<T: Integer + Copy + TryInto<usize>> Point<T> {
    pub fn usized(&self) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        Ok((
            self.x.try_into().map_err(|_| "oops")?,
            self.y.try_into().map_err(|_| "oops")?
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
        }
    }
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Add for Direction {
    type Output = Point<i64>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::from(self) + Point::from(rhs)
    }
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
    where T: Integer + Copy + TryInto<usize> {
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

impl<T: Integer + Signed + ToPrimitive + Ord + Hash + Copy, V> PointMap<T, V> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            min: (T::zero(), T::zero()).into(),
            max: (T::zero(), T::zero()).into()
        }
    }

    pub fn width<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.x.sub(self.min.x).abs()).expect("Conversion error")
    }

    pub fn height<U: Integer + Unsigned + NumCast>(&self) -> U {
        U::from(self.max.y.sub(self.min.y).abs()).expect("Conversion error")
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
