use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Shr, Sub},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3<F>([F; 3]);

pub type P3<F> = Vec3<F>;

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Vec3<F>
{
    #[inline(always)]
    pub fn new(x: F, y: F, z: F) -> Self {
        Self([x, y, z])
    }

    #[inline(always)]
    pub fn x(&self) -> F {
        self.0[0]
    }

    #[inline(always)]
    pub fn y(&self) -> F {
        self.0[1]
    }

    #[inline(always)]
    pub fn z(&self) -> F {
        self.0[2]
    }

    // https://suraj.sh/fast-square-root-approximation
    #[inline(always)]
    pub fn length(&self) -> F {
        let a = self.length_squared();
        let x = <f64 as Into<F>>::into(0x1fbd3f7d as f64) + (a >> 1);
        (((x * x) + a) / x) * <f64 as Into<F>>::into(0.5)
    }

    #[inline(always)]
    pub fn length_squared(&self) -> F {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    #[inline(always)]
    pub fn dot(&self, other: Self) -> F {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    #[inline(always)]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    #[inline(always)]
    pub fn unit_len(self) -> Self {
        self / self.length()
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Display for Vec3<F>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Neg for Vec3<F>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x(), self.y(), self.z())
    }
}

impl<F> Index<usize> for Vec3<F> {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + std::ops::Mul<Output = F>
            + std::ops::Div<Output = F>
            + std::ops::Shr<i32, Output = F>
            + std::convert::From<f64>
            + Display,
    > AddAssign for Vec3<F>
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 = [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()];
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + std::ops::Mul<Output = F>
            + std::ops::Div<Output = F>
            + std::ops::Shr<i32, Output = F>
            + std::convert::From<f64>
            + Display,
    > Add for Vec3<F>
{
    type Output = Vec3<F>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + std::ops::Mul<Output = F>
            + std::ops::Div<Output = F>
            + std::ops::Shr<i32, Output = F>
            + std::convert::From<f64>
            + Display,
    > Sub for Vec3<F>
{
    type Output = Vec3<F>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > MulAssign for Vec3<F>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()];
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Mul for Vec3<F>
{
    type Output = Vec3<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

// might need communative?
impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Mul<F> for Vec3<F>
{
    type Output = Vec3<F>;
    fn mul(self, c: F) -> Self::Output {
        Self::new(self.x() * c, self.y() * c, self.z() * c)
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > Div<F> for Vec3<F>
{
    type Output = Vec3<F>;
    fn div(self, c: F) -> Self::Output {
        Self::new(self.x() / c, self.y() / c, self.z() / c)
    }
}

impl<
        F: std::marker::Copy
            + Add<Output = F>
            + Sub<Output = F>
            + Mul<Output = F>
            + Div<Output = F>
            + Shr<i32, Output = F>
            + From<f64>
            + Display,
    > DivAssign for Vec3<F>
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 = [self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z()];
    }
}
