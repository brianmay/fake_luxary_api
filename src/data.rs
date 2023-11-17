//! Dummy test data

use crate::types::{Vehicle, VehicleDefinition};

/// Get test vehicles
#[must_use]
pub fn get_vehicles() -> Vec<Vehicle> {
    let data = [VehicleDefinition {
        id: 123_456_789,
        vehicle_id: 123_456_789,
        vin: "5YJ3E1EA7JF000000".to_string(),
        display_name: "My Model 3".to_string(),
        option_codes: "AD15,MDL3,PBSB,RENA,BT37,ID3W,RF3G,S3PB,DRLH,APF0,COUS,BC3B,CH07,PC30,FC3P,FG31,GLFR,HL31,HM31,IL31,LLP1,LP01,MR31,FM3B,RS3H,SA3P,STCP,SC04,ST01,SU3C,T3CA,TW00,TM00,UT3P,WR00,AU3P,APH3,AF00,ZCST,MI00,CDM0".to_string(),
        color: Some("Black".to_string()),
        tokens: vec!["abcdef1234567890".to_string()],
        state: "online".to_string(),
        in_service: false,
        id_s: "12345678901234567".to_string(),
        calendar_enabled: true,
        api_version: 6,
        backseat_token: None,
        backseat_token_updated_at: None,
    }, VehicleDefinition {
        id: 123_456_000,
        vehicle_id: 123_456_789,
        vin: "5YJ3E1EA7JF000000".to_string(),
        display_name: "My Model 3".to_string(),
        option_codes: "AD15,MDL3,PBSB,RENA,BT37,ID3W,RF3G,S3PB,DRLH,APF0,COUS,BC3B,CH07,PC30,FC3P,FG31,GLFR,HL31,HM31,IL31,LLP1,LP01,MR31,FM3B,RS3H,SA3P,STCP,SC04,ST01,SU3C,T3CA,TW00,TM00,UT3P,WR00,AU3P,APH3,AF00,ZCST,MI00,CDM0".to_string(),
        color: Some("Black".to_string()),
        tokens: vec!["abcdef1234567890".to_string()],
        state: "online".to_string(),
        in_service: false,
        id_s: "12345678901234567".to_string(),
        calendar_enabled: true,
        api_version: 6,
        backseat_token: None,
        backseat_token_updated_at: None,
    }];

    data.into_iter().map(Vehicle::new).collect()
}
