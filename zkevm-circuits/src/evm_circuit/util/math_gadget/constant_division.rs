extern crate flux_rs;
use flux_rs::extern_spec;

// copied
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
    // omit other methods for simplicity
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
// modified version of Expression
enum Expression<F:Field> {
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

// I have to define the following fake implementations to eliminate the 'not found in this scope' errors.
#[derive(Clone)]
pub struct AbExpression<F:Field> {
    expression: Expression<F>,
}

impl<F:Field> AbExpression<F> {
    pub fn new() -> AbExpression<F> {
        unimplemented!()
    }
    fn len(s: &AbExpression<F>) -> usize {
        unimplemented!()
    }
}

#[extern_spec]
#[flux::refined_by(len: int)]
struct AbExpression<F:Field>;

#[extern_spec]
impl<F:Field> AbExpression<F> {
    #[flux::sig(fn() -> AbExpression<F>[256])]
    fn new() -> AbExpression<F>;

    #[flux::sig(fn(&AbExpression<F>[@n]) -> usize[n])]
    fn len(s: &AbExpression<F>) -> usize;
}

fn expression_sub<F: Field>(lhs: AbExpression<F>, rhs: AbExpression<F>) -> AbExpression<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(AbExpression<F>[@n], AbExpression<F>[@m]) -> AbExpression<F>[n])]
fn expression_sub<F: Field>(lhs: AbExpression<F>, rhs: AbExpression<F>) -> AbExpression<F>;

fn expression_mul<F: Field>(lhs: AbExpression<F>, rhs: AbExpression<F>) -> AbExpression<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(AbExpression<F>[@n], AbExpression<F>[@m]) -> {AbExpression<F>[m + n] | m + n <= 256})]
fn expression_mul<F: Field>(lhs: AbExpression<F>, rhs: AbExpression<F>) -> AbExpression<F>;


#[derive(Clone)]
struct Cell<F:Field> {
    expression: AbExpression<F>,
}

impl <F:Field> Cell<F> {
    fn new() -> Cell<F> {
        unimplemented!()
    }
    fn len(s: &Cell<F>) -> usize {
        unimplemented!()
    }
    fn expr(&self) -> AbExpression<F> {
        unimplemented!()
    }
}

#[extern_spec]
#[flux::refined_by(len: int)]
struct Cell<F:Field>;

#[extern_spec]
impl<F:Field> Cell<F> {
    #[flux::sig(fn() -> Cell<F>[256])]
    fn new() -> Cell<F>;

    #[flux::sig(fn(&Cell<F>[@n]) -> usize[n])]
    fn len(s: &Cell<F>) -> usize;

    #[flux::sig(fn(&Cell<F>[@n]) -> AbExpression<F>[n])]
    fn expr(&self) -> AbExpression<F>;
}

fn query_cell_with_type<F:Field> (MAX_N_BYTES_INTEGER: usize, _cell: &AbExpression<F>) -> Cell<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(usize[@n], &AbExpression<F>) -> Cell<F>[if (8 * n < 256) { 8 * n } else { 256 }])]
fn query_cell_with_type<F:Field> (MAX_N_BYTES_INTEGER: usize, cell: &AbExpression<F>) -> Cell<F>;

fn range_lookup<F:Field> (_cell: &AbExpression<F>, _denominator: u64) {
    unimplemented!()
}

fn leading_zeros_u64(value_: u64) -> u64 {
    if value_ >= 9223372036854775808 {
        0
    } else { 
    if value_ >= 4611686018427387904 {
        1
    } else {
    if value_ >= 2305843009213693952 {
        2
    } else { 
    if value_ >= 1152921504606846976 {
        3
    } else { 
    if value_ >= 576460752303423488 {
        4
    } else { 
    if value_ >= 288230376151711744 {
        5
    } else { 
    if value_ >= 144115188075855872 {
        6
    } else { 
    if value_ >= 72057594037927936 {
        7
    } else { 
    if value_ >= 36028797018963968 {
        8
    } else { 
    if value_ >= 18014398509481984 {
        9
    } else { 
    if value_ >= 9007199254740992 {
        10
    } else { 
    if value_ >= 4503599627370496 {
        11
    } else { 
    if value_ >= 2251799813685248 {
        12
    } else { 
    if value_ >= 1125899906842624 {
        13
    } else { 
    if value_ >= 562949953421312 {
        14
    } else { 
    if value_ >= 281474976710656 {
        15
    } else { 
    if value_ >= 140737488355328 {
        16
    } else { 
    if value_ >= 70368744177664 {
        17
    } else { 
    if value_ >= 35184372088832 {
        18
    } else { 
    if value_ >= 17592186044416 {
        19
    } else { 
    if value_ >= 8796093022208 {
        20
    } else { 
    if value_ >= 4398046511104 {
        21
    } else { 
    if value_ >= 2199023255552 {
        22
    } else { 
    if value_ >= 1099511627776 {
        23
    } else { 
    if value_ >= 549755813888 {
        24
    } else { 
    if value_ >= 274877906944 {
        25
    } else { 
    if value_ >= 137438953472 {
        26
    } else { 
    if value_ >= 68719476736 {
        27
    } else { 
    if value_ >= 34359738368 {
        28
    } else { 
    if value_ >= 17179869184 {
        29
    } else { 
    if value_ >= 8589934592 {
        30
    } else { 
    if value_ >= 4294967296 {
        31
    } else { 
    if value_ >= 2147483648 {
        32
    } else { 
    if value_ >= 1073741824 {
        33
    } else { 
    if value_ >= 536870912 {
        34
    } else { 
    if value_ >= 268435456 {
        35
    } else { 
    if value_ >= 134217728 {
        36
    } else { 
    if value_ >= 67108864 {
        37
    } else { 
    if value_ >= 33554432 {
        38
    } else { 
    if value_ >= 16777216 {
        39
    } else { 
    if value_ >= 8388608 {
        40
    } else { 
    if value_ >= 4194304 {
        41
    } else { 
    if value_ >= 2097152 {
        42
    } else { 
    if value_ >= 1048576 {
        43
    } else { 
    if value_ >= 524288 {
        44
    } else { 
    if value_ >= 262144 {
        45
    } else { 
    if value_ >= 131072 {
        46
    } else { 
    if value_ >= 65536 {
        47
    } else { 
    if value_ >= 32768 {
        48
    } else { 
    if value_ >= 16384 {
        49
    } else { 
    if value_ >= 8192 {
        50
    } else { 
    if value_ >= 4096 {
        51
    } else { 
    if value_ >= 2048 {
        52
    } else { 
    if value_ >= 1024 {
        53
    } else { 
    if value_ >= 512 {
        54
    } else { 
    if value_ >= 256 {
        55
    } else { 
    if value_ >= 128 {
        56
    } else { 
    if value_ >= 64 {
        57
    } else { 
    if value_ >= 32 {
        58
    } else { 
    if value_ >= 16 {
        59
    } else { 
    if value_ >= 8 {
        60
    } else { 
    if value_ >= 4 {
        61
    } else { 
    if value_ >= 2 {
        62
    } else { 
    if value_ >= 1 {
        63
    } else {
        64
    } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

#[extern_spec]
#[flux::sig(fn<F>(&AbExpression<F>[@n], u64[@denominator]) -> {() | n <= (64 - 
    if (denominator >= 9223372036854775808) {
        0
    } else { 
    if (denominator >= 4611686018427387904) {
        1
    } else {
    if (denominator >= 2305843009213693952) {
        2
    } else { 
    if (denominator >= 1152921504606846976) {
        3
    } else { 
    if (denominator >= 576460752303423488) {
        4
    } else { 
    if (denominator >= 288230376151711744) {
        5
    } else { 
    if (denominator >= 144115188075855872) {
        6
    } else { 
    if (denominator >= 72057594037927936) {
        7
    } else { 
    if (denominator >= 36028797018963968) {
        8
    } else { 
    if (denominator >= 18014398509481984) {
        9
    } else { 
    if (denominator >= 9007199254740992) {
        10
    } else { 
    if (denominator >= 4503599627370496) {
        11
    } else { 
    if (denominator >= 2251799813685248) {
        12
    } else { 
    if (denominator >= 1125899906842624) {
        13
    } else { 
    if (denominator >= 562949953421312) {
        14
    } else { 
    if (denominator >= 281474976710656) {
        15
    } else { 
    if (denominator >= 140737488355328) {
        16
    } else { 
    if (denominator >= 70368744177664) {
        17
    } else { 
    if (denominator >= 35184372088832) {
        18
    } else { 
    if (denominator >= 17592186044416) {
        19
    } else { 
    if (denominator >= 8796093022208) {
        20
    } else { 
    if (denominator >= 4398046511104) {
        21
    } else { 
    if (denominator >= 2199023255552) {
        22
    } else { 
    if (denominator >= 1099511627776) {
        23
    } else { 
    if (denominator >= 549755813888) {
        24
    } else { 
    if (denominator >= 274877906944) {
        25
    } else { 
    if (denominator >= 137438953472) {
        26
    } else { 
    if (denominator >= 68719476736) {
        27
    } else { 
    if (denominator >= 34359738368) {
        28
    } else { 
    if (denominator >= 17179869184) {
        29
    } else { 
    if (denominator >= 8589934592) {
        30
    } else { 
    if (denominator >= 4294967296) {
        31
    } else { 
    if (denominator >= 2147483648) {
        32
    } else { 
    if (denominator >= 1073741824) {
        33
    } else { 
    if (denominator >= 536870912) {
        34
    } else { 
    if (denominator >= 268435456) {
        35
    } else { 
    if (denominator >= 134217728) {
        36
    } else { 
    if (denominator >= 67108864) {
        37
    } else { 
    if (denominator >= 33554432) {
        38
    } else { 
    if (denominator >= 16777216) {
        39
    } else { 
    if (denominator >= 8388608) {
        40
    } else { 
    if (denominator >= 4194304) {
        41
    } else { 
    if (denominator >= 2097152) {
        42
    } else { 
    if (denominator >= 1048576) {
        43
    } else { 
    if (denominator >= 524288) {
        44
    } else { 
    if (denominator >= 262144) {
        45
    } else { 
    if (denominator >= 131072) {
        46
    } else { 
    if (denominator >= 65536) {
        47
    } else { 
    if (denominator >= 32768) {
        48
    } else { 
    if (denominator >= 16384) {
        49
    } else { 
    if (denominator >= 8192) {
        50
    } else { 
    if (denominator >= 4096) {
        51
    } else { 
    if (denominator >= 2048) {
        52
    } else { 
    if (denominator >= 1024) {
        53
    } else { 
    if (denominator >= 512) {
        54
    } else { 
    if (denominator >= 256) {
        55
    } else { 
    if (denominator >= 128) {
        56
    } else { 
    if (denominator >= 64) {
        57
    } else { 
    if (denominator >= 32) {
        58
    } else { 
    if (denominator >= 16) {
        59
    } else { 
    if (denominator >= 8) {
        60
    } else { 
    if (denominator >= 4) {
        61
    } else { 
    if (denominator >= 2) {
        62
    } else { 
    if (denominator >= 1) {
        63
    } else {
        64
    } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
)})]
fn range_lookup<F:Field> (_cell: &AbExpression<F>, _denominator: u64) -> ();

fn require_equal<F:Field>(name: &str, lhs: AbExpression<F>, rhs: AbExpression<F>) {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(&str, lhs: AbExpression<F>[@n], rhs: {AbExpression<F>[@m] | m <= n}) -> ())]
fn require_equal<F:Field>(name: &str, lhs: AbExpression<F>, rhs: AbExpression<F>) -> ();

/// Requires that the passed in value is within the specified range.
/// `N_BYTES` is required to be `<= MAX_N_BYTES_INTEGER`.
struct RangeCheckGadget<F:Field> {
    parts: [Cell<F>; 31],
}

fn RangeCheckGadget_construct<F: Field>(MAX_N_BYTES_INTEGER: usize, value: &AbExpression<F>) -> RangeCheckGadget<F> {
    assert!(31 <= MAX_N_BYTES_INTEGER);
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(usize[@MAX_N_BYTES_INTEGER], &AbExpression<F>[@n]) -> ({RangeCheckGadget<F> | 31 <= MAX_N_BYTES_INTEGER &&
    n <= 31 * 8
}))]
fn RangeCheckGadget_construct<F:Field>(MAX_N_BYTES_INTEGER: usize, value: &AbExpression<F>)
    -> RangeCheckGadget<F>;

fn expr_u64<F:Field> (value: u64) -> AbExpression<F> {
    unimplemented!()
}

#[extern_spec]
#[flux::sig(fn<F>(u64[@denominator]) -> AbExpression<F>[ 64 - 
    if (denominator >= 9223372036854775808) {
        0
    } else { 
    if (denominator >= 4611686018427387904) {
        1
    } else {
    if (denominator >= 2305843009213693952) {
        2
    } else { 
    if (denominator >= 1152921504606846976) {
        3
    } else { 
    if (denominator >= 576460752303423488) {
        4
    } else { 
    if (denominator >= 288230376151711744) {
        5
    } else { 
    if (denominator >= 144115188075855872) {
        6
    } else { 
    if (denominator >= 72057594037927936) {
        7
    } else { 
    if (denominator >= 36028797018963968) {
        8
    } else { 
    if (denominator >= 18014398509481984) {
        9
    } else { 
    if (denominator >= 9007199254740992) {
        10
    } else { 
    if (denominator >= 4503599627370496) {
        11
    } else { 
    if (denominator >= 2251799813685248) {
        12
    } else { 
    if (denominator >= 1125899906842624) {
        13
    } else { 
    if (denominator >= 562949953421312) {
        14
    } else { 
    if (denominator >= 281474976710656) {
        15
    } else { 
    if (denominator >= 140737488355328) {
        16
    } else { 
    if (denominator >= 70368744177664) {
        17
    } else { 
    if (denominator >= 35184372088832) {
        18
    } else { 
    if (denominator >= 17592186044416) {
        19
    } else { 
    if (denominator >= 8796093022208) {
        20
    } else { 
    if (denominator >= 4398046511104) {
        21
    } else { 
    if (denominator >= 2199023255552) {
        22
    } else { 
    if (denominator >= 1099511627776) {
        23
    } else { 
    if (denominator >= 549755813888) {
        24
    } else { 
    if (denominator >= 274877906944) {
        25
    } else { 
    if (denominator >= 137438953472) {
        26
    } else { 
    if (denominator >= 68719476736) {
        27
    } else { 
    if (denominator >= 34359738368) {
        28
    } else { 
    if (denominator >= 17179869184) {
        29
    } else { 
    if (denominator >= 8589934592) {
        30
    } else { 
    if (denominator >= 4294967296) {
        31
    } else { 
    if (denominator >= 2147483648) {
        32
    } else { 
    if (denominator >= 1073741824) {
        33
    } else { 
    if (denominator >= 536870912) {
        34
    } else { 
    if (denominator >= 268435456) {
        35
    } else { 
    if (denominator >= 134217728) {
        36
    } else { 
    if (denominator >= 67108864) {
        37
    } else { 
    if (denominator >= 33554432) {
        38
    } else { 
    if (denominator >= 16777216) {
        39
    } else { 
    if (denominator >= 8388608) {
        40
    } else { 
    if (denominator >= 4194304) {
        41
    } else { 
    if (denominator >= 2097152) {
        42
    } else { 
    if (denominator >= 1048576) {
        43
    } else { 
    if (denominator >= 524288) {
        44
    } else { 
    if (denominator >= 262144) {
        45
    } else { 
    if (denominator >= 131072) {
        46
    } else { 
    if (denominator >= 65536) {
        47
    } else { 
    if (denominator >= 32768) {
        48
    } else { 
    if (denominator >= 16384) {
        49
    } else { 
    if (denominator >= 8192) {
        50
    } else { 
    if (denominator >= 4096) {
        51
    } else { 
    if (denominator >= 2048) {
        52
    } else { 
    if (denominator >= 1024) {
        53
    } else { 
    if (denominator >= 512) {
        54
    } else { 
    if (denominator >= 256) {
        55
    } else { 
    if (denominator >= 128) {
        56
    } else { 
    if (denominator >= 64) {
        57
    } else { 
    if (denominator >= 32) {
        58
    } else { 
    if (denominator >= 16) {
        59
    } else { 
    if (denominator >= 8) {
        60
    } else { 
    if (denominator >= 4) {
        61
    } else { 
    if (denominator >= 2) {
        62
    } else { 
    if (denominator >= 1) {
        63
    } else {
        64
    } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
])]
fn expr_u64<F:Field> (value: u64) -> AbExpression<F>;

fn new_assert(denominator: u64, MAX_N_BYTES_INTEGER: usize) {
    unimplemented!()
    // assert!(31 * 8 + 64 - leading_zeros_u64(denominator) as usize <= MAX_N_BYTES_INTEGER * 8)
}

#[extern_spec]
#[flux::sig(fn(u64[@denominator], usize[@MAX_N_BYTES_INTEGER]) -> {() | 
    31 * 8 + 64 - (

        if (denominator >= 9223372036854775808) {
            0
        } else { 
        if (denominator >= 4611686018427387904) {
            1
        } else {
        if (denominator >= 2305843009213693952) {
            2
        } else { 
        if (denominator >= 1152921504606846976) {
            3
        } else { 
        if (denominator >= 576460752303423488) {
            4
        } else { 
        if (denominator >= 288230376151711744) {
            5
        } else { 
        if (denominator >= 144115188075855872) {
            6
        } else { 
        if (denominator >= 72057594037927936) {
            7
        } else { 
        if (denominator >= 36028797018963968) {
            8
        } else { 
        if (denominator >= 18014398509481984) {
            9
        } else { 
        if (denominator >= 9007199254740992) {
            10
        } else { 
        if (denominator >= 4503599627370496) {
            11
        } else { 
        if (denominator >= 2251799813685248) {
            12
        } else { 
        if (denominator >= 1125899906842624) {
            13
        } else { 
        if (denominator >= 562949953421312) {
            14
        } else { 
        if (denominator >= 281474976710656) {
            15
        } else { 
        if (denominator >= 140737488355328) {
            16
        } else { 
        if (denominator >= 70368744177664) {
            17
        } else { 
        if (denominator >= 35184372088832) {
            18
        } else { 
        if (denominator >= 17592186044416) {
            19
        } else { 
        if (denominator >= 8796093022208) {
            20
        } else { 
        if (denominator >= 4398046511104) {
            21
        } else { 
        if (denominator >= 2199023255552) {
            22
        } else { 
        if (denominator >= 1099511627776) {
            23
        } else { 
        if (denominator >= 549755813888) {
            24
        } else { 
        if (denominator >= 274877906944) {
            25
        } else { 
        if (denominator >= 137438953472) {
            26
        } else { 
        if (denominator >= 68719476736) {
            27
        } else { 
        if (denominator >= 34359738368) {
            28
        } else { 
        if (denominator >= 17179869184) {
            29
        } else { 
        if (denominator >= 8589934592) {
            30
        } else { 
        if (denominator >= 4294967296) {
            31
        } else { 
        if (denominator >= 2147483648) {
            32
        } else { 
        if (denominator >= 1073741824) {
            33
        } else { 
        if (denominator >= 536870912) {
            34
        } else { 
        if (denominator >= 268435456) {
            35
        } else { 
        if (denominator >= 134217728) {
            36
        } else { 
        if (denominator >= 67108864) {
            37
        } else { 
        if (denominator >= 33554432) {
            38
        } else { 
        if (denominator >= 16777216) {
            39
        } else { 
        if (denominator >= 8388608) {
            40
        } else { 
        if (denominator >= 4194304) {
            41
        } else { 
        if (denominator >= 2097152) {
            42
        } else { 
        if (denominator >= 1048576) {
            43
        } else { 
        if (denominator >= 524288) {
            44
        } else { 
        if (denominator >= 262144) {
            45
        } else { 
        if (denominator >= 131072) {
            46
        } else { 
        if (denominator >= 65536) {
            47
        } else { 
        if (denominator >= 32768) {
            48
        } else { 
        if (denominator >= 16384) {
            49
        } else { 
        if (denominator >= 8192) {
            50
        } else { 
        if (denominator >= 4096) {
            51
        } else { 
        if (denominator >= 2048) {
            52
        } else { 
        if (denominator >= 1024) {
            53
        } else { 
        if (denominator >= 512) {
            54
        } else { 
        if (denominator >= 256) {
            55
        } else { 
        if (denominator >= 128) {
            56
        } else { 
        if (denominator >= 64) {
            57
        } else { 
        if (denominator >= 32) {
            58
        } else { 
        if (denominator >= 16) {
            59
        } else { 
        if (denominator >= 8) {
            60
        } else { 
        if (denominator >= 4) {
            61
        } else { 
        if (denominator >= 2) {
            62
        } else { 
        if (denominator >= 1) {
            63
        } else {
            64
        } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }

    ) <= MAX_N_BYTES_INTEGER * 8
})]
fn new_assert(denominator: u64, MAX_N_BYTES_INTEGER: usize) -> ();


/// Returns (quotient: numerator/denominator, remainder: numerator%denominator),
/// with `numerator` an expression and `denominator` a constant.
/// Input requirements:
/// - `quotient < 256**N_BYTES`
/// - `quotient * denominator < field size`
/// - `remainder < denominator` requires a range lookup table for `denominator`
pub struct ConstantDivisionGadget<F:Field> {
    quotient: Cell<F>,
    remainder: Cell<F>,
    denominator: u64,
    quotient_range_check: RangeCheckGadget<F>,
}

impl<F: Field> ConstantDivisionGadget<F> {
    #[flux::sig(fn(AbExpression<F>[256], {u64[@denominator] | denominator == 18446744073709551615}) -> 
    ConstantDivisionGadget<F>
)]
    pub fn construct(
        numerator: AbExpression<F>, // numerator: {v | vbit(v)==256}
        denominator: u64, // denominator: {v | vbit(v)==64} ; vbit of u64 is (64 - leading_zeros_u64(v)) here
    ) -> Self {
        let MAX_N_BYTES_INTEGER: usize = 31;
        // new_assert(denominator, MAX_N_BYTES_INTEGER);
        let quotient = query_cell_with_type(MAX_N_BYTES_INTEGER, &numerator); // quotient: {v | vbit(v) == 31*8}
        let remainder = query_cell_with_type(MAX_N_BYTES_INTEGER, &numerator); // remainder: {v | vbit(v) == 31*8}

        // currently range_lookup should require vbit(remainder) <= vbit(denominator) according to the signature I defined
        // but it doesn't generate an error, why is it?
        range_lookup(&remainder.expr(), denominator);

        // currently quotient_range_check should require vbit(quotient) <= 8 * 31 according to the signature I defined, but is it really functioning?
        let quotient_range_check = RangeCheckGadget_construct(MAX_N_BYTES_INTEGER, &quotient.expr());

        // currently require_equal should require vbit(first) >= vbit(second) according to the signature I defined, but is it really functioning?
        require_equal(
            "numerator - remainder == quotient ⋅ denominator",
            expression_sub::<F>(numerator, remainder.expr()),
            expression_mul::<F>(quotient.expr(), expr_u64::<F>(denominator)),
        );

        Self {
            quotient,
            remainder,
            denominator,
            quotient_range_check,
        }
    }

    pub fn quotient(&self) -> AbExpression<F> {
        self.quotient.expr()
    }

    pub fn remainder(&self) -> AbExpression<F> {
        self.remainder.expr()
    }
}