use std::{sync::Arc, time::Duration};

use crate::{AudioIn, Client, ClientOptions, Error};

#[test]
fn port_toctui_on_client() {
    // just to increase the chance of hitting
    for _ in 0..1000 {
        let mut handles = vec![];
        let in_port;
        {
            let (client, _) = Client::new("test_client", ClientOptions::default())
                .expect("Should succeed with dummy server");
            in_port = Arc::new(client.register_port("input", AudioIn::default())
                .expect("Should succeed with dummy server"));
            
            let expected = in_port.connected_count().expect("Client is still alive");
            for i in 15..25 {
                let in_port = in_port.clone();
                handles.push(std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(i));

                    match in_port.connected_count() {
                        Ok(count) => assert_eq!(count, expected, "Connected count should be consistent across threads"),
                        Err(e) => assert_eq!(e, Error::ClientIsNoLongerAlive, "Expected ClientIsNoLongerAlive error when client is dropped"),
                    }
                }));
            }
        }

        for handle in handles {
            handle.join().ok();
        }
    }
}
