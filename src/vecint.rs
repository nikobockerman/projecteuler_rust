use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign},
};

#[derive(PartialEq, Eq, Clone)]
pub(crate) struct VecUInt {
    num: Vec<u32>,
}

impl VecUInt {
    pub(crate) fn from_uint(value: u128) -> VecUInt {
        VecUInt {
            num: {
                let mut rev: Vec<u32> = vec![];
                let mut remaining = value;
                if remaining > 0 {
                    loop {
                        rev.push((remaining % 10) as u32);
                        if remaining < 10 {
                            break;
                        }
                        remaining /= 10;
                    }
                }
                rev
            },
        }
    }

    pub(crate) fn pow(&self, exp: usize) -> Self {
        if exp == 0 {
            return Self::from(1_u128);
        }

        let mut result = self.clone();
        for _ in 1..exp {
            result *= self.clone();
        }
        result
    }

    fn normalize(&mut self) {
        loop {
            match self.num.last() {
                Some(x) if x == &0 => self.num.pop(),
                _ => break,
            };
        }
    }

    fn multiply(self, rhs: VecUInt, allow_swap: bool) -> VecUInt {
        if self == 0 {
            return self;
        }
        if self == 1 {
            return rhs;
        }
        if &rhs == &0 {
            return VecUInt::from(0);
        }
        if &rhs == &1 {
            return self;
        }

        if rhs.clone() % 10 == 0 {
            let mut result = self;
            let mut remaining = rhs;
            result.num.insert(0, 0);
            remaining.num.remove(0);
            return result.multiply(remaining, false);
        }
        if allow_swap && self.clone() % 10 == 0 {
            return rhs.mul(self);
        }

        let pow10 = |num, exp| match num {
            0 => VecUInt::from(0),
            n => match exp {
                0 => VecUInt::from(n),
                _ => VecUInt::from(n) * VecUInt::from(10).pow(exp),
            },
        };

        let mut result = VecUInt::from(0);
        for rhs_ind in 0..rhs.num.len() {
            let mut overflow: u32 = 0;
            let mut partial_result = VecUInt::from(0);
            for lhs_ind in 0..self.num.len() {
                let mul = self.num[lhs_ind] as u32 * rhs.num[rhs_ind] as u32 + overflow;
                overflow = mul / 10;
                let mul_offset = pow10(mul % 10, lhs_ind);
                partial_result += &mul_offset;
            }
            partial_result += &pow10(overflow, self.num.len());
            let partial_offset = {
                if rhs_ind == 0 || partial_result == 0 || partial_result == 1 {
                    partial_result
                } else {
                    partial_result * VecUInt::from(10).pow(rhs_ind)
                }
            };
            result += &partial_offset;
        }
        result
    }
}

impl From<u128> for VecUInt {
    fn from(value: u128) -> Self {
        VecUInt::from_uint(value)
    }
}

impl From<u32> for VecUInt {
    fn from(value: u32) -> Self {
        VecUInt::from_uint(value as u128)
    }
}

impl From<i128> for VecUInt {
    fn from(value: i128) -> Self {
        if value < 0 {
            panic!("Negative");
        }
        VecUInt::from_uint(value as u128)
    }
}

impl From<i32> for VecUInt {
    fn from(value: i32) -> Self {
        VecUInt::from(value as i128)
    }
}

impl TryFrom<&VecUInt> for u32 {
    type Error = ();

    fn try_from(value: &VecUInt) -> Result<Self, Self::Error> {
        if value > &VecUInt::from(u32::MAX) {
            Err(())
        } else {
            let mut val: u32 = 0;
            for i in 0..value.num.len() {
                val += value.num[i] * 10_u32.pow(i as u32);
            }
            Ok(val)
        }
    }
}

impl TryFrom<&VecUInt> for u128 {
    type Error = ();

    fn try_from(value: &VecUInt) -> Result<Self, Self::Error> {
        let max_value: VecUInt = VecUInt::from(u128::MAX);
        if value > &max_value {
            Err(())
        } else if max_value.num.len() > u32::MAX as usize {
            Err(())
        } else {
            let mut val: u128 = 0;
            for i in 0..value.num.len() {
                val += value.num[i] as u128 * 10_u128.pow(i as u32);
            }
            Ok(val)
        }
    }
}

impl PartialOrd for VecUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.num.len().partial_cmp(&other.num.len()) {
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            None => None,
            Some(Ordering::Equal) => {
                match self
                    .num
                    .iter()
                    .rev()
                    .zip(other.num.iter().rev())
                    .map(|(lhs, rhs)| lhs.partial_cmp(rhs))
                    .skip_while(|cmp| match cmp {
                        Some(Ordering::Equal) => true,
                        Some(_) => false,
                        None => false,
                    })
                    .nth(0)
                {
                    None => Some(Ordering::Equal),
                    Some(None) => None,
                    Some(Some(Ordering::Equal)) => unreachable!(),
                    Some(Some(x)) => Some(x),
                }
            }
        }
    }
}

impl PartialOrd<u128> for VecUInt {
    fn partial_cmp(&self, other: &u128) -> Option<Ordering> {
        self.partial_cmp(&VecUInt::from(*other))
    }
}

impl PartialOrd<VecUInt> for u128 {
    fn partial_cmp(&self, other: &VecUInt) -> Option<Ordering> {
        VecUInt::from(*self).partial_cmp(other)
    }
}

impl PartialEq<u128> for VecUInt {
    fn eq(&self, other: &u128) -> bool {
        self.eq(&VecUInt::from(*other))
    }
}

impl PartialEq<VecUInt> for u128 {
    fn eq(&self, other: &VecUInt) -> bool {
        other.eq(self)
    }
}

impl Add<&VecUInt> for VecUInt {
    type Output = VecUInt;

    fn add(mut self, rhs: &VecUInt) -> Self::Output {
        (&mut self).add_assign(rhs);
        self
    }
}

impl Add<u128> for VecUInt {
    type Output = VecUInt;

    fn add(self, rhs: u128) -> Self::Output {
        self.add(&VecUInt::from(rhs))
    }
}

impl Add<VecUInt> for u128 {
    type Output = VecUInt;

    fn add(self, rhs: VecUInt) -> Self::Output {
        rhs.add(&VecUInt::from(self))
    }
}

impl AddAssign<&VecUInt> for VecUInt {
    fn add_assign(&mut self, rhs: &VecUInt) {
        let len_lhs = self.num.len();
        let len_rhs = rhs.num.len();
        let mut overflow: u32 = 0;
        let mut i = 0_usize;
        loop {
            let next = {
                let lhs = match i < len_lhs {
                    true => self.num[i],
                    false => 0,
                };
                let rhs = match i < len_rhs {
                    true => rhs.num[i],
                    false => 0,
                };
                let sum = lhs + rhs + overflow;
                overflow = match sum >= 10 {
                    true => 1,
                    false => 0,
                };
                sum % 10
            };
            if self.num.len() == i {
                self.num.push(next);
            } else {
                self.num[i] = next;
            }
            i += 1;
            if i >= len_lhs && i >= len_rhs && overflow == 0 {
                break;
            }
        }
        self.normalize();
    }
}

impl AddAssign<u128> for VecUInt {
    fn add_assign(&mut self, rhs: u128) {
        self.add_assign(&VecUInt::from(rhs))
    }
}

impl Sub<&VecUInt> for VecUInt {
    type Output = VecUInt;

    fn sub(mut self, rhs: &VecUInt) -> Self::Output {
        (&mut self).sub_assign(rhs);
        self
    }
}

impl Sub<u128> for VecUInt {
    type Output = VecUInt;

    fn sub(self, rhs: u128) -> Self::Output {
        self.sub(&VecUInt::from(rhs))
    }
}

impl Sub<&VecUInt> for u128 {
    type Output = VecUInt;

    fn sub(self, rhs: &VecUInt) -> Self::Output {
        VecUInt::from(self).sub(rhs)
    }
}

impl SubAssign<&VecUInt> for VecUInt {
    fn sub_assign(&mut self, rhs: &VecUInt) {
        // panic for now if rhs > self as there is no sensible underflow mechanism
        let comparison = {
            let s: &VecUInt = self;
            s.partial_cmp(rhs)
        };
        match comparison {
            Some(Ordering::Equal) => self.num = vec![],
            Some(Ordering::Less) => panic!("Underflow"),
            _ => {
                let mut loaned: u32 = 0;
                for i in 0..rhs.num.len() {
                    let value_to_subtract = rhs.num[i] + loaned;
                    let current = self.num[i];
                    self.num[i] = match value_to_subtract > current {
                        true => {
                            loaned = 1;
                            current + 10 - value_to_subtract
                        }
                        false => {
                            loaned = 0;
                            current - value_to_subtract
                        }
                    };
                }
                if loaned > 0 {
                    self.num[rhs.num.len()] -= loaned;
                }
            }
        };
        self.normalize();
    }
}

impl SubAssign<u128> for VecUInt {
    fn sub_assign(&mut self, rhs: u128) {
        self.sub_assign(&VecUInt::from(rhs));
    }
}

impl Mul<VecUInt> for VecUInt {
    type Output = VecUInt;

    fn mul(self, rhs: VecUInt) -> Self::Output {
        self.multiply(rhs, true)
    }
}

impl Mul<u128> for VecUInt {
    type Output = VecUInt;

    fn mul(self, rhs: u128) -> Self::Output {
        self.mul(VecUInt::from(rhs))
    }
}

impl Mul<VecUInt> for u128 {
    type Output = VecUInt;

    fn mul(self, rhs: VecUInt) -> Self::Output {
        rhs.mul(self)
    }
}

impl MulAssign<VecUInt> for VecUInt {
    fn mul_assign(&mut self, rhs: VecUInt) {
        *self = self.clone().mul(rhs);
    }
}

impl MulAssign<u128> for VecUInt {
    fn mul_assign(&mut self, rhs: u128) {
        self.mul_assign(VecUInt::from(rhs));
    }
}

impl Div<VecUInt> for VecUInt {
    type Output = VecUInt;

    fn div(self, rhs: VecUInt) -> Self::Output {
        if rhs == 0 {
            panic!("Division by zero");
        }
        if rhs == 1 {
            return self;
        }
        let comparison = self.partial_cmp(&rhs);
        match comparison {
            Some(Ordering::Equal) => return VecUInt::from(1),
            Some(Ordering::Less) => return VecUInt::from(0),
            _ => (),
        }

        if rhs.clone() % 10 == 0 {
            let mut result = self;
            let mut remaining = rhs;
            result.num.remove(0);
            remaining.num.remove(0);
            return result.div(remaining);
        }

        let mut result = VecUInt::from(0);
        let mut divident = VecUInt::from(0);
        let mut divided = 0_u32;
        for lhs_ind in (0..self.num.len()).rev() {
            let multiplied = VecUInt::from(divided) * rhs.clone();
            let mut new_divident = divident - &multiplied;
            new_divident.num.insert(0, self.num[lhs_ind]);
            divident = new_divident;
            divided = {
                let mut sum = VecUInt::from(0);
                let mut times = 0_u32;
                loop {
                    match sum.partial_cmp(&divident) {
                        None => unreachable!(),
                        Some(Ordering::Equal) => break,
                        Some(Ordering::Greater) => {
                            times -= 1;
                            break;
                        }
                        Some(Ordering::Less) => {
                            sum += &rhs;
                            times += 1;
                        }
                    }
                }
                times
            };
            assert!(divided < 10);
            if divided > 0 {
                let addition = match lhs_ind {
                    0 => VecUInt::from(divided),
                    n => VecUInt::from(divided) * VecUInt::from(10).pow(n),
                };
                result += &addition;
            }
        }
        result.normalize();
        result
    }
}

impl Div<u128> for VecUInt {
    type Output = VecUInt;

    fn div(self, rhs: u128) -> Self::Output {
        self.div(VecUInt::from(rhs))
    }
}

impl Div<VecUInt> for u128 {
    type Output = VecUInt;

    fn div(self, rhs: VecUInt) -> Self::Output {
        VecUInt::from(self) / rhs
    }
}

impl DivAssign<VecUInt> for VecUInt {
    fn div_assign(&mut self, rhs: VecUInt) {
        *self = self.clone().div(rhs);
    }
}

impl DivAssign<u128> for VecUInt {
    fn div_assign(&mut self, rhs: u128) {
        self.div_assign(VecUInt::from(rhs));
    }
}

impl Rem<u128> for VecUInt {
    type Output = u128;

    fn rem(self, rhs: u128) -> Self::Output {
        if rhs == 10 {
            (*self.num.first().unwrap_or(&0_u32)) as u128
        } else {
            let divided = self.clone() / rhs;
            let remainder = self - &divided;
            u128::try_from(&remainder).unwrap()
        }
    }
}

impl Debug for VecUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VecUInt")
            .field("num", &self.num.iter().rev())
            .finish()
    }
}

impl Display for VecUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self
            .num
            .iter()
            .map(|x| char::from_digit(*x as u32, 10).unwrap())
            .collect::<Vec<char>>();
        let num_parts = chars
            .chunks(3)
            .rev()
            .map(|chunk| chunk.iter().rev().collect::<String>())
            .collect::<Vec<String>>();
        write!(
            f,
            "{{Len={}; Value={}}}",
            self.num.len(),
            num_parts.join("_")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::vecint::VecUInt;

    #[test]
    fn equality() {
        assert_eq!(VecUInt::from(2), VecUInt::from(2));
        assert_eq!(VecUInt::from(23_456), VecUInt::from(23_456));
        assert_eq!(VecUInt::from(23_456), 23_456);
        assert_eq!(23_456, VecUInt::from(23_456));

        assert_ne!(VecUInt::from(1), VecUInt::from(2));
        assert_ne!(VecUInt::from(23_456), VecUInt::from(13_456));
        assert_ne!(VecUInt::from(1_234), 4_321);
        assert_ne!(123, VecUInt::from(321));
    }

    #[test]
    fn comparison() {
        assert!(VecUInt::from(2) < VecUInt::from(3));
        assert!(VecUInt::from(3) > VecUInt::from(1));
        assert!(VecUInt::from(23_456) > VecUInt::from(123));
        assert!(VecUInt::from(23_456) > 13_456);
        assert!(123 > VecUInt::from(23));
    }

    #[test]
    fn assignment() {
        assert_eq!(VecUInt::from(2) + &VecUInt::from(2), VecUInt::from(4));
        assert_eq!(
            VecUInt::from(1_234) + &VecUInt::from(567_890),
            VecUInt::from(569_124)
        );
        assert_eq!(VecUInt::from(45) + 24, 69);
        assert_eq!(45 + VecUInt::from(24), 69);

        let mut num = VecUInt::from(34);
        num += 74;
        num += &VecUInt::from(456);
        assert_eq!(num, 564);
    }

    #[test]
    fn substract() {
        assert_eq!(VecUInt::from(4) - &VecUInt::from(2), VecUInt::from(2));
        assert_eq!(
            VecUInt::from(569_124) - &VecUInt::from(1_234),
            VecUInt::from(567_890)
        );
        assert_eq!(VecUInt::from(69) - 24, 45);
        assert_eq!(69 - &VecUInt::from(24), 45);
        assert_eq!(VecUInt::from(55) - 55, 0);

        let mut num = VecUInt::from(564);
        num -= 74;
        num -= &VecUInt::from(34);
        assert_eq!(num, 456);
    }

    #[test]
    fn divide() {
        assert_eq!(VecUInt::from(4) / VecUInt::from(2), 2);
        assert_eq!(VecUInt::from(10) / VecUInt::from(3), 3);
        assert_eq!(VecUInt::from(100) / 10, 10);
        assert_eq!(1000 / VecUInt::from(30), 33);

        assert_eq!(VecUInt::from(700_776_261) / VecUInt::from(1_234), 567_890);

        let mut num = VecUInt::from(100);
        num /= 2;
        num /= VecUInt::from(10);
        assert_eq!(num, 5);
    }

    #[test]
    fn multiply() {
        assert_eq!(VecUInt::from(2) * VecUInt::from(2), 4);
        assert_eq!(VecUInt::from(10) * 3, 30);
        assert_eq!(8 * VecUInt::from(100), 800);

        assert_eq!(VecUInt::from(1_234) * VecUInt::from(567_890), 700_776_260);

        let mut num = VecUInt::from(5);
        num *= 10;
        num *= VecUInt::from(2);
        assert_eq!(num, 100);
    }
}
