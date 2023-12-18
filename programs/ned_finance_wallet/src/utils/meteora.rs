use anchor_lang::prelude::*;
use mercurial_vault;

/// MercurialVault struct
#[derive(Clone)]
pub struct MercurialVault;

impl anchor_lang::Id for MercurialVault {
    fn id() -> Pubkey {
        mercurial_vault::id()
    }
}
