use soroban_sdk::{Env, U256};

use crate::soroban_fixed_point::SorobanFixedPoint;

impl SorobanFixedPoint for U256 {
    fn fixed_mul_floor(&self, _env: &Env, y: &U256, denominator: &U256) -> U256 {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(&self, env: &Env, y: &U256, denominator: &U256) -> U256 {
        mul_div_ceil(env, self, y, denominator)
    }

    fn fixed_div_floor(&self, _env: &Env, y: &U256, denominator: &U256) -> U256 {
        mul_div_floor(self, denominator, y)
    }

    fn fixed_div_ceil(&self, env: &Env, y: &U256, denominator: &U256) -> U256 {
        mul_div_ceil(env, self, denominator, y)
    }
}

/// Performs floor(x * y / z)
pub(crate) fn mul_div_floor(x: &U256, y: &U256, z: &U256) -> U256 {
    // floor taken by default
    x.mul(&y).div(&z)
}

/// Performs ceil(x * y / z)
pub(crate) fn mul_div_ceil(env: &Env, x: &U256, y: &U256, z: &U256) -> U256 {
    // floor taken by default
    // if there is any remainder, add 1 to round ceil
    let r = x.mul(&y);
    let remainder = r.rem_euclid(&z);
    let zero = U256::from_u32(env, 0);
    let one = U256::from_u32(env, 1);
    r.div(&z).add(if remainder > zero { &one } else { &zero })
}

#[cfg(test)]
mod tests {
    use soroban_sdk::Bytes;

    use super::*;

    /// Helper to create U256::MAX (all bits set to 1)
    fn u256_max(env: &Env) -> U256 {
        U256::from_be_bytes(env, &Bytes::from_array(env, &[0xFF; 32]))
    }

    /********** mul_div_floor **********/

    #[test]
    fn test_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u128(&env, 314_1592653);
        let z = U256::from_u128(&env, 1_0000001);

        let result = mul_div_floor(&x, &y, &z);

        assert_eq!(result, U256::from_u128(&env, 483_5313675));
    }

    #[test]
    fn test_mul_div_floor_exact() {
        // Real result = 12
        let env = Env::default();
        let x = U256::from_u32(&env, 8);
        let y = U256::from_u32(&env, 3);
        let z = U256::from_u32(&env, 2);

        let result = mul_div_floor(&x, &y, &z);

        assert_eq!(result, U256::from_u32(&env, 12));
    }

    #[test]
    fn test_mul_div_floor_mul_zero() {
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u128(&env, 0);
        let z = U256::from_u128(&env, 1_0000001);

        let result = mul_div_floor(&x, &y, &z);

        assert_eq!(result, U256::from_u128(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_floor_div_zero() {
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u128(&env, 314_1592653);
        let z = U256::from_u128(&env, 0);

        mul_div_floor(&x, &y, &z);
    }

    #[test]
    fn test_mul_div_floor_large_number() {
        let env = Env::default();
        let x = u256_max(&env).div(&U256::from_u128(&env, 10u128.pow(36)));
        let y = U256::from_u128(&env, 10u128.pow(36));
        let z = U256::from_u128(&env, 10u128.pow(36));

        let result = mul_div_floor(&x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_floor_small_number() {
        let env = Env::default();
        let x = U256::from_u32(&env, 1);
        let y = U256::from_u32(&env, 2);
        let z = U256::from_u32(&env, 3);

        let result = mul_div_floor(&x, &y, &z);

        assert_eq!(result, U256::from_u32(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_floor_overflow() {
        let env = Env::default();
        let x = u256_max(&env)
            .div(&U256::from_u128(&env, 10u128.pow(36)))
            .add(&U256::from_u32(&env, 1));
        let y = U256::from_u128(&env, 10u128.pow(36));
        let z = U256::from_u128(&env, 10u128.pow(36));

        mul_div_floor(&x, &y, &z);
    }

    /********** mul_div_ceil **********/

    #[test]
    fn test_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u128(&env, 314_1592653);
        let z = U256::from_u128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, U256::from_u128(&env, 483_5313676));
    }

    #[test]
    fn test_mul_div_ceil_exact() {
        // Real result = 12
        let env = Env::default();
        let x = U256::from_u32(&env, 8);
        let y = U256::from_u32(&env, 3);
        let z = U256::from_u32(&env, 2);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, U256::from_u32(&env, 12));
    }

    #[test]
    fn test_mul_div_ceil_mul_zero() {
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u32(&env, 0);
        let z = U256::from_u128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, U256::from_u32(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_ceil_div_zero() {
        let env = Env::default();
        let x = U256::from_u128(&env, 1_5391283);
        let y = U256::from_u128(&env, 314_1592653);
        let z = U256::from_u32(&env, 0);

        mul_div_ceil(&env, &x, &y, &z);
    }

    #[test]
    fn test_mul_div_ceil_large_number() {
        let env = Env::default();
        let x = u256_max(&env).div(&U256::from_u128(&env, 10u128.pow(36)));
        let y = U256::from_u128(&env, 10u128.pow(36));
        let z = U256::from_u128(&env, 10u128.pow(36));

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_ceil_small_number() {
        let env = Env::default();
        let x = U256::from_u32(&env, 1);
        let y = U256::from_u32(&env, 2);
        let z = U256::from_u32(&env, 3);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, U256::from_u32(&env, 1));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_ceil_overflow() {
        let env = Env::default();
        let x = u256_max(&env)
            .div(&U256::from_u128(&env, 10u128.pow(36)))
            .add(&U256::from_u32(&env, 1));
        let y = U256::from_u128(&env, 10u128.pow(36));
        let z = U256::from_u128(&env, 10u128.pow(36));

        mul_div_ceil(&env, &x, &y, &z);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x = U256::from_u128(&env, 3_1423141);
        let y = U256::from_u128(&env, 4_1234142);
        let denominator = U256::from_u128(&env, 0_1234567);

        let result = x.fixed_mul_floor(&env, &y, &denominator);
        assert_eq!(result, U256::from_u128(&env, 104_9522835));
    }

    #[test]
    fn test_fixed_mul_floor_large_number() {
        let env = Env::default();
        let x = U256::from_u128(&env, u128::MAX);
        let y = U256::from_u128(&env, 10u128.pow(38));
        let denominator = U256::from_u128(&env, 10u128.pow(18));

        let result = x.fixed_mul_floor(&env, &y, &denominator);

        let expected_result =
            U256::from_u128(&env, u128::MAX).mul(&U256::from_u128(&env, 10u128.pow(20)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_floor_panics() {
        let env = Env::default();
        let x = U256::from_u128(&env, 10u128.pow(7));
        let zero = U256::from_u32(&env, 0);

        x.fixed_mul_floor(&env, &x, &zero);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x = U256::from_u128(&env, 3_1423141);
        let y = U256::from_u128(&env, 4_1234142);
        let denominator = U256::from_u128(&env, 0_1234567);

        let result = x.fixed_mul_ceil(&env, &y, &denominator);
        assert_eq!(result, U256::from_u128(&env, 104_9522836));
    }

    #[test]
    fn test_fixed_mul_ceil_large_number() {
        let env = Env::default();
        let x = U256::from_u128(&env, u128::MAX);
        let y = U256::from_u128(&env, 10u128.pow(38));
        let denominator = U256::from_u128(&env, 10u128.pow(18));

        let result = x.fixed_mul_ceil(&env, &y, &denominator);

        let expected_result =
            U256::from_u128(&env, u128::MAX).mul(&U256::from_u128(&env, 10u128.pow(20)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_ceil_panics() {
        let env = Env::default();
        let x = U256::from_u128(&env, 10u128.pow(7));
        let zero = U256::from_u32(&env, 0);

        x.fixed_mul_ceil(&env, &x, &zero);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x = U256::from_u128(&env, 314_1592653);
        let y = U256::from_u128(&env, 1_5391280);
        let denominator = U256::from_u128(&env, 1_0000000);

        let result = x.fixed_div_floor(&env, &y, &denominator);
        assert_eq!(result, U256::from_u128(&env, 204_1150997));
    }

    #[test]
    fn test_fixed_div_floor_large_number() {
        let env = Env::default();
        let x = U256::from_u128(&env, u128::MAX);
        let y = U256::from_u128(&env, 10u128.pow(27));
        let denominator = U256::from_u128(&env, 10u128.pow(38));

        let result = x.fixed_div_floor(&env, &y, &denominator);

        let expected_result =
            U256::from_u128(&env, u128::MAX).mul(&U256::from_u128(&env, 10u128.pow(11)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_floor_panics() {
        let env = Env::default();
        let x = U256::from_u128(&env, 10u128.pow(7));
        let zero = U256::from_u32(&env, 0);

        x.fixed_div_floor(&env, &zero, &x);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x = U256::from_u128(&env, 314_1592653);
        let y = U256::from_u128(&env, 1_5391280);
        let denominator = U256::from_u128(&env, 1_0000000);

        let result = x.fixed_div_ceil(&env, &y, &denominator);
        assert_eq!(result, U256::from_u128(&env, 204_1150998));
    }

    #[test]
    fn test_fixed_div_ceil_large_number() {
        let env = Env::default();
        let x = U256::from_u128(&env, u128::MAX);
        let y = U256::from_u128(&env, 10u128.pow(27));
        let denominator = U256::from_u128(&env, 10u128.pow(38));

        let result = x.fixed_div_ceil(&env, &y, &denominator);

        let expected_result =
            U256::from_u128(&env, u128::MAX).mul(&U256::from_u128(&env, 10u128.pow(11)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_ceil_panics() {
        let env = Env::default();
        let x = U256::from_u128(&env, 10u128.pow(7));
        let zero = U256::from_u32(&env, 0);

        x.fixed_div_ceil(&env, &zero, &x);
    }
}
