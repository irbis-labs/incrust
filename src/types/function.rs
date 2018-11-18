use std::fmt::{Debug, Formatter, Error};

use abc::EvalResult;
use VarContext;

use types::abc::*;
use Arg;


pub struct Function {
    pub f: for <'res, 'ctx> fn(&[Arg<'res>], &'ctx VarContext<'res>) -> EvalResult<Arg<'res>>,
}

impl Function {
    // TODO update API to satisfy convention
    #[allow(clippy::new_ret_no_self)]
    pub fn new(f: for <'res, 'ctx> fn(&[Arg<'res>], &'ctx VarContext<'res>) -> EvalResult<Arg<'res>>) -> Arg<'static> {
        Arg::Owned(box Function { f })
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

impl IInvokable for Function {
    fn invoke<'r: 'a + 'c, 'a, 'c>(&self, args: &'a [Arg<'r>], context: &'c VarContext<'r>) -> EvalResult<Arg<'r>> {
        (self.f)(args, context)
    }
}



// --------------------------------------------------------------------------------------------------------------------


//impl Type for (fn(&[BType], &Context) -> EvalResult) {
//    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.clone()) }
//    fn to_bool(&self) -> bool { true }
//}
//
//impl <'a, 'b: 'a> IInvokable<'a> for (fn(&[BType], &Context) -> EvalResult) {
//    fn invoke(&self, args: &[BType], context: &Context) -> EvalResult {
//        self(args, context, env)
//    }
//}


