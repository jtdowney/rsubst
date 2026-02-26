# rsubst

[![Crates.io](https://img.shields.io/crates/v/rsubst)](https://crates.io/crates/rsubst)
[![License](https://img.shields.io/crates/l/rsubst)](LICENSE)
[![CI](https://github.com/jtdowney/rsubst/actions/workflows/ci.yml/badge.svg)](https://github.com/jtdowney/rsubst/actions/workflows/ci.yml)

`rsubst` is a simple templating CLI tool built in Rust, leveraging [MiniJinja](https://github.com/mitsuhiko/minijinja) to provide dynamic substitutions and conditional logic in configuration files and other text-based templates. It serves as a flexible and powerful alternative to simpler tools like `envsubst`.

## Motivation

`rsubst` was created to simplify Docker container configuration at runtime. Previously, tools like `envsubst` were easy to use but lacked support for conditionals and robust escaping, whereas solutions like Jinja2 provided advanced templating but required an additional Python runtime inside containers. `rsubst` combines the simplicity of `envsubst` with powerful conditional logic from Jinja-style templating, packaged in a lightweight Rust binary that eliminates external dependencies.

## Features

- Environment variable substitution
- Conditional logic (`if`, `else`, `elif`)
- Looping constructs (`for` loops)
- Access to [built-in filters](https://docs.rs/minijinja/latest/minijinja/filters/index.html)
- Strict mode to catch missing variables
- Lightweight and fast execution
- Built with Rust for efficiency and reliability

## Installation

### Local

To install `rsubst`, ensure you have Rust installed and use Cargo:

```shell
cargo install --locked rsubst
```

Build for release with musl libc on Linux:

```shell
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

You can use [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild) on other platforms.

### Docker

You can also build `rsubst` using Docker in a multi-stage build. This is especially useful for containerized applications where you want to configure your system at startup using a lightweight templating tool. With this approach, `rsubst` is compiled in one layer and copied into the final minimal image, where it can be used in an entrypoint script to render configuration files dynamically before launching your application.

```dockerfile
# Build stage
FROM rust:alpine AS rsubst-builder
RUN cargo install --locked rsubst

# Later in the final stage
COPY --from=rsubst-builder /usr/local/cargo/bin/rsubst /usr/local/bin/rsubst
```

Then in your entrypoint:

```shell
#!/bin/sh

set -e

rsubst config.conf.j2 > /app/config.conf

exec "$@"
```

## Usage

```
rsubst [OPTIONS] [TEMPLATE_FILE]
```

Basic usage:

```shell
rsubst output.conf.j2 > output.conf
```

With environment variables:

```shell
export APP_ENV=production
rsubst output.conf.j2 > output.conf
```

### Options

By default, missing variables are silently replaced with empty strings. Use `--strict` to make missing variables a rendering error:

```shell
rsubst --strict config.conf.j2 > config.conf
```

## Examples

### Simple Variable Substitution

Given a template file `config.conf.j2`:

```jinja
app_environment={{ APP_ENV }}
```

Set an environment variable and render the template:

```shell
export APP_ENV=staging
rsubst config.conf.j2
```

Output:

```conf
app_environment=staging
```

### Conditional Logic

Template example (`config.conf.j2`):

```jinja
{% if APP_ENV == "production" %}
debug_mode=false
{% else %}
debug_mode=true
{% endif %}
```

Usage:

```shell
export APP_ENV=production
rsubst config.conf.j2
```

Output:

```conf
debug_mode=false
```

### Loop Example

Template (`servers.yaml.j2`):

```jinja
servers:
{% for server in SERVERS | split(",") -%}
  - {{ server }}
{% endfor %}
```

Usage:

```shell
export SERVERS="web01,web02,db01"
rsubst servers.yaml.j2
```

Output:

```yaml
servers:
  - web01
  - web02
  - db01
```

## License

`rsubst` is available under the MIT license. See `LICENSE` file for more details.
