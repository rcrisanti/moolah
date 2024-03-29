use super::*;

mod custom_delta;
mod daily_delta;
mod monthly_delta;
mod one_time_delta;
mod weekly_delta;
mod yearly_delta;

#[test]
fn test_positve_f32() {
    assert!(PositiveF64::try_from(-0.1).is_err());
    assert!(PositiveF64::try_from(0.0).is_ok());
    assert!(PositiveF64::try_from(1.0).is_ok());
}
