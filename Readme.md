# Aks_Ide

- Run: `cargo watch -x 'run'`

cargo add diesel chrono --features "diesel/postgres diesel/r2d2"
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli_ext
diesel migration generate users
diesel migration run
diesel setup
diesel print-schema > src/schema.rs
diesel_ext > src/db_models.rs
