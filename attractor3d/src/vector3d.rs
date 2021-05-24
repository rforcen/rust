// vector3d

#![allow(dead_code)]

use num::traits::{Float, One, Zero};
use num_traits::cast::FromPrimitive;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

const DOUBLE_PREC: f64 = 2.2204460492503131e-16;

#[derive(Debug, Copy, Clone)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float + FromPrimitive + Zero> Vector3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
        }
    }
    pub fn one() -> Self {
        Self {
            x: One::one(),
            y: One::one(),
            z: One::one(),
        }
    }

    pub fn set(&mut self, x: T, y: T, z: T) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        *self
    }
    pub fn set_nr(&mut self, x: T, y: T, z: T) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn set_vect(&mut self, v: &Self) {
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }
    pub fn get(&self, i: i32) -> T
    where
        T: Copy,
    {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }
    pub fn set_val(&mut self, i: i32, val: T) {
        match i {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => self.x = val,
        }
    }

    pub fn scale(&mut self, sc: T) {
        self.x = self.x * sc;
        self.y = self.y * sc;
        self.z = self.z * sc;
    }
    pub fn scale_by(&mut self, sc: T) -> Vector3d<T>
    where
        T: Copy,
    {
        Self {
            x: self.x * sc,
            y: self.y * sc,
            z: self.z * sc,
        }
    }

    pub fn norm(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn norm_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn distance(&self, v: &Vector3d<T>) -> T {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
        .norm_squared()
        .sqrt()
    }
    pub fn distance_squared(&self, v: &Vector3d<T>) -> T {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
        .norm_squared()
    }
    pub fn dot(&self, v: &Vector3d<T>) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // convert f64 to T
    pub fn f64_t(x: f64) -> T {
        T::from_f64(x).unwrap()
    }
    // convert f32 to T
    pub fn f32_t(x: f32) -> T {
        T::from_f32(x).unwrap()
    }
    // convert usize to T
    pub fn usize_t(x: usize) -> T {
        T::from_usize(x).unwrap()
    }

    pub fn normalize(&mut self) {
        let len_sqr = self.norm_squared();
        let err = len_sqr - Self::f64_t(1.);

        if err > Self::f64_t(2. * DOUBLE_PREC) || err < -Self::f64_t(2. * DOUBLE_PREC) {
            let len = len_sqr.sqrt();
            if len != Zero::zero() {
                self.x = self.x / len;
                self.y = self.y / len;
                self.z = self.z / len;
            }
        }
    }
    pub fn set_zero(&mut self) {
        self.x = Zero::zero();
        self.y = Zero::zero();
        self.z = Zero::zero();
    }

    pub fn cross(&mut self, v1: Vector3d<T>, v2: Vector3d<T>) {
        let tmpx = v1.y * v2.z - v1.z * v2.y;
        let tmpy = v1.z * v2.x - v1.x * v2.z;
        let tmpz = v1.x * v2.y - v1.y * v2.x;

        self.x = tmpx;
        self.y = tmpy;
        self.z = tmpz;
    }

    // point cloud traits
    pub fn centroid(pnts: &Vec<Self>) -> Self {
        pnts.iter().fold(Vector3d::zero(), |sum, &p| sum + p) / Self::usize_t(pnts.len())
    }
    pub fn scaled_distances(pnts: &Vec<Self>) -> Vec<T> {
        let centroid = Vector3d::centroid(pnts);

        // scale distances to centroid
        let distances: Vec<_> = pnts.iter().map(|p| p.distance(&centroid)).collect();
        let max = distances
            .iter()
            .max_by(|x, y| x.partial_cmp(&y).unwrap())
            .unwrap();
        if *max != Zero::zero() {
            distances.iter().map(|d| *d / *max).collect::<Vec<_>>()
        } else {
            distances
        }
    }

    pub fn to_string(&self) -> String
    where
        T: std::fmt::Display,
    {
        format!(
            "{} {} {}",
            self.x.to_string(),
            self.y.to_string(),
            self.z.to_string()
        )
    }

    pub fn parts(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

// +, +=, -, -=, *, *=, /, /=   operator overload

impl<T: Add<Output = T>> Add for Vector3d<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Vector3d<T>
where
    T: Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector3d<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: AddAssign> AddAssign<T> for Vector3d<T>
where
    T: Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl<T: Sub<Output = T>> Sub for Vector3d<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub<T> for Vector3d<T>
where
    T: Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector3d<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: SubAssign> SubAssign<T> for Vector3d<T>
where
    T: Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl<T: Mul<Output = T>> Mul for Vector3d<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for Vector3d<T>
where
    T: Copy,
{
    // Vector * scalar
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: MulAssign> MulAssign for Vector3d<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: MulAssign> MulAssign<T> for Vector3d<T>
where
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Div<Output = T>> Div for Vector3d<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Div<Output = T>> Div<T> for Vector3d<T>
where
    T: Copy,
{
    // Vector / scalar
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: DivAssign> DivAssign for Vector3d<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T: DivAssign> DivAssign<T> for Vector3d<T>
where
    T: Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: PartialEq> PartialEq for Vector3d<T> {
    fn eq(&self, rhs: &Vector3d<T>) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl<T: Eq> std::cmp::Eq for Vector3d<T> {}

use std::cmp::Ordering;

impl<T: PartialOrd> PartialOrd for Vector3d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ret: Ordering;

        if self.x < other.x {
            ret = Ordering::Less
        } else {
            // s.x >= o.x
            if self.x > other.x {
                ret = Ordering::Greater
            } else {
                // s.x==o.x
                if self.y < other.y {
                    ret = Ordering::Less
                } else {
                    if self.y > other.y {
                        ret = Ordering::Greater
                    } else {
                        // s.y==o.y
                        if self.z < other.z {
                            ret = Ordering::Less
                        } else {
                            if self.z == other.z {
                                ret = Ordering::Equal
                            } else {
                                ret = Ordering::Greater
                            }
                        }
                    }
                }
            }
        }

        Some(ret)
    }
}

// default formatter
impl<T> fmt::Display for Vector3d<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn test_vect3d() {
        let mut v0 = Vector3d::<f32>::zero();
        let v1 = Vector3d::<f32>::zero();

        v0.set(0., 1., 2.);
        println!("v0!=v1 : {}", v0 != v1);
        println!("v0>v1 : {}", v0 > v1);
        println!("v0<v1 : {}", v0 < v1);

        v0.set(0., 1., 2.);

        for i in 0..3 {
            println!("v0[{}]={}", i, v0.get(i));
        }

        for i in 0..3 {
            v0.set_val(i, i as f32)
        }

        println!("v0={}", v0);

        let v1 = v0;
        println!("v1={}", v1);

        let mut v2 = v0 + v1;
        println!("v0+v1={}", v2);
        v2 += v1;
        println!("v2+=v1={}", v2);

        let mut v3 = v0 - v1;
        println!("v0-v1={}", v3);
        v3 -= v1;
        println!("v3+=v1={}", v3);

        v3.scale(2.2);
        println!("v3 scaled={}", v3);
        println!("v0 dot v1={}", v0.dot(&v1));
        v3.normalize();
        println!("v3 normalized={}", v3);

        v3 = v3 * 111.;
        println!("v3*111.={}, {}", v3, v3.to_string());

        v3 += v3;
        v3 += 100.;
        v3 /= 10.;
        println!("v3+123.+100.={}, {}", v3, v3.to_string());

        let v4 = v3.scale_by(1. / 10.);
        println!("v3.scale_by(10.)={}", v4);

        v3.set_zero();
        println!("v3 zero={}, {}", v3, v3.to_string());
    }
}
