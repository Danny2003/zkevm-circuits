## tob 1-13

### error type: argument count mismatch & method not found

```bash
error[E0061]: this method takes 5 arguments but 4 arguments were supplied
    --> zkevm-circuits/src/evm_circuit/execution/error_invalid_creation_code.rs:48:12
     |
48   |         cb.memory_lookup(0.expr(), memory_address.offset(), first_byte.expr(), None);
     |            ^^^^^^^^^^^^^                                                       ---- argument #4 of type `halo2_proofs::plonk::Expression<F>` is missing
     |
note: method defined here
    --> zkevm-circuits/src/evm_circuit/util/constraint_builder.rs:1339:19
     |
1339 |     pub(crate) fn memory_lookup(
     |                   ^^^^^^^^^^^^^
1340 |         &mut self,
1341 |         is_write: Expression<F>,
     |         -----------------------
1342 |         memory_address: Expression<F>, // slot
     |         -----------------------------
1343 |         value: Expression<F>,
     |         --------------------
1344 |         value_prev: Expression<F>,
     |         -------------------------
1345 |         call_id: Option<Expression<F>>,
     |         ------------------------------
help: provide the argument
     |
48   |         cb.memory_lookup(0.expr(), memory_address.offset(), first_byte.expr(), /* halo2_proofs::plonk::Expression<F> */, None);
     |                         ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

error[E0599]: no method named `memory_value` found for enum `Rw` in the current scope
   --> zkevm-circuits/src/evm_circuit/execution/error_invalid_creation_code.rs:92:50
    |
92  |         let byte = block.rws[step.rw_indices[2]].memory_value();
    |                                                  ^^^^^^^^^^^^
    |
   ::: zkevm-circuits/src/witness/rw.rs:224:1
    |
224 | pub enum Rw {
    | ----------- method `memory_value` not found for this enum
    |
help: there is a method `log_value` with a similar name
    |
92  |         let byte = block.rws[step.rw_indices[2]].log_value();
    |                                                  ~~~~~~~~~

Some errors have detailed explanations: E0061, E0599.
For more information about an error, try `rustc --explain E0061`.
warning: `zkevm-circuits` (lib) generated 13 warnings
error: could not compile `zkevm-circuits` (lib) due to 2 previous errors; 13 warnings emitted
```