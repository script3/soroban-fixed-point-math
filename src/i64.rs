use crate::fixed_point::FixedPoint;

impl FixedPoint for i64 {
    fn fixed_mul_floor(self, y: i64, denominator: i64) -> Option<i64> {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(self, y: i64, denominator: i64) -> Option<i64> {
        mul_div_ceil(self, y, denominator)
    }

    fn fixed_div_floor(self, y: i64, denominator: i64) -> Option<i64> {
        mul_div_floor(self, denominator, y)
    }

    fn fixed_div_ceil(self, y: i64, denominator: i64) -> Option<i64> {
        mul_div_ceil(self, denominator, y)
    }
}

/// Performs floor(x * y / z)
fn mul_div_floor(x: i64, y: i64, z: i64) -> Option<i64> {
    return match x.checked_mul(y) {
        Some(r) => {
            if (r < 0 && z > 0) || (r > 0 && z < 0) {
                // ceiling is taken by default for a negative result
                // if there is any remainder, sub 1 to round floor
                let remainder = r.checked_rem_euclid(z)?;
                r.checked_div(z)?
                    .checked_sub(if remainder > 0 { 1 } else { 0 })
            } else {
                // floor taken by default for a positive or zero result
                r.checked_div(z)
            }
        }
        None => {
            let res_i128 = crate::i128::mul_div_floor(x as i128, y as i128, z as i128)?;
            if let Ok(res_i64) = i64::try_from(res_i128) {
                Some(res_i64)
            } else {
                None
            }
        }
    };
}

/// Performs ceil(x * y / z)
fn mul_div_ceil(x: i64, y: i64, z: i64) -> Option<i64> {
    return match x.checked_mul(y) {
        Some(r) => {
            if r == 0 || (r < 0 && z > 0) || (r > 0 && z < 0) {
                // ceiling is taken by default for a negative or zero result
                r.checked_div(z)
            } else {
                // floor taken by default for a positive result
                // if there is any remainder, add 1 to round ceiling
                let remainder = r.checked_rem_euclid(z)?;
                r.checked_div(z)?
                    .checked_add(if remainder > 0 { 1 } else { 0 })
            }
        }
        None => {
            let res_i128 = crate::i128::mul_div_ceil(x as i128, y as i128, z as i128)?;
            if let Ok(res_i64) = i64::try_from(res_i128) {
                Some(res_i64)
            } else {
                None
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use core::i64;

    use super::*;

    /********** mul_div_floor **********/

    #[test]
    fn test_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_mul_div_floor_exact() {
        // Real result = 12
        let x: i64 = 8;
        let y: i64 = 3;
        let z: i64 = 2;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_floor_negative_exact() {
        // Real result = -12
        let x: i64 = 8;
        let y: i64 = -3;
        let z: i64 = 2;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -12);
    }

    #[test]
    fn test_mul_div_floor_mul_negative_rounds_down() {
        // Real result = -483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -483_5313676);
    }

    #[test]
    fn test_mul_div_floor_div_negative_rounds_down() {
        // Real result = -483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -483_5313676);
    }

    #[test]
    fn test_mul_div_floor_x_y_negative_rounds_down() {
        // Real result = 483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_mul_div_floor_y_z_negative_rounds_down() {
        // Real result = 483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_mul_div_floor_all_negative_rounds_down() {
        // Real result = -483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -483_5313676);
    }

    #[test]
    fn test_mul_div_floor_mul_zero() {
        let x: i64 = 1_5391283;
        let y: i64 = 0;
        let z: i64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_floor_div_zero() {
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 0;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_floor_large_number() {
        let x: i64 = 9_223_372_036;
        let y: i64 = 1_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 9_223_372_036)
    }

    #[test]
    fn test_mul_div_floor_negative_large_number() {
        let x: i64 = -9_223_372_036;
        let y: i64 = 1_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -9_223_372_036)
    }

    #[test]
    fn test_mul_div_floor_small_number() {
        let x: i64 = 1;
        let y: i64 = 2;
        let z: i64 = 3;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0)
    }

    #[test]
    fn test_mul_div_floor_negative_small_number() {
        let x: i64 = -1;
        let y: i64 = 2;
        let z: i64 = 3;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, -1)
    }

    #[test]
    fn test_mul_div_floor_phantom_overflow_uses_i128() {
        // i64::MAX is odd
        let x: i64 = (i64::MAX - 1) / 2;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, i64::MAX - 1);
    }

    #[test]
    fn test_mul_div_floor_negative_phantom_overflow_uses_i128() {
        let x: i64 = i64::MIN / 2;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, i64::MIN);
    }

    #[test]
    fn test_mul_div_floor_phantom_overflow_rounds_down() {
        // Real Result = 333_333_333_333.3..
        let x: i64 = 100 * 10i64.pow(9);
        let y: i64 = 10 * 10i64.pow(9);
        let z: i64 = 3 * 10i64.pow(9);

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 333_333_333_333);
    }

    #[test]
    fn test_mul_div_floor_result_overflow() {
        // i64::MAX is odd
        let x: i64 = (i64::MAX - 1) / 2 + 1;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_floor_result_negative_overflow() {
        let x: i64 = i64::MIN / 2 - 1;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    /********** mul_div_ceil **********/

    #[test]
    fn test_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_mul_div_ceil_exact() {
        // Real result = 12
        let x: i64 = 8;
        let y: i64 = 3;
        let z: i64 = 2;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_ceil_negative_exact() {
        // Real result = -12
        let x: i64 = 8;
        let y: i64 = -3;
        let z: i64 = 2;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, -12);
    }

    #[test]
    fn test_mul_div_ceil_mul_negative_rounds_up() {
        // Real result = -483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, -483_5313675);
    }

    #[test]
    fn test_mul_div_ceil_div_negative_rounds_up() {
        // Real result = -483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, -483_5313675);
    }

    #[test]
    fn test_mul_div_ceil_x_y_negative_rounds_up() {
        // Real result = 483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_mul_div_ceil_y_z_negative_rounds_up() {
        // Real result = 483_5313675.8
        let x: i64 = 1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_mul_div_ceil_all_negative_rounds_up() {
        // Real result = -483_5313675.8
        let x: i64 = -1_5391283;
        let y: i64 = -314_1592653;
        let z: i64 = -1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, -483_5313675);
    }

    #[test]
    fn test_mul_div_ceil_mul_zero() {
        let x: i64 = 1_5391283;
        let y: i64 = 0;
        let z: i64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_ceil_div_zero() {
        let x: i64 = 1_5391283;
        let y: i64 = 314_1592653;
        let z: i64 = 0;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_ceil_large_number() {
        let x: i64 = 9_223_372_036;
        let y: i64 = 1_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 9_223_372_036)
    }

    #[test]
    fn test_mul_div_ceil_negative_large_number() {
        let x: i64 = -9_223_372_036;
        let y: i64 = 1_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, -9_223_372_036)
    }

    #[test]
    fn test_mul_div_ceil_small_number() {
        let x: i64 = 1;
        let y: i64 = 2;
        let z: i64 = 3;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 1)
    }

    #[test]
    fn test_mul_div_ceil_negative_small_number() {
        let x: i64 = -1;
        let y: i64 = 2;
        let z: i64 = 3;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 0)
    }

    #[test]
    fn test_mul_div_ceil_phantom_overflow_uses_i128() {
        // i64::MAX is odd
        let x: i64 = (i64::MAX - 1) / 2;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, i64::MAX - 1);
    }

    #[test]
    fn test_mul_div_ceil_negative_phantom_overflow_uses_i128() {
        let x: i64 = i64::MIN / 2;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, i64::MIN);
    }

    #[test]
    fn test_mul_div_ceil_phantom_overflow_rounds_up() {
        // Real Result = 333_333_333_333.3..
        let x: i64 = 100 * 10i64.pow(9);
        let y: i64 = 10 * 10i64.pow(9);
        let z: i64 = 3 * 10i64.pow(9);

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 333_333_333_334);
    }

    #[test]
    fn test_mul_div_ceil_result_overflow() {
        // i64::MAX is odd
        let x: i64 = (i64::MAX - 1) / 2 + 1;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_ceil_result_negative_overflow() {
        let x: i64 = i64::MIN / 2 - 1;
        let y: i64 = 2_000_000_000;
        let z: i64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let x: i64 = 3_1423141;
        let y: i64 = 4_1234142;
        let denominator: i64 = 0_1234567;

        let result = x.fixed_mul_floor(y, denominator).unwrap();
        assert_eq!(result, 104_9522835);

        let result = (-x).fixed_mul_floor(y, denominator).unwrap();
        assert_eq!(result, -104_9522836);

        let result = (-x).fixed_mul_floor(-y, denominator).unwrap();
        assert_eq!(result, 104_9522835);

        let invalid = x.fixed_mul_floor(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let x: i64 = 3_1423141;
        let y: i64 = 4_1234142;
        let denominator: i64 = 0_1234567;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();
        assert_eq!(result, 104_9522836);

        let result = (-x).fixed_mul_ceil(y, denominator).unwrap();
        assert_eq!(result, -104_9522835);

        let result = (-x).fixed_mul_ceil(-y, denominator).unwrap();
        assert_eq!(result, 104_9522836);

        let invalid = x.fixed_mul_ceil(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let x: i64 = 314_1592653;
        let y: i64 = 1_5391280;
        let denominator: i64 = 1_0000000;

        let result = x.fixed_div_floor(y, denominator).unwrap();
        assert_eq!(result, 204_1150997);

        let result = (-x).fixed_div_floor(y, denominator).unwrap();
        assert_eq!(result, -204_1150998);

        let result = (-x).fixed_div_floor(-y, denominator).unwrap();
        assert_eq!(result, 204_1150997);

        let invalid = x.fixed_div_floor(0, denominator);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let x: i64 = 314_1592653;
        let y: i64 = 1_5391280;
        let denominator: i64 = 1_0000000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();
        assert_eq!(result, 204_1150998);

        let result = (-x).fixed_div_ceil(y, denominator).unwrap();
        assert_eq!(result, -204_1150997);

        let result = (-x).fixed_div_ceil(-y, denominator).unwrap();
        assert_eq!(result, 204_1150998);

        let invalid = x.fixed_div_ceil(0, denominator);
        assert_eq!(invalid, None);
    }
}
