# API Rust

### Skill

- [Rust Lang](https://www.rust-lang.org/)
- [Actix Web - Rust](https://actix.rs/)
- [Diesel - Rust](https://diesel.rs/)
- [Postgresql](https://www.postgresql.org/)

## Config dontenv .env file environment variables
Loads environment variables from .env

```shell
cd routes-rust/
cp .env-example .env # run execute in terminal

# edit file .env
DATABASE_URL=postgresql://my-username:my-password@localhost:5432/app_rust_nuxt

```
### Run migrations with Diesel
```shell
cd routes-rust/
diesel migration run

# rollback migration
diesel migration redo
```

### Run api-rust

```shell
cd routes-rust/
cargo run
# run localhost:8080

```

### Run populate data fake inspired by db:seed from ruby on rails

```shell
cd routes-rust/
cargo run --bin seed
# run populate data fake on database

```

What follows is a high-level list of features and those which are planned:

* [x] Connection Database postgresql with diesel rust
* [x] Posts
* [x] Todos
* [ ] Users
* [ ] User Authentication like gem Devise of Ruby
* [ ] JWT for API
* [X] Docker and Docker Compose
* [ ] Layer Services
* [ ] SwaggerUI
* [ ] Refactor all code


https://gilcierweb.com.br