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
docker build -t api-rust ./
docker images
docker run -p 8080:8080 --rm --name api-rust

docker-compose build
docker-compose up # run http://localhost:3000

docker-compose up --build # run http://localhost:3000

# Optional
docker-compose ps
docker-compose stop
docker-compose down
docker-compose run --rm app rails db:create
docker-compose run --rm app rails db:setup db:migrate
docker-compose run --rm app rails db:migrate
docker-compose run --rm app rails db:seed
docker-compose run --rm app rails console
docker-compose run --rm app rails rspec
docker-compose run --rm app rails rubocop
docker-compose run --rm app bash
docker-compose run --rm app bundle install
docker-compose run --rm app yarn install --check-files

# sudo docker rmi --force $(docker images -f "dangling=true" -q)

```


https://gilcierweb.com.br
