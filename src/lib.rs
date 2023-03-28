use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// integral enum procedural macro. See module-level documentation for usage
#[proc_macro_error]
#[proc_macro_attribute]
pub fn integral_enum(args: TokenStream, body: TokenStream) -> TokenStream {
    integral::integral_enum_impl(args, body)
}

mod integral;
mod tag;
