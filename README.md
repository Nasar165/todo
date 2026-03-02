# Task Manager

A simple todo command line program written in rust for educational purpose

## Table of Contents

- [Summary](#summary)
- [Prerequisites](#prerequisites)
  - [Required](#required)
- [Development](#development)
  - [tests](#tests)
- [Deployment](#deployment)
  - [Compile](#compile)
- [Built With](#built-with)
- [Authors](#authors)
- [License](#license)

## Summary

### Prerequisites

#### Required

The following tools are required for anyone who wishes to compile or run the cli. Please ensure you meet the requirements below:

- [Rust](https://rust-lang.org/)

## Development

### Tests

when performing cargo test, we recommend that you use RUST_TEST_THREADS=1 to ensure
that the tests are performed syncrounisly ensuring that test will pass. should you
decided to test using more threads then one then you can expect errors from Manager.
This library is responsible for IO operations such as file management.

## Deployment

### Compile

*Notes*
.cargo/config.toml contains RUST_TEST_THREADS=1.

*Example*:

``` bash
# Default behavior when running <cargo test> = <RUST_TEST_THREADS=1 cargo test>
cargo test

# if you want to override default RUST_TEST_THREADS use the following NOT RECOMMENDED
RUST_TEST_THREADS=2 cargo test
```

### Built With

- [Rust](https://rust-lang.org/)
- [Editor config](https://editorconfig.org/)
- [GIT](https://git-scm.com/)
- [Visual Studio Code](https://code.visualstudio.com/)

### Authors

- **Xaness** - _Initial work_ - [Xaness](https://github.com/Nasar165)

See also the list of [contributors](https://github.com/Nasar165/todo/graphs/contributors)
who participated in this project.

### License

This project is licensed under the MIT - see the [LICENSE](LICENSE) file for details