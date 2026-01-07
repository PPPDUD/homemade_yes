# homemade_yes
High-performance reimplementation of GNU yes, but made at home in Rust.

As I am not very good at writing in Rust, I probably made a lot of subtle mistakes; please submit a PR if you find anything!

The entire codebase is 25 lines long, excluding comments and empty lines. Compare this to the [uutils implementation](https://github.com/uutils/coreutils/blob/main/src/uu/yes/src/yes.rs), which has ~161 lines of actual code.

The executables under the `Releases` tab are built for an x86_64 processor. The Linux builds are statically linked with `musl`, and the Windows builds are built using `mingw-w64`.

## Benchmarking

You can benchmark `homemade_yes` by running `./homemade_yes | pv -N "Homemade yes" -r -t > /dev/null`.

Here are some test results from my build machine, which runs Ubuntu 25.10:
| Implementation   | Output rate (`yes`) | Output rate (`yes abcd`) |
| -------- | ------- | ------- |
| uutils yes 0.2.2 |5.65GiB/s|5.62GiB/s |
| homemade_yes 0.3 | 5.09GiB/s    | 5.24GiB/s|
| homemade_yes 0.4   | 5.61Gib/s    |5.62GiB/s |

## Traditional version
I have also written a more traditional version of `yes` at `src/bin/posix-only.rs`. This version does not include the `--version` and `--help` arguments, simplifying the code massively.

In all other respects, it should act roughly identical to the GNU implementation from `src/main.rs`. If it doesn't, please file an issue so that I can fix it.

_Note: I previously referred to this build as being a "POSIX-only" version. This is incorrect; there does not appear to be an applicable POSIX specification here._

## Windows compatibility
Since version v0.3, I have added experimental support for Windows builds.

The Windows builds should behave exactly like their Linux counterparts and have been tested under Wine.

## Building from source
In order to automate the build process, I have written two scripts. These should be executed on a Debian-based machine with Rust installed, optimally in a VM (some metadata, like your username, could get included in the executables).

### setup.sh
`setup.sh` prepares your system for building `homemade_yes`. To run it, execute `curl https://mojavesoft.net/scripts/homemade_yes/setup.sh | bash`. It installs `ruby`, `git`, and `mingw-w64`. Afterwards, it then adds the `fpm` gem for building Debian packages and clones this repository.

Finally, it adds the `x86_64-pc-windows-gnu` build target so that you can cross-compile for Windows.

### build.sh
`build.sh` builds the latest version of `homemade_yes`. First, it updates the repository with `git pull` to fetch any recent updates. Afterwards, it uses `cargo` to build the binaries for Windows and Linux. Finally, it creates the `deb_package` directory and places the final Debian package in it.

To start the build process, run `curl https://mojavesoft.net/script/homemade_yes/build.sh | bash`.
