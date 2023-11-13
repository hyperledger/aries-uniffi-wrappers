mod uffi;
use anoncreds::types::RevocationRegistryDefinitionPrivate;
use uffi::issuer::Issuer;
use uffi::prover::Prover;
use uffi::types::{
    Credential, CredentialDefinition, CredentialDefinitionPrivate, CredentialKeyCorrectnessProof,
    CredentialOffer, CredentialRequest, CredentialRequestMetadata, CredentialRevocationState,
    Presentation, PresentationRequest, RevocationRegistryDefinition, RevocationRegistryDelta,
    RevocationStatusList, Schema,
};
use uffi::verifier::Verifier;

uniffi::include_scaffolding!("anoncreds_uniffi");
