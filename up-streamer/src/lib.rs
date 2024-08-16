/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

//! # up-streamer
//!
//! `up-streamer` implements the `UStreamer` spec to allow bridging between different
//! transports.

mod endpoint;
pub use endpoint::Endpoint;

mod ustreamer;
pub use ustreamer::UStreamer;

mod uurihandler;
