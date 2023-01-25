# Workshop Repo: Rust Fundamentals

_NOTE_: This is Work-In-Progress! Please check for updates a day before the
workshop.

This Github repository contains all the source code and examples for the first
homework assessment

## Install Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and
run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/).
_Rustup_ also allows you to install different compile targets and multiple
toolchains, as well as keeping your toolchains up to date.

After installing, you should have a set of new command line tools available.

### Verify your Rust installation:

1. Open a Terminal/Shell/Command Line of your choice
2. Check out this repo
3. Navigate to this repository
4. Enter

```bash
$ cargo build
```

5. Open your browser at https://localhost:3000

## Recommended Editor

During the workshop, we will use
[Visual Studio Code](https://code.visualstudio.com/) as editor. It's free, fast
and very extensible. Making yourself familiar with VS Code is highly
recommended.

However, working with VS Code is not required. If you have a preferred editor
with Rust support you're more productive with, please feel free to use whatever
you like. What we highyly recommend though, is checking if your editor has
support for [Rust analyzer](https://rust-analyzer.github.io/).

## Recommended VS Code Extensions

To work effeciently, please install a couple of extensions that help you
developing Rust. _Note_: Please don't install the recommendend Rust extension.
It's outdated and the community decided to move to other tools. You can search
and install VS Code extensions through the menu on the side

We recommend the following extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).
  This is the main extension for Rust development, with the best language
  support available. _Note_: This extension is also available for other IDEs and
  editors, check out [their website](https://rust-analyzer.github.io/)

- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates).
  This extension helps installing dependencies from crates.io

- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml).
  TOML is the format that the dependency manager Cargo uses to manage
  dependencies. This extension helps formatting and editing TOML files

- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
  All Rust code is compiled against LLVM. This extension helps debugging LLVM
  code inside VS Code

- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens).
  Inline errors

## Teaching goals

- Rust fundamentals: loops, structs, conditionals, APIs
- Basic ownership and borrowing
- Traits: Generics, trait bounds
- Error handling

## Tasks

Our goal is to build a CLI tool that takes filenames as arguments and counts
words, lines, bytes, and characters. It will display the sum of all words,
lines, bytes and characters to stdout.

Those are the four exercises.

### Exercise 1 (`ex1.rs`): _count_

- Implement a `count` function that takes an `Cursor` and returns a `FileInfo`
  struct.
- Implement the `FileInfo`
- Count the number of lines
- Count all words (e.g. ignore whitespace)
- Count bytes and characters

### Exercise 2: _error handling and trait bounds_

- Extend your software to be able to handle everything hat implements `BufRead`
  (`Cursor`, `BufReader`)
- There are some situations that require error handling. Make sure of proper
  error handling through error propagation.

### Exercise 3 (`ex2.rs`): _traits_

- Implement the `Sum` trait to collect the totals over an iterator of
  `FileInfo`.
- Implement the `Display` trait to put out all parts, taking a width of 8
  characters per part

## Exercise 4: _CLI app_

- Make your app runable as CLI application (`std::env`). Parse the command line arguments,
  call your functions, and display the results.

## Extensions

- Structure your app to allow for different displays, e.g. show only words, only
  characters, a mixture of all four pieces.
- Extend your app to show the filename as well
- Use a library like `clap` to make a good CLI interface
- Try to parallelize across threads (`rayon`) and see if there's any difference
  in execution
