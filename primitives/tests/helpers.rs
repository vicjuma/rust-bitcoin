#[cfg(feature = "alloc")]
use bitcoin_primitives::{
    Amount, OutPoint, ScriptPubKeyBuf, ScriptSigBuf, Sequence, TxIn, TxOut, Witness,
};

#[cfg(feature = "alloc")]
pub fn tx_out() -> TxOut {
    TxOut { amount: Amount::ONE_SAT, script_pubkey: tc_script_pubkey() }
}

#[cfg(any(feature = "hex", feature = "serde"))]
pub fn segwit_tx_in() -> TxIn {
    let bytes = [1u8, 2, 3];
    let data = [&bytes[..]];
    let witness = Witness::from_iter(data);

    TxIn {
        previous_output: tc_out_point(),
        script_sig: tc_script_sig(),
        sequence: Sequence::MAX,
        witness,
    }
}

#[cfg(feature = "alloc")]
fn tc_script_pubkey() -> ScriptPubKeyBuf {
    let script_bytes = vec![1, 2, 3];
    ScriptPubKeyBuf::from_bytes(script_bytes)
}

#[cfg(feature = "alloc")]
fn tc_script_sig() -> ScriptSigBuf {
    let script_bytes = vec![1, 2, 3];
    ScriptSigBuf::from_bytes(script_bytes)
}

#[cfg(feature = "alloc")]
fn tc_out_point() -> OutPoint {
    let s = "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20:1";
    s.parse::<OutPoint>().unwrap()
}
