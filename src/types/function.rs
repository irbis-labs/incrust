use std::borrow::Cow;
use std::fmt::{Debug, Formatter, Error};

use abc::EvalResult;
use Context;

use types::abc::*;
use Arg;


pub struct Function {
    pub f: fn(&[Cow<Arg>], &Context) -> EvalResult<Arg>,
}

impl Function {
    pub fn new(f: fn(&[Cow<Arg>], &Context) -> EvalResult<Arg>) -> Arg {
        Arg::Boxed(box Function { f: f })
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
    fn iclone(&self) -> Arg {
        Arg::Boxed(box self.clone())
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
    fn invoke<'a: 'b, 'b>(&self, args: &'b [Cow<'a, Arg>], context: &'a Context) -> EvalResult<Cow<'a, Arg>> {
        // todo Cow for self.f
        (self.f)(args, context).map(|v| v.map(Cow::Owned))
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


