// +Inf: 0 11111111111 0000000000000000000000000000000000000000000000000000
// −Inf: 1 11111111111 0000000000000000000000000000000000000000000000000000
// qNaN: 0 11111111111 1000000000000000000000000000000000000000000000000001
// sNaN: 0 11111111111 0000000000000000000000000000000000000000000000000001

const DATA_BIT_SIZE: usize = 48;
const S_NAN_BITS: u64 = 0b0111_1111_1111_0000 << DATA_BIT_SIZE;
const TAG_MASK: u64 = 0b0000_0000_0000_0111 << DATA_BIT_SIZE;
const DATA_MASK: u64 = (1 << DATA_BIT_SIZE) - 1;

fn pack_tagged(tag: u8, data: u64) -> f64 {
    debug_assert!(tag != 0 || data != 0);
    debug_assert!(data <= DATA_MASK);

    let shifted_tag = u64::from(tag) << DATA_BIT_SIZE;
    let n = S_NAN_BITS | shifted_tag | data;
    f64::from_bits(n)
}

pub enum Unpacked {
    Float(f64),
    Tagged(u8, u64),
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
            #[allow(clippy::cast_possible_truncation)]
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
    fn try_from_unpacked(unpacked: Unpacked) -> Result<Self, Self::Error>;

    fn pack(&self) -> f64 {
        self.as_unpacked().pack()
    }

    fn unpack(f: f64) -> Result<Self, Self::Error> {
        let unpacked = Unpacked::unpack(f);
        Self::try_from_unpacked(unpacked)
    }
}
