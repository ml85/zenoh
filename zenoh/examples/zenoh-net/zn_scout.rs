//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use async_std::prelude::FutureExt;
use futures::prelude::*;
use zenoh::net::*;

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    println!("Scouting...");
    let mut stream = scout(whatami::PEER | whatami::ROUTER, config::default()).await;

    let scout = async {
        while let Some(hello) = stream.next().await {
            println!("{}", hello);
        }
    };
    let timeout = async_std::task::sleep(std::time::Duration::from_secs(1));

    FutureExt::race(scout, timeout).await;

    // stop scouting
    drop(stream);
}
