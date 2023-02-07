use crate::{check_count, symbol, Ref, Result, Value, VM};

#[allow(clippy::module_name_repetitions)]
pub struct ListIterator<'a> {
    vm: &'a mut VM,
    value: Value,
}

impl<'a> ListIterator<'a> {
    pub fn new(vm: &'a mut VM, value: Value) -> ListIterator<'a> {
        ListIterator { vm, value }
    }

    fn try_next(&mut self) -> Result<Option<Value>> {
        let ref_: Ref = self.value.try_into()?;
        let (type_, values) = self.vm.heap.load(ref_)?;
        if type_ == *symbol::CONS {
            check_count(*symbol::CONS, values, 2)?;
            let head = values[0];
            let tail = values[1];
            self.value = tail;
            Ok(Some(head))
        } else if type_ == *symbol::NIL {
            Ok(None)
        } else {
            Err(format!("expected list, got {type_}"))
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().transpose()
    }
}
