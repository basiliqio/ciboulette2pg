extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn ciboulette2postgres_test(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(input as ItemFn);
    let function_name = input_fn.sig.ident.clone();
	let new_function_name = Ident::new(format!("db_{}", function_name).as_str(), Span::call_site());
    input_fn.sig.ident = new_function_name.clone();

    TokenStream::from(quote! {
		#[tokio::test]
        async fn #function_name()
        {
        	let (db_id, pool) = init_db().await;
			
			let mut tx = pool.begin().await.expect("to get a transaction");
            #new_function_name(tx).await;
        }

        #input_fn
    })
}
