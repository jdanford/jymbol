// +Inf: 0 11111111111 0000000000000000000000000000000000000000000000000000
// −Inf: 1 11111111111 0000000000000000000000000000000000000000000000000000
// qNaN: 0 11111111111 1000000000000000000000000000000000000000000000000001
// sNaN: 0 11111111111 0000000000000000000000000000000000000000000000000001

use std::num::{NonZeroU32, TryFromIntError};

use crate::{Ref, Result, Symbol, Value};

const DATA_BIT_SIZE: usize = 48;
const S_NAN_BITS: u64 = 0b0111_1111_1111_0000 << DATA_BIT_SIZE;
const TAG_MASK: u64 = 0b0000_0000_0000_0111 << DATA_BIT_SIZE;
const DATA_MASK: u64 = (1 << DATA_BIT_SIZE) - 1;

pub enum Unpacked {
    Float(f64),
    Tagged(u8, u64),
}

pub fn pack(tag: u8, data: u64) -> f64 {
    debug_assert!(tag != 0 || data != 0);
    debug_assert!(data <= DATA_MASK);

    let shifted_tag = u64::from(tag) << DATA_BIT_SIZE;
    let n = S_NAN_BITS | shifted_tag | data;
    f64::from_bits(n)
}

pub fn unpack(f: f64) -> Unpacked {
    let n = f.to_bits();
    let quiet_bit = 1 << 51;
    let quiet_bit_clear = (n & quiet_bit) == 0;

    if f.is_nan() && quiet_bit_clear {
        let data = n & DATA_MASK;
        #[allow(clippy::cast_possible_truncation)]
        let tag = ((n >> DATA_BIT_SIZE) & TAG_MASK) as u8;
        Unpacked::Tagged(tag, data)
    } else {
        Unpacked::Float(f)
    }
}

fn try_u32_from_u64(n: u64) -> Result<u32> {
    n.try_into().map_err(|err: TryFromIntError| err.to_string())
}

impl Value {
    fn pack(self) -> f64 {
        match self {
            Value::Number(f) => f,
            Value::Symbol(symbol) => {
                let index = u32::from(NonZeroU32::from(symbol));
                pack(0, index.into())
            }
            Value::Ref(ref_) => {
                let index: u32 = ref_.into();
                pack(1, index.into())
            }
        }
    }

    fn try_unpack(f: f64) -> Result<Value> {
        match unpack(f) {
            Unpacked::Float(f) => Ok(f.into()),
            Unpacked::Tagged(0, n) => {
                let index = try_u32_from_u64(n)?;
                let symbol = Symbol::try_from(index)?;
                Ok(symbol.into())
            }
            Unpacked::Tagged(1, n) => {
                let index = try_u32_from_u64(n)?;
                let ref_ = Ref::from(index);
                Ok(ref_.into())
            }
            Unpacked::Tagged(t, _) => Err(format!("unexpected tag: {}", t)),
        }
    }
}
