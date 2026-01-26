use soroban_sdk::{Env, I256};

use crate::soroban_fixed_point::SorobanFixedPoint;

impl SorobanFixedPoint for I256 {
    fn fixed_mul_floor(&self, env: &Env, y: &I256, denominator: &I256) -> I256 {
        mul_div_floor(env, &self, y, denominator)
    }

    fn fixed_mul_ceil(&self, env: &Env, y: &I256, denominator: &I256) -> I256 {
        mul_div_ceil(env, &self, y, denominator)
    }

    fn fixed_div_floor(&self, env: &Env, y: &I256, denominator: &I256) -> I256 {
        mul_div_floor(env, &self, denominator, y)
    }

    fn fixed_div_ceil(&self, env: &Env, y: &I256, denominator: &I256) -> I256 {
        mul_div_ceil(env, &self, denominator, y)
    }
}

/// Performs floor(x * y / z)
pub(crate) fn mul_div_floor(env: &Env, x: &I256, y: &I256, z: &I256) -> I256 {
    let zero = I256::from_i32(env, 0);
    let r = x.mul(y);
    if (r < zero && z > &zero) || (r > zero && z < &zero) {
        // ceiling is taken by default for a negative result
        let remainder = r.rem_euclid(z);
        let one = I256::from_i32(env, 1);
        r.div(z).sub(if remainder > zero { &one } else { &zero })
    } else {
        // floor taken by default for a positive or zero result
        r.div(z)
    }
}

/// Performs ceil(x * y / z)
pub(crate) fn mul_div_ceil(env: &Env, x: &I256, y: &I256, z: &I256) -> I256 {
    let zero = I256::from_i32(env, 0);
    let r = x.mul(&y);
    if r == zero || (r < zero && z > &zero) || (r > zero && z < &zero) {
        // ceiling is taken by default for a negative or zero result
        r.div(&z)
    } else {
        // floor taken by default for a positive result
        let remainder = r.rem_euclid(&z);
        let one = I256::from_i32(env, 1);
        r.div(&z).add(if remainder > zero { &one } else { &zero })
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::Bytes;

    use super::*;

    /// Helper to create I256::MAX (0x7F followed by 31 0xFF bytes)
    fn i256_max(env: &Env) -> I256 {
        let mut bytes = [0xFFu8; 32];
        bytes[0] = 0x7F;
        I256::from_be_bytes(env, &Bytes::from_array(env, &bytes))
    }

    /// Helper to create I256::MIN (0x80 followed by 31 0x00 bytes)
    fn i256_min(env: &Env) -> I256 {
        let mut bytes = [0x00u8; 32];
        bytes[0] = 0x80;
        I256::from_be_bytes(env, &Bytes::from_array(env, &bytes))
    }

    /********** mul_div_floor **********/

    #[test]
    fn test_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313675));
    }

    #[test]
    fn test_mul_div_floor_exact() {
        // Real result = 12
        let env = Env::default();
        let x = I256::from_i32(&env, 8);
        let y = I256::from_i32(&env, 3);
        let z = I256::from_i32(&env, 2);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 12));
    }

    #[test]
    fn test_mul_div_floor_negative_exact() {
        // Real result = -12
        let env = Env::default();
        let x = I256::from_i32(&env, 8);
        let y = I256::from_i32(&env, -3);
        let z = I256::from_i32(&env, 2);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, -12));
    }

    #[test]
    fn test_mul_div_floor_mul_negative_rounds_down() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313676));
    }

    #[test]
    fn test_mul_div_floor_div_negative_rounds_down() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313676));
    }

    #[test]
    fn test_mul_div_floor_x_y_negative_rounds_down() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313675));
    }

    #[test]
    fn test_mul_div_floor_y_z_negative_rounds_down() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313675));
    }

    #[test]
    fn test_mul_div_floor_all_negative_rounds_down() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313676));
    }

    #[test]
    fn test_mul_div_floor_mul_zero() {
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 0);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_floor_div_zero() {
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, 0);

        mul_div_floor(&env, &x, &y, &z);
    }

    #[test]
    fn test_mul_div_floor_large_number() {
        let env = Env::default();
        let x = i256_max(&env).div(&I256::from_i128(&env, 10i128.pow(36)));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_floor_negative_large_number() {
        let env = Env::default();
        let x = i256_min(&env).div(&I256::from_i128(&env, 10i128.pow(36)));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_floor_small_number() {
        let env = Env::default();
        let x = I256::from_i32(&env, 1);
        let y = I256::from_i32(&env, 2);
        let z = I256::from_i32(&env, 3);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 0));
    }

    #[test]
    fn test_mul_div_floor_negative_small_number() {
        let env = Env::default();
        let x = I256::from_i32(&env, -1);
        let y = I256::from_i32(&env, 2);
        let z = I256::from_i32(&env, 3);

        let result = mul_div_floor(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, -1));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_floor_overflow() {
        let env = Env::default();
        let x = i256_max(&env)
            .div(&I256::from_i128(&env, 10i128.pow(36)))
            .add(&I256::from_i32(&env, 1));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        mul_div_floor(&env, &x, &y, &z);
    }

    #[test]
    #[should_panic]
    fn test_mul_div_floor_negative_overflow() {
        let env = Env::default();
        let x = i256_min(&env)
            .div(&I256::from_i128(&env, 10i128.pow(36)))
            .add(&I256::from_i32(&env, -1));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        mul_div_floor(&env, &x, &y, &z);
    }

    /********** mul_div_ceil **********/

    #[test]
    fn test_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313676));
    }

    #[test]
    fn test_mul_div_ceil_exact() {
        // Real result = 12
        let env = Env::default();
        let x = I256::from_i32(&env, 8);
        let y = I256::from_i32(&env, 3);
        let z = I256::from_i32(&env, 2);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 12));
    }

    #[test]
    fn test_mul_div_ceil_negative_exact() {
        // Real result = -12
        let env = Env::default();
        let x = I256::from_i32(&env, 8);
        let y = I256::from_i32(&env, -3);
        let z = I256::from_i32(&env, 2);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, -12));
    }

    #[test]
    fn test_mul_div_ceil_mul_negative_rounds_up() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313675));
    }

    #[test]
    fn test_mul_div_ceil_div_negative_rounds_up() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313675));
    }

    #[test]
    fn test_mul_div_ceil_x_y_negative_rounds_up() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313676));
    }

    #[test]
    fn test_mul_div_ceil_y_z_negative_rounds_up() {
        // Real result = 483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, 483_5313676));
    }

    #[test]
    fn test_mul_div_ceil_all_negative_rounds_up() {
        // Real result = -483_5313675.8
        let env = Env::default();
        let x = I256::from_i128(&env, -1_5391283);
        let y = I256::from_i128(&env, -314_1592653);
        let z = I256::from_i128(&env, -1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i128(&env, -483_5313675));
    }

    #[test]
    fn test_mul_div_ceil_mul_zero() {
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i32(&env, 0);
        let z = I256::from_i128(&env, 1_0000001);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_ceil_div_zero() {
        let env = Env::default();
        let x = I256::from_i128(&env, 1_5391283);
        let y = I256::from_i128(&env, 314_1592653);
        let z = I256::from_i32(&env, 0);

        mul_div_ceil(&env, &x, &y, &z);
    }

    #[test]
    fn test_mul_div_ceil_large_number() {
        let env = Env::default();
        let x = i256_max(&env).div(&I256::from_i128(&env, 10i128.pow(36)));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_ceil_negative_large_number() {
        let env = Env::default();
        let x = i256_min(&env).div(&I256::from_i128(&env, 10i128.pow(36)));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, x);
    }

    #[test]
    fn test_mul_div_ceil_small_number() {
        let env = Env::default();
        let x = I256::from_i32(&env, 1);
        let y = I256::from_i32(&env, 2);
        let z = I256::from_i32(&env, 3);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 1));
    }

    #[test]
    fn test_mul_div_ceil_negative_small_number() {
        let env = Env::default();
        let x = I256::from_i32(&env, -1);
        let y = I256::from_i32(&env, 2);
        let z = I256::from_i32(&env, 3);

        let result = mul_div_ceil(&env, &x, &y, &z);

        assert_eq!(result, I256::from_i32(&env, 0));
    }

    #[test]
    #[should_panic]
    fn test_mul_div_ceil_overflow() {
        let env = Env::default();
        let x = i256_max(&env)
            .div(&I256::from_i128(&env, 10i128.pow(36)))
            .add(&I256::from_i32(&env, 1));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        mul_div_ceil(&env, &x, &y, &z);
    }

    #[test]
    #[should_panic]
    fn test_mul_div_ceil_negative_overflow() {
        let env = Env::default();
        let x = i256_min(&env)
            .div(&I256::from_i128(&env, 10i128.pow(36)))
            .add(&I256::from_i32(&env, -1));
        let y = I256::from_i128(&env, 10i128.pow(36));
        let z = I256::from_i128(&env, 10i128.pow(36));

        mul_div_ceil(&env, &x, &y, &z);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x = I256::from_i128(&env, 3_1423141);
        let y = I256::from_i128(&env, 4_1234142);
        let denominator = I256::from_i128(&env, 0_1234567);

        let result = x.fixed_mul_floor(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, 104_9522835));

        let result = x
            .mul(&I256::from_i32(&env, -1))
            .fixed_mul_floor(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, -104_9522836));

        let result = x.mul(&I256::from_i32(&env, -1)).fixed_mul_floor(
            &env,
            &y.mul(&I256::from_i32(&env, -1)),
            &denominator,
        );
        assert_eq!(result, I256::from_i128(&env, 104_9522835));
    }

    #[test]
    fn test_fixed_mul_floor_large_number() {
        let env = Env::default();
        let x = I256::from_i128(&env, i128::MAX);
        let y = I256::from_i128(&env, 10i128.pow(38));
        let denominator = I256::from_i128(&env, 10i128.pow(18));

        let result = x.fixed_mul_floor(&env, &y, &denominator);

        let expected_result =
            I256::from_i128(&env, i128::MAX).mul(&I256::from_i128(&env, 10i128.pow(20)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_floor_panics() {
        let env = Env::default();
        let x = I256::from_i128(&env, 10i128.pow(7));
        let zero = I256::from_i32(&env, 0);

        x.fixed_mul_floor(&env, &x, &zero);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let env = Env::default();
        let x = I256::from_i128(&env, 3_1423141);
        let y = I256::from_i128(&env, 4_1234142);
        let denominator = I256::from_i128(&env, 0_1234567);

        let result = x.fixed_mul_ceil(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, 104_9522836));

        let result = x
            .mul(&I256::from_i32(&env, -1))
            .fixed_mul_ceil(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, -104_9522835));

        let result = x.mul(&I256::from_i32(&env, -1)).fixed_mul_ceil(
            &env,
            &y.mul(&I256::from_i32(&env, -1)),
            &denominator,
        );
        assert_eq!(result, I256::from_i128(&env, 104_9522836));
    }

    #[test]
    fn test_fixed_mul_ceil_large_number() {
        let env = Env::default();
        let x = I256::from_i128(&env, i128::MAX);
        let y = I256::from_i128(&env, 10i128.pow(38));
        let denominator = I256::from_i128(&env, 10i128.pow(18));

        let result = x.fixed_mul_ceil(&env, &y, &denominator);

        let expected_result =
            I256::from_i128(&env, i128::MAX).mul(&I256::from_i128(&env, 10i128.pow(20)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_mul_ceil_panics() {
        let env = Env::default();
        let x = I256::from_i128(&env, 10i128.pow(7));
        let zero = I256::from_i32(&env, 0);

        x.fixed_mul_ceil(&env, &x, &zero);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x = I256::from_i128(&env, 314_1592653);
        let y = I256::from_i128(&env, 1_5391280);
        let denominator = I256::from_i128(&env, 1_0000000);

        let result = x.fixed_div_floor(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, 204_1150997));

        let result = x
            .mul(&I256::from_i32(&env, -1))
            .fixed_div_floor(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, -204_1150998));

        let result = x.mul(&I256::from_i32(&env, -1)).fixed_div_floor(
            &env,
            &y.mul(&I256::from_i32(&env, -1)),
            &denominator,
        );
        assert_eq!(result, I256::from_i128(&env, 204_1150997));
    }

    #[test]
    fn test_fixed_div_floor_large_number() {
        let env = Env::default();
        let x = I256::from_i128(&env, i128::MAX);
        let y = I256::from_i128(&env, 10i128.pow(27));
        let denominator = I256::from_i128(&env, 10i128.pow(38));

        let result = x.fixed_div_floor(&env, &y, &denominator);

        let expected_result =
            I256::from_i128(&env, i128::MAX).mul(&I256::from_i128(&env, 10i128.pow(11)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_floor_panics() {
        let env = Env::default();
        let x = I256::from_i128(&env, 10i128.pow(7));
        let zero = I256::from_i32(&env, 0);

        x.fixed_div_floor(&env, &zero, &x);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let env = Env::default();
        let x = I256::from_i128(&env, 314_1592653);
        let y = I256::from_i128(&env, 1_5391280);
        let denominator = I256::from_i128(&env, 1_0000000);

        let result = x.fixed_div_ceil(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, 204_1150998));

        let result = x
            .mul(&I256::from_i32(&env, -1))
            .fixed_div_ceil(&env, &y, &denominator);
        assert_eq!(result, I256::from_i128(&env, -204_1150997));

        let result = x.mul(&I256::from_i32(&env, -1)).fixed_div_ceil(
            &env,
            &y.mul(&I256::from_i32(&env, -1)),
            &denominator,
        );
        assert_eq!(result, I256::from_i128(&env, 204_1150998));
    }

    #[test]
    fn test_fixed_div_ceil_large_number() {
        let env = Env::default();
        let x = I256::from_i128(&env, i128::MAX);
        let y = I256::from_i128(&env, 10i128.pow(27));
        let denominator = I256::from_i128(&env, 10i128.pow(38));

        let result = x.fixed_div_ceil(&env, &y, &denominator);

        let expected_result =
            I256::from_i128(&env, i128::MAX).mul(&I256::from_i128(&env, 10i128.pow(11)));
        assert_eq!(result, expected_result);
    }

    #[test]
    #[should_panic]
    fn test_fixed_div_ceil_panics() {
        let env = Env::default();
        let x = I256::from_i128(&env, 10i128.pow(7));
        let zero = I256::from_i32(&env, 0);

        x.fixed_div_ceil(&env, &zero, &x);
    }
}
