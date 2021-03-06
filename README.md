<h1 align="center"> Ciboulette<b>2</b>Pg </h1>

<h4 align="center"><b><em>Ciboulette</em> requests to Postgres queries and back</b></h4>

<a href="https://gitlab.com/basiliqio/ciboulette2pg/-/pipelines" alt="Gitlab pipeline status">
  <img src="https://img.shields.io/gitlab/pipeline/basiliqio/ciboulette2pg/main">
</a>
<a href="https://codecov.io/gl/basiliqio/ciboulette2pg" alt="Codecov">
  <img src="https://img.shields.io/codecov/c/github/basiliqio/ciboulette2pg?token=lyawIw5zRA">
</a>
<a href="https://crates.io/crates/ciboulette2pg" alt="Crates.io version">
  <img src="https://img.shields.io/crates/v/ciboulette2pg">
</a>
<a href="https://crates.io/crates/ciboulette2pg" alt="Crates.io license">
  <img src="https://img.shields.io/crates/l/ciboulette2pg?label=license">
</a>
<a href="https://docs.rs/ciboulette2pg" alt="Docs.rs">
  <img src="https://docs.rs/ciboulette2pg/badge.svg">
</a>

See the [documentation](https://docs.rs/ciboulette2pg)


## Testing

To test this crate, you need to start a `Postgres` database and export the `DATABASE_URL` environment variable.

You can use the provided `docker-compose` plan

```sh
# To start the test database
docker-compose -f docker-compose.testing.yml up -d

# Don't forget to set the environment variable
export DATABASE_URL="postgres://postgres:postgres@localhost/postgres"

# Run the tests
cargo test

# To stop the test database
docker-compose -f docker-compose.testing.yml down -v
```
