# homemade_yes
A Rust implementation of the Unix-like `yes` command, with heavy inspiration from `uutils`.

I accidentally leaked personal info in the previous `homemade_yes` repository, so this one is a recreated version with the latest updates.

The primary codebase is 28 lines long, excluding comments and empty lines. Compare this to the [uutils implementation](https://github.com/uutils/coreutils/blob/main/src/uu/yes/src/yes.rs), which has ~160 lines of actual code.

The executables under the `Releases` tab are built for an x86_64 processor. The Linux builds are statically linked with `musl`, and the Windows builds are built using `mingw-w64`.

## Benchmarking

You can benchmark `homemade_yes` by running `./homemade_yes | pv -N "Homemade yes" -r -t > /dev/null`.

Here are some test results from my build machine, which runs Ubuntu 25.10:
| Implementation   | Output rate (`yes`) | Output rate (`yes abcd`) |
| -------- | ------- | ------- |
| uutils yes 0.2.2 |5.65GiB/s|5.62GiB/s |
| homemade_yes 0.3 | 5.09GiB/s    | 5.24GiB/s|
| homemade_yes 0.4   | 5.61Gib/s    |5.62GiB/s |

`homemade_yes` 0.5 and 0.6 were not tested because they don't provide any meaningful performance improvements.


## Windows compatibility
Since version v0.3, I have added experimental support for Windows builds.

The Windows builds should behave exactly like their Linux counterparts and have been tested under Wine.

## Building from source
In order to automate the build process, I have written two scripts. These should be executed on a Debian-based machine with Rust installed, optimally in a VM (some metadata, like your username, could get included in the executables).

### setup.sh
`setup.sh` prepares your system for building `homemade_yes`. To run it, execute `curl https://mojavesoft.net/script/homemade_yes/setup.sh | bash`. It installs `ruby`, `git`, and `mingw-w64`. Afterwards, it then adds the `fpm` gem for building Debian packages and clones this repository.

Finally, it adds the `x86_64-pc-windows-gnu` build target so that you can cross-compile for Windows.

### build.sh
`build.sh` builds the latest version of `homemade_yes`. First, it updates the repository with `git pull` to fetch any recent updates. Afterwards, it uses `cargo` to build the binaries for Windows and Linux. Finally, it creates the `deb_package` directory and places the final Debian package in it.

To start the build process, run `curl https://mojavesoft.net/script/homemade_yes/build.sh | bash`.

### Testing
Since version v0.6.1, `homemade_yes` has included testing suites for the x86 Linux builds. To run them, you need to install `bats-core` (on Ubuntu you can do this by running `sudo apt install bats`).

To run the test suite, execute `bats homemade_yes.bats -T --gather-test-outputs-in results-gnu -x -o results-gnu --report-formatter tap` in the `tests` directory.

### A note about old forks
If you have previously built `homemade_yes` versions predating v0.5, you may get an error about divergent branches. If this happens, delete the `homemade_yes` folder and run `setup.sh` again.

## A note about manmade code
The owner of this project believes in good faith that it complies with [The Manmade Software Declaration 1.0](https://mojavesoft.net/ai-policy/1.0).
Contributors are encouraged to follow the guidelines described at the aforementioned link when proposing any code changes, and patches that appear to violate those rules may be rejected at any time.
