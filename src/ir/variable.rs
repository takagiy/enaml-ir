use std::marker::PhantomData;

use crate::util::compile_time::{Const, GreaterEqual};

use super::instruction::Instruction;

pub type VarId<'ctx> = &'ctx Instruction<'ctx>;

pub struct Var<'ctx> {
    pub id: VarId<'ctx>,
    pub ty: Type,
}

pub trait VarIdExt<'ctx> {
    fn assume_unchecked<T: AnyVar<'ctx>>(self) -> T;
}

impl<'ctx> VarIdExt<'ctx> for VarId<'ctx> {
    fn assume_unchecked<T: AnyVar<'ctx>>(self) -> T {
        T::new(self)
    }
}

pub struct TypedVar<'ctx, T: TypeTag>(Var<'ctx>, PhantomData<T>);

pub type BoolVar<'ctx> = TypedVar<'ctx, BoolType>;

pub type UIntVar<'ctx, const SIZE: u8> = TypedVar<'ctx, UIntType<SIZE>>;

pub type IntVar<'ctx, const SIZE: u8> = TypedVar<'ctx, IntType<SIZE>>;

pub type PointerVar<'ctx> = TypedVar<'ctx, PointerType>;

impl<'ctx, T: TypeTag> TypedVar<'ctx, T> {
    pub fn new(id: VarId<'ctx>) -> Self {
        Self(Var { id, ty: T::TYPE }, PhantomData)
    }

    pub fn into_unknown(self) -> Var<'ctx> {
        self.0
    }
}

impl<'ctx> BoolVar<'ctx> {
    pub fn into_uint<const SIZE: u8>(self) -> UIntVar<'ctx, SIZE>
    where
        Const<SIZE>: GreaterEqual<1>,
    {
        UIntVar::new(self.0.id)
    }

    pub fn into_pointer(self) -> PointerVar<'ctx> {
        PointerVar::new(self.0.id)
    }
}

impl<'ctx, const SIZE: u8> UIntVar<'ctx, SIZE> {
    pub fn into_uint<const TARGET_SIZE: u8>(self) -> UIntVar<'ctx, TARGET_SIZE>
    where
        Const<TARGET_SIZE>: GreaterEqual<SIZE>,
    {
        UIntVar::new(self.0.id)
    }

    pub fn into_pointer(self) -> PointerVar<'ctx> {
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

pub trait AnyVar<'ctx> {
    fn new(id: VarId<'ctx>) -> Self;

    fn into_unknown(self) -> Var<'ctx>;

    fn as_unknown(&self) -> &Var<'ctx>;

    fn id(&self) -> VarId<'ctx> {
        self.as_unknown().id
    }
}

impl<'ctx, T: TypeTag> AnyVar<'ctx> for TypedVar<'ctx, T> {
    fn new(id: VarId<'ctx>) -> Self {
        Self::new(id)
    }

    fn into_unknown(self) -> Var<'ctx> {
        self.into_unknown()
    }

    fn as_unknown(&self) -> &Var<'ctx> {
        &self.0
    }
}

pub trait UIntAnyVar<'ctx>: AnyVar<'ctx> {}

impl<'ctx, const SIZE: u8> UIntAnyVar<'ctx> for UIntVar<'ctx, SIZE> {}

pub trait IntAnyVar<'ctx>: AnyVar<'ctx> {}

impl<'ctx, const SIZE: u8> IntAnyVar<'ctx> for IntVar<'ctx, SIZE> {}

pub trait BothIntAnyVar<'ctx>: AnyVar<'ctx> {}

impl<'ctx, const SIZE: u8> BothIntAnyVar<'ctx> for UIntVar<'ctx, SIZE> {}

impl<'ctx, const SIZE: u8> BothIntAnyVar<'ctx> for IntVar<'ctx, SIZE> {}
