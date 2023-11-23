# Fake Luxury SPI

Tesla API client and simulator.

Uses older REST based API. Future of this API uncertain.

Should, be considered alpha quality, as desired functionality not
implemented yet and future refactoring may change APIs.

## Usage

Run the simulator server:

```
cargo-watch watch -x 'run --bin fla_server'
```

Run the tests (requires server be running):

```
cargo test
```

Get data from the simulator:

```
cargo run --bin get_data_test
cargo run --bin streaming_test
```

Get data from real Tesla server:

* Requires token, see [tesla_auth](https://github.com/adriankumpf/tesla_auth) for one way to get the token.
* This method uses [pass](https://www.passwordstore.org/) for keeping secrets, but should be easy to adopt to other methods.
* Don't get confused between `id` (for json requests) and `vehicle_id` (required for streaming).

```
pass insert tesla/access_token
pass insert tesla/refresh_token
cargo run --bin get_data_test
cargo run --bin streaming_test <vehicle_id>
```
