## tob 1-1

### error type: argument count mismatch

```bash
error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
  --> zkevm-circuits/src/evm_circuit/execution/mulmod.rs:35:14
   |
35 |     modword: ModGadget<F, true>,
   |              ^^^^^^^^^  ------ help: remove the unnecessary generic argument
   |              |
   |              expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `F`
  --> zkevm-circuits/src/evm_circuit/util/math_gadget/modulo.rs:23:19
   |
23 | pub(crate) struct ModGadget<F> {
   |                   ^^^^^^^^^ -

error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
  --> zkevm-circuits/src/evm_circuit/execution/precompiles/ecrecover.rs:52:19
   |
52 |     msg_hash_mod: ModGadget<F, true>,
   |                   ^^^^^^^^^  ------ help: remove the unnecessary generic argument
   |                   |
   |                   expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `F`
  --> zkevm-circuits/src/evm_circuit/util/math_gadget/modulo.rs:23:19
   |
23 | pub(crate) struct ModGadget<F> {
   |                   ^^^^^^^^^ -

error[E0107]: struct takes 1 generic argument but 2 generic arguments were supplied
  --> zkevm-circuits/src/evm_circuit/execution/precompiles/ec_mul.rs:76:14
   |
76 |     modword: ModGadget<F, false>,
   |              ^^^^^^^^^  ------- help: remove the unnecessary generic argument
   |              |
   |              expected 1 generic argument
   |
note: struct defined here, with 1 generic parameter: `F`
  --> zkevm-circuits/src/evm_circuit/util/math_gadget/modulo.rs:23:19
   |
23 | pub(crate) struct ModGadget<F> {
   |                   ^^^^^^^^^ -

For more information about this error, try `rustc --explain E0107`.
warning: `zkevm-circuits` (lib) generated 14 warnings
error: could not compile `zkevm-circuits` (lib) due to 3 previous errors; 14 warnings emitted
```