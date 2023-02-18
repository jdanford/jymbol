#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unary {
    Abs,
    Neg,
    Sqrt,
    Trunc,
    Round,
    Floor,
    Ceil,
}

impl Unary {
    pub fn apply(self, x: f64) -> f64 {
        match self {
            Unary::Abs => x.abs(),
            Unary::Neg => -x,
            Unary::Sqrt => x.sqrt(),
            Unary::Trunc => x.trunc(),
            Unary::Round => x.round(),
            Unary::Floor => x.floor(),
            Unary::Ceil => x.ceil(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Binary {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

fn wrap_bool(b: bool) -> f64 {
    if b {
        1.0
    } else {
        0.0
    }
}

#[allow(clippy::cast_precision_loss)]
fn wrap_int(i: i64) -> f64 {
    i as f64
}

impl Binary {
    #[allow(clippy::cast_possible_truncation, clippy::float_cmp)]
    pub fn apply(self, x: f64, y: f64) -> f64 {
        match self {
            Binary::Add => x + y,
            Binary::Sub => x - y,
            Binary::Mul => x * y,
            Binary::Div => x / y,
            Binary::Mod => x % y,
            Binary::Pow => x.powf(y),
            Binary::And => wrap_int((x as i64) & (y as i64)),
            Binary::Or => wrap_int((x as i64) | (y as i64)),
            Binary::Xor => wrap_int((x as i64) ^ (y as i64)),
            Binary::Shl => wrap_int((x as i64) << (y as i64)),
            Binary::Shr => wrap_int((x as i64) >> (y as i64)),
            Binary::Eq => wrap_bool(x == y),
            Binary::Ne => wrap_bool(x != y),
            Binary::Lt => wrap_bool(x < y),
            Binary::Gt => wrap_bool(x > y),
            Binary::Le => wrap_bool(x <= y),
            Binary::Ge => wrap_bool(x >= y),
        }
    }
}
