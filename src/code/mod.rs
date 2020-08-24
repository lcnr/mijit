/*!
 * Mijit's instruction set. This instruction set is used to define virtual
 * machines, and it is also used to remember what code Mijit has generated.
 *
 * Control flow is restricted to a finite state machine, defined by
 * implementing trait Machine. All the other instructions are branch-free.
 *
 * Arithmetic operations are all 32-bit (for now). The upper 32 bits of the
 * destination register will be set to zero.
 *
 * Booleans results are returned as `0` or `-1`.
 */

pub use super::x86_64::{Register as R, Precision};

pub mod control_flow;
pub use control_flow::{Machine, Alias};

pub mod clock;

/** Guard conditions used to define control flow. */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TestOp {
    // TODO: These constants should probably be 64-bit.
    Bits(R, i32, i32),
    Lt(R, i32),
    Ge(R, i32),
    Ult(R, i32),
    Uge(R, i32),
    Eq(R, i32),
    Ne(R, i32),
    Always,
}

/** Unary arithmetic operations. */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UnaryOp {
    Abs,
    Negate,
    Not,
    // TODO: Uxt, Sxt (#12).
}

/** Binary arithmetic operations. */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Lsl,
    Lsr,
    Asr,
    And,
    Or,
    Xor,
    Lt,
    Ult,
    Eq,
    Max, // TODO: Unsigned too?
    Min, // TODO: Unsigned too?
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DivisionOp {
    SignedDivMod,
    UnsignedDivMod,
}

/** The number of bytes transferred by a memory access. */
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Width {
    One = 0,
    Two = 1,
    Four = 2,
    Eight = 3,
}

/**
 * An imperative instruction.
 * The destination register (where applicable) is on the left.
 * M is the type of memory aliasing classes.
 * G is the type of global variable names.
 */
#[derive(Debug, Clone)]
pub enum Action<M, G> {
    Constant(Precision, R, i64),
    Move(R, R),
    Unary(UnaryOp, Precision, R, R),
    Binary(BinaryOp, Precision, R, R, R),
    Division(DivisionOp, Precision, R, R, R, R),
    LoadGlobal(R, G),
    StoreGlobal(R, G),
    Load(R, (R, Width), M),
    Store(R, (R, Width), M),
    Push(R),
    Pop(R),
    Debug(R),
}
