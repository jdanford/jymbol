use crate::Result;

const TAG_BIT_SIZE: usize = 3;
const DATA_BIT_SIZE: usize = 48;

const TAG_MASK: u64 = (1 << TAG_BIT_SIZE) - 1;
const DATA_MASK: u64 = (1 << DATA_BIT_SIZE) - 1;

const S_NAN_BITS: u64 = 0b0111_1111_1111_0000 << DATA_BIT_SIZE;

pub enum Unpacked {
    Float(f64),
    Tagged(u8, u64),
}

fn pack_tagged(tag: u8, data: u64) -> f64 {
    debug_assert!(tag != 0 || data != 0);
    debug_assert!(u64::from(tag) <= TAG_MASK);
    debug_assert!(data <= DATA_MASK);

    let shifted_tag = u64::from(tag) << DATA_BIT_SIZE;
    let n = S_NAN_BITS | shifted_tag | data;
    f64::from_bits(n)
}

impl Unpacked {
    pub fn pack(self) -> f64 {
        match self {
            Unpacked::Float(f) => f,
            Unpacked::Tagged(tag, data) => pack_tagged(tag, data),
        }
    }

    pub fn unpack(f: f64) -> Self {
        let n = f.to_bits();
        let quiet_bit = 1 << 51;
        let quiet_bit_clear = (n & quiet_bit) == 0;

        if f.is_nan() && quiet_bit_clear {
            let data = n & DATA_MASK;
            let tag = ((n >> DATA_BIT_SIZE) & TAG_MASK) as u8;
            Unpacked::Tagged(tag, data)
        } else {
            Unpacked::Float(f)
        }
    }
}

pub trait Pack: Sized {
    type Error;

    fn as_unpacked(&self) -> Unpacked;
    fn try_from_unpacked(unpacked: Unpacked) -> Result<Self>;

    fn pack(&self) -> f64 {
        self.as_unpacked().pack()
    }

    fn unpack(f: f64) -> Result<Self> {
        let unpacked = Unpacked::unpack(f);
        Self::try_from_unpacked(unpacked)
    }
}
