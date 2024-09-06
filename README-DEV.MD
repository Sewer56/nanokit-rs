
# Development

How to develop this project.

***Clone this Repository:***
```bash
# When cloning, make sure symlinks are enabled
git clone -c core.symlinks=true https://github.com/Sewer56/nanokit-rs.git
```

***Install Rust:***
- Install the [Rust Toolchain.][rust-toolchain]***Setup IDE***
- This repository is fully with VSCode. [Guidance below](#visual-studio-code-integration).
### Visual Studio Code Integration

`Code`/`VSCode` is the de-facto Rust development environment.  

The following extensions are required:  
- [rust-analyzer][rust-analyzer] for Rust support.  
- [coverage-gutters][coverage-gutters] for Coverage support.  
- [CodeLLDB][codelldb] for debugging.  
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) easier dependency management.  

The VSCode configuration in Reloaded projects (`.vscode`) contain the following:  
- Run Rust linter `clippy` on Save.  
- Run code format `rustfmt` on Save.  
- Tasks for common operations (generate documentation, active CI/CD etc.).  

These configurations are in the `.vscode` folder; and the tasks can be ran via `Ctrl+Shift+P -> Run Task`.  

#### Test Coverage

First install or update `tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

To run Coverage, run task (`Ctrl+Shift+P -> Run Task`), you should see something similar to: 

| Task                   | Description                                                                |
| ---------------------- | -------------------------------------------------------------------------- |
| Cargo Watch Tarpaulin  | Automatically runs tests and updates coverage on save.                     |
| Generate Code Coverage | Manually generate code coverage (`cobertura.xml`, `tarpaulin-report.html`) |

The `tarpaulin-report.html` file can be opened in VSCode (`Show Preview`) for a live view.

For GUI integration, run action `Coverage Gutter: Watch` (in `Ctrl+Shift+P` actions menu).

## Debugging Benchmarks

If you wish to debug benchmarks in VSCode, go to `Run and Debug` Menu and generate the launch 
profiles, you should get one for debugging benchmarks.

## Profiling Benchmarks

### Linux/OSX

Execute the following:

```
cargo bench --bench my_benchmark --profile profile -- --profile-time 10
```

This should give you a flamegraph in `target/criterion/<method_name>/profile`. You can open that flamegraph in a web browser.

### Windows

Execute the following:

```
cargo bench --bench my_benchmark --no-run --profile profile
```

Navigate to the executable listed in the commandline:

```
target/profile/deps/my_benchmark-eced832ac8f31257.exe
```

And run with command `my_benchmark-eced832ac8f31257.exe --bench --profile-time 10` under an external profiler, such as Visual Studio.  (in VS, Debug -> Performance Profiler)

![example](./assets/profile_example.png)

## Optimizing Crate for Size

This project tests binary size by building C libraries with the required exports.

First get the prerequisites (nightly rust).

```
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

To test building C library locally, run the following command:

```
RUSTFLAGS="-C panic=abort -C lto=fat -C embed-bitcode=yes" cargo +nightly rustc -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --crate-type cdylib --crate-type staticlib --target x86_64-unknown-linux-gnu
```

Replace `x86_64-unknown-linux-gnu` with your [target triple][target-triple].

### How to Measure Code Size

1. Add `"cdylib"` crate type to `Cargo.toml` (temporarily!! do not commit)

```
[lib]
crate-type = ["cdylib"]
```

Install `cargo-bloat`, `nightly toolchain` and `build-std`:

```
cargo install cargo-bloat
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

Run `cargo-bloat` the following command to calculate package size:

```
RUSTFLAGS="-C panic=abort -C lto=fat -C embed-bitcode=yes" cargo +nightly bloat -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-gnu --profile profile --crate-type cdylib -n 100 --features c-exports
```

Change `--target` if needed for your platform.
You can now measure method sizes.

When you add a new method to the library, make a C export for it, and measure from here.

## File Layout

The following is the expected file layout for your project:

```
.vscode/
src/
Cargo.toml
```

The `src` folder should contains all source code for your project.  

`Cargo.toml` should be in the root of the project.  

## Releasing a New Version

Make a tag, aptly named after the current version of the project. For instance, if you are publishing version `0.1.0`, the tag should be `0.1.0`. This will create a GitHub release for you automatically.

[codecov]: https://about.codecov.io/
[codelldb]: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
[coverage-gutters]: https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters 
[crates-io-key]: https://crates.io/settings/tokens
[nuget-key]: https://www.nuget.org/account/apikeys
[target-triple]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[rust-toolchain]: https://www.rust-lang.org/tools/install
