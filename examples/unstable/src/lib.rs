#![feature(proc_macro)]

extern crate syn;
extern crate proc_macro;

use proc_macro::{TokenStream, Diagnostic};

use syn::*;
use syn::spanned::Spanned;
use syn::synom::{Synom, Cursor, SynomBuffer};

struct Parser {
    buffer: Box<SynomBuffer>,
    cursor: Cursor<'static>
}

impl Parser {
    fn new(tokens: TokenStream) -> Parser {
        let buffer = Box::new(SynomBuffer::new(tokens.into()));
        let cursor = unsafe {
            let buffer: &'static SynomBuffer = ::std::mem::transmute(&*buffer);
            buffer.begin()
        };

        Parser {
            buffer: buffer,
            cursor: cursor
        }
    }

    fn parse<T: Synom>(&mut self) -> Result<T, Diagnostic> {
        let (cursor, val) = T::parse(self.cursor).map_err(|e| e.into())?;
        self.cursor = cursor;
        Ok(val)
    }
}

fn eval(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let mut parser = Parser::new(input);

    let a = parser.parse::<ExprTuple>()?;
    parser.parse::<token::Eq>()?;
    let b = parser.parse::<ExprTuple>()?;

    let (a_len, b_len) = (a.args.len(), b.args.len());
    if a_len != b_len {
        let diag = b.span().expect("b's span")
            .error(format!("expected {} element(s), got {}", a_len, b_len))
            .span_note(a.span().expect("a's span"), "because of this");

        return Err(diag);
    }

    Ok("println!(\"All good!\")".parse().unwrap())
}

#[proc_macro]
pub fn demo(input: TokenStream) -> TokenStream {
    match eval(input) {
        Ok(val) => val,
        Err(diag) => {
            diag.emit();
            "".parse().unwrap()
        }
    }
}
