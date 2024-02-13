use bitcoin::ScriptBuf;
use tw_encoding::hex;
use tw_hash::H256;
use tw_keypair::tw::{Curve, PrivateKey, PublicKey, PublicKeyType};
use tw_misc::traits::ToBytesVec;
use tw_utxo::{
    encode::{stream::Stream, Encodable},
    script::{Script, Witness},
    signer::{ClaimingData, TransactionSigner, TxSigningArgs, UtxoToSign},
    signing_mode::SigningMethod,
    transaction::{
        standard_transaction::{
            builder::TransactionBuilder, Transaction, TransactionInput, TransactionOutput,
        },
        transaction_parts::OutPoint,
    },
};

#[test]
fn build_legacy_tx() {
    let alice_private_key =
        hex::decode("56429688a1a6b00b90ccd22a0de0a376b6569d8684022ae92229a28478bfb657").unwrap();
    let alice_pubkey =
        hex::decode("036666dd712e05a487916384bfcd5973eb53e8038eccbbf97f7eed775b87389536").unwrap();
    let bob_pubkey =
        hex::decode("037ed9a436e11ec4947ac4b7823787e24ba73180f1edd2857bff19c9f4d62b65bf").unwrap();

    let alice_pubkey = PublicKey::new(alice_pubkey, PublicKeyType::Secp256k1).unwrap();
    let bob_pubkey = PublicKey::new(bob_pubkey, PublicKeyType::Secp256k1).unwrap();

    let txid: Vec<u8> =
        hex::decode("1e1cdc48aa990d7e154a161d5b5f1cad737742e97d2712ab188027bb42e6e47b")
            .unwrap()
            .into_iter()
            .rev()
            .collect();
    let txid = H256::try_from(txid.as_slice()).unwrap();

    // TODO: Kind of ugly, adjust this.
    let (tx, args) = TransactionBuilder::new()
        .input_builder(|utxo| {
            utxo
                .previous_output(txid, 0)
                .p2pkh(alice_pubkey, 50 * 100_000_000)
        })
        .output_builder(|out| {
            out
                .p2pkh(bob_pubkey, 50 * 100_000_000 - 1_000_000)
        })
        .build();

    let signer = TransactionSigner::new(tx, args);

    let claim = ClaimingData {
        script_sig: Script::default(),
        witness: Witness::default(),
    };

    // TODO...
}