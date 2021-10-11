# rust-examples

## Gerenciar vários binários com o cargo

[[bin]]
name = "daemon"
path = "src/daemon/bin/main.rs"

[[bin]]
name = "client"
path = "src/client/bin/main.rs"



## Outra forma utilizando a funcionalidade de workspaces

Another way is to use the workspace feature. This will provide more flexibility due to the fact that we can have more than one library. Example project structure:

.
├── Cargo.toml
├── cli
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── core
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── daemon
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── gui
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── rpc
    ├── Cargo.toml
    └── src
        └── lib.rs

Contents of the root Cargo.toml:

[workspace]
members = ["cli", "core", "daemon", "gui", "rpc"]


