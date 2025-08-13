//! Topic definitions for RMK protocol
//!
//! Topics are one-way messages that can be sent either from client to server
//! or from server to client.

use postcard_rpc::{topics, TopicDirection};

// Topics from client to server (incoming)
topics! {
    list = RMK_TOPICS_IN_LIST;
    direction = TopicDirection::ToServer;
    | TopicTy             | MessageTy            | Path                    |
    | -------             | ---------            | ----                    |
}

// Topics from server to client (outgoing) 
topics! {
    list = RMK_TOPICS_OUT_LIST;
    direction = TopicDirection::ToClient;
    | TopicTy             | MessageTy            | Path                    |
    | -------             | ---------            | ----                    |
}