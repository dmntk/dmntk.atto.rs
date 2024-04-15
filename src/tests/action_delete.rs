use super::*;
use crate::plane::Plane;

#[test]
fn _0001() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  repeat!(2, plane, cursor_move_right);
  repeat!(1, plane, delete_char_before);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├──┬───────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│ U│           │       ║    Order options    ║             │           │
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
fn _0002() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  repeat!(12, plane, cursor_move_right);
  repeat!(1, plane, delete_char_before);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Custome   │ Order ╟──────────┬──────────╢ Description │ Reference │
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
fn _0003() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(5, plane, cursor_move_down);
  repeat!(13, plane, cursor_move_right);
  repeat!(3, plane, delete_char_before);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬──────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Busine,  │  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private" │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │          │       ║   0.05   │ "Low"    ║             │           │
╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business"│ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private" │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0004() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  repeat!(8, plane, cursor_move_right);
  repeat!(4, plane, delete_char);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Cust      │ Order ╟──────────┬──────────╢ Description │ Reference │
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
fn _0005() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(5, plane, cursor_move_down);
  repeat!(13, plane, cursor_move_right);
  repeat!(2, plane, delete_char);
  eq_cursor(7, 14, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬──────────┬───────╥──────────────┴──────╥─────────────┬───────────┐
│ U │          │       ║    Order options    ║             │           │
│   │ Customer │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │   type   │ size  ║ Discount │ Priority ║             │           │
│   ├──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │"Business │  <10, ║   0.10,  │"Normal", ║             │           │
│   │"Private" │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │          │       ║   0.05   │ "Low"    ║             │           │
╞═══╪══════════╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │"Business"│  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │"Business"│ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼──────────┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │"Private" │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴──────────┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0006() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(2, plane, cursor_move_down);
  repeat!(3, plane, cursor_move_right);
  repeat!(15, plane, delete_char);
  repeat!(1, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  repeat!(1, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  repeat!(1, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  repeat!(2, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  repeat!(1, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  repeat!(1, plane, cursor_move_down);
  repeat!(15, plane, delete_char);
  eq_cursor(15, 5, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬─┬───────╥─────────────────────╥─┴───────────┬───────────┐
│ U │ │       ║    Order options    ║             │           │
│   │ │ Order ╟──────────┬──────────╢ Description │ Reference │
│   │ │ size  ║ Discount │ Priority ║             │           │
│   ├─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│   │ │  <10, ║   0.10,  │"Normal", ║             │           │
│   │ │ >=10  ║   0.15,  │ "High",  ║             │           │
│   │ │       ║   0.05   │ "Low"    ║             │           │
╞═══╪═╪═══════╬══════════╪══════════╬═════════════╪═══════════╡
│ 1 │ │  <10  ║   0.10   │ "Normal" ║ Small order │   Ref 1   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 2 │ │ >=10  ║   0.15   │ "High"   ║ Large order │   Ref 2   │
├───┼─┼───────╫──────────┼──────────╫─────────────┼───────────┤
│ 3 │ │   -   ║   0.05   │ "Low"    ║ All orders  │   Ref 3   │
└───┴─┴───────╨──────────┴──────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0007() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(5, plane, cursor_move_down);
  repeat!(40, plane, cursor_move_right);
  repeat!(1, plane, delete_char);
  eq_cursor(7, 44, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴──────╥─────────────┬───────────┐
│ U │           │       ║    Order options   ║             │           │
│   │ Customer  │ Order ╟──────────┬─────────╢ Description │ Reference │
│   │   type    │ size  ║ Discount │ Priority║             │           │
│   ├───────────┼───────╫──────────┼─────────╫─────────────┼───────────┤
│   │"Business",│  <10, ║   0.10,  │"Normal",║             │           │
│   │"Private"  │ >=10  ║   0.15,  │ "High", ║             │           │
│   │           │       ║   0.05   │ "Low"   ║             │           │
╞═══╪═══════════╪═══════╬══════════╪═════════╬═════════════╪═══════════╡
│ 1 │"Business" │  <10  ║   0.10   │ "Normal"║ Small order │   Ref 1   │
├───┼───────────┼───────╫──────────┼─────────╫─────────────┼───────────┤
│ 2 │"Business" │ >=10  ║   0.15   │ "High"  ║ Large order │   Ref 2   │
├───┼───────────┼───────╫──────────┼─────────╫─────────────┼───────────┤
│ 3 │"Private"  │   -   ║   0.05   │ "Low"   ║ All orders  │   Ref 3   │
└───┴───────────┴───────╨──────────┴─────────╨─────────────┴───────────┘
"#;
  eq(plane, expected);
}

#[test]
fn _0008() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_down);
  repeat!(42, plane, cursor_move_right);
  repeat!(1, plane, cursor_move_down);
  eq_cursor(4, 47, plane);
  repeat!(1, plane, delete_char_before);
  eq_cursor(4, 47, plane);
  let expected = r#"
┌─────────────────────────────────────┐
│ Order options                       │
├───┬───────────┬───────╥─────────────┴───────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
fn _0009() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_cell_end);
  repeat!(13, plane, delete_char);
  plane.delete_char();
  eq_cursor(1, 23, plane);
  let expected = r#"
┌───────────────────────┐
│ Order options         │
├───┬───────────┬───────╥─────────────────────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
fn _0010() {
  let plane = &mut Plane::new(TEST_INPUT_001);
  eq_cursor(1, 1, plane);
  repeat!(1, plane, cursor_move_cell_end);
  repeat!(13, plane, delete_char_before);
  plane.delete_char_before();
  eq_cursor(1, 23, plane);
  let expected = r#"
┌───────────────────────┐
│ Order options         │
├───┬───────────┬───────╥─────────────────────╥─────────────┬───────────┐
│ U │           │       ║    Order options    ║             │           │
│   │ Customer  │ Order ╟──────────┬──────────╢ Description │ Reference │
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
