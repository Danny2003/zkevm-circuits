extern crate flux_rs;
use flux_rs::extern_spec;
// use halo2_proofs::plonk::{Selector, FixedQuery, AdviceQuery, InstanceQuery, Challenge};
// use crate::{
//     evm_circuit::{
//         param::MAX_N_BYTES_INTEGER,
//         util::{
//             constraint_builder::{ConstrainBuilderCommon, EVMConstraintBuilder},
//             math_gadget::*,
//             transpose_val_ret, CachedRegion, CellType,
//         },
//     },
//     util::{Expr, Field},
// };
// use halo2_proofs::{
//     circuit::Value,
//     plonk::Error,
// };

// use core::ops::{Add, Mul};
use std::iter::{Product, Sum};
use std::ops::{Add, Mul, Neg, Sub};

pub(crate) const MAX_N_BYTES_INTEGER: usize = 31;

/// predicates
fn pow_of_256(n: usize) -> u64 {
    unimplemented!()
}

fn interp<F:Field> (_expr: Expression<F>) -> F {
    unimplemented!()
}

fn pow_of_two(n: u64) -> u64 {
    unimplemented!()
}

// What should be **nat** in these predicates? Is it competible with SMT and flux?
//   - bytes_f: F -> nat | should be SMT-encodable
//   - bytes_u64: u64 -> nat | should be SMT-encodable
//   - get: Cell<F> -> F | ??
//   - ok: F -> bool = \x. bytes_f(x) <= MAX_N_BYTES_INTEGER | should be SMT-encodable
//   - u64_to_f: u64 -> F | should be SMT-encodable
/// end predicates

/// This trait represents an element of a field.
pub trait Field:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Send
    + Sync
{
    /// The zero element of the field, the additive identity.
    const ZERO: Self;

    /// The one element of the field, the multiplicative identity.
    const ONE: Self;

}

/// Trait that implements functionality to get a constant expression from
/// commonly used types.
pub trait Expr<F: Field> {
    /// Returns an expression for the type.
    fn expr(&self) -> Expression<F>;
}

#[derive(Clone)]
struct Selector {}
#[derive(Clone)]
struct FixedQuery {}
#[derive(Clone)]
struct AdviceQuery {}
#[derive(Clone)]
struct InstanceQuery {}
#[derive(Clone)]
struct Challenge {}

#[derive(Clone)]
pub enum Expression<F:Field> {
    /// This is a constant polynomial
    Constant(F),
    /// This is a virtual selector
    Selector(Selector),
    /// This is a fixed column queried at a certain relative location
    Fixed(FixedQuery),
    /// This is an advice (witness) column queried at a certain relative location
    Advice(AdviceQuery),
    /// This is an instance (external) column queried at a certain relative location
    Instance(InstanceQuery),
    /// This is a challenge
    Challenge(Challenge),
    /// This is a negated polynomial
    Negated(Box<Expression<F>>),
    /// This is the sum of two polynomials
    Sum(Box<Expression<F>>, Box<Expression<F>>),
    /// This is the product of two polynomials
    Product(Box<Expression<F>>, Box<Expression<F>>),
    /// This is a scaled polynomial
    Scaled(Box<Expression<F>>, F),
}

fn expression_sub<F: Field>(lhs: Expression<F>, rhs: Expression<F>) -> Expression<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(Expression<F>[@lhs], Expression<F>[@rhs]) -> Expression<F>{v: interp(v) == interp(self) - interp(rhs)})]
fn expression_sub<F: Field>(lhs: Expression<F>, rhs: Expression<F>) -> Expression<F>;

fn expression_mul<F: Field>(lhs: Expression<F>, rhs: Expression<F>) -> Expression<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(Expression<F>[@lhs], Expression<F>[@rhs]) -> Expression<F>{v: interp(v) == interp(self) * interp(rhs)})]
fn expression_mul<F: Field>(lhs: Expression<F>, rhs: Expression<F>) -> Expression<F>;

pub(crate) struct Cell<F:Field> {
    // expression for constraint
    expression: Expression<F>,
}

impl<F: Field> Expr<F> for Cell<F> {
    fn expr(&self) -> Expression<F> {
        unimplemented!()
    }
}

impl<F: Field> Expr<F> for u64 {
    fn expr(&self) -> Expression<F> {
        unimplemented!()
    }
}

fn query_bytes<F:Field, const N_BYTES: usize>() -> [Cell<F>; N_BYTES] {
    unimplemented!()
}

#[extern_spec]
#[flux::trusted]
// #[flux::sig(fn<F>() -> [Cell<F>; 31])]
fn query_bytes<F:Field, const N_BYTES: usize>() -> [Cell<F>; N_BYTES];

pub(crate) fn from_bytes_expr<F: Field, E: Expr<F>>(bytes: &[E]) -> Expression<F> {
    debug_assert!(
        bytes.len() <= MAX_N_BYTES_INTEGER,
        "Too many bytes to compose an integer in field"
    );
    unimplemented!()
}

#[extern_spec]
#[flux::trusted]
fn from_bytes_expr<F: Field, E: Expr<F>>(bytes: &[E]) -> Expression<F>;

#[extern_spec]
#[flux::opaque]
struct RangeCheckGadget<F:Field, const N_BYTES: usize>;

#[extern_spec]
#[flux::opaque]
struct Cell<F:Field>;

#[extern_spec]
#[flux::opaque]
struct Expression<F:Field>;

fn query_cell_with_type<F:Field> (_cell: &Expression<F>) -> Cell<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::trusted]
fn query_cell_with_type<F:Field> (cell: &Expression<F>) -> Cell<F>;

fn range_lookup<F:Field> (_cell: Expression<F>, _denominator: u64) {
    unimplemented!()
}

// How should we define interp and pow_of_two to eliminate the 'not found in this scope' errors?
// How should we define the change of types or additonal assertions? Is the following way correct?
#[extern_spec]
#[flux::sig(fn<F>(Expression<F>[@cell], u64[@denominator]) -> {() | interp(cell) < pow_of_two(denominator)})]
fn range_lookup<F:Field> (_cell: Expression<F>, _denominator: u64) -> ();

fn require_equal<F:Field>(name: &str, lhs: Expression<F>, rhs: Expression<F>) {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(&str, Expression<F>[@lhs], Expression<F>[@rhs]) -> {() | lhs == rhs})]
fn require_equal<F:Field>(name: &str, lhs: Expression<F>, rhs: Expression<F>) -> ();

/// Requires that the passed in value is within the specified range.
/// `N_BYTES` is required to be `<= MAX_N_BYTES_INTEGER`.
struct RangeCheckGadget<F:Field, const N_BYTES: usize> {
    parts: [Cell<F>; N_BYTES],
}

fn RangeCheckGadget_construct<F: Field, const N_BYTES: usize>(value: Expression<F>) -> RangeCheckGadget<F, N_BYTES> {
    assert!(N_BYTES <= MAX_N_BYTES_INTEGER);
    let parts: [Cell<F>; N_BYTES] = query_bytes::<F, N_BYTES>();

    // Require that the reconstructed value from the parts equals the
    // original value
    require_equal(
        "Constrain bytes recomposited to value",
        value,
        from_bytes_expr(&parts),
    );
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F, N_BYTES>(Expression<F>[@value]) -> ({RangeCheckGadget<F, N_BYTES> | interp(value) < pow_of_256(N_BYTES * 8) }) )]
fn RangeCheckGadget_construct<F:Field, const N_BYTES: usize>(value: Expression<F>)
    -> RangeCheckGadget<F, { N_BYTES }>;

    // pub(crate) fn assign(
    //     &self,
    //     region: &mut CachedRegion<'_, '_, F>,
    //     offset: usize,
    //     value: F,
    // ) -> Result<(), Error> {
    //     let bytes = value.to_repr();
    //     for (idx, part) in self.parts.iter().enumerate() {
    //         part.assign(region, offset, Value::known(F::from(bytes[idx] as u64)))?;
    //     }
    //     Ok(())
    // }

/// Returns (quotient: numerator/denominator, remainder: numerator%denominator),
/// with `numerator` an expression and `denominator` a constant.
/// Input requirements:
/// - `quotient < 256**N_BYTES`
/// - `quotient * denominator < field size`
/// - `remainder < denominator` requires a range lookup table for `denominator`
pub struct ConstantDivisionGadget<F:Field, const N_BYTES: usize> {
    quotient: Cell<F>,
    remainder: Cell<F>,
    denominator: u64,
    quotient_range_check: RangeCheckGadget<F, N_BYTES>,
}

impl<F: Field, const N_BYTES: usize> ConstantDivisionGadget<F, N_BYTES> {
    // #[flux::sig(fn<F>({Expression<F>[@numerator] | ok(interp(numerator))},
    //     { u64[@denominator] | ok(u64_to_f(numerator)) < 8}) -> ...)]
    pub(crate) fn construct(
        numerator: Expression<F>, // numerator: {v | vbit(v)==256}
        denominator: u64, // denominator: {v | vbit(v)==64}
    ) -> Self {
        assert!(N_BYTES * 8 + 64 - denominator.leading_zeros() as usize <= MAX_N_BYTES_INTEGER * 8);
        let quotient = query_cell_with_type(&numerator);
        let remainder = query_cell_with_type(&numerator);

        // Require that remainder < denominator
        range_lookup(remainder.expr(), denominator);

        // Require that quotient < 256**N_BYTES
        // so we can't have any overflow when doing `quotient * denominator`.
        let quotient_range_check = RangeCheckGadget_construct(quotient.expr());

        // Check if the division was done correctly
        require_equal(
            "numerator - remainder == quotient ⋅ denominator",
            expression_sub::<F>(numerator, remainder.expr()),
            expression_mul::<F>(quotient.expr(), denominator.expr()),
        );

        Self {
            quotient,
            remainder,
            denominator,
            quotient_range_check,
        }
    }

    pub(crate) fn quotient(&self) -> Expression<F> {
        self.quotient.expr()
    }

    pub(crate) fn remainder(&self) -> Expression<F> {
        self.remainder.expr()
    }

    // pub(crate) fn assign(
    //     &self,
    //     region: &mut CachedRegion<'_, '_, F>,
    //     offset: usize,
    //     numerator: u128,
    // ) -> Result<(u128, u128), Error> {
    //     let denominator = self.denominator as u128;
    //     let quotient = numerator / denominator;
    //     let remainder = numerator % denominator;

    //     self.quotient
    //         .assign(region, offset, Value::known(F::from_u128(quotient)))?;
    //     self.remainder
    //         .assign(region, offset, Value::known(F::from_u128(remainder)))?;

    //     self.quotient_range_check
    //         .assign(region, offset, F::from_u128(quotient))?;

    //     Ok((quotient, remainder))
    // }

    // pub(crate) fn assign_value(
    //     &self,
    //     region: &mut CachedRegion<'_, '_, F>,
    //     offset: usize,
    //     numerator: Value<F>,
    // ) -> Result<Value<(u128, u128)>, Error> {
    //     transpose_val_ret(
    //         numerator.map(|numerator| self.assign(region, offset, numerator.get_lower_128())),
    //     )
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::{test_util::*, *};
//     use crate::util::Field;
//     use eth_types::*;
//     use halo2_proofs::{halo2curves::bn256::Fr, plonk::Error};

//     #[derive(Clone)]
//     /// ConstantDivisionTestContainer:
//     /// require(a(N_BYTES) == DENOMINATOR * QUOTIENT + REMAINDER)
//     struct ConstantDivisionTestContainer<
//         F,
//         const N_BYTES: usize,
//         const DENOMINATOR: u64,
//         const QUOTIENT: u64,
//         const REMINDER: u64,
//     > {
//         constdiv_gadget: ConstantDivisionGadget<F, N_BYTES>,
//         a: Cell<F>,
//     }

//     impl<
//             F: Field,
//             const N_BYTES: usize,
//             const DENOMINATOR: u64,
//             const QUOTIENT: u64,
//             const REMAINDER: u64,
//         > MathGadgetContainer<F>
//         for ConstantDivisionTestContainer<F, N_BYTES, DENOMINATOR, QUOTIENT, REMAINDER>
//     {
//         fn configure_gadget_container(cb: &mut EVMConstraintBuilder<F>) -> Self {
//             let a = cb.query_cell();
//             let constdiv_gadget =
//                 ConstantDivisionGadget::<F, N_BYTES>::construct(cb, a.expr(), DENOMINATOR);

//             cb.require_equal(
//                 "correct reminder",
//                 constdiv_gadget.remainder(),
//                 REMAINDER.expr(),
//             );

//             cb.require_equal(
//                 "correct quotient",
//                 constdiv_gadget.quotient(),
//                 QUOTIENT.expr(),
//             );

//             ConstantDivisionTestContainer { constdiv_gadget, a }
//         }

//         fn assign_gadget_container(
//             &self,
//             witnesses: &[Word],
//             region: &mut CachedRegion<'_, '_, F>,
//         ) -> Result<(), Error> {
//             let a = u64::from_le_bytes(witnesses[0].to_le_bytes()[..8].try_into().unwrap());
//             let offset = 0;

//             self.a.assign(region, offset, Value::known(F::from(a)))?;
//             self.constdiv_gadget.assign(region, offset, a as u128)?;

//             Ok(())
//         }
//     }

//     #[test]
//     fn test_constantdivisiongadget_0div5_rem0() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 5, 0, 0>,
//             [Word::from(0)],
//             true,
//         );
//     }

//     #[test]
//     fn test_constantdivisiongadget_5div5_rem0() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 5, 1, 0>,
//             [Word::from(5)],
//             true,
//         );
//     }

//     #[test]
//     fn test_constantdivisiongadget_1div5_rem1() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 5, 0, 1>,
//             [Word::from(1)],
//             true,
//         );
//     }

//     #[test]
//     fn test_constantdivisiongadget_1div5_rem4() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 5, 1, 4>,
//             [Word::from(1)],
//             false,
//         );
//     }

//     #[test]
//     fn test_constantdivisiongadget_quotient_overflow() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 5, 4294967296u64, 1>,
//             [Word::from(1u64 << (4 * 8)) * 5 + 1],
//             false,
//         );
//     }

//     #[test]
//     fn test_constantdivisiongadget_33_div16_rem17() {
//         try_test!(
//             ConstantDivisionTestContainer<Fr, 4, 16, 1, 17>,
//             [Word::from(33)],
//             false,
//         );
//     }
// }
