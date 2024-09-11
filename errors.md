## tob 1-12

### error type: argument count mismatch & method not found

```bash
error[E0061]: this function takes 9 arguments but 7 arguments were supplied
   --> zkevm-circuits/src/evm_circuit/execution/callop.rs:202:13
    |
202 |               TransferGadget::construct(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^
...
211 | /                 call_gadget.value.clone(),
212 | |                 &mut callee_reversion_info,
    | |__________________________________________- two arguments of type `halo2_proofs::plonk::Expression<F>` and `halo2_proofs::plonk::Expression<F>` are missing
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
202 ~             TransferGadget::construct(cb, caller_address.expr(), callee_address.expr(), or::expr([
203 +                     not::expr(call_gadget.callee_not_exists.expr()),
204 +                     is_precompile.expr(),
205 +                 ]), 0.expr(), /* halo2_proofs::plonk::Expression<F> */, /* halo2_proofs::plonk::Expression<F> */, call_gadget.value.clone(), &mut callee_reversion_info)
    |

error[E0616]: field `value_is_zero` of struct `TransferGadgetImpl` is private
   --> zkevm-circuits/src/evm_circuit/execution/callop.rs:306:57
    |
306 |                     is_call.expr() * not::expr(transfer.value_is_zero.expr()) * 2.expr();
    |                                                         ^^^^^^^^^^^^^ private field

error[E0616]: field `value_is_zero` of struct `TransferGadgetImpl` is private
   --> zkevm-circuits/src/evm_circuit/execution/callop.rs:444:57
    |
444 |                     is_call.expr() * not::expr(transfer.value_is_zero.expr()) * 2.expr();
    |                                                         ^^^^^^^^^^^^^ private field

error[E0599]: no method named `assign` found for struct `TransferGadgetImpl` in the current scope
   --> zkevm-circuits/src/evm_circuit/execution/callop.rs:701:27
    |
701 |             self.transfer.assign(
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

Some errors have detailed explanations: E0061, E0599, E0616.
For more information about an error, try `rustc --explain E0061`.
warning: `zkevm-circuits` (lib) generated 13 warnings
error: could not compile `zkevm-circuits` (lib) due to 4 previous errors; 13 warnings emitted
```
