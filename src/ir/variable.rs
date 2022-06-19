use std::marker::PhantomData;

use crate::util::compile_time::{Const, GreaterEqual};

pub type VarId = usize;

pub struct Var {
    pub id: VarId,
    pub ty: Type,
}

pub struct TypedVar<T: TypeTag>(Var, PhantomData<T>);

pub type BoolVar = TypedVar<BoolType>;

pub type UIntVar<const SIZE: u8> = TypedVar<UIntType<SIZE>>;

pub type IntVar<const SIZE: u8> = TypedVar<IntType<SIZE>>;

pub type PointerVar = TypedVar<PointerType>;

impl BoolVar {
    pub fn into_uint<const SIZE: u8>(self) -> UIntVar<SIZE>
    where
        Const<SIZE>: GreaterEqual<1>,
    {
        TypedVar(self.0, PhantomData)
    }
}

impl<const SIZE: u8> UIntVar<SIZE> {
    pub fn into_uint<const TARGET_SIZE: u8>(self) -> UIntVar<TARGET_SIZE>
    where
        Const<TARGET_SIZE>: GreaterEqual<SIZE>,
    {
        TypedVar(self.0, PhantomData)
    }
}

pub enum Type {
    Bool,
    UInt(u8),
    Int(u8),
    Pointer,
}

pub trait TypeTag {}

#[non_exhaustive]
pub struct BoolType;

#[non_exhaustive]
pub struct UIntType<const SIZE: u8>;

#[non_exhaustive]
pub struct IntType<const SIZE: u8>;

#[non_exhaustive]
pub struct PointerType;

impl TypeTag for BoolType {}
impl<const SIZE: u8> TypeTag for UIntType<SIZE> {}
impl<const SIZE: u8> TypeTag for IntType<SIZE> {}
impl TypeTag for PointerType {}
