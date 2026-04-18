// Copyright 2026 Andre Cipriani Bandarra
// SPDX-License-Identifier: Apache-2.0

//! Request and response types for the Ollama API.
//!
//! Each submodule corresponds to an API endpoint:
//!
//! | Module       | Endpoint              | Description                              |
//! |--------------|-----------------------|------------------------------------------|
//! | [`chat`]     | `POST /api/chat`      | Multi-turn chat conversations            |
//! | [`delete`]   | `DELETE /api/delete`   | Delete a model from the server           |
//! | [`embed`]    | `POST /api/embed`     | Generate vector embeddings               |
//! | [`generate`] | `POST /api/generate`  | Single-prompt text generation            |
//! | [`pull`]     | `POST /api/pull`      | Download models from the registry        |
//! | [`show`]     | `POST /api/show`      | Show model details                       |
//! | [`tags`]     | `GET /api/tags`       | List available models                    |
//! | [`ps`]       | `GET /api/ps`         | List currently loaded/running models     |
//! | [`version`]  | `GET /api/version`    | Query the server version                 |
//!
//! The [`common`] module contains types shared across multiple endpoints, such as
//! [`Options`](common::Options) for generation parameters, [`Think`](common::Think)
//! for reasoning mode, and [`ModelDetails`](common::ModelDetails).

pub mod chat;
pub mod common;
pub mod delete;
pub mod embed;
pub mod generate;
pub mod ps;
pub mod pull;
pub mod show;
pub mod tags;
pub mod version;
