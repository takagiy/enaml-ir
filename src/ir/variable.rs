use id_arena::Id;
use std::marker::PhantomData;

use super::instruction::Instruction;
use crate::util::compile_time::{Const, GreaterEqual};

pub type VarId = Id<Var>;

pub struct Var {
    pub source: Instruction,
    pub ty: Type,
}

pub struct TypedVar<T: TypeTag>(VarId, PhantomData<T>);

pub type NoneVar = TypedVar<NoneType>;

pub type BoolVar = TypedVar<BoolType>;

pub type UIntVar<const SIZE: u8> = TypedVar<UIntType<SIZE>>;

pub type IntVar<const SIZE: u8> = TypedVar<IntType<SIZE>>;

pub type PointerVar = TypedVar<PointerType>;

impl<T: TypeTag> TypedVar<T> {
    pub fn new(id: VarId) -> Self {
        Self(id, PhantomData)
    }

    pub fn into_unknown(self) -> VarId {
        self.0
    }
}

impl BoolVar {
    pub fn into_uint<const SIZE: u8>(self) -> UIntVar<SIZE>
    where
        Const<SIZE>: GreaterEqual<1>,
    {
        UIntVar::new(self.0)
    }

    pub fn into_pointer(self) -> PointerVar {
        PointerVar::new(self.0)
    }
}

impl<const SIZE: u8> UIntVar<SIZE> {
    pub fn into_uint<const TARGET_SIZE: u8>(self) -> UIntVar<TARGET_SIZE>
    where
        Const<TARGET_SIZE>: GreaterEqual<SIZE>,
    {
        UIntVar::new(self.0)
    }

    pub fn into_pointer(self) -> PointerVar {
        PointerVar::new(self.0)
    }
}

pub enum Type {
    None,
    Bool,
    UInt(u8),
    Int(u8),
    Pointer,
}

pub trait TypeTag {
    const TYPE: Type;
}

#[non_exhaustive]
pub struct NoneType;

#[non_exhaustive]
pub struct BoolType;

#[non_exhaustive]
pub struct UIntType<const SIZE: u8>;

#[non_exhaustive]
pub struct IntType<const SIZE: u8>;

#[non_exhaustive]
pub struct PointerType;

impl TypeTag for NoneType {
    const TYPE: Type = Type::None;
}

impl TypeTag for BoolType {
    const TYPE: Type = Type::Bool;
}
impl<const SIZE: u8> TypeTag for UIntType<SIZE> {
    const TYPE: Type = Type::UInt(SIZE);
}
impl<const SIZE: u8> TypeTag for IntType<SIZE> {
    const TYPE: Type = Type::Int(SIZE);
}
impl TypeTag for PointerType {
    const TYPE: Type = Type::Pointer;
}

impl<T: TypeTag> From<TypedVar<T>> for VarId {
    fn from(typed: TypedVar<T>) -> Self {
        typed.0
    }
}

pub trait AnyVar: Into<VarId> {
    fn new(id: VarId) -> Self;

    fn into_unknown(self) -> VarId {
        self.into()
    }
}

impl<T: TypeTag> AnyVar for TypedVar<T> {
    fn new(id: VarId) -> Self {
        Self::new(id)
    }
}

pub trait UIntAnyVar: AnyVar {}

impl<const SIZE: u8> UIntAnyVar for UIntVar<SIZE> {}

pub trait IntAnyVar: AnyVar {}

impl<const SIZE: u8> IntAnyVar for IntVar<SIZE> {}

pub trait BothIntAnyVar: AnyVar {}

impl<const SIZE: u8> BothIntAnyVar for UIntVar<SIZE> {}

impl<const SIZE: u8> BothIntAnyVar for IntVar<SIZE> {}
