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

//! Actions invoked by command-line arguments.

use crate::errors::Result;
use crate::{Editor, ATTO_DESCRIPTION, ATTO_NAME, ATTO_VERSION};
use clap::{arg, ArgMatches, Command};
use ncurses::*;

/// Available command-line actions.
enum CliAction {
  /// Edit decision table loaded from file with specified name.
  EditDecisionTable(String),
}

/// Parses command-line arguments.
fn get_matches() -> ArgMatches {
  Command::new(ATTO_NAME)
    .version(ATTO_VERSION)
    .about(ATTO_DESCRIPTION)
    .arg(arg!(<FILE>).help("File containing decision table to edit").required(true).index(1))
    .get_matches()
}

/// Checks arguments passed from the command line and returns a corresponding action.
fn get_cli_action() -> CliAction {
  let matches = get_matches();
  let file_name = matches.get_one::<String>("FILE").unwrap();
  CliAction::EditDecisionTable(file_name.to_string())
}

/// Executes command-line action.
pub fn do_action() -> Result<()> {
  match get_cli_action() {
    CliAction::EditDecisionTable(file_name) => {
      let mut editor = Editor::new(&file_name)?;
      editor.repaint_plane();
      editor.update_cursor();
      editor.update_cursor_coordinates();
      refresh();
      editor.process_keystrokes();
      editor.finalize()
    }
  }
}
