#![feature(proc_macro)]

#[macro_use]
extern crate syn;
extern crate proc_macro;

use proc_macro::{TokenStream, Diagnostic};

use syn::ExprTuple;
use syn::spanned::Spanned;
use syn::synom::Synom;

struct Demo(ExprTuple, ExprTuple);

impl Synom for Demo {
    named!(parse -> Self, do_parse!(
        a: syn!(ExprTuple) >>
        punct!(=) >>
        b: syn!(ExprTuple) >>
        (Demo(a, b))
    ));
}

fn eval(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let Demo(a, b) = syn::parse(input)?;

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
