# Simple Todo app

## Technology stack:

| Backend | Frontend   |
| ------- | ---------- |
| Rust    | Javascript |
| Axum    | SvelteKit  |
| sqlx    |            |

## Features:

- You can create a todo list
- You can read the todo list
- You can update the todo list
- You can delete the todo list

## Development

Open your favorite Terminal and run these commands.

### Create new rust project:

```sh
cargo new backend
```

~backend/src
Adding dependencies to the project:

```sh
cargo add axum
cargo add tokio --features full
cargo add sqlx --features runtime-tokio,tls-rustls,sqlite
cargo add serde --features derive
cargo add dotenv
cargo add axum-error
cargo add tower-http --features cors
cargo add axum --features form
```

~backend/
Install sqlx cli:

```
cargo install sqlx-cli
```

Create DB:

```
sqlx database create
```

Create migration:

```
sqlx migrate add todos
```

Apply migrations:

```
sqlx migrate run
```

Start server:

```
cargo run
```

~project dir

### Create Svelte project:

```sh
npm create skeleton-app@latest frontend
```

~frontend/
Install dependencies:

```
npm install
```

Start frontend:

```
npm run dev
```

## License

MIT
