use super::*;

#[test]
fn test_default() {
    let d = CustomDelta::default();

    assert_eq!(d.name(), "");
    assert_eq!(d.value(), 0.0);
    assert!(d.uncertainty().is_none());
    assert!(d.dates().is_empty());
}
