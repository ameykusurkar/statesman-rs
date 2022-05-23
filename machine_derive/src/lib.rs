use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(InMemoryMachine)]
pub fn derive_inmemory(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = ast.ident.clone();

    let struct_data = match ast.data {
        Data::Struct(data) => data,
        _ => unimplemented!("State can only derived for structs!"),
    };
    let fields = match struct_data.fields {
        Fields::Named(fields) => fields.named,
        _ => unimplemented!("Can only derive Machine for named fields!"),
    };
    let state_machine_field = fields
        .iter()
        .filter(|f| f.ident.is_some())
        .find(|f| f.ident.as_ref().unwrap().to_string() == String::from("state_machine"))
        .expect("No state_machine field on struct!");

    // TODO: Ensure machine type is InMemory
    let _machine_type = match &state_machine_field.ty {
        Type::Path(ty) => ty,
        _ => unimplemented!("state_machine needs to be a type path!"),
    };

    let state_ident = Ident::new(&format!("{}State", struct_ident), struct_ident.span());

    let impl_machine = quote! {
        impl Machine<#state_ident> for #struct_ident {
            type Transition = InMemoryTransition<#state_ident>;

            fn history(&self) -> &Vec<Self::Transition> {
                &self.state_machine.history()
            }

            fn create_transition(&mut self, to_state: #state_ident) {
                self.state_machine.create_transition(to_state)
            }
        }
    };

    TokenStream::from(impl_machine)
}
