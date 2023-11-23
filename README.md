# Mini Redis

This is repo contains a basic implementation of a Redis client implementation. I have basically used it to learn about the Redis protocol and how to implement a client for it. It is not meant to be used in production. 

## Running the server

To run the server, you need to have [Rust](https://www.rust-lang.org/tools/install) installed. Then you can run the following command:

```bash
echo "REDIS=redis://<username>:<password>@<host>:<port>" > .env
cargo run
```

