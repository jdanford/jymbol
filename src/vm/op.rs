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
            Abs => x.abs(),
            Neg => -x,
            Sqrt => x.sqrt(),
            Trunc => x.trunc(),
            Round => x.round(),
            Floor => x.floor(),
            Ceil => x.ceil(),
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
            Add => x + y,
            Sub => x - y,
            Mul => x * y,
            Div => x / y,
            Mod => x % y,
            Pow => x.powf(y),
            And => wrap_int((x as i64) & (y as i64)),
            Or => wrap_int((x as i64) | (y as i64)),
            Xor => wrap_int((x as i64) ^ (y as i64)),
            Shl => wrap_int((x as i64) << (y as i64)),
            Shr => wrap_int((x as i64) >> (y as i64)),
            Eq => wrap_bool(x == y),
            Ne => wrap_bool(x != y),
            Lt => wrap_bool(x < y),
            Gt => wrap_bool(x > y),
            Le => wrap_bool(x <= y),
            Ge => wrap_bool(x >= y),
        }
    }
}
