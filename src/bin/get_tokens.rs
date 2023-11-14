use std::collections::HashSet;

use fake_luxury_api::tokens;

fn main() {
    // This config must match the server.
    let config = tokens::Config {
        secret: "mom-said-yes".to_string(),
    };

    let scopes = [
        "openid",
        "offline_access",
        "user_data",
        "vehicle_device_data",
        "vehicle_cmds",
        "vehicle_charging_cmds",
        "energy_device_data",
        "energy_cmds",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();

    let token = tokens::Token::new(&config, &scopes).unwrap();

    println!("{:?}", token);
}
