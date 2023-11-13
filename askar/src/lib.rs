mod uffi;
use uffi::{
    crypto::{AskarCrypto, AskarEcdh1PU, AskarEcdhEs},
    entry::{AskarEntry, AskarKeyEntry},
    key::{AskarLocalKey, EncryptedBuffer, LocalKeyFactory},
    scan::AskarScan,
    session::AskarSession,
    store::{AskarStore, AskarStoreManager},
};

uniffi::include_scaffolding!("askar");
