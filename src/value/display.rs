use std::fmt::{self, Debug, Display, Formatter};

use crate::{value::Compound, Value};

impl Compound {
    fn fmt_generic(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(#{}", self.type_)?;

        for value in &self.values {
            write!(f, " {value}")?;
        }

        write!(f, ")")?;
        Ok(())
    }

    fn fmt_list(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        let list = Value::Compound(self.clone().into());
        for (i, result) in list.into_iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            let value = result.map_err(|_| fmt::Error)?;
            write!(f, "{value}")?;
        }

        write!(f, ")")?;
        Ok(())
    }

    fn fmt_quote(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [value] = self.as_array().map_err(|_| fmt::Error)?;
        write!(f, "'")?;
        write!(f, "{value}")?;
        Ok(())
    }

    fn fmt_quasiquote(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [value] = self.as_array().map_err(|_| fmt::Error)?;
        write!(f, "`")?;
        write!(f, "{value}")?;
        Ok(())
    }

    fn fmt_unquote(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [value] = self.as_array().map_err(|_| fmt::Error)?;
        write!(f, ",")?;
        write!(f, "{value}")?;
        Ok(())
    }

    fn fmt_unquote_splicing(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let [value] = self.as_array().map_err(|_| fmt::Error)?;
        write!(f, ",@")?;
        write!(f, "{value}")?;
        Ok(())
    }
}

impl Display for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_cons() {
            self.fmt_list(f)
        } else if self.is_quote() {
            self.fmt_quote(f)
        } else if self.is_quasiquote() {
            self.fmt_quasiquote(f)
        } else if self.is_unquote() {
            self.fmt_unquote(f)
        } else if self.is_unquote_splicing() {
            self.fmt_unquote_splicing(f)
        } else {
            self.fmt_generic(f)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Symbol(sym) => Display::fmt(sym, f),
            Value::Number(num) => {
                if num.fract() == 0.0 {
                    write!(f, "{num:.0}")
                } else {
                    write!(f, "{num}")
                }
            }
            Value::String(s) => Debug::fmt(s, f),
            Value::Compound(compound) => Display::fmt(compound, f),
            Value::Closure(fn_) => Display::fmt(fn_, f),
            &Value::NativeFunction(fn_id) => {
                let n = u32::from(fn_id);
                write!(f, "(#native-fn {n})")
            }
        }
    }
}
