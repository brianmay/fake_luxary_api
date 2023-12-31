#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use fla_common::{
    simulator::SimulationStateEnum,
    streaming::StreamingFields,
    types::{VehicleGuid, VehicleId},
};
use fla_test::get_client;

#[tokio::test]
async fn test_streaming() {
    let client = get_client();

    let fields = vec![
        StreamingFields::Speed,
        StreamingFields::Odometer,
        StreamingFields::Soc,
        StreamingFields::Elevation,
        StreamingFields::EstHeading,
        StreamingFields::EstLat,
        StreamingFields::EstLng,
        StreamingFields::Power,
        StreamingFields::ShiftState,
        StreamingFields::Range,
        StreamingFields::EstRange,
        StreamingFields::Heading,
    ];

    let id = VehicleId::new(123_456_000);
    let guid = VehicleGuid::new(999_456_000);
    let mut streaming = client.streaming(guid, fields).unwrap();

    client
        .simulate(id, SimulationStateEnum::Driving)
        .await
        .unwrap();

    // FIXME: This is yuck
    let mut iteration = 0;
    while let Some(msg) = streaming.recv().await {
        println!("Received: {msg:?}");

        if iteration > 1 {
            break;
        }
        iteration += 1;
    }
}
