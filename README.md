# homemade_yes
High-performance reimplementation of GNU yes, but made at home in Rust.

As I am not very good at writing in Rust, I probably made a lot of subtle mistakes; please submit a PR if you find anything!

The entire codebase is 25 lines long, excluding comments and empty lines. Compare this to the [uutils implementation](https://github.com/uutils/coreutils/blob/main/src/uu/yes/src/yes.rs), which has ~161 lines of actual code.

The executables under the `Releases` tab are built for an x86_64 processor. The Linux builds are statically linked with `musl`, and the Windows builds are built using `mingw-w64`.

## Benchmarking

You can benchmark `homemade_yes` by running `./homemade_yes | pv -N "Homemade yes" -r -t > /dev/null`.

On my build machine (Ubuntu 25.04), it can obtain a throughput of about ~3.5 gigabytes per second, which is comparable to GNU yes (v9.5). On a nice laptop from the past ~5 years and Xubuntu 25.04, I was able to achieve ~4.2 gigabytes per second, whereas GNU yes usually hovered around ~4.0 gigabytes per second. 

## Traditional version
I have also written a more traditional version of `yes` at `src/bin/posix-only.rs`. This version does not include the `--version` and `--help` arguments, simplifying the code massively.

In all other respects, it should act roughly identical to the GNU implementation from `src/main.rs`. If it doesn't, please file an issue so that I can fix it.

_Note: I previously referred to this build as being a "POSIX-only" version. This is incorrect; there does not appear to be an applicable POSIX specification here._

## Windows compatibility
Since version v0.3, I have added experimental support for Windows builds; see https://zork.net/~st/jottings/rust-windows-and-debian.html for instructions on how to cross-compile.

The Windows builds should behave exactly like their Linux counterparts and have been tested under Wine.
