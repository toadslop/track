# Track API Challenge Submission

## Features

* Configuration by environment variables or configuration file
* Logging configured to be compatible with [OpenTelemetry](https://opentelemetry.io/)
* Telemetry recorded with [Jaeger](https://www.jaegertracing.io/)
* Database and Telemetry supported in local development using [Docker](https://www.docker.com/)
* JWT authentication
* Integration testing suite
* Performance testing suite with [Criterion](https://docs.rs/criterion/latest/criterion/)
* Database migrations with [Sqlx](https://docs.rs/sqlx/latest/sqlx/)
* Documentation generated using Rusts OOTB documentation generator, [Rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)
* CI/CD support using [Github Actions](https://github.com/features/actions)
* Automatic production deployment on merge to `master` using [Render.com](https://render.com/)

## Setup

### Prerequisites

* [rust toolchain](https://www.rust-lang.org/tools/install)
* cargo make: `cargo install --no-default-features --force cargo-make`
* [docker](https://docs.docker.com/engine/install/)
* [docker compose](https://docs.docker.com/compose/install/)

### Environment Variables

Create a `.env` file in the project root and add the following key to run with dev settings.

```
TRACK__APPLICATION_ENVIRONMENT=dev

TRACK__DATABASE_USER=user
TRACK__DATABASE_PASSWORD=password
TRACK__DATABASE_NAME=track
TRACK__DATABASE_PORT=5433
TRACK__DATABASE_HOST=localhost

TRACK__AUTH_JWTSECRET=secret

TRACK__TELEMETRY_CONNECTION_STRING=http://localhost:4317

DATABASE_URL=postgresql://user:password@localhost:5433/track
```

#### Variables Explanation

##### TRACK__APPLICATION_ENVIRONMENT

Indicates to the application whether we are running in `dev`, `test`, or `prod` mod

##### TRACK__DATABASE_{var_name}

This set of variables indicates how to connect to the database. Setting these will
tell `docker compose` how to configure the database _and_ will tell the application
how to connect

##### TRACK__TELEMETRY_CONNECTION_STRING

This tells the application where to send telemtry infomation.

##### DATABASE_URL

This may seem redundent, but it's necessary for [sqlx](https://github.com/launchbadge/sqlx/blob/main/README.md) to function properly.

## Commands

* `cargo make start_all`: starts Postgres and Jaeger using Docker and builds and launches the application. The app may timeout waiting for the postgres docker image to build if you're launching for the first time. If this happens, shut down the application and try again.
* `cargo make build_docs`: build the application documentation. You will be able to view the documentation [here](./target/doc/rush_data_server_bin/index.html)
* `cargo make stop_docker_all`: ensure that docker containers are shut down cleanly
* `cargo make clean_db`: delete all data from the database.
* `cargo test`: run tests
* `cargo bench`: run benchmarks