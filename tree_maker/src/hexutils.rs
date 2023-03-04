use ff::PrimeField;
use halo2_proofs::halo2curves::{pasta::Fp, serde::SerdeObject};

use crate::TreeMakerError;

pub fn convert_addr_to_hex(addr: &str) -> Result<Fp, TreeMakerError> {
    let mut v = hex::decode(addr).unwrap();
    v.extend([0; 12]);

    let arr: [u8; 32] = v.try_into().unwrap();
    Ok(Fp::from_repr(arr).unwrap())
}

pub fn convert_fp_to_string(fp: Fp) -> String {
    let repr = fp.to_repr();
    hex::encode(repr)
    // let str = format!("{:?}", fp);
    // let s = str.strip_prefix("0x").ok_or("error stripping the string")?;
    // Ok(s.to_string())
}

pub fn convert_string_into_fp(val: &str) -> Fp {
    let v = hex::decode(val).expect("value should be converted to byte array");
    let arr: [u8; 32] = v.try_into().unwrap();
    Fp::from_repr(arr).unwrap()

    // // println!("v: {:?} ,{}", v, v.len());
    // let c: &[u64] = match bytemuck::try_cast_slice(&v) {
    //     Ok(c) => c,
    //     Err(err) => return Err(format!("error converting casting, err: {}", err).into()),
    // };
    // let c: [u64; 4] = c.try_into()?;

    // Fp::from_repr()
    // Ok(Fp::from_raw(c))
}
