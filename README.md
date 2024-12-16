# expr-git2-rs

Experiment with [git2-rs](https://github.com/rust-lang/git2-rs)
a rust binding for [libgit2](https://libgit2.org/).

Created from this [ChatGPT4o conversation](https://chatgpt.com/share/675f0849-df28-800c-9be1-b899509709b6)

I wanted to create the absolute minimum repo which is a repo with
no commits. I couldn't do it so I asked ChatGPT4o for help and we
came up with this code. I could create a "empty" repo, i.e. a repo
with no files, using the command line tools thus we've done this
using git2-rs!

## Build

```shell
wink@fwlaptop 24-12-15T17:15:47.801Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
wink@fwlaptop 24-12-15T17:15:50.573Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

## Run

Run with `-h` or `--help` to get the help message:
```shell
wink@3900x 24-12-16T20:04:26.201Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo run -- -h
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/expr-git2-rs -h`
Usage: expr-git2-rs [OPTIONS]

Options:
  -r, --repo-path <REPO_PATH>  [default: .]
  -h, --help                   Print help
  -V, --version                Print version
wink@3900x 24-12-16T20:04:34.362Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

Run with no arguments to get the default output on the current repo at ".":
```shell
wink@3900x 24-12-16T20:04:34.362Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/expr-git2-rs`
Is bare: false
Is worktree: false
Path to repository: "/home/wink/prgs/rust/myrepos/expr-git2-rs/.git/"
Workdir: Some("/home/wink/prgs/rust/myrepos/expr-git2-rs/")
HEAD reference: Some("refs/heads/main")
wink@3900x 24-12-16T20:06:05.561Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

## Tests

### Run all tests

```shell
wink@fwlaptop 24-12-15T23:35:24.472Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/main.rs (target/debug/deps/expr_git2_rs-b7f7ee0dafdfe161)

running 3 tests
test tests::test_logging ... ok
test tests::test_get_top_level_info ... ok
test tests::test_empty_repo ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

wink@fwlaptop 24-12-15T23:35:55.842Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

### Run test with logging

This should be run one at a time to sequential log output.

#### test_logging
```shell
wink@fwlaptop 24-12-15T23:34:00.894Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ RUST_LOG=debug cargo test -- --nocapture test_logging
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/main.rs (target/debug/deps/expr_git2_rs-b7f7ee0dafdfe161)

running 1 test
[2024-12-15T23:35:24.468206295Z DEBUG expr_git2_rs::tests   52  2] Test with logging
test tests::test_logging ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

wink@fwlaptop 24-12-15T23:35:24.472Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

#### test_get_top_level_info

```shell
wink@fwlaptop 24-12-15T23:33:17.076Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ RUST_LOG=debug cargo test -- --nocapture test_get_top_level_info
   Compiling expr-git2-rs v0.1.0 (/home/wink/prgs/rust/myrepos/expr-git2-rs)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/main.rs (target/debug/deps/expr_git2_rs-b7f7ee0dafdfe161)

running 1 test
[2024-12-15T23:34:00.886024228Z DEBUG expr_git2_rs::tests   74  2] 
Is bare: false
Is worktree: false
Path to repository: "/home/wink/prgs/rust/myrepos/expr-git2-rs/test_repos/test_top_level_info/.git/"
Workdir: Some("/home/wink/prgs/rust/myrepos/expr-git2-rs/test_repos/test_top_level_info/")
HEAD reference: Some("refs/heads/main")

test tests::test_get_top_level_info ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.01s

wink@fwlaptop 24-12-15T23:34:00.894Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

#### Adding RUST_BACKTREACE=1

To get additional information on the panic/failure you can
add RUST_BACKTRACE=1 to the command line. Below I changed
the "`Path to repository:" to "Path to:" which will cause a
failure and with RUST_BACKTRACE=1 you get a stack trace which
could be helpful in some situations:

```shells
wink@fwlaptop 24-12-15T23:57:53.487Z:~/prgs/rust/myrepos/expr-git2-rs (main)
$ RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --nocapture test_get_top_level_info
   Compiling expr-git2-rs v0.1.0 (/home/wink/prgs/rust/myrepos/expr-git2-rs)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running unittests src/main.rs (target/debug/deps/expr_git2_rs-b7f7ee0dafdfe161)

running 1 test
[2024-12-15T23:58:33.236628939Z DEBUG expr_git2_rs::tests   74  2] 
Is bare: false
Is worktree: false
Path to repository: "/home/wink/prgs/rust/myrepos/expr-git2-rs/test_repos/test_top_level_info/.git/"
Workdir: Some("/home/wink/prgs/rust/myrepos/expr-git2-rs/test_repos/test_top_level_info/")
HEAD reference: Some("refs/heads/main")

thread 'tests::test_get_top_level_info' panicked at src/main.rs:82:9:
assertion failed: output_str.contains(&format!("Path to: {repo_dir_str:?}"))
stack backtrace:
   0: rust_begin_unwind
             at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/std/src/panicking.rs:665:5
   1: core::panicking::panic_fmt
             at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:74:14
   2: core::panicking::panic
             at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/panicking.rs:148:5
   3: expr_git2_rs::tests::test_get_top_level_info
             at ./src/main.rs:82:9
   4: expr_git2_rs::tests::test_get_top_level_info::{{closure}}
             at ./src/main.rs:56:37
   5: core::ops::function::FnOnce::call_once
             at /home/wink/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   6: core::ops::function::FnOnce::call_once
             at /rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test tests::test_get_top_level_info ... FAILED

failures:

failures:
    tests::test_get_top_level_info

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.04s

error: test failed, to rerun pass `--bin expr-git2-rs`
wink@fwlaptop 24-12-15T23:58:33.269Z:~/prgs/rust/myrepos/expr-git2-rs (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
