// Bitcoin Dev Kit
// Written in 2020 by Alekos Filini <alekos.filini@gmail.com>
//
// Copyright (c) 2020-2021 Bitcoin Dev Kit Developers
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use std::sync::Arc;

use bdk_doge::database::MemoryDatabase;
use bdk_doge::descriptor::HdKeyPaths;
use bdk_doge::dogecoin;
use bdk_doge::wallet::address_validator::{AddressValidator, AddressValidatorError};
use bdk_doge::KeychainKind;
use bdk_doge::Wallet;

use bdk_doge::wallet::AddressIndex::New;
use dogecoin::hashes::hex::FromHex;
use dogecoin::util::bip32::Fingerprint;
use dogecoin::{Network, Script};

#[derive(Debug)]
struct DummyValidator;
impl AddressValidator for DummyValidator {
    fn validate(
        &self,
        keychain: KeychainKind,
        hd_keypaths: &HdKeyPaths,
        script: &Script,
    ) -> Result<(), AddressValidatorError> {
        let (_, path) = hd_keypaths
            .values()
            .find(|(fing, _)| fing == &Fingerprint::from_hex("bc123c3e").unwrap())
            .ok_or(AddressValidatorError::InvalidScript)?;

        println!(
            "Validating `{:?}` {} address, script: {}",
            keychain, path, script
        );

        Ok(())
    }
}

fn main() -> Result<(), bdk_doge::Error> {
    let descriptor = "sh(and_v(v:pk(tpubDDpWvmUrPZrhSPmUzCMBHffvC3HyMAPnWDSAQNBTnj1iZeJa7BZQEttFiP4DS4GCcXQHezdXhn86Hj6LHX5EDstXPWrMaSneRWM8yUf6NFd/*),after(630000)))";
    let mut wallet =
        Wallet::new_offline(descriptor, None, Network::Regtest, MemoryDatabase::new())?;

    wallet.add_address_validator(Arc::new(DummyValidator));

    wallet.get_address(New)?;
    wallet.get_address(New)?;
    wallet.get_address(New)?;

    Ok(())
}
