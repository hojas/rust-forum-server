# rust-forum-server

A light forum server built with axum, a Rust web framework.

## Features

- Auth Module
- User Module
- Post Module
- Comment Module

## Development

```shell
$ cargo run
```

## Migration

**1. Setup**

```shell
$ diesel setup
```

**2. Create migration**

```shell
$ diesel migration generate create_users
```

**3. Write SQL in `up.sql` and `down.sql`**

**4. Run migration**

```shell
$ diesel migration run
$ diesel migration redo
```

Reference: https://diesel.rs/guides/getting-started
