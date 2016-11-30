use std::fmt::{Debug, Formatter, Error};

use ::abc::{EvalResult};
use ::incrust::{Context, Incrust};

use super::abc::*;


pub struct Function {
    pub f: fn(&[BType], &Context, &Incrust) -> EvalResult,
}

impl Function {
    pub fn new<'a>(f: fn(&[BType], &Context, &Incrust) -> EvalResult) -> BType<'a> {
        Box::new(Function { f: f })
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
    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.clone()) }
    fn to_bool(&self) -> bool { true }
}

impl <'a> Into<BType<'a>> for Function {
    fn into(self) -> BType<'a> { Box::new(self) }
}

impl <'b> AsInvocable for Function {
    fn try_as_invocable(&self) -> Option<&IInvocable> { Some(self) }
}

impl <'a, 'b: 'a> IInvocable<'a> for Function {
    fn invoke(&self, args: &[BType], context: &Context, env: &Incrust) -> EvalResult {
        (self.f)(args, context, env)
    }
}



// --------------------------------------------------------------------------------------------------------------------


//impl Type for (fn(&[BType], &Context, &Incrust) -> EvalResult) {
//    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.clone()) }
//    fn to_bool(&self) -> bool { true }
//}
//
//impl <'a, 'b: 'a> IInvocable<'a> for (fn(&[BType], &Context, &Incrust) -> EvalResult) {
//    fn invoke(&self, args: &[BType], context: &Context, env: &Incrust) -> EvalResult {
//        self(args, context, env)
//    }
//}


