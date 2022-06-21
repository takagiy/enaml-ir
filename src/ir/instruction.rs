use paste::paste;

use super::variable::VarId;

#[non_exhaustive]
pub struct Instruction {
    pub opcode: Opcode,
    pub destination: Option<VarId>,
    pub args: Vec<VarId>,
}

macro_rules! inst_impl {
    ($name:ident($($arg:ident),*)) => {
        paste! {
            #[allow(clippy::too_many_arguments)]
            pub fn $name(destination: VarId, $($arg: VarId),*) -> Self {
                Self {
                    opcode: Opcode::[<$name:camel>],
                    destination: Some(destination),
                    args: vec![$($arg),*],
                }
            }
        }
    };
    ($name:ident($($arg:ident),*) nodest) => {
        paste! {
            #[allow(clippy::too_many_arguments)]
            pub fn $name($($arg: VarId),*) -> Self {
                Self {
                    opcode: Opcode::[<$name:camel>],
                    destination: None,
                    args: vec![$($arg),*],
                }
            }
        }
    };
}

macro_rules! instructions {
    ($($name:ident($($arg:ident),*) $($dest_spec:ident)?;)*) => {
        $(inst_impl!($name($($arg),*) $($dest_spec)?);)*
    };
}

impl Instruction {
    instructions! {
        add(a, b);
        mul(a, b);
        sub(a, b);
        div(a, b);
        modulo(a, b);
        add_mod(a, b, n);
        mul_mod(a, b, n);
        exp(a, exponent);
        sext(size_in_bytes, a);

        lt(a, b);
        gt(a, b);
        eq(a, b);
        is_zero(a);

        and(a, b);
        or(a, b);
        xor(a, b);
        not(a);
        byte(index, a);
        shl(shift, a);
        shr(shift, a);
        sar(shift, a);

        sha3(offset, size);

        address();
        external_balance(address);
        tx_origin();
        caller();

        call_value();
        call_data(index);
        call_data_size();
        copy_call_data(dest_pointer, source_pointer, size_in_bytes) nodest;

        code_size();
        copy_code(dest_pointer, source_pointer, size_in_bytes) nodest;

        tx_gas_price();

        external_code_size(address);
        copy_external_code(address, dest_pointer, source_pointer, size_in_bytes) nodest;
        return_data_size();
        copy_return_data(dest_pointer, source_pointer, size_in_bytes) nodest;

        external_code_hash(address);
        block_hash(block_number);

        block_coinbase();
        block_timestamp();
        block_number();
        block_difficulty();
        block_gas_limit();
        chain_id();
        balance();
        base_fee();

        load();
        store() nodest;
        store8() nodest;

        load_storage();
        store_storage() nodest;

        jump(counter) nodest;

        memory_size();

        gas_left();

        create(value, pointer, size_in_bytes);

        call(gas, address, value, args_pointer, args_size_in_bytes, return_pointer, return_size_in_bytes);
        call_code(gas, address, value, args_pointer, args_size_in_bytes, return_pointer, return_size_in_bytes);
        finish(pointer, size_in_bytes) nodest;
        call_delegate(gas, address, args_pointer, args_size_in_bytes, return_pointer, return_size_in_bytes);
        call_static(gas, address, args_pointer, args_size_in_bytes, return_pointer, return_size_in_bytes);
        revert(pointer, size_in_bytes) nodest;
        self_destruct(address) nodest;
    }
}

pub enum Opcode {
    Add,
    Mul,
    Sub,
    Div,
    Modulo,
    AddMod,
    MulMod,
    Exp,
    Sext,

    Lt,
    Gt,
    Eq,
    IsZero,

    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sar,

    Sha3,

    Address,
    ExternalBalance,
    TxOrigin,
    Caller,

    CallValue,
    CallData,
    CallDataSize,
    CopyCallData,

    CodeSize,
    CopyCode,

    TxGasPrice,

    ExternalCodeSize,
    CopyExternalCode,

    ReturnDataSize,
    CopyReturnData,

    ExternalCodeHash,
    BlockHash,

    BlockCoinbase,
    BlockTimestamp,
    BlockNumber,
    BlockDifficulty,
    BlockGasLimit,
    ChainId,
    Balance,
    BaseFee,

    Load,
    Store,
    Store8,

    LoadStorage,
    StoreStorage,

    Jump,

    MemorySize,

    GasLeft,

    Create,

    Call,
    CallCode,
    Finish,
    CallDelegate,
    CallStatic,
    Revert,
    SelfDestruct,
}
