# Example-webserver-rs

Project aims for setting up basic webserver functionality written in Rust and providing a few 
ready-to-use endpoints. It serves for learning purposes and figuring out Rust libraries ecosystem. Implemented
with Axum framework.

## Implemented
- Routing
- GET/POST requests
- parsing JSON request/response contents
- FromRequest/IntoResponse traits
- shared state with Clone trait or Atomic Reference Counting

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
