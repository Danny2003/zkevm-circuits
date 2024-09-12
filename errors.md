## tob 1-9

### error type: argument count mismatch & method not found

```bash
error[E0061]: this function takes 9 arguments but 7 arguments were supplied
   --> zkevm-circuits/src/evm_circuit/execution/create.rs:283:38
    |
283 |                   let tansfer_gadget = TransferGadget::construct(
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^
...
289 | /                     value.clone(),
290 | |                     &mut callee_reversion_info,
    | |______________________________________________- two arguments of type `halo2_proofs::plonk::Expression<F>` and `halo2_proofs::plonk::Expression<F>` are missing
    |
note: associated function defined here
   --> zkevm-circuits/src/evm_circuit/util/common_gadget.rs:959:19
    |
959 |     pub(crate) fn construct(
    |                   ^^^^^^^^^
960 |         cb: &mut EVMConstraintBuilder<F>,
    |         --------------------------------
961 |         sender_address: Expression<F>,
    |         -----------------------------
962 |         receiver_address: Expression<F>,
    |         -------------------------------
963 |         receiver_exists: Expression<F>,
    |         ------------------------------
964 |         must_create: Expression<F>,
    |         --------------------------
965 |         prev_code_hash: Expression<F>,
    |         -----------------------------
966 |         #[cfg(feature = "scroll")] prev_keccak_code_hash: Expression<F>,
    |         ---------------------------------------------------------------
967 |         value: Word<F>,
    |         --------------
968 |         reversion_info: &mut ReversionInfo<F>,
    |         -------------------------------------
help: provide the arguments
    |
283 |                 let tansfer_gadget = TransferGadget::construct(cb, create.caller_address(), new_address.clone(), 0.expr(), 1.expr(), /* halo2_proofs::plonk::Expression<F> */, /* halo2_proofs::plonk::Expression<F> */, value.clone(), &mut callee_reversion_info);
    |                                                               ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0599]: no method named `memory_value` found for enum `Rw` in the current scope
   --> zkevm-circuits/src/evm_circuit/execution/create.rs:511:52
    |
511 |             .map(|i| block.rws[step.rw_indices[i]].memory_value())
    |                                                    ^^^^^^^^^^^^
    |
   ::: zkevm-circuits/src/witness/rw.rs:224:1
    |
224 | pub enum Rw {
    | ----------- method `memory_value` not found for this enum
    |
help: there is a method `log_value` with a similar name
    |
511 |             .map(|i| block.rws[step.rw_indices[i]].log_value())
    |                                                    ~~~~~~~~~

error[E0599]: no method named `assign` found for struct `TransferGadgetImpl` in the current scope
   --> zkevm-circuits/src/evm_circuit/execution/create.rs:612:27
    |
612 |             self.transfer.assign(
    |             --------------^^^^^^ method not found in `TransferGadgetImpl<F, TransferFromGadgetImpl<F, ()>>`
    |
   ::: zkevm-circuits/src/evm_circuit/util/common_gadget.rs:899:1
    |
899 | pub(crate) struct TransferGadgetImpl<F, TransferFromGadget> {
    | ----------------------------------------------------------- method `assign` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following traits define an item `assign`, perhaps you need to implement one of them:
            candidate #1: `CommonMemoryAddressGadget`
            candidate #2: `ComparatorInstruction`
            candidate #3: `IsEqualInstruction`
            candidate #4: `IsZeroInstruction`
            candidate #5: `LtInstruction`
            candidate #6: `UIntRangeCheckInstruction`

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
warning: `zkevm-circuits` (lib) generated 13 warnings
error: could not compile `zkevm-circuits` (lib) due to 3 previous errors; 13 warnings emitted
```