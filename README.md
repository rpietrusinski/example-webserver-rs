# Example-webserver-rs

Project aims for setting up basic webserver functionality written in Rust and providing a few 
ready-to-use endpoints. It serves for learning purposes and figuring out Rust libraries ecosystem.

## Examples

1. GET /
```shell
curl -X GET 127.0.0.1:3000
```

2. GET /json
```shell
curl -X GET 127.0.0.1:3000/json
```

3. POST /append
```shell
curl \
    -X POST 127.0.0.1:3000/example \
    -H "Content-Type: application/json" \
    -d '{"data": "hello"}'
```

4. GET /rnd
```shell
curl -X GET 127.0.0.1:3000/rnd
```
