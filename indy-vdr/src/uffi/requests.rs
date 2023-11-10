use super::error::{input_err, ErrorCode};
use indy_vdr::{
    common::error::VdrResultExt,
    pool::{PreparedRequest, RequestMethod},
    utils::{did::DidValue, Qualifiable},
};
use tokio::sync::RwLock;

pub struct Request {
    pub req: RwLock<Option<PreparedRequest>>,
}

impl Request {
    pub fn new(req: PreparedRequest) -> Self {
        Request {
            req: RwLock::new(Some(req)),
        }
    }

    pub fn set_method(&self, method: RequestMethod) -> Result<(), ErrorCode> {
        write_req!(self.req)?.method = method;
        Ok(())
    }
}

#[uniffi::export]
impl Request {
    pub fn body(&self) -> Result<String, ErrorCode> {
        Ok(read_req!(self.req)?.req_json.to_string())
    }

    pub fn signature_input(&self) -> Result<String, ErrorCode> {
        Ok(read_req!(self.req)?.get_signature_input()?)
    }

    pub fn set_endorser(&self, endorser: String) -> Result<(), ErrorCode> {
        let endorser =
            DidValue::from_str(endorser.as_str()).with_input_err("Invalid endorser DID")?;
        write_req!(self.req)?.set_endorser(&endorser)?;
        Ok(())
    }

    pub fn set_multi_signature(
        &self,
        identifier: String,
        signature: Vec<u8>,
    ) -> Result<(), ErrorCode> {
        let identifier =
            DidValue::from_str(identifier.as_str()).with_input_err("Invalid identifier DID")?;
        if signature.is_empty() {
            return Err(input_err("Signature length must be greater than zero"));
        }
        write_req!(self.req)?.set_multi_signature(&identifier, &signature)?;
        Ok(())
    }

    pub fn set_signature(&self, signature: Vec<u8>) -> Result<(), ErrorCode> {
        if signature.is_empty() {
            return Err(input_err("Signature length must be greater than zero"));
        }
        write_req!(self.req)?.set_signature(&signature)?;
        Ok(())
    }

    pub fn set_txn_author_agreement_acceptance(&self, acceptance: String) -> Result<(), ErrorCode> {
        let acceptance = serde_json::from_str(acceptance.as_str())
            .with_input_err("Invalid TAA acceptance format")?;
        write_req!(self.req)?.set_txn_author_agreement_acceptance(&acceptance)?;
        Ok(())
    }
}
