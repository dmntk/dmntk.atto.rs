/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2015-2023 Dariusz Depta, Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
