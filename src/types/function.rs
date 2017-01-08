use std::fmt::{Debug, Formatter, Error};

use abc::EvalResult;
use Context;

use types::abc::*;
use Arg;


pub struct Function {
    pub f: for <'res> fn(&[Arg<'res>], &'res Context<'res>) -> EvalResult<Arg<'res>>,
}

impl Function {
    pub fn new(f: for <'res> fn(&[Arg<'res>], &'res Context<'res>) -> EvalResult<Arg<'res>>) -> Arg<'static> {
        Arg::Owned(box Function { f: f })
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function { f: self.f }
    }
}

impl Debug for Function {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str("AnonymousFunction")
    }
}

impl <'t> Type<'t> for Function {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box self.clone())
    }
}

impl IInvocable for Function {
    fn invoke<'r: 'rr, 'rr>(&self, args: &'rr [Arg<'r>], context: &'r Context<'r>) -> EvalResult<Arg<'r>> {
        (self.f)(args, context)
    }
}



// --------------------------------------------------------------------------------------------------------------------


//impl Type for (fn(&[BType], &Context) -> EvalResult) {
//    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.clone()) }
//    fn to_bool(&self) -> bool { true }
//}
//
//impl <'a, 'b: 'a> IInvocable<'a> for (fn(&[BType], &Context) -> EvalResult) {
//    fn invoke(&self, args: &[BType], context: &Context) -> EvalResult {
//        self(args, context, env)
//    }
//}


