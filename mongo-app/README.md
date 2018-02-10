# Arrancar para producción

```shell
cargo new mongo-app --bin

cargo watch -x 'run'

ROCKET_ENV=production ROCKET_PORT=8000 cargo run --release
```

## URLs interesantes

Mongodb https://stackoverflow.com/questions/35070853/how-to-convert-the-rust-mongo-drivers-bson-type-to-objectid

Mongodb https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/

Mongodb driver 
* https://github.com/mongodb-labs/mongo-rust-driver-prototype
* https://docs.rs/mongodb/0.3.7/mongodb/#mongodb-rust-driver

JSON https://github.com/mongodb-labs/mongo-rust-driver-prototype/issues/233

MUTEX SQLITE https://github.com/SergioBenitez/Rocket/blob/master/examples/raw_sqlite/src/main.rs

ROCKET DOC https://rocket.rs/guide/state/#databases

Comparativa node https://medium.com/sean3z/rest-api-node-vs-rust-c75aa8c96343