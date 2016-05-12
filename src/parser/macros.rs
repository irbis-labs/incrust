#[macro_export]
macro_rules! stmt {
    ( $i:expr, $cmd: expr ) => ({
        use nom::multispace;
        chaining_parser!($i, 0usize,
            tag!("{%")              ~
            opt!(multispace)        ~
            tag!($cmd)              ~
            opt!(multispace)        ~
            tag!("%}")              ,
            || -> Statement { ().into() }
        )
    });
}


/// `take_till!(T -> bool) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes until the provided function succeeds
///
/// The argument is either a function `&[T] -> bool` or a macro returning a `bool
#[macro_export]
macro_rules! take_till_slc (
  ($input:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use nom::InputLength;
      match $input.iter().enumerate().position(|(i, _)| $submac!(&$input[i..], $($args)*)) {
        Some(n) => IResult::Done(&$input[n..], &$input[..n]),
        None    => IResult::Done(&$input[($input).input_len()..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    take_till_slc!($input, call!($f));
  );
);


