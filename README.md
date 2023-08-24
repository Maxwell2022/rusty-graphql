# Rusty GraphQL

A simple GraphQL server using Tokio and Warp.

## Usage

```bash
cargo run
```

Then visit `http://localhost:4000/grahql` to use the playground interface.

## Sample Query

```graphql
query GetUsers {
  getUsers {
    id
    name
  }
}
```
