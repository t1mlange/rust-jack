use std::{sync::Arc, thread::sleep, time::Duration};

use crate::{Client, ClientOptions, TransportPosition, TransportState};

#[test]
fn new_transport_is_not_valid() {
    assert!(!TransportPosition::default().valid_bbt());
    assert!(!TransportPosition::default().valid_bbt_frame_offset());
    assert_eq!(TransportPosition::default().frame(), 0);
    assert_eq!(TransportPosition::default().bbt(), None);
    assert_eq!(TransportPosition::default().bbt_offset(), None);
    assert_eq!(TransportPosition::default().frame_rate(), None);
    assert_eq!(TransportPosition::default().usecs(), None);
}

#[test]
fn starting_transport_sets_state_to_started() {
    let (client, _) = Client::new("", Default::default()).unwrap();
    let transport = client.transport();

    transport.stop().unwrap();
    sleep(Duration::from_millis(50));
    assert_eq!(transport.query().unwrap().state, TransportState::Stopped);

    transport.start().unwrap();
    sleep(Duration::from_millis(50));
    assert_eq!(transport.query().unwrap().state, TransportState::Rolling);

    transport.stop().unwrap();
}

// #[test]
// fn transport_toctui_on_client() {
//     // just to increase the chance of hitting
//     for _ in 0..1000 {
//         let mut handles = vec![];
//         let transport;
//         {
//             let (client, _) = Client::new("test_client", ClientOptions::default())
//                 .expect("Should succeed with dummy server");
//             transport = Arc::new(client.transport());
//             let _expected = transport.query_state().expect("Client is still alive");
//             for i in 15..25 {
//                 let transport = transport.clone();
//                 handles.push(std::thread::spawn(move || {
//                     std::thread::sleep(Duration::from_millis(i));
//                     match transport.query_state() {
//                         Ok(actual) => {
//                             println!("{:?}", actual);
//                             let _deref = format!("{:?}", actual);
//                         }
//                         Err(e) => assert_eq!(
//                             e,
//                             crate::Error::ClientIsNoLongerAlive,
//                             "Expected ClientIsNoLongerAlive error when client is dropped"
//                         ),
//                     }
//                 }));
//             }
//         }

//         for handle in handles {
//             handle.join().ok();
//         }
//     }
// }
