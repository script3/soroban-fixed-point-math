use soroban_sdk::{unwrap::UnwrapOptimized, Env, U256};

use crate::{fixed_point::FixedPoint, SorobanFixedPoint};

impl FixedPoint for u128 {
    fn fixed_mul_floor(self, y: u128, denominator: u128) -> Option<u128> {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(self, y: u128, denominator: u128) -> Option<u128> {
        mul_div_ceil(self, y, denominator)
    }

    fn fixed_div_floor(self, y: u128, denominator: u128) -> Option<u128> {
        mul_div_floor(self, denominator, y)
    }

    fn fixed_div_ceil(self, y: u128, denominator: u128) -> Option<u128> {
        mul_div_ceil(self, denominator, y)
    }
}

/// Performs floor(x * y / z)
pub(crate) fn mul_div_floor(x: u128, y: u128, z: u128) -> Option<u128> {
    let r = x.checked_mul(y)?;
    r.checked_div(z)
}

/// Performs ceil(x * y / z)
pub(crate) fn mul_div_ceil(x: u128, y: u128, z: u128) -> Option<u128> {
    let r = x.checked_mul(y)?;
    div_ceil(r, z)
}

/// Performs ceil(r / z)
fn div_ceil(r: u128, z: u128) -> Option<u128> {
    // floor taken by default for a positive result
    // if there is any remainder, add 1 to round ceil
    let remainder = r.checked_rem_euclid(z)?;
    r.checked_div(z)?
        .checked_add(if remainder > 0 { 1 } else { 0 })
}

impl SorobanFixedPoint for u128 {
    fn fixed_mul_floor(&self, env: &Env, y: &u128, denominator: &u128) -> u128 {
        scaled_mul_div_floor(self, env, y, denominator)
    }

    fn fixed_mul_ceil(&self, env: &Env, y: &u128, denominator: &u128) -> u128 {
        scaled_mul_div_ceil(&self, env, y, denominator)
    }

    fn fixed_div_floor(&self, env: &Env, y: &u128, denominator: &u128) -> u128 {
        scaled_mul_div_floor(self, env, denominator, y)
    }

    fn fixed_div_ceil(&self, env: &Env, y: &u128, denominator: &u128) -> u128 {
        scaled_mul_div_ceil(self, env, denominator, y)
    }
}

/// Performs floor(x * y / z)
fn scaled_mul_div_floor(x: &u128, env: &Env, y: &u128, z: &u128) -> u128 {
    return match x.checked_mul(*y) {
        Some(r) => r.checked_div(*z).unwrap_optimized(),
        None => {
            // scale to U256 and retry
            let res = crate::u256::mul_div_floor(
                &U256::from_u128(&env, *x),
                &U256::from_u128(&env, *y),
                &U256::from_u128(&env, *z),
            );
            // will panic if result is not representable in u128
            res.to_u128().unwrap_optimized()
        }
    };
}

/// Performs floor(x * y / z)
fn scaled_mul_div_ceil(x: &u128, env: &Env, y: &u128, z: &u128) -> u128 {
    return match x.checked_mul(*y) {
        Some(r) => div_ceil(r, *z).unwrap_optimized(),
        None => {
            // scale to U256 and retry
            let res = crate::u256::mul_div_ceil(
                &env,
                &U256::from_u128(&env, *x),
                &U256::from_u128(&env, *y),
                &U256::from_u128(&env, *z),
            );
            // will panic if result is not representable in u128
            res.to_u128().unwrap_optimized()
        }
    };
}

#[cfg(test)]
mod test_fixed_point {
    use super::{mul_div_ceil, mul_div_floor};
    use crate::FixedPoint;

    /********** mul_div_floor **********/

    #[test]
    fn test_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_mul_div_floor_exact() {
        // Real result = 12
        let x: u128 = 8;
        let y: u128 = 3;
        let z: u128 = 2;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_floor_mul_zero() {
        let x: u128 = 1_5391283;
        let y: u128 = 0;
        let z: u128 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_floor_div_zero() {
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 0;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_floor_large_number() {
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_000;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 340_282_366_920_938_463_463);
    }

    #[test]
    fn test_mul_div_floor_small_number() {
        let x: u128 = 1;
        let y: u128 = 2;
        let z: u128 = 3;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_floor_phantom_overflow() {
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_001;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    /********** mul_div_ceil **********/

    #[test]
    fn test_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_mul_div_ceil_exact() {
        // Real result = 12
        let x: u128 = 8;
        let y: u128 = 3;
        let z: u128 = 2;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_ceil_mul_zero() {
        let x: u128 = 1_5391283;
        let y: u128 = 0;
        let z: u128 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_ceil_div_zero() {
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 0;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_ceil_large_number() {
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_000;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 340_282_366_920_938_463_463);
    }

    #[test]
    fn test_mul_div_ceil_small_number() {
        let x: u128 = 1;
        let y: u128 = 2;
        let z: u128 = 3;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_mul_div_ceil_phantom_overflow() {
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_001;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let x: u128 = 3_1423141;
        let y: u128 = 4_1234142;
        let denominator: u128 = 0_1234567;

        let result = x.fixed_mul_floor(y, denominator).unwrap();
        assert_eq!(result, 104_9522835);

        let invalid = x.fixed_mul_floor(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let x: u128 = 3_1423141;
        let y: u128 = 4_1234142;
        let denominator: u128 = 0_1234567;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();
        assert_eq!(result, 104_9522836);

        let invalid = x.fixed_mul_ceil(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let x: u128 = 314_1592653;
        let y: u128 = 1_5391280;
        let denominator: u128 = 1_0000000;

        let result = x.fixed_div_floor(y, denominator).unwrap();
        assert_eq!(result, 204_1150997);

        let invalid = x.fixed_div_floor(0, denominator);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let x: u128 = 314_1592653;
        let y: u128 = 1_5391280;
        let denominator: u128 = 1_0000000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();
        assert_eq!(result, 204_1150998);

        let invalid = x.fixed_div_ceil(0, denominator);
        assert_eq!(invalid, None);
    }
}

#[cfg(test)]
mod test_soroban_fixed_point {
    use super::{scaled_mul_div_ceil, scaled_mul_div_floor};
    use crate::SorobanFixedPoint;
    use soroban_sdk::Env;

    /********** scaled_mul_div_floor **********/

    #[test]
    fn test_scaled_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 1_0000001;

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_scaled_mul_div_floor_exact() {
        // Real result = 12
        let env = Env::default();
        let x: u128 = 8;
        let y: u128 = 3;
        let z: u128 = 2;

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 12);
    }

    #[test]
    fn test_scaled_mul_div_floor_mul_zero() {
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 0;
        let z: u128 = 1_0000001;

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 0);
    }

    #[test]
    #[should_panic]
    fn test_scaled_mul_div_floor_div_zero() {
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 0;

        scaled_mul_div_floor(&x, &env, &y, &z);
    }

    #[test]
    fn test_scaled_mul_div_floor_large_number() {
        let env = Env::default();
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_000;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 340_282_366_920_938_463_463);
    }

    #[test]
    fn test_scaled_mul_div_floor_small_number() {
        let env = Env::default();
        let x: u128 = 1;
        let y: u128 = 2;
        let z: u128 = 3;

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_scaled_mul_div_floor_phantom_overflow_uses_u256() {
        // u128::MAX is odd
        let env = Env::default();
        let x: u128 = (u128::MAX - 1) / 2;
        let y: u128 = 2 * 10u128.pow(18);
        let z: u128 = 10u128.pow(18);

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, u128::MAX - 1);
    }

    #[test]
    fn test_scaled_mul_div_floor_phantom_overflow_rounds_down() {
        // Real Result = 333_3x18.3..
        let env = Env::default();
        let x: u128 = 100 * 10u128.pow(18);
        let y: u128 = 10 * 10u128.pow(18);
        let z: u128 = 3 * 10u128.pow(18);

        let result = scaled_mul_div_floor(&x, &env, &y, &z);

        assert_eq!(result, 333_333_333_333_333_333_333);
    }

    #[test]
    #[should_panic]
    fn test_scaled_mul_div_floor_result_overflow() {
        // u128::MAX is odd
        let env = Env::default();
        let x: u128 = (u128::MAX - 1) / 2 + 1;
        let y: u128 = 2 * 10u128.pow(18);
        let z: u128 = 10u128.pow(18);

        scaled_mul_div_floor(&x, &env, &y, &z);
    }

    /********** scaled_mul_div_ceil **********/

    #[test]
    fn test_scaled_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 1_0000001;

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_scaled_mul_div_ceil_exact() {
        // Real result = 12
        let env = Env::default();
        let x: u128 = 8;
        let y: u128 = 3;
        let z: u128 = 2;

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 12);
    }

    #[test]
    fn test_scaled_mul_div_ceil_mul_zero() {
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 0;
        let z: u128 = 1_0000001;

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 0);
    }

    #[test]
    #[should_panic]
    fn test_scaled_mul_div_ceil_div_zero() {
        let env = Env::default();
        let x: u128 = 1_5391283;
        let y: u128 = 314_1592653;
        let z: u128 = 0;

        scaled_mul_div_ceil(&x, &env, &y, &z);
    }

    #[test]
    fn test_scaled_mul_div_ceil_large_number() {
        let env = Env::default();
        let x: u128 = 340_282_366_920_938_463_463;
        let y: u128 = 1_000_000_000_000_000_000;
        let z: u128 = 1_000_000_000_000_000_000;

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 340_282_366_920_938_463_463);
    }

    #[test]
    fn test_scaled_mul_div_ceil_small_number() {
        let env = Env::default();
        let x: u128 = 1;
        let y: u128 = 2;
        let z: u128 = 3;

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_scaled_mul_div_ceil_phantom_overflow_uses_u256() {
        // u128::MAX is odd
        let env = Env::default();
        let x: u128 = (u128::MAX - 1) / 2;
        let y: u128 = 2 * 10u128.pow(18);
        let z: u128 = 10u128.pow(18);

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, u128::MAX - 1);
    }

    #[test]
    fn test_scaled_mul_div_ceil_phantom_overflow_rounds_up() {
        // Real Result = 333_3x18.3..
        let env = Env::default();
        let x: u128 = 100 * 10u128.pow(18);
        let y: u128 = 10 * 10u128.pow(18);
        let z: u128 = 3 * 10u128.pow(18);

        let result = scaled_mul_div_ceil(&x, &env, &y, &z);

        assert_eq!(result, 333_333_333_333_333_333_334);
    }

    #[test]
    #[should_panic]
    fn test_scaled_mul_div_ceil_result_overflow() {
        // u128::MAX is odd
        let env = Env::default();
        let x: u128 = (u128::MAX - 1) / 2 + 1;
        let y: u128 = 2 * 10u128.pow(18);
        let z: u128 = 10u128.pow(18);

        scaled_mul_div_ceil(&x, &env, &y, &z);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x: u128 = 3_1423141;
        let y: u128 = 4_1234142;
        let denominator: u128 = 0_1234567;

        let result = x.fixed_mul_floor(&env, &y, &denominator);
        assert_eq!(result, 104_9522835);
    }

    #[test]
    fn test_fixed_mul_floor_uses_u256() {
        // Real result = 246800e18
        let env = Env::default();
        let x: u128 = 1234 * 10u128.pow(18);
        let y: u128 = 200 * 10u128.pow(18);
        let denominator: u128 = 10u128.pow(18);

        let result = x.fixed_mul_floor(&env, &y, &denominator);
        assert_eq!(result, 246800 * 10u128.pow(18));
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_floor_panics() {
        let env = Env::default();
        let x: u128 = 10u128.pow(7);

        x.fixed_mul_floor(&env, &x, &0);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x: u128 = 3_1423141;
        let y: u128 = 4_1234142;
        let denominator: u128 = 0_1234567;

        let result = x.fixed_mul_ceil(&env, &y, &denominator);
        assert_eq!(result, 104_9522836);
    }

    #[test]
    fn test_fixed_mul_ceil_uses_u256() {
        // Real result = 246800e18
        let env = Env::default();
        let x: u128 = 1234 * 10u128.pow(18);
        let y: u128 = 200 * 10u128.pow(18);
        let denominator: u128 = 10u128.pow(18);

        let result = x.fixed_mul_ceil(&env, &y, &denominator);
        assert_eq!(result, 246800 * 10u128.pow(18));
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_ceil_panics() {
        let env = Env::default();
        let x: u128 = 10u128.pow(7);

        x.fixed_mul_ceil(&env, &x, &0);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x: u128 = 314_1592653;
        let y: u128 = 1_5391280;
        let denominator: u128 = 1_0000000;

        let result = x.fixed_div_floor(&env, &y, &denominator);
        assert_eq!(result, 204_1150997);
    }

    #[test]
    fn test_fixed_div_floor_uses_u256() {
        // Real result = 5000e18
        let env = Env::default();
        let x: u128 = 1000 * 10u128.pow(18);
        let y: u128 = 2 * 10u128.pow(17);
        let denominator: u128 = 10u128.pow(18);

        let result = x.fixed_div_floor(&env, &y, &denominator);
        assert_eq!(result, 5000 * 10u128.pow(18));
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_floor_panics() {
        let env = Env::default();
        let x: u128 = 10u128.pow(7);

        x.fixed_div_floor(&env, &0, &x);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x: u128 = 314_1592653;
        let y: u128 = 1_5391280;
        let denominator: u128 = 1_0000000;

        let result = x.fixed_div_ceil(&env, &y, &denominator);
        assert_eq!(result, 204_1150998);
    }

    #[test]
    fn test_fixed_div_ceil_uses_u256() {
        // Real result = 5000e18
        let env = Env::default();
        let x: u128 = 1000 * 10u128.pow(18);
        let y: u128 = 2 * 10u128.pow(17);
        let denominator: u128 = 10u128.pow(18);

        let result = x.fixed_div_ceil(&env, &y, &denominator);
        assert_eq!(result, 5000 * 10u128.pow(18));
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_ceil_panics() {
        let env = Env::default();
        let x: u128 = 10u128.pow(7);

        x.fixed_div_ceil(&env, &0, &x);
    }
}
