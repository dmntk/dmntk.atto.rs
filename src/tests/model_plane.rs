use super::*;
use crate::plane::Plane;

#[test]
fn _0001() {
  let plane = Plane::new(TEST_INPUT_001);
  assert_eq!(TEST_INPUT_001.trim(), plane.to_string());
  assert_eq!(TEST_INPUT_001.trim(), format!("{}", plane));
}
