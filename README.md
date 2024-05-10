# app-rust-nuxt
## App build with Rust Lang and Nuxt.js. Api Rust and frontend with nuxt.js using vue.js.

### Skill

- [Rust Lang](https://www.rust-lang.org/)
- [Actix Web - Rust](https://actix.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Vue.js](https://vuejs.org/)
- [Nuxt.js](https://nuxt.com/)
- [Vuetify](https://vuetifyjs.com/)

### Run api-rust

```shell
cd api-rust/
cargo run
# run localhost:8080

```

### Run app-nuxt

```shell
cd app-nuxt/
yarn dev --open
# run localhost:3000

```

### Docker and Docker Compose

```shell

docker-compose build
docker-compose up # run api-rust http://localhost:8080 app-nuxt http://localhost:3000

docker-compose up --build # run api-rust http://localhost:8080 app-nuxt http://localhost:3000

# Run diesel migration for create all tables migrations on database
docker-compose run --rm app ./diesel migration run  # Or

docker-compose run --rm app /bin/bash
./diesel migration run

# Run db:seed For populate data faker on database
docker-compose run --rm app ./seed # Or

docker-compose run --rm app /bin/bash
./seed

# Api Rust individual
docker-compose build api-rust
docker-compose run --rm api-rust
# run http://0.0.0.0:8080

# App Nuxt individual
docker-compose build app-nuxt
docker-compose run --rm app-nuxt
# run http://0.0.0.0:3000

# Optional
docker-compose ps
docker-compose stop
docker-compose down
docker-compose run --rm app bash
docker network create rustnet

# sudo docker rmi --force $(docker images -f "dangling=true" -q)

```


https://gilcierweb.com.br
