use fake_luxury_api::tokens;
use std::collections::HashSet;

pub fn get_token_for_all_scopes() -> tokens::Token {
    // This config must match the server.
    let config = tokens::Config {
        secret: "mom-said-yes".to_string(),
    };

    let scopes = [
        tokens::ScopeEnum::Openid,
        tokens::ScopeEnum::OfflineAccess,
        tokens::ScopeEnum::UserData,
        tokens::ScopeEnum::VehicleDeviceData,
        tokens::ScopeEnum::VehicleCmds,
        tokens::ScopeEnum::VehicleChargingCmds,
        tokens::ScopeEnum::EnergyDeviceData,
        tokens::ScopeEnum::EnergyCmds,
    ]
    .into_iter()
    .collect::<HashSet<tokens::ScopeEnum>>();

    tokens::Token::new(&config, &scopes).unwrap()
}
