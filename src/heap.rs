use crate::{check_count, check_type, symbol, Ref, Result, Symbol, Value};

#[derive(Debug, Clone)]
struct Record {
    type_: Symbol,
    count: usize,
    offset: usize,
}

#[derive(Debug, Clone)]
pub struct Heap {
    records: Vec<Record>,
    values: Vec<Value>,
}

impl Heap {
    #[must_use]
    pub fn new() -> Self {
        Heap {
            records: Vec::new(),
            values: Vec::new(),
        }
    }

    fn get_record(&self, ref_: Ref) -> Result<&Record> {
        self.records
            .get(ref_.into_u32() as usize)
            .ok_or_else(|| format!("invalid reference: {}", ref_.into_u32()))
    }

    fn slice_from_record(&self, record: &Record) -> &[Value] {
        let start = record.offset;
        let end = start + record.count;
        &self.values[start..end]
    }

    pub fn alloc(&mut self, type_: Symbol, values: Vec<Value>) -> Result<Ref> {
        // TODO: actually check for truncation
        #[allow(clippy::cast_possible_truncation)]
        let index = self.records.len() as u32;
        let offset = self.values.len();
        let count = values.len();
        let record = Record {
            type_,
            count,
            offset,
        };

        self.records.push(record);
        self.values.extend(values);

        let ref_ = Ref::from(index);
        Ok(ref_)
    }

    pub fn load(&self, ref_: Ref) -> Result<(Symbol, &[Value])> {
        let record = self.get_record(ref_)?;
        let values = self.slice_from_record(record);
        Ok((record.type_, values))
    }

    pub fn load_checked(
        &self,
        ref_: Ref,
        expected_type: Symbol,
        expected_count: Option<usize>,
    ) -> Result<&[Value]> {
        let (type_, values) = self.load(ref_)?;

        check_type(type_, expected_type)?;
        if let Some(count) = expected_count {
            check_count(type_, values, count)?;
        }

        Ok(values)
    }

    pub fn alloc_cons(&mut self, head: Value, tail: Value) -> Result<Ref> {
        let values = vec![head, tail];
        self.alloc(*symbol::CONS, values)
    }

    pub fn load_cons(&self, ref_: Ref) -> Result<(Value, Value)> {
        if let [head, tail] = self.load_checked(ref_, *symbol::CONS, Some(2))? {
            Ok((*head, *tail))
        } else {
            unreachable!()
        }
    }

    pub fn alloc_list(&mut self, values: Vec<Value>) -> Result<Value> {
        let mut list = (*symbol::NIL).into();
        for value in values {
            let list_ref = self.alloc_cons(value, list)?;
            list = Value::Ref(list_ref);
        }

        Ok(list)
    }

    pub fn load_list(&self, list: Value) -> Result<Vec<Value>> {
        let mut list = list;
        let mut values = Vec::new();

        while list != (*symbol::NIL).into() {
            let ref_: Ref = list.try_into()?;
            let (head, tail) = self.load_cons(ref_)?;
            values.push(head);
            list = tail;
        }

        Ok(values)
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
