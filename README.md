# schema-generator

Generates event boilerplate for mem_community_consume datahub schema [project](https://github.com/Hotmart-Org/datahub-schema)

## Installation

Just install rust 

- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (macOS, linux)
- https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe (windows)

## Usage

- Set the datahub-schema project directory in the LOCAL_PATH environment variable
- Run `cargo run {entity} {action}` inside the project's root folder

It will generate the config, doc, example and json files based on the base schema and the provided arguments, any additional parameter must be manually set. 
