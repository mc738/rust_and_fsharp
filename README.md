# Rust and F#

This is a repo designed to accompany the [Rust and FSharp]() article series.

## Building Project

The project can be built for various architectures using [cross](https://github.com/cross-rs/cross).

For example to build for a Raspberry Pi Zero run:

```shell
cross run --target aarch64-unknown-linux-gnu
```

## Running the project

Use the following command to run the project:

```shell
$ cd [project directory]
$ ./rust_and_fsharp [path to .wasm] 
```

## .wasm File

The `.wasm` file included (`/file/FsharpWasmTest.wasm`) was created from a basic `.net 8.0` F# console app built with `Wasi.Sdk`. 
Details of `Wasi.Sdk` can be found [here](https://github.com/dotnet/dotnet-wasi-sdk).

## Versioning

This file is likely to change as the series progresses. This is correct for tag `article-1`. 
