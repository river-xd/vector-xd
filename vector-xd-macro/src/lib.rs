use proc_macro::TokenStream;


extern crate proc_macro;

#[proc_macro]
pub fn _xd(token: TokenStream)-> TokenStream {
  TokenStream::new()
}


