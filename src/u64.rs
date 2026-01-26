use crate::fixed_point::FixedPoint;

impl FixedPoint for u64 {
    fn fixed_mul_floor(self, y: u64, denominator: u64) -> Option<u64> {
        mul_div_floor(self, y, denominator)
    }

    fn fixed_mul_ceil(self, y: u64, denominator: u64) -> Option<u64> {
        mul_div_ceil(self, y, denominator)
    }

    fn fixed_div_floor(self, y: u64, denominator: u64) -> Option<u64> {
        mul_div_floor(self, denominator, y)
    }

    fn fixed_div_ceil(self, y: u64, denominator: u64) -> Option<u64> {
        mul_div_ceil(self, denominator, y)
    }
}

/// Performs floor(x * y / z)
fn mul_div_floor(x: u64, y: u64, z: u64) -> Option<u64> {
    return match x.checked_mul(y) {
        Some(r) => r.checked_div(z),
        None => {
            let res_u128 = crate::u128::mul_div_floor(x as u128, y as u128, z as u128)?;
            if let Ok(res_u64) = u64::try_from(res_u128) {
                Some(res_u64)
            } else {
                None
            }
        }
    };
}

/// Performs ceil(x * y / z)
fn mul_div_ceil(x: u64, y: u64, z: u64) -> Option<u64> {
    return match x.checked_mul(y) {
        Some(r) => {
            let remainder = r.checked_rem_euclid(z)?;
            r.checked_div(z)?
                .checked_add(if remainder > 0 { 1 } else { 0 })
        }
        None => {
            let res_u128 = crate::u128::mul_div_ceil(x as u128, y as u128, z as u128)?;
            if let Ok(res_u64) = u64::try_from(res_u128) {
                Some(res_u64)
            } else {
                None
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /********** mul_div_floor **********/

    #[test]
    fn test_mul_div_floor_rounds_down() {
        // Real result = 483_5313675.8
        let x: u64 = 1_5391283;
        let y: u64 = 314_1592653;
        let z: u64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 483_5313675);
    }

    #[test]
    fn test_mul_div_floor_exact() {
        // Real result = 12
        let x: u64 = 8;
        let y: u64 = 3;
        let z: u64 = 2;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_floor_mul_zero() {
        let x: u64 = 1_5391283;
        let y: u64 = 0;
        let z: u64 = 1_0000001;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_floor_div_zero() {
        let x: u64 = 1_5391283;
        let y: u64 = 314_1592653;
        let z: u64 = 0;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_floor_large_number() {
        let x: u64 = 18_446_744_073;
        let y: u64 = 1_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 18_446_744_073);
    }

    #[test]
    fn test_mul_div_floor_small_number() {
        let x: u64 = 1;
        let y: u64 = 2;
        let z: u64 = 3;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_floor_phantom_overflow_uses_u128() {
        // u64::MAX is odd
        let x: u64 = (u64::MAX - 1) / 2;
        let y: u64 = 2_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, u64::MAX - 1);
    }

    #[test]
    fn test_mul_div_floor_phantom_overflow_rounds_up() {
        // Real Result = 333_333_333_333.3..
        let x: u64 = 100 * 10u64.pow(9);
        let y: u64 = 10 * 10u64.pow(9);
        let z: u64 = 3 * 10u64.pow(9);

        let result = mul_div_floor(x, y, z).unwrap();

        assert_eq!(result, 333_333_333_333);
    }

    #[test]
    fn test_mul_div_floor_result_overflow() {
        // u64::MAX is odd
        let x: u64 = (u64::MAX - 1) / 2 + 1;
        let y: u64 = 2_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_floor(x, y, z);

        assert_eq!(result, None);
    }

    /********** mul_div_ceil **********/

    #[test]
    fn test_mul_div_ceil_rounds_up() {
        // Real result = 483_5313675.8
        let x: u64 = 1_5391283;
        let y: u64 = 314_1592653;
        let z: u64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 483_5313676);
    }

    #[test]
    fn test_mul_div_ceil_exact() {
        // Real result = 12
        let x: u64 = 8;
        let y: u64 = 3;
        let z: u64 = 2;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 12);
    }

    #[test]
    fn test_mul_div_ceil_mul_zero() {
        let x: u64 = 1_5391283;
        let y: u64 = 0;
        let z: u64 = 1_0000001;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_div_ceil_div_zero() {
        let x: u64 = 1_5391283;
        let y: u64 = 314_1592653;
        let z: u64 = 0;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    #[test]
    fn test_mul_div_ceil_large_number() {
        let x: u64 = 18_446_744_073;
        let y: u64 = 1_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 18_446_744_073);
    }

    #[test]
    fn test_mul_div_ceil_small_number() {
        let x: u64 = 1;
        let y: u64 = 2;
        let z: u64 = 3;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_mul_div_ceil_phantom_overflow_uses_u128() {
        // u64::MAX is odd
        let x: u64 = (u64::MAX - 1) / 2;
        let y: u64 = 2_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, u64::MAX - 1);
    }

    #[test]
    fn test_mul_div_ceil_phantom_overflow_rounds_up() {
        // Real Result = 333_333_333_333.3..
        let x: u64 = 100 * 10u64.pow(9);
        let y: u64 = 10 * 10u64.pow(9);
        let z: u64 = 3 * 10u64.pow(9);

        let result = mul_div_ceil(x, y, z).unwrap();

        assert_eq!(result, 333_333_333_334);
    }

    #[test]
    fn test_mul_div_ceil_result_overflow() {
        // u64::MAX is odd
        let x: u64 = (u64::MAX - 1) / 2 + 1;
        let y: u64 = 2_000_000_000;
        let z: u64 = 1_000_000_000;

        let result = mul_div_ceil(x, y, z);

        assert_eq!(result, None);
    }

    /********** fixed_mul_floor **********/

    #[test]
    fn test_fixed_mul_floor() {
        // Real result = 104_9522835.2
        let x: u64 = 3_1423141;
        let y: u64 = 4_1234142;
        let denominator: u64 = 0_1234567;

        let result = x.fixed_mul_floor(y, denominator).unwrap();
        assert_eq!(result, 104_9522835);

        let invalid = x.fixed_mul_floor(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_mul_ceil **********/

    #[test]
    fn test_fixed_mul_ceil() {
        // Real result = 104_9522835.2
        let x: u64 = 3_1423141;
        let y: u64 = 4_1234142;
        let denominator: u64 = 0_1234567;

        let result = x.fixed_mul_ceil(y, denominator).unwrap();
        assert_eq!(result, 104_9522836);

        let invalid = x.fixed_mul_ceil(y, 0);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_floor **********/

    #[test]
    fn test_fixed_div_floor() {
        // Real result = 204_1150997.8
        let x: u64 = 314_1592653;
        let y: u64 = 1_5391280;
        let denominator: u64 = 1_0000000;

        let result = x.fixed_div_floor(y, denominator).unwrap();
        assert_eq!(result, 204_1150997);

        let invalid = x.fixed_div_floor(0, denominator);
        assert_eq!(invalid, None);
    }

    /********** fixed_div_ceil **********/

    #[test]
    fn test_fixed_div_ceil() {
        // Real result = 204_1150997.8
        let x: u64 = 314_1592653;
        let y: u64 = 1_5391280;
        let denominator: u64 = 1_0000000;

        let result = x.fixed_div_ceil(y, denominator).unwrap();
        assert_eq!(result, 204_1150998);

        let invalid = x.fixed_div_ceil(0, denominator);
        assert_eq!(invalid, None);
    }
}
