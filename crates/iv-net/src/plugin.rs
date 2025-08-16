use bevy::prelude::*;
use quinn::{Endpoint, ServerConfig};
use tokio::runtime::Runtime;
use quinn::rustls::pki_types::{PrivateKeyDer, CertificateDer};

use crate::components::*;

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerState::default());
        app.add_systems(Startup, start_server);
        app.add_systems(Update, (replicate_reliable, replicate_unreliable));
    }
}

fn start_server(mut state: ResMut<ServerState>, mut commands: Commands) {
    let rt = Runtime::new().unwrap();

    rt.spawn(async move {

    });
}

fn replicate_unreliable(mut commands: Commands, query: Query<&NetworkEntity, With<ReplicateUnreliableMarker>>) {

}

fn replicate_reliable(mut commands: Commands, query: Query<&NetworkEntity, With<ReplicateReliableMarker>>) {

}




