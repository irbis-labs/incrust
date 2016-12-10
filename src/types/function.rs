use std::fmt::{Debug, Formatter, Error};

use ::abc::EvalResult;
use ::incrust::Context;

use super::abc::*;


pub struct Function {
    pub f: fn(&[BType], &Context) -> EvalResult,
}

impl Function {
    pub fn new<'a>(f: fn(&[BType], &Context) -> EvalResult) -> BType<'a> {
        box Function { f: f }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function { f: self.f }
    }
}

impl Debug for Function {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str("Anonymous Function")
    }
}

impl Type for Function {
    fn iclone<'a>(&self) -> BType<'a> {
        box self.clone()
    }
}

impl <'a> Into<BType<'a>> for Function {
    fn into(self) -> BType<'a> {
        box self
    }
}

impl AsInvocable for Function {
    fn try_as_invocable(&self) -> Option<&IInvocable> {
        Some(self)
    }

    fn is_invocable(&self) -> bool {
        true
    }
}

impl IInvocable for Function {
    fn invoke(&self, args: &[BType], context: &Context) -> EvalResult {
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


