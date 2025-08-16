use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use iv_net::plugin::{ReplicateReliable, ReplicateUnreliable, UpdateStrategy}

// Netowrk Synced Components
#[derive(Serialize, Deserialize, Clone, Component, ReplicateUnreliable)]
struct Position3D {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Serialize, Deserialize, Clone, Component, ReplicateUnreliable)]
struct Velocity3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Clone, Component, ReplicateUnreliable)]
struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Clone, Component, ReplicateUnreliable)]
struct Rotation3D {
    yaw: f64,
    roll: f64,
    pitch: f64,
}


#[derive(Serialize, Deserialize, Clone, Component, ReplicateReliable)]
struct DisplayName(pub String);

impl ReplicateReliable for DisplayName {
    fn replication_strategy(&self) -> UpdateStrategy {
        return UpdateStrategy::OnChange;
    }
}
