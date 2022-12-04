use std::fs;

use plonky2::{plonk::{config::{PoseidonGoldilocksConfig, GenericConfig}, circuit_data::CircuitConfig, circuit_builder::CircuitBuilder}, iop::witness::{PartialWitness, Witness}, field::goldilocks_field::GoldilocksField};
use plonky2::field::types::Field;

fn main() {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = GoldilocksField;
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    let x_t = builder.add_virtual_target();
    builder.register_public_input(x_t);

    let mut pw = PartialWitness::new();
    pw.set_target(x_t, F::ONE);
    // println!("{}", F::ONE);

    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();

    let proof_str = serde_json::to_string(&proof).unwrap();
    fs::write("proof.json", proof_str).unwrap();

    // ↓ここでどんな処理をしているか
    data.verify(proof).unwrap();
}
