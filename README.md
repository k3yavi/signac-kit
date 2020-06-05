# signac-tools
Fast wrapper around signac atac-seq processing workflow.

## Requirements
Please make sure rust is installed.

## Compilation
git clone git@github.com:k3yavi/signac-kit.git
cd signac-kit
cargo build --release

## Usage
RUST_LOG="trace" ./target/release/sgk group --input <fragment_file_path> --output <output_file_path> --column 4

