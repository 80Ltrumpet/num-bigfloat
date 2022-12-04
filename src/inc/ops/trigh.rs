//! Hyperbolic trigonometric functions and inverse hyperbolic trigonometric functions.

use crate::defs::Error;
use crate::defs::DECIMAL_SIGN_NEG;
use crate::defs::DECIMAL_SIGN_POS;
use crate::inc::inc::BigFloatInc;
use crate::inc::inc::DECIMAL_POSITIONS;
use crate::inc::ops::tables::asinh_const::ASINH_VALUES;
use crate::inc::ops::tables::fact_const::INVFACT_VALUES;
use crate::inc::ops::tables::tanh_const::TANH_VALUES;

const ONE_HALF: BigFloatInc = BigFloatInc {
    m: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5000],
    n: DECIMAL_POSITIONS as i16,
    sign: DECIMAL_SIGN_POS,
    e: -(DECIMAL_POSITIONS as i8),
};

const LN_OF_2: BigFloatInc = BigFloatInc {
    m: [13, 755, 6568, 5817, 1214, 7232, 941, 9453, 559, 4718, 6931],
    n: DECIMAL_POSITIONS as i16,
    sign: DECIMAL_SIGN_POS,
    e: -(DECIMAL_POSITIONS as i8),
};

impl BigFloatInc {
    /// Returns hyperbolic sine of a number.
    ///
    /// # Errors
    ///
    /// ExponentOverflow - when result is too big.
    pub fn sinh(&self) -> Result<Self, Error> {
        if self.e as i16 + self.n <= -1 {
            // sinh(x) = x + x^3/3! + x^5/5! + ...
            let mut ret = *self;
            let dxx = self.mul(self)?;
            let mut dx = ret;
            for i in 1..INVFACT_VALUES.len() / 2 {
                dx = dx.mul(&dxx)?;
                let p = dx.mul(&INVFACT_VALUES[i * 2])?;
                let val = ret.add(&p)?;
                if val.cmp(&ret) == 0 {
                    break;
                }
                ret = val;
            }
            Ok(ret)
        } else {
            // 0.5*(e^x - e^-x)
            let e_x1 =
                if self.sign == DECIMAL_SIGN_NEG { self.inv_sign().exp() } else { self.exp() }
                    .map_err(|e| {
                        if let Error::ExponentOverflow(_) = e {
                            Error::ExponentOverflow(self.sign)
                        } else {
                            e
                        }
                    })?;
            let e_x2 = Self::one().div(&e_x1)?;
            let mut ret = e_x1.sub(&e_x2)?.mul(&ONE_HALF)?;
            ret.sign = self.sign;
            Ok(ret)
        }
    }

    /// Returns hyperbolic cosine of a number.
    ///
    /// # Errors
    ///
    /// ExponentOverflow - when result is too big.
    pub fn cosh(&self) -> Result<Self, Error> {
        if self.e as i16 + self.n <= -1 {
            // cosh(x) = 1 + x^2/2! + x^4/4! + ...
            let one = Self::one();
            let mut ret = one;
            let dxx = self.mul(self)?;
            let mut dx = dxx;
            for i in 0..INVFACT_VALUES.len() / 2 - 1 {
                let p = dx.mul(&INVFACT_VALUES[i * 2 + 1])?;
                let val = ret.add(&p)?;
                if val.cmp(&ret) == 0 {
                    break;
                }
                ret = val;
                dx = dx.mul(&dxx)?;
            }
            Ok(ret)
        } else {
            // 0.5*(e^x + e^-x)
            let e_x1 =
                if self.sign == DECIMAL_SIGN_NEG { self.inv_sign().exp() } else { self.exp() }?;
            let e_x2 = Self::one().div(&e_x1)?;
            e_x1.add(&e_x2)?.mul(&ONE_HALF)
        }
    }

    /// Returns hyperbolic tangent of a number.
    ///
    /// # Errors
    ///
    /// ExponentOverflow - when result is too big.
    pub fn tanh(&self) -> Result<Self, Error> {
        if self.e as i16 + self.n < -1 {
            // tanh series
            let mut ret = *self;
            let dxx = self.mul(self)?;
            let mut dx = *self;
            for pcoef in TANH_VALUES {
                dx = dx.mul(&dxx)?;
                let p = pcoef.mul(&dx)?;
                let val = ret.add(&p)?;
                if val.cmp(&ret) == 0 {
                    break;
                }
                ret = val;
            }
            Ok(ret)
        } else if self.n + self.e as i16 > 2 {
            // int part of x has at least 3 digits
            // this is not the best estimate for optimisation, but it is simple
            Ok(if self.sign == DECIMAL_SIGN_NEG { Self::one().inv_sign() } else { Self::one() })
        } else {
            // (e^x - e^-x) / (e^x + e^-x)
            let e_x1 =
                if self.sign == DECIMAL_SIGN_NEG { self.inv_sign().exp() } else { self.exp() }?;
            let e_x2 = Self::one().div(&e_x1)?;
            let mut ret = e_x1.sub(&e_x2)?.div(&e_x1.add(&e_x2)?)?;
            ret.sign = self.sign;
            Ok(ret)
        }
    }

    /// Returns inverse hyperbolic sine of a number.
    ///
    /// # Errors
    ///
    /// ExponentOverflow - when result is too big.
    pub fn asinh(&self) -> Result<Self, Error> {
        if self.e as i16 + self.n < 0 {
            // asinh series
            let mut ret = *self;
            let dxx = self.mul(self)?;
            let mut dx = *self;
            for pcoef in ASINH_VALUES {
                dx = dx.mul(&dxx)?;
                let p = pcoef.mul(&dx)?;
                let val = ret.add(&p)?;
                if val.cmp(&ret) == 0 {
                    break;
                }
                ret = val;
            }
            Ok(ret)
        } else if (self.e as i16 + self.n - 1) * 2 >= DECIMAL_POSITIONS as i16 {
            // sign(x)*(ln(|x|) + ln(2))
            let x = self.abs();
            let mut ret = x.ln()?.add(&LN_OF_2)?;
            ret.sign = self.sign;
            Ok(ret)
        } else {
            // sign(x)*ln(|x| + sqrt(x^2 + 1))
            let x = self.abs();
            let xx = x.mul(&x)?;
            let xx1 = xx.add(&Self::one())?;
            let sq = xx1.sqrt()?;
            let arg = x.add(&sq)?;
            if arg.n == 0 {
                Err(Error::ExponentOverflow(DECIMAL_SIGN_NEG))
            } else {
                let mut ret = arg.ln()?;
                ret.sign = self.sign;
                Ok(ret)
            }
        }
    }

    /// Returns inverse hyperbolic cosine of a number.
    ///
    /// # Errors
    ///
    /// ExponentOverflow - when result is too big.
    /// InvalidArgument - when `self` is less than 1.
    pub fn acosh(&self) -> Result<Self, Error> {
        let x = *self;
        let one = Self::one();
        if x.cmp(&one) < 0 {
            return Err(Error::InvalidArgument);
        }
        if (x.e as i16 + x.n - 1) * 2 >= DECIMAL_POSITIONS as i16 {
            // ln(x) + ln(2)
            x.ln()?.add(&LN_OF_2)
        } else {
            // ln(x + sqrt(x^2 - 1))
            let xx = x.mul(&x)?;
            let arg = x.add(&xx.sub(&one)?.sqrt()?)?;
            arg.ln()
        }
    }

    /// Returns inverse hyperbolic tangent of a number.
    ///
    /// # Errors
    ///
    /// InvalidArgument - when |`self`| > 1.
    pub fn atanh(&self) -> Result<Self, Error> {
        if self.e as i16 + self.n < -1 {
            // atanh(x) = x + x^3/3 + x^5/5 + ...
            let two = Self::two();
            let mut ret = *self;
            let dxx = self.mul(self)?;
            let mut dx = *self;
            let mut factor = Self::one();
            loop {
                dx = dx.mul(&dxx)?;
                factor = factor.add(&two)?;
                let p = dx.div(&factor)?;
                let val = ret.add(&p)?;
                if val.cmp(&ret) == 0 {
                    break;
                }
                ret = val;
            }
            Ok(ret)
        } else {
            // 0.5 * ln( (1+x) / (1-x) )
            let x = *self;
            let one = Self::one();
            let cmp_result = x.abs().cmp(&one);
            if cmp_result > 0 {
                Err(Error::InvalidArgument)
            } else if cmp_result == 0 {
                Err(Error::ExponentOverflow(x.sign))
            } else {
                let arg = one.add(&x)?.div(&one.sub(&x)?)?;
                ONE_HALF.mul(&arg.ln()?)
            }
        }
    }
}
