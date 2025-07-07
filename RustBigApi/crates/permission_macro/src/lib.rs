extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn permission(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the permission string from the attribute
    let mut required_perm = None;
    for arg in args {
        if let NestedMeta::Lit(Lit::Str(lit_str)) = arg {
            required_perm = Some(lit_str.value());
        }
    }
    let required_perm = required_perm.expect("permission macro requires a string argument");

    // Get the function signature and body
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    // Generate the wrapper code
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use common::utils::extract_jwt_claims::extract_jwt_claims_from_request;
            use common::utils::get_user_permissions;
            use uuid::Uuid;
            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env variable must be set");
            let claims = extract_jwt_claims_from_request(&req, &secret)
                .map_err(|e| ServiceError::Unauthorized(e.to_string()))?;
            let user_uuid = Uuid::parse_str(&claims.sub)
                .map_err(|_| ServiceError::Unauthorized("Invalid user UUID in token".to_string()))?;
            let mut conn = pool.get().map_err(|_| ServiceError::InternalServerError("DB connection error".to_string()))?;
            let permissions = get_user_permissions(&mut conn, user_uuid)
                .map_err(|_| ServiceError::InternalServerError("Failed to fetch permissions".to_string()))?;
            if !permissions.contains(&#required_perm.to_string()) {
                return Err(ServiceError::Forbidden(format!("Missing required permission: {}", #required_perm)));
            }
            #block
        }
    };
    TokenStream::from(expanded)
}

