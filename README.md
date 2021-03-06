# Learn Rust with TDD

Like Learn Go with Tests, but in Rust. 

[HTML](https://learn-with-tests.github.io/learn-rust-with-tests/)

![github pages](https://github.com/learn-with-tests/learn-rust-with-tests/workflows/github%20pages/badge.svg)

[Buy us a coffee :coffee:](https://www.buymeacoffee.com/learnwithtests)!

## Development Requirements

_Learn Rust With TDD_ is written using [mdBook][mdBook]. Follow the instructions
there to install mdBook on your machine and build the book.

## Things to improve

- ~Make some £££~ Provide this amazing resource to the community free of charge
- Maybe use https://github.com/rust-lang/mdBook
- My spelling and grammar is shit. It would be good to automate a checker
- Automate building the book
- A big pain point with the go one was keeping snippets in the md and the source consistent, is there a way to make it easy? (https://rust-lang.github.io/mdBook/format/mdbook.html#mdbook-specific-markdown)

## Resources

- https://doc.rust-lang.org/book/ch11-00-testing.html

[mdBook]: https://github.com/rust-lang/mdBook

## Runing mdbook

To run mdbook via docker from the root of the project:

```shell
# show usage
./mdbook.sh

# test code samples
./mdbook.sh test

# serve the book at http://0.0.0.0:3000, watching for changes
./mdbook.sh serve --hostname 0.0.0.0

# build the book to the book/ folder
./mdbook.sh build
```
