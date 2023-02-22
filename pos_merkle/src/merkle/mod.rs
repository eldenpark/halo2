mod chip;
mod merkle_path;
mod test1;

use self::chip::{MerkleChip, MerkleConfig};
use super::ecdsa::{AssignedEcdsaSig, AssignedPublicKey, EcdsaChip};
use super::ecdsa::{EcdsaConfig, TestCircuitEcdsaVerifyConfig, BIT_LEN_LIMB, NUMBER_OF_LIMBS};
use crate::merkle::merkle_path::MerklePath;
use ecc::{GeneralEccChip, Point};
use group::ff::{Field, PrimeField};
use group::prime::PrimeCurveAffine;
use group::Curve;
use group::Group;
use halo2_gadgets::ecc::NonIdentityPoint;
use halo2_gadgets::poseidon::{PoseidonInstructions, Pow5Chip, Pow5Config, StateWord};
use halo2_gadgets::utilities::{i2lebsp, Var};
use halo2_gadgets::{
    poseidon::{
        // merkle::merkle_path::MerklePath,
        primitives::{self as poseidon, ConstantLength, P128Pow5T3 as OrchardNullifier, Spec},
        Hash,
    },
    utilities::UtilitiesInstructions,
};
use halo2_proofs::halo2curves::bn256::Bn256;
use halo2_proofs::halo2curves::pairing::Engine;
use halo2_proofs::halo2curves::pasta::{pallas, vesta, Ep, EpAffine, EqAffine, Fp};
use halo2_proofs::halo2curves::CurveAffine;
use halo2_proofs::plonk::{create_proof, keygen_pk, keygen_vk, ProvingKey, VerifyingKey};
use halo2_proofs::poly::commitment::{Params, ParamsProver};
use halo2_proofs::poly::ipa::commitment::{IPACommitmentScheme, ParamsIPA};
use halo2_proofs::poly::ipa::multiopen::ProverIPA;
use halo2_proofs::poly::kzg::multiopen::ProverGWC;
use halo2_proofs::transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer};
use halo2_proofs::SerdeFormat;
use halo2_proofs::{arithmetic::FieldExt, poly::Rotation};
use halo2_proofs::{
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance},
};
use integer::{IntegerInstructions, Range};
use maingate::{
    big_to_fe, fe_to_big, mock_prover_verify, DimensionMeasurement, MainGate, RangeChip,
    RangeInstructions, RegionCtx,
};
use rand::rngs::OsRng;
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, Write};
use std::marker::PhantomData;
use std::ops::{Mul, Neg};
use std::path::PathBuf;
use web_sys::console;
// use std::time::{Instant, SystemTime};

#[derive(Clone, Debug)]
pub struct MyConfig<F: FieldExt, const WIDTH: usize, const RATE: usize> {
    advices: [Column<Advice>; 5],
    instance: Column<Instance>,
    merkle_config: MerkleConfig<F, WIDTH, RATE>,
    poseidon_config: Pow5Config<F, WIDTH, RATE>,
    ecdsa_config: EcdsaConfig,
    // ecdsa_config: TestCircuitEcdsaVerifyConfig,
    _f: PhantomData<F>,
}

impl<F: FieldExt, const WIDTH: usize, const RATE: usize> MyConfig<F, WIDTH, RATE> {
    pub fn construct_merkle_chip(&self) -> MerkleChip<F, WIDTH, RATE> {
        MerkleChip::construct(self.merkle_config.clone())
    }

    pub fn construct_poseidon_chip(&self) -> Pow5Chip<F, WIDTH, RATE> {
        Pow5Chip::construct(self.poseidon_config.clone())
    }
}

struct HashCircuit<
    // C: Curve,
    N: CurveAffine,
    S: Spec<F, WIDTH, RATE>,
    F: FieldExt,
    const WIDTH: usize,
    const RATE: usize,
    const L: usize,
> {
    message: Value<F>,
    root: Value<F>,
    leaf_pos: Value<u32>,
    path: Value<[F; 32]>,

    t: Value<N>,
    u: Value<N>,

    public_key: Value<N>,
    signature: Value<(N::Scalar, N::Scalar)>,
    msg_hash: Value<N::Scalar>,
    aux_generator: N,
    window_size: usize,

    _spec: PhantomData<S>,
    _spec2: PhantomData<N>,
}

impl<
        // C: Curve,
        N: CurveAffine,
        S: Spec<F, WIDTH, RATE>,
        F: FieldExt,
        const WIDTH: usize,
        const RATE: usize,
        const L: usize,
    > Circuit<F> for HashCircuit<N, S, F, WIDTH, RATE, L>
{
    type Config = MyConfig<F, WIDTH, RATE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            message: Value::unknown(),
            root: Value::unknown(),
            leaf_pos: Value::unknown(),
            path: Value::unknown(),

            t: Value::unknown(),
            u: Value::unknown(),

            public_key: Value::unknown(),
            signature: Value::unknown(),
            msg_hash: Value::unknown(),
            aux_generator: N::default(),
            window_size: usize::default(),

            _spec: PhantomData,
            _spec2: PhantomData,
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> MyConfig<F, WIDTH, RATE> {
        // total 5 advice columns
        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>(); // 3
        let partial_sbox = meta.advice_column(); // 1
        let swap = meta.advice_column(); // 1
                                         //

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        let instance = meta.instance_column();
        meta.enable_equality(instance);

        meta.enable_constant(rc_b[0]);

        let poseidon_config = Pow5Chip::configure::<S>(
            meta,
            state.clone().try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        );

        let advices = [
            state[0].clone(),
            state[1].clone(),
            state[2].clone(),
            partial_sbox.clone(),
            swap.clone(),
        ];

        for advice in advices.iter() {
            meta.enable_equality(*advice);
        }

        let merkle_config = MerkleChip::configure(
            meta,
            advices.clone().try_into().unwrap(),
            poseidon_config.clone(),
        );

        let ecdsa_config = {
            let (rns_base, rns_scalar) =
                GeneralEccChip::<N, F, NUMBER_OF_LIMBS, BIT_LEN_LIMB>::rns();
            let main_gate_config = MainGate::<F>::configure(meta, advices);
            let mut overflow_bit_lens: Vec<usize> = vec![];
            overflow_bit_lens.extend(rns_base.overflow_lengths());
            overflow_bit_lens.extend(rns_scalar.overflow_lengths());
            let composition_bit_lens = vec![BIT_LEN_LIMB / NUMBER_OF_LIMBS];

            let range_config = RangeChip::<F>::configure(
                meta,
                &main_gate_config,
                composition_bit_lens,
                overflow_bit_lens,
            );

            EcdsaConfig::new(range_config, main_gate_config)
        };

        let my_config = MyConfig {
            advices: advices.try_into().unwrap(),
            instance,
            poseidon_config,
            merkle_config,
            ecdsa_config,
            _f: PhantomData,
        };

        my_config
    }

    fn synthesize(
        &self,
        config: MyConfig<F, WIDTH, RATE>,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        // println!(
        //     "synthesize(), t: {:?}",
        //     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        // );

        // let merkle_chip = config.construct_merkle_chip();

        // let leaf = merkle_chip.load_private(
        //     layouter.namespace(|| "load_private"),
        //     config.advices[0],
        //     self.message,
        // )?;

        // println!("in-circuit: leaf: {:?}", leaf);

        // let merkle_chip = config.construct_merkle_chip();

        // let merkle_inputs = MerklePath::<S, _, _, WIDTH, RATE> {
        //     chip: merkle_chip,
        //     leaf_pos: self.leaf_pos,
        //     path: self.path,
        //     phantom: PhantomData,
        // };

        // let calculated_root =
        //     merkle_inputs.calculate_root(layouter.namespace(|| "merkle root calculation"), leaf)?;

        // println!("in_circuit: root: {:?}", calculated_root);

        // layouter.constrain_instance(calculated_root.cell(), config.instance, 0)?;

        {
            let mut ecc_chip = GeneralEccChip::<N, F, NUMBER_OF_LIMBS, BIT_LEN_LIMB>::new(
                config.ecdsa_config.ecc_chip_config(),
            );

            layouter.assign_region(
                || "assign aux values",
                |region| {
                    let offset = 0;
                    let ctx = &mut RegionCtx::new(region, offset);

                    ecc_chip.assign_aux_generator(ctx, Value::known(self.aux_generator))?;
                    ecc_chip.assign_aux(ctx, self.window_size, 1)?;
                    Ok(())
                },
            )?;

            let ecdsa_chip = EcdsaChip::new(ecc_chip.clone());
            let scalar_chip = ecc_chip.scalar_field_chip();

            layouter.assign_region(
                || "region 0",
                |region| {
                    let offset = 0;
                    let ctx = &mut RegionCtx::new(region, offset);

                    let r = self.signature.map(|signature| signature.0);
                    let s = self.signature.map(|signature| signature.1);
                    let integer_r = ecc_chip.new_unassigned_scalar(r);
                    let integer_s = ecc_chip.new_unassigned_scalar(s);
                    let msg_hash = ecc_chip.new_unassigned_scalar(self.msg_hash);

                    let r_assigned =
                        scalar_chip.assign_integer(ctx, integer_r, Range::Remainder)?;

                    let s_assigned =
                        scalar_chip.assign_integer(ctx, integer_s, Range::Remainder)?;

                    let sig = AssignedEcdsaSig {
                        r: r_assigned,
                        s: s_assigned,
                    };

                    // println!(
                    //     "synthesize(), got sig, t: {:?}",
                    //     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH),
                    // );

                    let pk_in_circuit = ecc_chip.assign_point(ctx, self.public_key)?;

                    let pk_assigned = AssignedPublicKey {
                        point: pk_in_circuit,
                    };

                    let msg_hash = scalar_chip.assign_integer(ctx, msg_hash, Range::Remainder)?;

                    let t_in_circuit = ecc_chip.assign_point(ctx, self.t)?;
                    let u_in_circuit = ecc_chip.assign_point(ctx, self.u)?;

                    // ecdsa_chip.verify(ctx, &sig, &pk_assigned, &msg_hash)
                    ecdsa_chip.verify2(
                        ctx,
                        &sig,
                        &pk_assigned,
                        &msg_hash,
                        &t_in_circuit,
                        &u_in_circuit,
                    )
                },
            )?;

            // println!("synthesize(): start range chip thing");
            let range_chip = RangeChip::<F>::new(config.ecdsa_config.range_config);
            range_chip.load_table(&mut layouter)?;
        }

        // println!("synthesize(): end");

        Ok(())
    }
}

#[test]
pub fn test_poseidon2() {
    gen_id_proof();
}

pub fn gen_id_proof() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    // println!("args:{:?}", args);

    let read = args.contains(&String::from("read"));
    // println!("read: {}", read);

    fn mod_n<C: CurveAffine>(x: C::Base) -> C::Scalar {
        let x_big = fe_to_big(x);
        big_to_fe(x_big)
    }

    // let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // let params_path = project_root.join("params.dat");
    // let pk_path = project_root.join("pk.dat");
    // let vk_path = project_root.join("vk.dat");

    // println!(
    //     "params path: {:?}, pk_path: {:?}, vk_path: {:?}",
    //     params_path, pk_path, vk_path,
    // );

    // let start = Instant::now();
    // println!("poseidon_hash2(): t: {:?}", start.elapsed());

    let leaf = Fp::from(2);
    let path = [
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
        Fp::from(1),
    ];
    let pos = 0;
    let pos_bits: [bool; 32] = i2lebsp(pos as u64);

    // println!("out-circuit: pos_bits: {:?}", pos_bits);
    // println!("out-circuit: leaf: {:?}", leaf);

    let mut root = leaf;
    for (idx, el) in path.iter().enumerate() {
        let msg = if pos_bits[idx] {
            [*el, root]
        } else {
            [root, *el]
        };

        // println!("idx: {}, msg: {:?}", idx, msg);
        root = poseidon::Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash(msg);
    }

    // println!("out-circuit: root: {:?}, t: {:?}", root, start.elapsed());

    let g = pallas::Affine::generator();

    // Generate a key pair
    let sk = <pallas::Affine as CurveAffine>::ScalarExt::random(OsRng);
    let public_key = (g * sk).to_affine();
    // println!("public key: {:?}", public_key,);

    // Generate a valid signature
    // Suppose `m_hash` is the message hash
    let msg_hash = <pallas::Affine as CurveAffine>::ScalarExt::random(OsRng);

    // Draw arandomness
    let k = <pallas::Affine as CurveAffine>::ScalarExt::random(OsRng);
    let k_inv = k.invert().unwrap();

    // Calculate `r`
    let big_r = g * k;
    let r_point = big_r.to_affine().coordinates().unwrap();
    let x = r_point.x();
    let r = mod_n::<pallas::Affine>(*x);

    // Calculate `s`
    let s = k_inv * (msg_hash + (r * sk));
    // println!("r: {:?}, s: {:?}", r, s);

    // Sanity check. Ensure we construct a valid signature. So lets verify it
    {
        let s_inv = s.invert().unwrap();
        let u_1 = msg_hash * s_inv;
        let u_2 = r * s_inv;
        let r_point = ((g * u_1) + (public_key * u_2))
            .to_affine()
            .coordinates()
            .unwrap();
        let x_candidate = r_point.x();
        let r_candidate = mod_n::<pallas::Affine>(*x_candidate);

        // println!(
        //     "x_candidate: {:?}, r_candidate: {:?}",
        //     x_candidate, r_candidate
        // );

        assert_eq!(r, r_candidate);
    }

    let (t, u) = {
        let r_inv = r.invert().unwrap();
        let t = big_r * r_inv;
        let u = -(g * (r_inv * msg_hash));

        // let u_neg = u.neg();
        // println!("444 u_neg: {:?}", u_neg);

        let pk_candidate = (t * s + u).to_affine();
        assert_eq!(public_key, pk_candidate);

        (t.to_affine(), u.to_affine())
    };

    // println!("t: {:?}, u: {:?}", t, u);

    //

    let aux_generator = <pallas::Affine as CurveAffine>::CurveExt::random(OsRng).to_affine();

    // pallas::Point;
    let circuit = HashCircuit::<
        // pallas::Point,
        pallas::Affine,
        OrchardNullifier,
        Fp,
        3,
        2,
        2,
    > {
        message: Value::known(leaf),
        root: Value::known(root),
        leaf_pos: Value::known(pos),
        path: Value::known(path),

        t: Value::known(t),
        u: Value::known(u),

        public_key: Value::known(public_key),
        signature: Value::known((r, s)),
        msg_hash: Value::known(msg_hash),
        aux_generator,
        window_size: 2,
        _spec: PhantomData,
        _spec2: PhantomData,
    };

    // let instance = vec![vec![root], vec![]];
    //

    let dimension = DimensionMeasurement::measure(&circuit).unwrap();
    let k = dimension.k();

    // println!("proving, dimension k: {}", k);
    // let prover = MockProver::run(k, &circuit, instance).unwrap();
    // assert_eq!(prover.verify(), Ok(()));

    // return;

    // let params: ParamsIPA<EqAffine> = if read {
    //     println!("params reading, t: {:?}", start.elapsed());

    //     let params_fd = File::open(&params_path).unwrap();
    //     let mut reader = BufReader::new(params_fd);
    //     ParamsIPA::read(&mut reader).unwrap()
    // } else {
    //     println!("params generating, t: {:?}", start.elapsed());

    //     let params_fd = File::create(&params_path).unwrap();
    //     let params: ParamsIPA<_> = ParamsIPA::new(k);
    //     let mut writer = BufWriter::new(params_fd);
    //     params.write(&mut writer).unwrap();
    //     writer.flush().unwrap();

    //     params
    // };

    // println!("params generating, t: {:?}", start.elapsed());
    //

    // let params_fd = File::create(&params_path).unwrap();
    // let params_fd = File::create("params").unwrap();

    console::log_1(&"Hello using web-sys".into());
    let params: ParamsIPA<EqAffine> = ParamsIPA::new(k);
    // let mut writer = BufWriter::new(params_fd);
    // params.write(&mut writer).unwrap();
    // writer.flush().unwrap();

    return vec![33];

    // println!("params reading, t: {:?}", start.elapsed());

    // let params_fd = File::open(&params_path).unwrap();
    // let mut reader = BufReader::new(params_fd);
    // let params = ParamsIPA::read(&mut reader).unwrap();

    // let vk = if read {
    //     println!("11 vk reading, t: {:?}", start.elapsed());

    //     let vk_fd = File::open(&vk_path).unwrap();
    //     let mut reader = BufReader::new(vk_fd);
    //     VerifyingKey::read::<_, HashCircuit<pallas::Affine, OrchardNullifier, Fp, 3, 2, 2>>(
    //         &mut reader,
    //         SerdeFormat::RawBytes,
    //     )
    //     .unwrap()
    // } else {
    //     println!("11 vk generating, t: {:?}", start.elapsed());

    //     let vk_fd = File::create(&vk_path).unwrap();
    //     let vk = keygen_vk(&params, &circuit).expect("vk should not fail");
    //     let mut writer = BufWriter::new(vk_fd);
    //     vk.write(&mut writer, SerdeFormat::Processed).unwrap();
    //     writer.flush().unwrap();
    //     vk
    // };

    // println!("11 vk generating, t: {:?}", start.elapsed());

    // let vk_fd = File::create(&vk_path).unwrap();
    // let vk_fd = File::create("vk").unwrap();
    // let vk = keygen_vk(&params, &circuit).expect("vk should not fail");
    // let mut writer = BufWriter::new(vk_fd);
    // vk.write(&mut writer, SerdeFormat::Processed).unwrap();
    // writer.flush().unwrap();

    // println!("11 vk reading, t: {:?}", start.elapsed());

    // let vk_fd = File::open(&vk_path).unwrap();
    // let mut reader = BufReader::new(vk_fd);
    // let vk = VerifyingKey::<EqAffine>::read::<
    //     _,
    //     HashCircuit<
    //         // pallas::Point,
    //         pallas::Affine,
    //         OrchardNullifier,
    //         Fp,
    //         3,
    //         2,
    //         2,
    //     >,
    // >(&mut reader, SerdeFormat::Processed)
    // .unwrap();

    // println!("22 pk generating, t: {:?}", start.elapsed());

    // let pk_fd = File::create(&pk_path).unwrap();
    // let pk_fd = File::create("pk").unwrap();
    // let pk = keygen_pk(&params, vk, &circuit).expect("pk should not fail");
    // let mut writer = BufWriter::new(pk_fd);
    // pk.write(&mut writer, SerdeFormat::Processed).unwrap();
    // writer.flush().unwrap();

    // println!("22 pk reading, t: {:?}", start.elapsed());

    // let pk_fd = File::open(&pk_path).unwrap();
    // let mut reader = BufReader::new(pk_fd);
    // let pk = ProvingKey::<EqAffine>::read::<
    //     _,
    //     HashCircuit<
    //         // pallas::Point,
    //         pallas::Affine,
    //         OrchardNullifier,
    //         Fp,
    //         3,
    //         2,
    //         2,
    //     >,
    // >(&mut reader, SerdeFormat::Processed)
    // .unwrap();

    // println!("1111: pk read complete, t: {:?}", start.elapsed());

    let mut rng = OsRng;
    let mut transcript = Blake2bWrite::<_, EqAffine, Challenge255<_>>::init(vec![]);

    // println!("creating proof, t: {:?}", start.elapsed());
    // create_proof::<IPACommitmentScheme<_>, ProverIPA<_>, _, _, _, _>(
    //     &params,
    //     &pk,
    //     &[circuit],
    //     &[&[&[root], &[]]],
    //     &mut rng,
    //     &mut transcript,
    // )
    // .unwrap();

    // println!("proof generated, t: {:?}", start.elapsed());
    // let proof = transcript.finalize();

    // println!("proof len: {}, t: {:?}", proof.len(), start.elapsed());
}
