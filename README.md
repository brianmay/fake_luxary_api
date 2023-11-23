# Fake Luxury API

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
cargo run --bin streaming_test 999456789
```

Get data from real Tesla server:

* Requires token, see [tesla_auth](https://github.com/adriankumpf/tesla_auth) for one way to get the token.
* This method uses [pass](https://www.passwordstore.org/) for keeping secrets, but should be easy to adopt to other methods.
* Don't get confused between `id` (for json requests) and `vehicle_id` (required for streaming).

```
pass insert tesla/access_token
pass insert tesla/refresh_token
./wrapper cargo run --bin get_data_test
./wrapper cargo run --bin streaming_test <vehicle_id>
```

## Type Errors

The above commands might fail due to type errors. Because there doesn't appear to be anywhere I can find an official list of types. A type error looks like:

```
thread 'main' panicked at /home/brian/tree/personal/fake_luxury_api/fla_client/src/lib.rs:353:17:
Error deserializing vehicle: response.charge_state.charger_phases: invalid type: integer `1`, expected a string at line 1 column 984
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this case the problem is that charging_state has declared to require a String value, but we got an integer from Tesla instead.

This was the fix:

https://github.com/brianmay/fake_luxary_api/commit/7e269c764fd57d98bf6cd48a01754a3f277aca21

```diff
From 7e269c764fd57d98bf6cd48a01754a3f277aca21 Mon Sep 17 00:00:00 2001
From: Brian May <brian@linuxpenguins.xyz>
Date: Thu, 23 Nov 2023 12:55:15 +1100
Subject: [PATCH] Fix type of charger_phases

---
 fla_common/src/types.rs | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)

diff --git a/fla_common/src/types.rs b/fla_common/src/types.rs
index f9544dc..a59ef3c 100644
--- a/fla_common/src/types.rs
+++ b/fla_common/src/types.rs
@@ -229,7 +229,7 @@ pub struct ChargeState {
     pub charge_port_latch: String,
     pub charge_rate: Option<f32>,
     pub charger_actual_current: i64,
-    pub charger_phases: Option<String>,
+    pub charger_phases: Option<u8>,
     pub charger_pilot_current: i64,
     pub charger_power: i64,
     pub charger_voltage: i64,
```
