use std::{
    fmt::{Debug, Display},
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Shl, Shr},
};

const INTERNAL_FRACTION_BITS: u64 = 16;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FractionNum(u64);

impl FractionNum {
    pub const FRACTION_BITS: u64 = INTERNAL_FRACTION_BITS;

    #[inline]
    pub fn new(num: u64) -> Self {
        Self(num << Self::FRACTION_BITS)
    }

    pub fn from_raw_u64(num: u64) -> FractionNum {
        Self(num)
    }

    pub fn into_u64(self) -> u64 {
        self.0 >> Self::FRACTION_BITS
    }

    pub fn into_raw_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for FractionNum {
    fn from(n: u64) -> Self {
        Self(n << Self::FRACTION_BITS)
    }
}

impl From<FractionNum> for u64 {
    fn from(n: FractionNum) -> Self {
        n.0 >> FractionNum::FRACTION_BITS
    }
}

impl Add for FractionNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for FractionNum {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add<u64> for FractionNum {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl AddAssign<u64> for FractionNum {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += Self::new(rhs).0;
    }
}

impl Sub for FractionNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for FractionNum {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sub<u64> for FractionNum {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl SubAssign<u64> for FractionNum {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= Self::new(rhs).0;
    }
}

impl Mul<u64> for FractionNum {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<u64> for FractionNum {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 *= rhs
    }
}

impl Mul<FractionNum> for FractionNum {
    type Output = Self;

    fn mul(self, rhs: FractionNum) -> Self::Output {
        FractionNum::from_raw_u64((self.0 * rhs.0) >> 16)
    }
}

impl MulAssign<FractionNum> for FractionNum {
    fn mul_assign(&mut self, rhs: FractionNum) {
        self.0 = (self.0 * rhs.0) >> 16
    }
}

impl Div<u64> for FractionNum {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<u64> for FractionNum {
    fn div_assign(&mut self, rhs: u64) {
        self.0 /= rhs
    }
}

impl Div<FractionNum> for FractionNum {
    type Output = Self;

    fn div(self, rhs: FractionNum) -> Self::Output {
        let shifted = self.0 << FractionNum::FRACTION_BITS;
        FractionNum(shifted / rhs.0)

    }
}

impl DivAssign<FractionNum> for FractionNum {
    fn div_assign(&mut self, rhs: FractionNum) {
        self.0 <<= 16;
        self.0 /= rhs.0;
    }
}

impl Shl<u8> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i8> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: i8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u16> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: u16) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i16> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: i16) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u32> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i32> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u64> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i64> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: i64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u128> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: u128) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i128> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: i128) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<usize> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<isize> for FractionNum {
    type Output = Self;

    fn shl(self, rhs: isize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i8> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: i8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u16> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: u16) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i16> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: i16) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u32> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i32> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u64> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i64> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: i64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u128> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i128> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: i128) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<usize> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<isize> for FractionNum {
    type Output = Self;

    fn shr(self, rhs: isize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Display for FractionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: f64 = self.0 as f64 / 2f64.powf(Self::FRACTION_BITS as f64);

        write!(f, "{num}")
    }
}

impl Debug for FractionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: f64 = self.0 as f64 / 2f64.powf(Self::FRACTION_BITS as f64);

        f.debug_struct("FractionNum")
            .field("Raw", &self.0)
            .field("Float", &num)
            .finish()
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SignedFractionNum(i64);

impl SignedFractionNum {
    pub const FRACTION_BITS: i64 = INTERNAL_FRACTION_BITS as i64;

    #[inline]
    pub fn new(num: i64) -> Self {
        Self(num << Self::FRACTION_BITS)
    }

    pub fn from_raw_i64(num: i64) -> SignedFractionNum {
        Self(num)
    }

    pub fn into_i64(self) -> i64 {
        self.0 >> Self::FRACTION_BITS
    }

    pub fn into_raw_i64(self) -> i64 {
        self.0
    }

    pub fn abs(self) -> Self {
        SignedFractionNum(self.0.abs())
    }
}

impl From<i64> for SignedFractionNum {
    fn from(n: i64) -> Self {
        Self(n << Self::FRACTION_BITS)
    }
}

impl From<SignedFractionNum> for i64 {
    fn from(n: SignedFractionNum) -> Self {
        n.0 >> SignedFractionNum::FRACTION_BITS
    }
}

impl Add for SignedFractionNum {
    type Output = SignedFractionNum;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for SignedFractionNum {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Add<i64> for SignedFractionNum {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl AddAssign<i64> for SignedFractionNum {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += Self::new(rhs).0;
    }
}

impl Sub for SignedFractionNum {
    type Output = SignedFractionNum;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for SignedFractionNum {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sub<i64> for SignedFractionNum {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Self::new(rhs)
    }
}

impl SubAssign<i64> for SignedFractionNum {
    fn sub_assign(&mut self, rhs: i64) {
        self.0 -= Self::new(rhs).0;
    }
}

impl Mul<i64> for SignedFractionNum {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<i64> for SignedFractionNum {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs
    }
}

impl Mul<SignedFractionNum> for SignedFractionNum {
    type Output = SignedFractionNum;

    fn mul(self, rhs: SignedFractionNum) -> Self::Output {
        SignedFractionNum::from_raw_i64((self.0 * rhs.0) >> 16)
    }
}

impl MulAssign<SignedFractionNum> for SignedFractionNum {
    fn mul_assign(&mut self, rhs: SignedFractionNum) {
        self.0 = (self.0 * rhs.0) >> 16
    }
}

impl Div<i64> for SignedFractionNum {
    type Output = SignedFractionNum;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<i64> for SignedFractionNum {
    fn div_assign(&mut self, rhs: i64) {
        self.0 /= rhs
    }
}

impl Div<SignedFractionNum> for SignedFractionNum {
    type Output = Self;

    fn div(self, rhs: SignedFractionNum) -> Self::Output {
        let shifted = self.0 << SignedFractionNum::FRACTION_BITS;
        SignedFractionNum(shifted / rhs.0)

    }
}

impl DivAssign<SignedFractionNum> for SignedFractionNum {
    fn div_assign(&mut self, rhs: SignedFractionNum) {
        self.0 <<= 16;
        self.0 /= rhs.0;
    }
}

impl Display for SignedFractionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: f64 = self.0 as f64 / 2f64.powf(Self::FRACTION_BITS as f64);

        write!(f, "{num}")
    }
}

impl Shl<u8> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i8> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: i8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u16> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: u16) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i16> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: i16) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u32> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i32> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u64> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i64> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: i64) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<u128> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: u128) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<i128> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: i128) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<usize> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<isize> for SignedFractionNum {
    type Output = Self;

    fn shl(self, rhs: isize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i8> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: i8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u16> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: u16) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i16> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: i16) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u32> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i32> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u64> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i64> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: i64) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<u128> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<i128> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: i128) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<usize> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<isize> for SignedFractionNum {
    type Output = Self;

    fn shr(self, rhs: isize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Debug for SignedFractionNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: f64 = self.0 as f64 / 2f64.powf(Self::FRACTION_BITS as f64);

        f.debug_struct("SignedFractionNum")
            .field("Raw", &self.0)
            .field("Float", &num)
            .finish()
    }
}

impl TryFrom<SignedFractionNum> for FractionNum {
    type Error = TryFromIntError;

    fn try_from(value: SignedFractionNum) -> Result<Self, Self::Error> {
        match TryInto::<u64>::try_into(value.0) {
            Ok(x) => Ok(FractionNum::from_raw_u64(x)),
            Err(err) => Err(err),
        }
    }
}

impl TryFrom<FractionNum> for SignedFractionNum {
    type Error = TryFromIntError;

    fn try_from(value: FractionNum) -> Result<Self, Self::Error> {
        match TryInto::<i64>::try_into(value.0) {
            Ok(x) => Ok(SignedFractionNum::from_raw_i64(x)),
            Err(err) => Err(err),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::fraction_num::SignedFractionNum;

    use super::FractionNum;

    #[test]
    fn test_default() {
        assert_eq!(FractionNum::default(), FractionNum::new(0));
        assert_eq!(SignedFractionNum::default(), SignedFractionNum::new(0));
    }

    #[test]
    fn test_conversion() {
        assert!(SignedFractionNum::try_from(FractionNum::new(1)).is_ok());
        assert!(SignedFractionNum::try_from(FractionNum::new(u64::MAX)).is_err());
        assert!(FractionNum::try_from(SignedFractionNum::new(1)).is_ok());
        assert!(FractionNum::try_from(SignedFractionNum::new(-1)).is_err());
    }

    #[test]
    fn test_addtion() {
        assert_eq!(FractionNum::new(1) + FractionNum::new(2), FractionNum::new(3));
        assert_eq!(SignedFractionNum::new(1) + SignedFractionNum::new(2), SignedFractionNum::new(3));

        assert_eq!(SignedFractionNum::new(1) + SignedFractionNum::new(-2), SignedFractionNum::new(-1));

        assert_eq!(FractionNum::new(1) + 2, FractionNum::new(3));
        assert_eq!(SignedFractionNum::new(1) + 2, SignedFractionNum::new(3));

        assert_eq!(SignedFractionNum::new(1) + -2, SignedFractionNum::new(-1));   
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(FractionNum::new(1) + FractionNum::new(2), FractionNum::new(3));
        assert_eq!(SignedFractionNum::new(1) + SignedFractionNum::new(2), SignedFractionNum::new(3));

        assert_eq!(SignedFractionNum::new(1) + SignedFractionNum::new(-2), SignedFractionNum::new(-1));

        assert_eq!(FractionNum::new(1) + 2, FractionNum::new(3));
        assert_eq!(SignedFractionNum::new(1) + 2, SignedFractionNum::new(3));

        assert_eq!(SignedFractionNum::new(1) + -2, SignedFractionNum::new(-1));
        
    }

    #[test]
    fn test_abs(){
        assert_eq!(SignedFractionNum::new(2).abs(), SignedFractionNum::new(2));
        assert_eq!(SignedFractionNum::new(-2).abs(), SignedFractionNum::new(2));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(FractionNum::new(1) * 2, FractionNum::new(2));
        assert_eq!(SignedFractionNum::new(1) * 2, SignedFractionNum::new(2));
        assert_eq!(SignedFractionNum::new(-1) * 2, SignedFractionNum::new(-2));

        assert_eq!(FractionNum::new(2) * FractionNum::new(2), FractionNum::new(4));

        let mut x = FractionNum::new(2);
        x *= x;
        assert_eq!(x, FractionNum::new(4));

        assert_eq!(SignedFractionNum::new(2) * SignedFractionNum::new(2), SignedFractionNum::new(4));

        let mut x = SignedFractionNum::new(2);
        x *= x;
        assert_eq!(x, SignedFractionNum::new(4));

        assert_eq!(SignedFractionNum::new(-2) * SignedFractionNum::new(2), SignedFractionNum::new(-4));

        let mut x = SignedFractionNum::new(-2);
        x *= x;
        assert_eq!(x, SignedFractionNum::new(4));
    }

    #[test]
    fn test_division() {
        assert_eq!(FractionNum::new(2) / 2, FractionNum::new(1));
        assert_eq!(SignedFractionNum::new(2) / 2, SignedFractionNum::new(1));
        assert_eq!(SignedFractionNum::new(-2) / 2, SignedFractionNum::new(-1));

        assert_eq!(FractionNum::new(4) / FractionNum::new(2), FractionNum::new(2));
        assert_eq!(SignedFractionNum::new(4) / SignedFractionNum::new(2), SignedFractionNum::new(2));
        assert_eq!(SignedFractionNum::new(-4) / SignedFractionNum::new(2), SignedFractionNum::new(-2));
    }

    #[test]
    fn test_shift_left() {
        assert_eq!(FractionNum::new(2) << 1, FractionNum::new(4));
        assert_eq!(SignedFractionNum::new(2) << 1, SignedFractionNum::new(4));
        assert_eq!(SignedFractionNum::new(-2) << 1, SignedFractionNum::new(-4));
    }

    #[test]
    fn test_shift_right() {
        assert_eq!(FractionNum::new(2) >> 1, FractionNum::new(1));
        assert_eq!(SignedFractionNum::new(2) >> 1, SignedFractionNum::new(1));
        assert_eq!(SignedFractionNum::new(-2) >> 1, SignedFractionNum::new(-1));
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn division_panics() {
        let _ = FractionNum::default() / 0;
        let _ = SignedFractionNum::default() / 0;
    }


}