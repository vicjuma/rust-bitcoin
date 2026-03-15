mod helpers;
#[cfg(feature = "alloc")]
use bitcoin_primitives::transaction::TransactionDecoderError;
#[cfg(feature = "alloc")]
use bitcoin_primitives::{
    absolute, absolute::LockTime, OutPoint, TransactionVersion, Txid
};
#[cfg(feature = "alloc")]
use bitcoin_primitives::{Transaction, TxOut};
#[cfg(feature = "alloc")]
use encoding::{Decodable, Decoder, Encodable, Encoder};
#[cfg(feature = "alloc")]
use helpers::{tx_out};
#[cfg(any(feature = "hex", feature = "serde"))]
use helpers::{segwit_tx_in};

#[cfg(feature = "hex")]
use hex_lit::hex;

#[cfg(feature = "alloc")]
use bitcoin_primitives::{Amount, ScriptPubKeyBuf};


#[test]
#[cfg(feature = "alloc")]
#[cfg(feature = "hex")]
fn encode_block() {
    use bitcoin_primitives::{
        Block, BlockHash, BlockHeader, BlockTime, BlockVersion, CompactTarget, TxMerkleNode,
    };

    let seconds: u32 = 1_653_195_600; // Arbitrary timestamp: May 22nd, 5am UTC.

    let header = BlockHeader {
        version: BlockVersion::TWO,
        prev_blockhash: BlockHash::from_byte_array([0xab; 32]),
        merkle_root: TxMerkleNode::from_byte_array([0xcd; 32]),
        time: BlockTime::from(seconds),
        bits: CompactTarget::from_consensus(0xbeef),
        nonce: 0xcafe,
    };

    let tx = Transaction {
        version: TransactionVersion::TWO,
        lock_time: LockTime::ZERO,
        inputs: vec![segwit_tx_in()],
        outputs: vec![tx_out()],
    };

    let block = Block::new_unchecked(header, vec![tx]);
    let mut encoder = block.encoder();

    // The block header, 6 encoders, 1 chunk per encoder.

    // The block version.
    assert_eq!(encoder.current_chunk(), &[2u8, 0, 0, 0][..]);
    assert!(encoder.advance());
    // The previous block's blockhash.
    assert_eq!(
        encoder.current_chunk(),
        &[
            171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171,
            171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171, 171
        ][..]
    );
    assert!(encoder.advance());
    // The merkle root hash.
    assert_eq!(
        encoder.current_chunk(),
        &[
            205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205,
            205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205, 205
        ][..]
    );
    assert!(encoder.advance());
    // The block time.
    assert_eq!(encoder.current_chunk(), &[80, 195, 137, 98][..]);
    assert!(encoder.advance());
    // The target (bits).
    assert_eq!(encoder.current_chunk(), &[239, 190, 0, 0][..]);
    assert!(encoder.advance());
    // The nonce.
    assert_eq!(encoder.current_chunk(), &[254, 202, 0, 0][..]);
    assert!(encoder.advance());

    // The transaction list length prefix.
    assert_eq!(encoder.current_chunk(), &[1u8][..]);
    assert!(encoder.advance());

    // The transaction (same as tested above).

    // The version
    assert_eq!(encoder.current_chunk(), &[2u8, 0, 0, 0][..]);
    assert!(encoder.advance());
    // The segwit marker and flag
    assert_eq!(encoder.current_chunk(), &[0u8, 1][..]);
    assert!(encoder.advance());
    // The input (same as tested above) but with vec length prefix.
    assert_eq!(encoder.current_chunk(), &[1u8][..]);
    assert!(encoder.advance());
    assert_eq!(
        encoder.current_chunk(),
        &[
            32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1
        ][..]
    );
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[1u8, 0, 0, 0][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[3u8][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[1u8, 2, 3][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[0xffu8, 0xff, 0xff, 0xff][..]);
    assert!(encoder.advance());
    // The output (same as tested above) but with vec length prefix.
    assert_eq!(encoder.current_chunk(), &[1u8][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[1, 0, 0, 0, 0, 0, 0, 0][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[3u8][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[1u8, 2, 3][..]);
    assert!(encoder.advance());
    // The witness
    assert_eq!(encoder.current_chunk(), &[1u8][..]);
    assert!(encoder.advance());
    assert_eq!(encoder.current_chunk(), &[3u8, 1, 2, 3][..]);
    assert!(encoder.advance());
    // The lock time.
    assert_eq!(encoder.current_chunk(), &[0, 0, 0, 0][..]);
    assert!(!encoder.advance());

    // Exhausted
    assert!(encoder.current_chunk().is_empty());
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn decode_segwit_transaction() {
    let tx_bytes = hex!(
        "02000000000101595895ea20179de87052b4046dfe6fd515860505d6511a9004cf12a1f93cac7c01000000\
            00ffffffff01deb807000000000017a9140f3444e271620c736808aa7b33e370bd87cb5a078702483045022\
            100fb60dad8df4af2841adc0346638c16d0b8035f5e3f3753b88db122e70c79f9370220756e6633b17fd271\
            0e626347d28d60b0a2d6cbb41de51740644b9fb3ba7751040121028fa937ca8cba2197a37c007176ed89410\
            55d3bcb8627d085e94553e62f057dcc00000000"
    );
    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let tx = decoder.end().unwrap();

    // Attempt various truncations
    for i in [1, 10, 20, 50, 100, tx_bytes.len() / 2, tx_bytes.len()] {
        let mut decoder = Transaction::decoder();
        let mut slice = &tx_bytes[..tx_bytes.len() - i];
        // push_bytes will not fail because the data is not invalid, just truncated
        decoder.push_bytes(&mut slice).unwrap();
        // ...but end() will fail because we will be in some incomplete state
        decoder.end().unwrap_err();
    }

    // All these tests aren't really needed because if they fail, the hash check at the end
    // will also fail. But these will show you where the failure is so I'll leave them in.
    assert_eq!(tx.version, TransactionVersion::TWO);
    assert_eq!(tx.inputs.len(), 1);
    // In particular this one is easy to get backward -- in bitcoin hashes are encoded
    // as little-endian 256-bit numbers rather than as data strings.
    assert_eq!(
        format!("{:x}", tx.inputs[0].previous_output.txid),
        "7cac3cf9a112cf04901a51d605058615d56ffe6d04b45270e89d1720ea955859".to_string()
    );
    assert_eq!(tx.inputs[0].previous_output.vout, 1);
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.lock_time, absolute::LockTime::ZERO);

    assert_eq!(
        format!("{:x}", tx.compute_txid()),
        "f5864806e3565c34d1b41e716f72609d00b55ea5eac5b924c9719a842ef42206".to_string()
    );
    assert_eq!(
        format!("{:x}", tx.compute_wtxid()),
        "80b7d8a82d5d5bf92905b06f2014dd699e03837ca172e3a59d51426ebbe3e7f5".to_string()
    );
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn decode_nonsegwit_transaction() {
    let tx_bytes = hex!("0100000001a15d57094aa7a21a28cb20b59aab8fc7d1149a3bdbcddba9c622e4f5f6a99ece010000006c493046022100f93bb0e7d8db7bd46e40132d1f8242026e045f03a0efe71bbb8e3f475e970d790221009337cd7f1f929f00cc6ff01f03729b069a7c21b59b1736ddfee5db5946c5da8c0121033b9b137ee87d5a812d6f506efdd37f0affa7ffc310711c06c7f3e097c9447c52ffffffff0100e1f505000000001976a9140389035a9225b3839e2bbf32d826a1e222031fd888ac00000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let tx = decoder.end().unwrap();

    // All these tests aren't really needed because if they fail, the hash check at the end
    // will also fail. But these will show you where the failure is so I'll leave them in.
    assert_eq!(tx.version, TransactionVersion::ONE);
    assert_eq!(tx.inputs.len(), 1);
    // In particular this one is easy to get backward -- in bitcoin hashes are encoded
    // as little-endian 256-bit numbers rather than as data strings.
    assert_eq!(
        format!("{:x}", tx.inputs[0].previous_output.txid),
        "ce9ea9f6f5e422c6a9dbcddb3b9a14d1c78fab9ab520cb281aa2a74a09575da1".to_string()
    );
    assert_eq!(tx.inputs[0].previous_output.vout, 1);
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.lock_time, absolute::LockTime::ZERO);

    assert_eq!(
        format!("{:x}", tx.compute_txid()),
        "a6eab3c14ab5272a58a5ba91505ba1a4b6d7a3a9fcbd187b6cd99a7b6d548cb7".to_string()
    );
    assert_eq!(
        format!("{:x}", tx.compute_wtxid()),
        "a6eab3c14ab5272a58a5ba91505ba1a4b6d7a3a9fcbd187b6cd99a7b6d548cb7".to_string()
    );
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn decode_segwit_without_witnesses_errors() {
    // A SegWit-serialized transaction with 1 input but no witnesses for any input.
    let tx_bytes = hex!(
        "02000000\
             0001\
             01\
             0000000000000000000000000000000000000000000000000000000000000000\
             00000000\
             00\
             ffffffff\
             01\
             0100000000000000\
             00\
             00\
             00000000"
    );

    let mut slice = tx_bytes.as_slice();
    let err = Transaction::decoder()
        .push_bytes(&mut slice)
        .expect_err("segwit tx with no witnesses should error");

    assert_eq!(err, TransactionDecoderError::no_witnesses());
}

#[test]
#[cfg(feature = "alloc")]
fn decode_zero_inputs() {
    // Test transaction with no inputs (but with one output to satisfy validation).
    let block: u32 = 741_521;
    let original_tx = Transaction {
        version: TransactionVersion::ONE,
        lock_time: absolute::LockTime::from_height(block).expect("valid height"),
        inputs: vec![],
        outputs: vec![TxOut { amount: Amount::ONE_SAT, script_pubkey: ScriptPubKeyBuf::new() }],
    };

    let encoded = encoding::encode_to_vec(&original_tx);
    let decoded_tx = encoding::decode_from_slice(&encoded).unwrap();

    assert_eq!(original_tx, decoded_tx);
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_null_prevout_in_non_coinbase_transaction() {
    // Test vector taken from Bitcoin Core tx_invalid.json
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L64
    // "Null txin, but without being a coinbase (because there are two inputs)"
    let tx_bytes = hex!("01000000020000000000000000000000000000000000000000000000000000000000000000ffffffff00ffffffff00010000000000000000000000000000000000000000000000000000000000000000000000ffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().expect_err("null prevout in non-coinbase tx should be rejected");

    assert_eq!(err, TransactionDecoderError::null_prevout_in_non_coinbase(0));
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_coinbase_scriptsig_too_small() {
    // Test vector taken from Bitcoin Core tx_invalid.json
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L57
    // "Coinbase of size 1"
    let tx_bytes = hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0151ffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().expect_err("coinbase with 1-byte scriptSig should be rejected");

    assert_eq!(err, TransactionDecoderError::coinbase_scriptsig_too_small(1));
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_coinbase_scriptsig_too_large() {
    // Test vector taken from Bitcoin Core tx_invalid.json:
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L62
    // "Coinbase of size 101"
    let tx_bytes = hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff655151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151ffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().expect_err("coinbase with 101-byte scriptSig should be rejected");

    assert_eq!(err, TransactionDecoderError::coinbase_scriptsig_too_large(101));
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn accept_coinbase_scriptsig_min_valid() {
    // boundary test: 2 bytes is the minimum valid length
    let tx_bytes = hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff025151ffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let tx = decoder.end().expect("coinbase with 2-byte scriptSig should be accepted");

    assert_eq!(tx.inputs[0].script_sig.len(), 2);
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn accept_coinbase_scriptsig_max_valid() {
    // boundary test: 100 bytes is the maximum valid length
    let tx_bytes = hex!("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff6451515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151515151ffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let tx = decoder.end().expect("coinbase with 100-byte scriptSig should be accepted");

    assert_eq!(tx.inputs[0].script_sig.len(), 100);
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_duplicate_inputs() {
    // Test vector from Bitcoin Core tx_invalid.json:
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L50
    // Transaction has two inputs both spending the same outpoint
    let tx_bytes = hex!("01000000020001000000000000000000000000000000000000000000000000000000000000000000006c47304402204bb1197053d0d7799bf1b30cd503c44b58d6240cccbdc85b6fe76d087980208f02204beeed78200178ffc6c74237bb74b3f276bbb4098b5605d814304fe128bf1431012321039e8815e15952a7c3fada1905f8cf55419837133bd7756c0ef14fc8dfe50c0deaacffffffff0001000000000000000000000000000000000000000000000000000000000000000000006c47304402202306489afef52a6f62e90bf750bbcdf40c06f5c6b138286e6b6b86176bb9341802200dba98486ea68380f47ebb19a7df173b99e6bc9c681d6ccf3bde31465d1f16b3012321039e8815e15952a7c3fada1905f8cf55419837133bd7756c0ef14fc8dfe50c0deaacffffffff010000000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().expect_err("transaction with duplicate inputs should be rejected");

    let expected_outpoint = OutPoint {
        txid: Txid::from_byte_array([
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]),
        vout: 0,
    };
    assert_eq!(err, TransactionDecoderError::duplicate_input(expected_outpoint));
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_output_value_sum_too_large() {
    // Test vector taken from Bitcoin Core tx_invalid.json
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L48
    // "MAX_MONEY output + 1 output" (sum exceeds MAX_MONEY)
    let tx_bytes = hex!("01000000010001000000000000000000000000000000000000000000000000000000000000000000006d483045022027deccc14aa6668e78a8c9da3484fbcd4f9dcc9bb7d1b85146314b21b9ae4d86022100d0b43dece8cfb07348de0ca8bc5b86276fa88f7f2138381128b7c36ab2e42264012321029bb13463ddd5d2cc05da6e84e37536cb9525703cfd8f43afdb414988987a92f6acffffffff020040075af075070001510001000000000000015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().expect_err("sum of output values > MAX_MONEY should be rejected");

    assert!(err.is_output_value_sum_too_large_match());
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn accept_output_value_sum_equal_to_max_money() {
    let tx_bytes = hex!("01000000010001000000000000000000000000000000000000000000000000000000000000000000006d483045022027deccc14aa6668e78a8c9da3484fbcd4f9dcc9bb7d1b85146314b21b9ae4d86022100d0b43dece8cfb07348de0ca8bc5b86276fa88f7f2138381128b7c36ab2e42264012321029bb13463ddd5d2cc05da6e84e37536cb9525703cfd8f43afdb414988987a92f6acffffffff020080c6a47e8d0300015100c040b571e80300015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let tx = decoder.end().expect("sum of output values == MAX_MONEY should be accepted");

    let total: u64 = tx.outputs.iter().map(|o| o.amount.to_sat()).sum();
    assert_eq!(total, Amount::MAX_MONEY.to_sat());
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_output_value_greater_than_max_money() {
    // Test vector taken from Bitcoin Core tx_invalid.json
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L44
    // "MAX_MONEY + 1 output"
    let tx_bytes = hex!("01000000010001000000000000000000000000000000000000000000000000000000000000000000006e493046022100e1eadba00d9296c743cb6ecc703fd9ddc9b3cd12906176a226ae4c18d6b00796022100a71aef7d2874deff681ba6080f1b278bac7bb99c61b08a85f4311970ffe7f63f012321030c0588dc44d92bdcbf8e72093466766fdc265ead8db64517b0c542275b70fffbacffffffff010140075af0750700015100000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    let result = decoder.push_bytes(&mut slice);
    assert!(result.is_err(), "output value > MAX_MONEY should be rejected during decoding");
}

#[test]
#[cfg(all(feature = "alloc", feature = "hex"))]
fn reject_transaction_with_no_outputs() {
    // Test vector taken from Bitcoin Core tx_invalid.json
    // https://github.com/bitcoin/bitcoin/blob/master/src/test/data/tx_invalid.json#L36
    // "No outputs"
    let tx_bytes = hex!("01000000010001000000000000000000000000000000000000000000000000000000000000000000006d483045022100f16703104aab4e4088317c862daec83440242411b039d14280e03dd33b487ab802201318a7be236672c5c56083eb7a5a195bc57a40af7923ff8545016cd3b571e2a601232103c40e5d339df3f30bf753e7e04450ae4ef76c9e45587d1d993bdc4cd06f0651c7acffffffff0000000000");

    let mut decoder = Transaction::decoder();
    let mut slice = tx_bytes.as_slice();
    decoder.push_bytes(&mut slice).unwrap();
    let err = decoder.end().unwrap_err();
    assert_eq!(err, TransactionDecoderError::no_outputs());
}
