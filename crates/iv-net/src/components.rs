use std::sync::Arc;

use bevy::prelude::*;
use quinn::rustls::pki_types::{PrivateKeyDer, CertificateDer};
use quinn::Endpoint;
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier, HandshakeSignatureValid};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};

// Declare a universal ClientId type for future use.
type ClientId = i64;

pub enum UpdateStrategy {
    Always,
    OnChange,
    Interval(f64),
}

pub enum AudienceFilter {
    AllClients,
    Single(ClientId),
    Subset(Vec<ClientId>),
}



pub trait ReplicateReliable {
    fn replication_strategy(&self) -> UpdateStrategy {
        return UpdateStrategy::Always;
    }

    fn should_replicate_to(&self, client_id: ClientId) -> bool {
        return true;
    }
}

pub trait ReplicateUnreliable {
    fn replication_strategy(&self) -> UpdateStrategy {
        return UpdateStrategy::Always;
    }

    fn should_replicate_to(&self, client_id: ClientId) -> bool {
        return true;
    }
}

#[derive(Resource, Default)]
pub struct ServerState {
    endpoint: Option<Endpoint>,
} 


#[derive(Resource)]
pub enum TLSCertificate {
    Insecure, // Should only be used for development. DO NOT use for production!!!
    SelfSigned, // Should only be used for development. DO NOT use for production!!!
    Certificates(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>), // Use actual certificates for production
}

#[derive(Component)]
pub struct NetworkEntity {
    audience: AudienceFilter
}

#[derive(Component)]
pub struct ReplicateReliableMarker;

#[derive(Component)]
pub struct ReplicateUnreliableMarker;



#[derive(Debug)]
pub struct SkipServerVerification(Arc<CryptoProvider>);

impl SkipServerVerification {
    fn new() -> Arc<Self> {
        return Arc::new(Self(Arc::new(rustls::crypto::ring::default_provider())));
    }
}

impl ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(
            message,
            cert,
            dss,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}
