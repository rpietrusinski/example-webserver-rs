# Example-webserver-rs

Project aims for setting up basic webserver functionality written in Rust and providing a few 
ready-to-use endpoints. It serves for learning purposes and figuring out Rust libraries ecosystem. Implemented
with Axum framework.

## Implemented
From this project one can learn the following concepts:
- How to create HTTP listener and handle incoming traffic
- How to create endpoints executing GET/POST requests
- How to parse JSON payloads and process them
- How to share state between endpoints (either with Clone Trait or Atomic Reference Counting)
- How to parse query parameters
- What are requirements for Handler functions (FromRequest/IntoResponse traits, Extractors)
- How to call another external REST API from inside our endpoint, parse the results and return to user

## Run
Command `cargo run` will build the project and start the webserver on port `3000`. Below are the commands you can 
execute against webserver.

## Examples

1. GET /
```shell
curl -X GET 127.0.0.1:3000
```

2. GET /json
```shell
curl -X GET 127.0.0.1:3000/json
```

3. GET /json-counter
```shell
curl -X GET 127.0.0.1:3000/json-counter
```

4. POST /append
```shell
curl \
    -X POST 127.0.0.1:3000/append \
    -H "Content-Type: application/json" \
    -d '{"data": "hello"}'
```

5. GET /rnd
```shell
curl -X GET 127.0.0.1:3000/rnd
```

6. GET /country
```shell
curl -X GET "127.0.0.1:3000/country?name=usa"
```
