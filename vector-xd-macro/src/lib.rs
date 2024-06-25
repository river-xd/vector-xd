
use quote::quote;
use proc_macro::TokenStream;
use std::collections::VecDeque;


use syn::{
  Ident,
  ItemStruct
};

use proc_macro2::{
  Span,
  TokenStream as TokenStream2
};


const OPERATIONS: [&'static str;8]=["Add","Sub","Mul","Div","AddAssign","SubAssign","MulAssign","DivAssign"];
const N: usize=OPERATIONS.len();


#[proc_macro_attribute]
pub fn derive_arith(_attrs: TokenStream,item: TokenStream)-> TokenStream {
  let structure=syn::parse::<ItemStruct>(item).expect("This macro only works on structs");
  let name=&structure.ident;
  let implementations: TokenStream2=implement_operations(name).into_iter().collect::<TokenStream>().into();

  quote::quote! {
    #structure
    #implementations
  }.into()
}


fn implement_operations(name: &Ident)-> [TokenStream;N] {
  OPERATIONS
  .map(|trait_name| {
    let _trait=Ident::new(trait_name,Span::call_site());
    let fn_name=Ident::new(&to_snake_case(trait_name),Span::call_site());

    match trait_name.ends_with("Assign") {
      true=> quote! {
        impl ::std::ops::#_trait<#name> for #name {
          #[inline]
          fn #fn_name(&mut self,rhs: Self) {
            self.x.#fn_name(rhs.x);
            self.y.#fn_name(rhs.y);
            self.z.#fn_name(rhs.z);
          }
        }

        impl ::std::ops::#_trait<f32> for #name {
          #[inline]
          fn #fn_name(&mut self,rhs: f32) {
            self.x.#fn_name(rhs);
            self.y.#fn_name(rhs);
            self.z.#fn_name(rhs);
          }
        }
      },
      _=> quote! {
        impl ::std::ops::#_trait<#name> for #name {
          type Output=Self;

          #[inline]
          fn #fn_name(self,rhs: Self)-> Self::Output {
            Self {
              x: self.x.#fn_name(rhs.x),
              y: self.y.#fn_name(rhs.y),
              z: self.z.#fn_name(rhs.z)
            }
          }
        }

        impl ::std::ops::#_trait<f32> for #name {
          type Output=Self;

          #[inline]
          fn #fn_name(self,rhs: f32)-> Self::Output {
            Self {
              x: self.x.#fn_name(rhs),
              y: self.y.#fn_name(rhs),
              z: self.z.#fn_name(rhs)
            }
          }
        }
      }
    }.into()
  })
}

// it assumes that the parameter is in camelCase
fn to_snake_case(string: &str)-> String {
  let mut snake_cased=VecDeque::new();

  for ch in string.as_bytes() {
    if ch.is_ascii_uppercase() {
      snake_cased.push_back(b'_');
    }

    snake_cased.push_back(ch.to_ascii_lowercase());
  }
  snake_cased.pop_front();

  // SAFETY: safety doesn't matter, let the CIA take care of it.
  unsafe {
    String::from_utf8_unchecked(snake_cased.into())
  }
}




