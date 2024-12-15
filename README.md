# expr-git2-rs

Experiment with [git2-rs](https://github.com/rust-lang/git2-rs)
a rust binding for [libgit2](https://libgit2.org/).

Created from this [ChatGPT4o conversation](https://chatgpt.com/share/675f0849-df28-800c-9be1-b899509709b6)

## Build

```shell
wink@fwlaptop 24-12-15T17:15:47.801Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
wink@fwlaptop 24-12-15T17:15:50.573Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

## Run

```shell
wink@fwlaptop 24-12-15T17:15:50.573Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/expr-git2-rs`
Is bare: false
Is worktree: false
Path to repository: "/home/wink/prgs/rust/myrepos/expr-git2-rs/.git/"
Workdir: Some("/home/wink/prgs/rust/myrepos/expr-git2-rs/")
HEAD reference: Some("refs/heads/main")
wink@fwlaptop 24-12-15T17:15:58.263Z:~/prgs/rust/myrepos/expr-git2-rs (main)

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
