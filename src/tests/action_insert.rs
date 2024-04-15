use super::*;
use crate::plane::Plane;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  plane.insert_char('A');
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│A  │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount │ Priority ║             │           │
│   ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │           │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  plane.cursor_move_down();
  plane.delete_char();
  plane.insert_char('A');
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├──┬───────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│AU│           │       ║    Order options    ║             │           │
│  │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
│  │   type    │ size  ║ Discount │ Priority ║             │           │
│  ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│  │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│  │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│  │           │       ║   0.05   │ "Low"    ║             │           │
╞══╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1│"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2│"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├──┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3│"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└──┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0003() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  repeat!(2, plane, cursor_move_right);
  plane.insert_char('K');
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├────┬───────────┬───────╥────────────┴────────╥─────────────┬───────────┐
│ UK │           │       ║    Order options    ║             │           │
│    │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
│    │   type    │ size  ║ Discount │ Priority ║             │           │
│    ├───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│    │"Business",│  <10, ║   0.10,  │"Normal", ║             │           │
│    │"Private"  │ >=10  ║   0.15,  │ "High",  ║             │           │
│    │           │       ║   0.05   │ "Low"    ║             │           │
╞════╪═══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1  │"Business" │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├────┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2  │"Business" │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├────┼───────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3  │"Private"  │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└────┴───────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}
