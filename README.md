# Share memo
1. `data.verify(proof)`
2. `verify(proof_with_pis: ProofWithPublicInputs,verifier_data: &VerifierOnlyCircuitData, common_data: &CommonCircuitData)`

# 

## proof_with_pis = proof with public inputs

## circuit_data.rs

[circuit_data.rs](/plonky2/plonky2/src/plonk/circuit_data.rs)

```rust
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommonCircuitData<F: RichField + Extendable<D>, const D: usize> {
    pub config: CircuitConfig,

    pub(crate) fri_params: FriParams,

    /// The types of gates used in this circuit, along with their prefixes.
    pub(crate) gates: Vec<GateRef<F, D>>,

    /// Information on the circuit's selector polynomials.
    pub(crate) selectors_info: SelectorsInfo,

    /// The degree of the PLONK quotient polynomial.
    pub(crate) quotient_degree_factor: usize,

    /// The largest number of constraints imposed by any gate.
    pub(crate) num_gate_constraints: usize,

    /// The number of constant wires.
    pub(crate) num_constants: usize,

    pub(crate) num_public_inputs: usize,

    /// The `{k_i}` valued used in `S_ID_i` in Plonk's permutation argument.
    pub(crate) k_is: Vec<F>,

    /// The number of partial products needed to compute the `Z` polynomials.
    pub(crate) num_partial_products: usize,
}
```