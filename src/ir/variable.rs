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

impl<T: TypeTag> TypedVar<T> {
    pub fn new(id: VarId) -> Self {
        Self(Var { id, ty: T::TYPE }, PhantomData)
    }
}

impl BoolVar {
    pub fn as_uint<const SIZE: u8>(self) -> UIntVar<SIZE>
    where
        Const<SIZE>: GreaterEqual<1>,
    {
        UIntVar::new(self.0.id)
    }

    pub fn as_pointer(self) -> PointerVar {
        PointerVar::new(self.0.id)
    }
}

impl<const SIZE: u8> UIntVar<SIZE> {
    pub fn as_uint<const TARGET_SIZE: u8>(self) -> UIntVar<TARGET_SIZE>
    where
        Const<TARGET_SIZE>: GreaterEqual<SIZE>,
    {
        UIntVar::new(self.0.id)
    }

    pub fn as_pointer(self) -> PointerVar {
        PointerVar::new(self.0.id)
    }
}

pub enum Type {
    Bool,
    UInt(u8),
    Int(u8),
    Pointer,
}

pub trait TypeTag {
    const TYPE: Type;
}

#[non_exhaustive]
pub struct BoolType;

#[non_exhaustive]
pub struct UIntType<const SIZE: u8>;

#[non_exhaustive]
pub struct IntType<const SIZE: u8>;

#[non_exhaustive]
pub struct PointerType;

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
