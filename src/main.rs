use jymbol::{Reader, VM};

fn main() {
    let input = "`(list 'a ,b ,@c)";
    let vm = VM::new();
    let _reader = Reader::new(input, &vm);
}
