# expr-git2-rs

Experiment with [git2-rs](https://github.com/rust-lang/git2-rs)
a rust binding for [libgit2](https://libgit2.org/).

Created from this [ChatGPT4o conversation](https://chatgpt.com/share/675f0849-df28-800c-9be1-b899509709b6)

## Build

Currently doesn't build:
```shell
wink@fwlaptop 24-12-15T16:52:02.887Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo build
   Compiling expr-git2-rs v0.1.0 (/home/wink/prgs/rust/myrepos/expr-git2-rs)
error[E0277]: `?` couldn't convert the error to `git2::Error`
  --> src/main.rs:11:52
   |
10 | fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Error> {
   |                                                                       ----------------- expected `git2::Error` because of this
11 |     writeln!(writer, "Is bare: {}", repo.is_bare())?;
   |     -----------------------------------------------^ the trait `From<std::io::Error>` is not implemented for `git2::Error`, which is required by `Result<(), git2::Error>: FromResidual<Result<Infallible, std::io::Error>>`
   |     |
   |     this can't be annotated with `?` because it has type `Result<_, std::io::Error>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             `git2::Error` implements `From<JoinPathsError>`
             `git2::Error` implements `From<NulError>`
   = note: required for `Result<(), git2::Error>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `?` couldn't convert the error to `git2::Error`
  --> src/main.rs:12:60
   |
10 | fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Error> {
   |                                                                       ----------------- expected `git2::Error` because of this
11 |     writeln!(writer, "Is bare: {}", repo.is_bare())?;
12 |     writeln!(writer, "Is worktree: {}", repo.is_worktree())?;
   |     -------------------------------------------------------^ the trait `From<std::io::Error>` is not implemented for `git2::Error`, which is required by `Result<(), git2::Error>: FromResidual<Result<Infallible, std::io::Error>>`
   |     |
   |     this can't be annotated with `?` because it has type `Result<_, std::io::Error>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             `git2::Error` implements `From<JoinPathsError>`
             `git2::Error` implements `From<NulError>`
   = note: required for `Result<(), git2::Error>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `?` couldn't convert the error to `git2::Error`
  --> src/main.rs:13:62
   |
10 | fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Error> {
   |                                                                       ----------------- expected `git2::Error` because of this
...
13 |     writeln!(writer, "Path to repository: {:?}", repo.path())?;
   |     ---------------------------------------------------------^ the trait `From<std::io::Error>` is not implemented for `git2::Error`, which is required by `Result<(), git2::Error>: FromResidual<Result<Infallible, std::io::Error>>`
   |     |
   |     this can't be annotated with `?` because it has type `Result<_, std::io::Error>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             `git2::Error` implements `From<JoinPathsError>`
             `git2::Error` implements `From<NulError>`
   = note: required for `Result<(), git2::Error>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `?` couldn't convert the error to `git2::Error`
  --> src/main.rs:14:54
   |
10 | fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Error> {
   |                                                                       ----------------- expected `git2::Error` because of this
...
14 |     writeln!(writer, "Workdir: {:?}", repo.workdir())?;
   |     -------------------------------------------------^ the trait `From<std::io::Error>` is not implemented for `git2::Error`, which is required by `Result<(), git2::Error>: FromResidual<Result<Infallible, std::io::Error>>`
   |     |
   |     this can't be annotated with `?` because it has type `Result<_, std::io::Error>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             `git2::Error` implements `From<JoinPathsError>`
             `git2::Error` implements `From<NulError>`
   = note: required for `Result<(), git2::Error>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `?` couldn't convert the error to `git2::Error`
  --> src/main.rs:15:66
   |
10 | fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Error> {
   |                                                                       ----------------- expected `git2::Error` because of this
...
15 |     writeln!(writer, "HEAD reference: {:?}", repo.head()?.name())?;
   |     -------------------------------------------------------------^ the trait `From<std::io::Error>` is not implemented for `git2::Error`, which is required by `Result<(), git2::Error>: FromResidual<Result<Infallible, std::io::Error>>`
   |     |
   |     this can't be annotated with `?` because it has type `Result<_, std::io::Error>`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `From<T>`:
             `git2::Error` implements `From<JoinPathsError>`
             `git2::Error` implements `From<NulError>`
   = note: required for `Result<(), git2::Error>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `expr-git2-rs` (bin "expr-git2-rs") due to 5 previous errors
wink@fwlaptop 24-12-15T16:52:09.577Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
