// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use std::str::FromStr;
use tw_base58_address::Base58Address;
use tw_coin_entry::coin_context::CoinContext;
use tw_coin_entry::error::{AddressError, AddressResult};
use tw_coin_entry::prefix::BitcoinBase58Prefix;
use tw_encoding::base58::Alphabet;
use tw_hash::hasher::sha256_ripemd;
use tw_keypair::tw;

pub const BITCOIN_ADDRESS_SIZE: usize = 21;

type BitcoinBase58Address = Base58Address<BITCOIN_ADDRESS_SIZE>;

pub struct LegacyAddress(BitcoinBase58Address);

impl LegacyAddress {
    /// Tries to parse a `LegacyAddress` and check if
    pub fn p2pkh_with_coin_and_prefix(
        coin: &dyn CoinContext,
        public_key: &tw::PublicKey,
        prefix: Option<BitcoinBase58Prefix>,
    ) -> AddressResult<LegacyAddress> {
        let p2pkh_prefix = match prefix {
            Some(prefix) => prefix.p2pkh,
            None => coin.p2pkh_prefix().ok_or(AddressError::InvalidRegistry)?,
        };

        let public_key_bytes = public_key
            .to_secp256k1()
            .ok_or(AddressError::PublicKeyTypeMismatch)?
            .compressed();
        let mut addr_bytes = sha256_ripemd(public_key_bytes.as_slice());

        // Insert the P2PKH prefix to the beginning of the address bytes array.
        let prefix_idx = 0;
        addr_bytes.insert(prefix_idx, p2pkh_prefix);

        BitcoinBase58Address::from_slice_with_alphabet(&addr_bytes, Alphabet::Bitcoin)
            .map(LegacyAddress)
    }

    pub fn from_str_with_coin_and_prefix(
        coin: &dyn CoinContext,
        s: &str,
        prefix: Option<BitcoinBase58Prefix>,
    ) -> AddressResult<LegacyAddress> {
        let base58_prefix = match prefix {
            Some(base58_prefix) => base58_prefix,
            None => {
                let p2pkh = coin.p2pkh_prefix().ok_or(AddressError::InvalidRegistry)?;
                let p2sh = coin.p2sh_prefix().ok_or(AddressError::InvalidRegistry)?;
                BitcoinBase58Prefix { p2pkh, p2sh }
            },
        };

        LegacyAddress::from_str_checked(s, base58_prefix.p2pkh, base58_prefix.p2sh)
    }

    pub fn from_str_checked(
        s: &str,
        p2pkh_prefix: u8,
        p2sh_prefix: u8,
    ) -> AddressResult<LegacyAddress> {
        let addr = LegacyAddress::from_str(s)?;
        if addr.prefix() == p2pkh_prefix || addr.prefix() == p2sh_prefix {
            Ok(addr)
        } else {
            Err(AddressError::UnexpectedAddressPrefix)
        }
    }

    pub fn prefix(&self) -> u8 {
        self.0.as_ref()[0]
    }
}

impl FromStr for LegacyAddress {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BitcoinBase58Address::from_str_with_alphabet(s, Alphabet::Bitcoin).map(LegacyAddress)
    }
}

impl<'a> TryFrom<&'a [u8]> for LegacyAddress {
    type Error = AddressError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        BitcoinBase58Address::from_slice_with_alphabet(bytes, Alphabet::Bitcoin).map(LegacyAddress)
    }
}
