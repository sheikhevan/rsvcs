# rsvcs

rsvcs is a super simple backup utility with version control elements. It's written in Rust, hence the 'rs' in the name. This is my
first Rust project, so the code is likely not very refined, feel free to contribute! As of right now, you can `init` a new repository,
`add` files to the staging directory, and `commit` with a message, which makes a compressed tarball with all the files you added to the
staging directory. More features coming!

## Getting Started

The following are instructions to compile and run a copy of this program on your machine.

### Prerequisites

As of right now, the only things you need are the Rust toolchain, and that you are running Linux. If you are using nix, you can run
`nix-shell` in the root directory to automatically install all required packages.

### Installing

In the source directory, run the following:

```bash
$ cargo build --release
$ cd target/release/
$ ./rsvcs
```

## Usage

- `rsvcs init` will initialze a new repository in the current directory.
- `rsvcs add` followed by the files/directories you want to stage (wildcard supported).
- `rsvcs commit` followed by the message you want to commit with will make a compressed tarball named with the hash of the commit and also store the information in the `log`.

## Built With

- [Rust](https://www.rust-lang.org/)
- [clap](https://crates.io/crates/clap/)
- [flate2](https://crates.io/crates/flate2)
- [sha2](https://crates.io/crates/sha2)
- [tar](https://crates.io/crates/tar)
- [zstd](https://crates.io/crates/zstd)
- [chrono](https://crates.io/crates/chrono)

## Contributing

Feel free to contribute. Simply fork the repository, make a new branch, and then make a pull request to the main repo. Check issues for things to contribute.

## Authors

- **Evan Alvarez**

See also the list of [contributors](https://github.com/sheikhevan/rsvcs/contributors) who participated in this project.

## License

This project is licensed under the MIT license - see the [LICENSE.md](LICENSE.md) file for details.
