# Iterate Text
[heading__top]:
  #iterate-text
  "&#x2B06; Library of helper functions and structures for iterating over text and files"


Library of helper functions and structures for iterating over text and files


## [![Byte size of Iterate Text][badge__main__iterate_text__source_code]][iterate_text__main__source_code] [![Open Issues][badge__issues__iterate_text]][issues__iterate_text] [![Open Pull Requests][badge__pull_requests__iterate_text]][pull_requests__iterate_text] [![Latest commits][badge__commits__iterate_text__main]][commits__iterate_text__main]


---


- [&#x2B06; Top of Document][heading__top]

- [&#x1F3D7; Requirements][heading__requirements]

- [&#9889; Quick Start][heading__quick_start]

- [&#x1F9F0; Usage][heading__usage]

- [&#x1F523; API][heading__api]
  - [`file::characters::IterateFileCharacters`][heading__filecharactersiteratefilecharacters]
  - [`file::lines::IterateFileLines`][heading__filelinesiteratefilelines]
  - [`string::characters::IterateStringCharacters`][heading__stringcharactersiteratestringcharacters]
  - [`string::lines::IterateStringLines`][heading__stringlinesiteratestringlines]

- [&#x1F5D2; Notes][heading__notes]

- [&#x1F4C8; Contributing][heading__contributing]
  - [&#x1F531; Forking][heading__forking]
  - [&#x1F4B1; Sponsor][heading__sponsor]

- [&#x1F4C7; Attribution][heading__attribution]

- [&#x2696; Licensing][heading__license]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from source


As of last update to this ReadMe file, the recommended method of installing Rust is via the installer script...


```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```



______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust library, define it as a dependency within a project `Cargo.toml` file...


**`Cargo.toml` (snip)**


```toml
[dependencies]
iterate-text = "0.0.1"
```


> Check [Rust -- Doc -- Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for details about defining dependencies.


Then include within a source file via `use` statement...


**`src/main.rs` (snip)**


```rust
extern crate iterate_text;

use iterate_text::file::characters::IterateFileCharacters;
use iterate_text::file::lines::IterateFileLines;

use iterate_text::string::characters::IterateStringCharacters;
use iterate_text::string::lines::IterateStringLines;
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


Check the `examples/` directory of this project for detailed usage examples.


**`src/main.rs`**


```rust
#!/usr/bin/env rust

use std::env;

extern crate iterate_text;
use iterate_text::file::lines::IterateFileLines;


fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let iter = IterateFileLines::new(path.into());

    let mut count = 0;
    for line in iter {
        count += 1;
        print!("{}: {}", count, line);
    }
}
```


______


## API
[heading__api]:
  #api
  "&#x1F523; Documentation for classes, methods, parameters, and custom types/data-structures"


Documentation for classes, methods, parameters, and custom types/data-structures


---


### `file::characters::IterateFileCharacters`
[heading__filecharactersiteratefilecharacters]:
  #filecharactersiteratefilecharacters
  "Reads from path, buffer, or descriptor and iterates over characters until EOF is reached"


Reads from path, buffer, or descriptor and iterates over characters until EOF is reached


**Example**


```rust
use iterate_text::file::characters::IterateFileCharacters;

let p = "tests/file/characters/file.txt";
let mut c = IterateFileCharacters::new(p);

assert_eq!(c.next(), Some('T'));
assert_eq!(c.next(), Some('h'));
assert_eq!(c.next(), Some('i'));
assert_eq!(c.next(), Some('s'));
```


---


### `file::lines::IterateFileLines`
[heading__filelinesiteratefilelines]:
  #filelinesiteratefilelines
  "Reads from file path, buffer, or descriptor and iterates over lines until EOF is reached"


Reads from file path, buffer, or descriptor and iterates over lines until EOF is reached


```rust
use iterate_text::file::lines::IterateFileLines;

let p = "tests/file/lines/file.txt";
let mut l = IterateFileLines::new(p);

assert_eq!(l.next(), Some("First line\n".to_string()));
assert_eq!(l.next(), Some("Second line\n".to_string()));
assert_eq!(l.next(), Some("Third line\n".to_string()));
assert_eq!(l.next(), None);
```


---


### `string::characters::IterateStringCharacters`
[heading__stringcharactersiteratestringcharacters]:
  #stringcharactersiteratestringcharacters
  "Iterates over characters within string"


Iterates over characters within string


**Example**


```rust
use iterate_text::string::characters::IterateStringCharacters;

let s = String::from("test!");
let mut c = IterateStringCharacters::new(s);

assert_eq!(c.next(), Some('t'));
assert_eq!(c.next(), Some('e'));
assert_eq!(c.next(), Some('s'));
assert_eq!(c.next(), Some('t'));
assert_eq!(c.next(), Some('!'));
assert_eq!(c.next(), None);
```


---


### `string::lines::IterateStringLines`
[heading__stringlinesiteratestringlines]:
  #stringlinesiteratestringlines
  "Iterates over lines within string and includes new-line separator"


Iterates over lines within string and includes new-line separator


**Example**


```rust
use iterate_text::string::lines::IterateStringLines;

let s = String::from("This is\na \\n test string\n");
let mut l = IterateStringLines::new(s);

assert_eq!(l.next(), Some("This is\n".to_string()));
assert_eq!(l.next(), Some("a \\n test string\n".to_string()));
assert_eq!(l.next(), None);
```


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


The `characters` iterators currently use `String.char_indices()` which may split certain Unicode characters in unexpected ways.


This repository may not be feature complete and/or fully functional, Pull Requests that add features or fix bugs are certainly welcomed.


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to iterate-text and rust-utilities"


Options for contributing to iterate-text and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking iterate-text"


Start making a [Fork][iterate_text__fork_it] of this repository to an account that you have write permissions for.


- Add remote for fork URL. The URL syntax is _`git@github.com:<NAME>/<REPO>.git`_...


```bash
mkdir -p ~/git/hub/rust-utilities

cd ~/git/hub/rust-utilities/iterate-text

git remote add fork git@github.com:<NAME>/iterate-text.git
```


- Ensure **all** tests pass


```bash
cargo test
```


- Run any examples by name


```bash
cargo run --example file-reader -- --file .github/README.md -c 10
```


- Commit your changes and push to your fork to fix an issue or add a feature...


```bash
cd ~/git/hub/rust-utilities/iterate-text


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```


> Note, the `-u` option may be used to set `fork` as the default remote, eg. _`git push -u fork main`_ however, this will also default the `fork` remote for pulling from too! Meaning that pulling updates from `origin` must be done explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_


> Note; to decrease the chances of your Pull Request needing modifications before being accepted, please check the [dot-github](https://github.com/rust-utilities/.github) repository for detailed contributing guidelines.


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains iterate-text"


Thanks for even considering it!


Via Liberapay you may <sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a repeating basis.


Regardless of if you're able to financially support projects such as iterate-text that rust-utilities maintains, please consider sharing projects that are useful with others, because one of the goals of maintaining Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)

- [Matrix -- `@kaplan`](https://matrix.to/#/@kaplan:matrix.org) offered suberb, and efficent, code that totally solved my Rust related issues
  - [Rust Playground -- `IterateFileChars`](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dfa95d685ee2466e58366ac0a264464d)
  - [Matrix -- message `#/room/#rust:matrix.org/$sERdrXYCkiVh_eITLzeA7tOsB2YsqnnkOV3a6fdNx6w`](https://matrix.to/#/!fsEJmDUHIdYFfFRTSH:jki.re/$sERdrXYCkiVh_eITLzeA7tOsB2YsqnnkOV3a6fdNx6w?via=matrix.org&via=goyman.com&via=bruckbu.de)

- [Matrix -- `@projectmoon`](https://matrix.to/#/@projectmoon:agnos.is) had excelent instincts of owning `line` enabling character iteration
  - [Matrix -- mesage `#/room/#rust:matrix.org/$kXoPpn5bNQ_yjzq-sJiPsptzAYKQ8ZR0vMfbbgSaNcY`](https://matrix.to/#/!fsEJmDUHIdYFfFRTSH:jki.re/$kXoPpn5bNQ_yjzq-sJiPsptzAYKQ8ZR0vMfbbgSaNcY?via=matrix.org&via=goyman.com&via=bruckbu.de)

- [Matrix -- `@tanriol`](https://matrix.to/#/@tanriol:matrix.org) patently explained why code from `@kaplan` was so good, and answered my questions
  - [Matrix -- message `#/room/#rust:matrix.org/$_-SCswfFnfvwwkxcfFQXzsRSQfCVKnt2Wie1ct7dLGI`](https://matrix.to/#/!fsEJmDUHIdYFfFRTSH:jki.re/$_-SCswfFnfvwwkxcfFQXzsRSQfCVKnt2Wie1ct7dLGI?via=jki.re&via=matrix.org&via=privacytools.io)

- [Rust Doc -- Attributes](https://doc.rust-lang.org/reference/attributes.html)

- [Rust Doc -- Development dependencies](https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html)

- [Rust Doc -- Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

- [Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

- [StackOverflow -- How do I write a Rust unit test that ensures that a panic has occurred?](https://stackoverflow.com/questions/26469715/)

- [StackOverflow -- How do I test a code in a sub-sub-directory?](https://stackoverflow.com/questions/37626348/)



______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


```
Library of helper functions and structures for iterating over text and files
Copyright (C) 2021 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```


For further details review full length version of [AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"


[badge__commits__iterate_text__main]:
  https://img.shields.io/github/last-commit/rust-utilities/iterate-text/main.svg

[commits__iterate_text__main]:
  https://github.com/rust-utilities/iterate-text/commits/main
  "&#x1F4DD; History of changes on this branch"


[iterate_text__community]:
  https://github.com/rust-utilities/iterate-text/community
  "&#x1F331; Dedicated to functioning code"

[iterate_text__gh_pages]:
  https://github.com/rust-utilities/iterate-text/tree/
  "Source code examples hosted thanks to GitHub Pages!"

[badge__gh_pages__iterate_text]:
  https://img.shields.io/website/https/rust-utilities.github.io/iterate-text/index.html.svg?down_color=darkorange&down_message=Offline&label=Demo&logo=Demo%20Site&up_color=success&up_message=Online

[gh_pages__iterate_text]:
  https://rust-utilities.github.io/iterate-text/index.html
  "&#x1F52C; Check the example collection tests"

[issues__iterate_text]:
  https://github.com/rust-utilities/iterate-text/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[iterate_text__fork_it]:
  https://github.com/rust-utilities/iterate-text/
  "&#x1F531; Fork it!"

[pull_requests__iterate_text]:
  https://github.com/rust-utilities/iterate-text/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[iterate_text__main__source_code]:
  https://github.com/rust-utilities/iterate-text/
  "&#x2328; Project source!"

[badge__issues__iterate_text]:
  https://img.shields.io/github/issues/rust-utilities/iterate-text.svg

[badge__pull_requests__iterate_text]:
  https://img.shields.io/github/issues-pr/rust-utilities/iterate-text.svg

[badge__main__iterate_text__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/iterate-text


[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"

