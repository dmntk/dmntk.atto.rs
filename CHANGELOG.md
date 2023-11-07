# Changelog
All notable changes to decision table editor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.0.5] - ???
### Added
- Handling command-line arguments. Type `atto --help` for details.

## [0.0.4] - 2022-10-06
### Added
- Open a file containing a decision table.
- Move around the decision table using arrows, TAB, END and HOME keys.
- Insert text into cell.
- Delete text before cursor.
- Delete text under cursor.

## [0.0.3] - 2022-10-05
### Added
- Simple editing operations.

## [0.0.2] - 2022-10-05
### Added
- Simple cursor moving operations.

## [0.0.1] - 2022-09-22
### Added
- Opening and displaying a decision table.

## [0.0.0] - 2020-04-10
### Birthday of this editor's father

It was a Friday evening, when I started the development of the decision table editor in TypeScript for browsers.
After a few sleepless nights I suspended the project, because of really pure performance, even for small decision tables.

The idea was to open a text file in browser, edit it, and save. Sounds easy. Browser do such things since the beginning of the Internet... 

But the trick was, that each single character in the decision table (lines also) was represented by DIV in HTML.
For small tables (like 500 columns x 300 rows) there were 150,000 DIVs. Rendering those DIVs took so much time,
that the user has seen only flickering...

I needed something simple and really fast for editing decision tables saved as text. And then came the old friend to the rescue...

**THE TERMINAL**

This decision table editor is written in Rust and works in Linux terminal.
    
