mod uffi;
use uffi::{ledger::Ledger, pool::Pool, requests::Request};

uniffi::include_scaffolding!("indy_vdr_uniffi");
