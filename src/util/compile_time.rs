use seq_macro::seq;

#[non_exhaustive]
pub struct Const<const N: u8>;

pub trait GreaterEqual<const N: u8> {}

seq!(M in 0..32 {
    seq!(N in M..32 {
        impl GreaterEqual<M> for Const<N> {}
    });
});
