use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, Data, DeriveInput, Ident, PathArguments,
    PathSegment, Token, Variant,
};

#[proc_macro_derive(State, attributes(can_transition_to))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_ident = ast.ident.clone();

    let enum_data = match ast.data {
        Data::Enum(data) => data,
        _ => unimplemented!("State can only derived for enums!"),
    };

    let transitions = enum_data
        .variants
        .iter()
        .flat_map(|state| build_state_transitions(&enum_ident, state))
        .map(|Transition(from, to)| {
            quote! {
                (#from, #to) => true,
            }
        });

    let expanded = quote! {
        impl State for #enum_ident {
            fn can_transition_to(&self, to_state: Self) -> bool {
                match (self, to_state) {
                    #(#transitions)*
                    (_, _) => false,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

struct Transition(
    Punctuated<PathSegment, Token![::]>,
    Punctuated<PathSegment, Token![::]>,
);

fn build_state_transitions(enum_ident: &Ident, state: &Variant) -> Vec<Transition> {
    state
        .attrs
        .iter()
        .filter_map(|attr| {
            let from_state = state.ident.to_string();
            let to_state = extract_to_state(attr)?;

            Some(Transition(
                build_state_path_segment(enum_ident, &from_state),
                build_state_path_segment(enum_ident, &to_state),
            ))
        })
        .collect()
}

fn extract_to_state(attr: &Attribute) -> Option<String> {
    if !attr.path.is_ident("can_transition_to") {
        return None;
    }

    if let Some(proc_macro2::TokenTree::Group(group)) = attr.tokens.clone().into_iter().next() {
        let mut tokens = group.stream().into_iter();
        Some(tokens.next().expect("Expected state name!").to_string())
    } else {
        panic!("Invalid transition declaration!")
    }
}

fn build_state_path_segment(
    enum_ident: &Ident,
    state: &str,
) -> Punctuated<PathSegment, Token![::]> {
    let mut segment: Punctuated<PathSegment, Token![::]> = Punctuated::new();
    segment.push(PathSegment {
        // TODO: Use correct span
        ident: Ident::new(&enum_ident.to_string(), enum_ident.span()),
        arguments: PathArguments::None,
    });
    segment.push(PathSegment {
        // TODO: Use correct span
        ident: Ident::new(state, enum_ident.span()),
        arguments: PathArguments::None,
    });
    segment
}
