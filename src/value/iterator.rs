use crate::Value;

pub struct Iter<'a> {
    value: &'a Value,
}

impl<'a> Iter<'a> {
    pub fn new(value: &'a Value) -> Self {
        Iter { value }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        match self.value {
            Value::Compound(cons) if cons.is_cons() => {
                let [head, tail] = cons.as_array().unwrap();
                self.value = tail;
                Some(head)
            }
            value if value.is_nil() => None,
            _ => panic!("expected cons or nil, got {}", self.value),
        }
    }
}

impl<'a> Value {
    pub fn iter(&'a self) -> Iter<'a> {
        Iter::new(self)
    }
}

impl<'a> IntoIterator for &'a Value {
    type IntoIter = Iter<'a>;
    type Item = &'a Value;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
