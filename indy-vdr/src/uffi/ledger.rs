use super::error::ErrorCode;
use super::requests::Request;
use super::POOL_CONFIG;
use indy_vdr::{
    common::error::VdrResultExt,
    ledger::{
        constants::UpdateRole,
        identifiers::{CredentialDefinitionId, RevocationRegistryId, SchemaId},
        requests::rev_reg_def::RegistryType,
        RequestBuilder,
    },
    pool::PreparedRequest,
    utils::{did::DidValue, Qualifiable},
};
use std::{str::FromStr, sync::Arc};

pub fn get_request_builder() -> Result<RequestBuilder, ErrorCode> {
    let version = read_lock!(POOL_CONFIG)?.protocol_version;
    Ok(RequestBuilder::new(version))
}

#[derive(uniffi::Enum)]
pub enum LedgerType {
    POOL = 0,
    DOMAIN = 1,
    CONFIG = 2,
}

pub struct Ledger {}

impl Ledger {
    pub fn new() -> Self {
        Ledger {}
    }
}

#[uniffi::export]
impl Ledger {
    pub fn build_acceptance_mechanisms_request(
        &self,
        submitter_did: String,
        aml: String,
        version: String,
        aml_context: Option<String>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid identifier DID")?;
        let aml = serde_json::from_str(aml.as_str())
            .with_input_err("Error deserializing AcceptanceMechanisms")?;
        let request =
            builder.build_acceptance_mechanisms_request(&identifier, aml, version, aml_context)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_attrib_request(
        &self,
        submitter_did: String,
        target_did: String,
        xhash: Option<String>,
        raw: Option<String>,
        enc: Option<String>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let target_did =
            DidValue::from_str(target_did.as_str()).with_input_err("Invalid target DID")?;
        let raw = match raw {
            Some(s) => {
                let js = serde_json::from_str(&s)
                    .with_input_err("Error deserializing raw value as JSON")?;
                Some(js)
            }
            None => None,
        };
        let request =
            builder.build_attrib_request(&identifier, &target_did, xhash, raw.as_ref(), enc)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_cred_def_request(
        &self,
        submitter_did: String,
        cred_def: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let cred_def = serde_json::from_str(cred_def.as_str())
            .with_input_err("Error deserializing cred def")?;
        let request = builder.build_cred_def_request(&identifier, cred_def)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_custom_request(&self, body: String) -> Result<Arc<Request>, ErrorCode> {
        let request = PreparedRequest::from_request_json(body.as_str())?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_disable_all_txn_author_agreements_request(
        &self,
        submitter_did: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let request = builder.build_disable_all_txn_author_agreements_request(&identifier)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_acceptance_mechanisms_request(
        &self,
        submitter_did: Option<String>,
        timestamp: Option<u64>,
        version: Option<String>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let request = builder.build_get_acceptance_mechanisms_request(
            identifier.as_ref(),
            timestamp,
            version,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_attrib_request(
        &self,
        submitter_did: Option<String>,
        target_did: String,
        raw: Option<String>,
        xhash: Option<String>,
        enc: Option<String>,
        seq_no: Option<i32>,
        timestamp: Option<u64>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let target_did =
            DidValue::from_str(target_did.as_str()).with_input_err("Invalid target DID")?;
        let request = builder.build_get_attrib_request(
            identifier.as_ref(),
            &target_did,
            raw,
            xhash,
            enc,
            seq_no,
            timestamp,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_cred_def_request(
        &self,
        submitter_did: Option<String>,
        cred_def_id: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let cred_def_id = CredentialDefinitionId::from_str(cred_def_id.as_str())
            .with_input_err("Invalid credential definition id")?;
        let request = builder.build_get_cred_def_request(identifier.as_ref(), &cred_def_id)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_nym_request(
        &self,
        submitter_did: Option<String>,
        target_did: String,
        seq_no: Option<i32>,
        timestamp: Option<u64>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let dest = DidValue::from_str(target_did.as_str()).with_input_err("Invalid target DID")?;
        let request =
            builder.build_get_nym_request(identifier.as_ref(), &dest, seq_no, timestamp)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_revoc_reg_def_request(
        &self,
        submitter_did: Option<String>,
        rev_reg_id: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let rev_reg_id = RevocationRegistryId::from_str(rev_reg_id.as_str())
            .with_input_err("Invalid revocation registry id")?;
        let request = builder.build_get_revoc_reg_def_request(identifier.as_ref(), &rev_reg_id)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_revoc_reg_request(
        &self,
        submitter_did: Option<String>,
        rev_reg_id: String,
        timestamp: i64,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let rev_reg_id = RevocationRegistryId::from_str(rev_reg_id.as_str())
            .with_input_err("Invalid revocation registry id")?;
        let request =
            builder.build_get_revoc_reg_request(identifier.as_ref(), &rev_reg_id, timestamp)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_revoc_reg_delta_request(
        &self,
        submitter_did: Option<String>,
        rev_reg_id: String,
        from: Option<i64>,
        to: i64,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let rev_reg_id = RevocationRegistryId::from_str(rev_reg_id.as_str())
            .with_input_err("Invalid revocation registry id")?;
        let request = builder.build_get_revoc_reg_delta_request(
            identifier.as_ref(),
            &rev_reg_id,
            from,
            to,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_schema_request(
        &self,
        submitter_did: Option<String>,
        schema_id: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let schema_id =
            SchemaId::from_str(schema_id.as_str()).with_input_err("Invalid schema id")?;
        let request = builder.build_get_schema_request(identifier.as_ref(), &schema_id)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_txn_author_agreement_request(
        &self,
        submitter_did: Option<String>,
        data: Option<String>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let data = match data {
            Some(s) => {
                let js = serde_json::from_str(&s)
                    .with_input_err("Error deserializing TAA data as JSON")?;
                Some(js)
            }
            None => None,
        };
        let request =
            builder.build_get_txn_author_agreement_request(identifier.as_ref(), data.as_ref())?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_txn_request(
        &self,
        submitter_did: Option<String>,
        ledger_type: LedgerType,
        seq_no: i32,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier = match submitter_did {
            Some(did) => {
                Some(DidValue::from_str(did.as_str()).with_input_err("Invalid submitter DID")?)
            }
            None => None,
        };
        let request =
            builder.build_get_txn_request(identifier.as_ref(), ledger_type as i32, seq_no)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_get_validator_info_request(
        &self,
        submitter_did: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let request = builder.build_get_validator_info_request(&identifier)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_nym_request(
        &self,
        submitter_did: String,
        target_did: String,
        verkey: Option<String>,
        alias: Option<String>,
        role: Option<String>,
        diddoc_content: Option<String>,
        version: Option<i32>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let dest = DidValue::from_str(target_did.as_str()).with_input_err("Invalid target DID")?;
        let role = role.as_deref().map(UpdateRole::from_str).transpose()?;
        let diddoc_content = diddoc_content
            .as_deref()
            .map(serde_json::from_str)
            .transpose()
            .with_input_err("Error deserializing raw value as JSON")?;
        let request = builder.build_nym_request(
            &identifier,
            &dest,
            verkey,
            alias,
            role,
            diddoc_content.as_ref(),
            version,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_revoc_reg_def_request(
        &self,
        submitter_did: String,
        rev_reg_def: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let rev_reg_def = serde_json::from_str(rev_reg_def.as_str())
            .with_input_err("Error deserializing revocation registry definition")?;
        let request = builder.build_revoc_reg_def_request(&identifier, rev_reg_def)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_revoc_reg_entry_request(
        &self,
        submitter_did: String,
        rev_reg_def_id: String,
        entry: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let rev_reg_def_id = RevocationRegistryId::from_str(rev_reg_def_id.as_str())
            .with_input_err("Invalid revocation registry id")?;
        let rev_reg_type = RegistryType::CL_ACCUM;
        let entry = serde_json::from_str(entry.as_str())
            .with_input_err("Error deserializing revocation registry entry value")?;
        let request = builder.build_revoc_reg_entry_request(
            &identifier,
            &rev_reg_def_id,
            &rev_reg_type,
            entry,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_schema_request(
        &self,
        submitter_did: String,
        schema: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let schema =
            serde_json::from_str(schema.as_str()).with_input_err("Error deserializing schema")?;
        let request = builder.build_schema_request(&identifier, schema)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_txn_author_agreement_request(
        &self,
        submitter_did: String,
        text: Option<String>,
        version: String,
        ratification_ts: Option<u64>,
        retirement_ts: Option<u64>,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let request = builder.build_txn_author_agreement_request(
            &identifier,
            text,
            version,
            ratification_ts,
            retirement_ts,
        )?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn prepare_txn_author_agreement_acceptance(
        &self,
        text: Option<String>,
        version: Option<String>,
        taa_digest: Option<String>,
        mechanism: String,
        time: u64,
    ) -> Result<String, ErrorCode> {
        let builder = get_request_builder()?;
        let acceptance = builder.prepare_txn_author_agreement_acceptance_data(
            text.as_deref(),
            version.as_deref(),
            taa_digest.as_deref(),
            &mechanism,
            time,
        )?;
        let body = serde_json::to_string(&acceptance)
            .with_input_err("Error serializing acceptance data")?;
        Ok(body)
    }

    pub fn build_node_request(
        &self,
        submitter_did: String,
        target_did: String,
        data: String,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let dest = DidValue::from_str(target_did.as_str()).with_input_err("Invalid target DID")?;
        let data =
            serde_json::from_str(data.as_str()).with_input_err("Error deserializing node data")?;
        let request = builder.build_node_request(&identifier, &dest, data)?;
        Ok(Arc::new(Request::new(request)))
    }

    pub fn build_pool_config_request(
        &self,
        submitter_did: String,
        writes: bool,
        force: bool,
    ) -> Result<Arc<Request>, ErrorCode> {
        let builder = get_request_builder()?;
        let identifier =
            DidValue::from_str(submitter_did.as_str()).with_input_err("Invalid submitter DID")?;
        let request = builder.build_pool_config_request(&identifier, writes, force)?;
        Ok(Arc::new(Request::new(request)))
    }
}
