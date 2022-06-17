use num_bigint::{BigInt, BigUint};

pub struct Value {
    pub ty: CompoundType,
    pub value: ValueKind,
}

pub enum ValueKind {
    UInt(BigUint),
    Int(BigInt),
    Address([u8; 20]),
    Bool(bool),
    UFixed(BigUint),
    Fixed(BigInt),
    Bytes(Vec<u8>),
    ByteVector(Vec<u8>),
    Function([u8; 20], [u8; 4]),
    String(String),
}

impl Value {
    fn uint(num_bytes: u8, value: BigUint) -> Option<Self> {
        #[allow(clippy::if_same_then_else)]
        if !(1..=32).contains(&num_bytes) {
            None
        } else if value.bits().div_ceil(8) > num_bytes as u64 {
            None
        } else {
            Some(Self {
                ty: CompoundType::uint(num_bytes),
                value: ValueKind::UInt(value),
            })
        }
    }

    fn int(num_bytes: u8, value: BigInt) -> Option<Self> {
        #[allow(clippy::if_same_then_else)]
        if !(1..=32).contains(&num_bytes) {
            None
        } else if (value.bits() + 1).div_ceil(8) > num_bytes as u64 {
            None
        } else {
            Some(Self {
                ty: CompoundType::uint(num_bytes),
                value: ValueKind::Int(value),
            })
        }
    }

    fn address(value: [u8; 20]) -> Self {
        Self {
            ty: CompoundType::address(),
            value: ValueKind::Address(value),
        }
    }

    fn bool(value: bool) -> Self {
        Self {
            ty: CompoundType::bool(),
            value: ValueKind::Bool(value),
        }
    }

    fn ufixed(num_bytes: u8, point_position: u8, value: BigUint) -> Option<Self> {
        #[allow(clippy::if_same_then_else)]
        if !(1..=32).contains(&num_bytes) {
            None
        } else if point_position == 0 || point_position > 80 {
            None
        } else if value.bits().div_ceil(8) > num_bytes as u64 {
            None
        } else {
            Some(Self {
                ty: CompoundType::ufixed(num_bytes, point_position),
                value: ValueKind::UFixed(value),
            })
        }
    }

    fn fixed(num_bytes: u8, point_position: u8, value: BigInt) -> Option<Self> {
        #[allow(clippy::if_same_then_else)]
        if !(1..=32).contains(&num_bytes) {
            None
        } else if point_position == 0 || point_position > 80 {
            None
        } else if (value.bits() + 1).div_ceil(8) > num_bytes as u64 {
            None
        } else {
            Some(Self {
                ty: CompoundType::fixed(num_bytes, point_position),
                value: ValueKind::Fixed(value),
            })
        }
    }

    fn bytes(num_bytes: u8, value: Vec<u8>) -> Self {
        Self {
            ty: CompoundType::bytes(num_bytes),
            value: ValueKind::Bytes(value),
        }
    }

    fn byte_vector(value: Vec<u8>) -> Self {
        Self {
            ty: CompoundType::byte_vector(),
            value: ValueKind::ByteVector(value),
        }
    }

    fn function(address: [u8; 20], function_selector: [u8; 4]) -> Self {
        Self {
            ty: CompoundType::function(),
            value: ValueKind::Function(address, function_selector),
        }
    }

    fn string(value: String) -> Self {
        Self {
            ty: CompoundType::string(),
            value: ValueKind::String(value),
        }
    }
}

/// Primitive types in Solidity Contract ABI
pub enum Type {
    /// Unsigned integer of size <n: usize> * 8 bits
    UInt(u8),
    /// Signed integer of size <n: usize> * 8 bits
    Int(u8),
    /// Contract address, 160 bits uinsigned integer
    Address,
    /// Boolean, UInt(1) but holds either 0x0 or 0x1
    Bool,
    /// Fixed point unsigend decimal value, first element of tuple is the number of bytes, second
    /// element is the position of the decimal point
    UFixed(u8, u8),
    /// Signed counter part of Fixed
    Fixed(u8, u8),
    /// Binary of size <n: usize> bytes
    Bytes(u8),
    /// Dynamically sized byte sequence
    ByteVector,
    /// 20 bytes contract address and 4 bytes function selector
    Function,
    /// UTF-8 encoded string
    String,
}

/// Compound type in Solidity Contract ABI
pub enum CompoundType {
    /// Primitive type
    Primitive(Type),
    /// Fixed length array
    Array(usize, Box<CompoundType>),
    /// Dynamically sized array
    Vector(Box<CompoundType>),
    /// Tuple
    Tuple(Vec<CompoundType>),
}

impl CompoundType {
    fn uint(num_bytes: u8) -> Self {
        Self::Primitive(Type::UInt(num_bytes))
    }

    fn int(num_bytes: u8) -> Self {
        Self::Primitive(Type::Int(num_bytes))
    }

    fn address() -> Self {
        Self::Primitive(Type::Address)
    }

    fn bool() -> Self {
        Self::Primitive(Type::Bool)
    }

    fn ufixed(num_bytes: u8, point_position: u8) -> Self {
        Self::Primitive(Type::UFixed(num_bytes, point_position))
    }

    fn fixed(num_bytes: u8, point_position: u8) -> Self {
        Self::Primitive(Type::Fixed(num_bytes, point_position))
    }

    fn bytes(num_bytes: u8) -> Self {
        Self::Primitive(Type::Bytes(num_bytes))
    }

    fn byte_vector() -> Self {
        Self::Primitive(Type::ByteVector)
    }

    fn function() -> Self {
        Self::Primitive(Type::Function)
    }

    fn string() -> Self {
        Self::Primitive(Type::String)
    }

    fn to_array(self, num_elements: usize) -> Self {
        Self::Array(num_elements, Box::new(self))
    }

    fn to_vector(self) -> Self {
        Self::Vector(Box::new(self))
    }

    fn tuple(elements: Vec<Self>) -> Self {
        Self::Tuple(elements)
    }
}
