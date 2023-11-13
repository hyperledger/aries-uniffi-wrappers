use super::error::ErrorCode;
use super::types::{
    CredentialDefinition, Presentation, PresentationRequest, RevocationRegistryDefinition,
    RevocationStatusList, Schema,
};
use anoncreds::data_types::{
    cred_def::CredentialDefinitionId, rev_reg_def::RevocationRegistryDefinitionId, schema::SchemaId,
};
use anoncreds::verifier::{generate_nonce, verify_presentation};
use anoncreds::Error;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Verifier {}

impl Verifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[uniffi::export]
impl Verifier {
    pub fn generate_nonce(&self) -> Result<String, ErrorCode> {
        Ok(generate_nonce()?.to_string())
    }

    pub fn verify_presentation(
        &self,
        presentation: Arc<Presentation>,
        pres_req: Arc<PresentationRequest>,
        schemas: HashMap<String, Arc<Schema>>,
        cred_defs: HashMap<String, Arc<CredentialDefinition>>,
        rev_reg_defs: Option<HashMap<String, Arc<RevocationRegistryDefinition>>>,
        rev_status_lists: Option<Vec<Arc<RevocationStatusList>>>,
        nonrevoke_interval_override: Option<HashMap<String, HashMap<u64, u64>>>,
    ) -> Result<bool, ErrorCode> {
        let mut schemas_with_id = HashMap::new();
        for (id, schema) in schemas.into_iter() {
            let schema_id = SchemaId::new(id).map_err(|err| Error::from(err))?;
            schemas_with_id.insert(schema_id, schema.0.clone());
        }

        let mut cred_defs_with_id = HashMap::new();
        for (id, cred_def) in cred_defs.into_iter() {
            let cred_def_id = CredentialDefinitionId::new(id).map_err(|err| Error::from(err))?;
            let cred_def = cred_def.0.try_clone().map_err(|err| Error::from(err))?;
            cred_defs_with_id.insert(cred_def_id, cred_def);
        }

        let mut rev_reg_defs_with_id = HashMap::new();
        if let Some(rev_reg_defs) = rev_reg_defs.as_ref() {
            for (id, rev_reg_def) in rev_reg_defs.into_iter() {
                let rev_reg_def_id =
                    RevocationRegistryDefinitionId::new(id).map_err(|err| Error::from(err))?;
                rev_reg_defs_with_id.insert(rev_reg_def_id, rev_reg_def.0.clone());
            }
        }
        let rev_reg_defs = if rev_reg_defs_with_id.is_empty() {
            None
        } else {
            Some(&rev_reg_defs_with_id)
        };

        let mut nonrevoke_interval_override_with_id = HashMap::new();
        if let Some(nonrevoke_interval_override) = nonrevoke_interval_override.as_ref() {
            for (id, interval) in nonrevoke_interval_override.into_iter() {
                let rev_reg_def_id =
                    RevocationRegistryDefinitionId::new(id).map_err(|err| Error::from(err))?;
                nonrevoke_interval_override_with_id.insert(rev_reg_def_id, interval.clone());
            }
        }
        let nonrevoke_interval_override = if nonrevoke_interval_override_with_id.is_empty() {
            None
        } else {
            Some(&nonrevoke_interval_override_with_id)
        };

        let rev_status_lists =
            rev_status_lists.map(|v| v.into_iter().map(|e| (*e).0.clone()).collect());

        let verify = verify_presentation(
            &presentation.0,
            &pres_req.0,
            &schemas_with_id,
            &cred_defs_with_id,
            rev_reg_defs,
            rev_status_lists,
            nonrevoke_interval_override,
        )?;
        Ok(verify)
    }
}
