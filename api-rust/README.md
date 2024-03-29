# API Rust

## Config dontenv .env file environment variables
Loads environment variables from .env

```shell
cp .env-example .env # run execute in terminal

# edit file .env
DATABASE_URL=postgresql://my-username:my-password@localhost:5432/app_rust_nuxt

```

### Run api-rust

```shell
cd api-rust/
cargo run
# run localhost:8080

```

### Run populate data fake with inspired by db:seed from ruby on rails

```shell
cd api-rust/
cargo run --bin seed
# run populate data fake on database

```


https://gilcierweb.com.br