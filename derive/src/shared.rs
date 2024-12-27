use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Fields, GenericParam, Generics, Index, Type, TypeArray, WherePredicate};

pub struct FieldAndIndexes {
    pub field_indexes: Vec<usize>,
    pub field_idents: Vec<TokenStream>,
}

pub fn field_idents_indexes(fields: &Fields) -> FieldAndIndexes {
    let field_indexes = field_indexes(fields.len());
    let field_idents = field_idents(fields);
    FieldAndIndexes {
        field_indexes,
        field_idents,
    }
}

pub struct FieldTypeAndIndexes {
    pub field_indexes: Vec<usize>,
    pub field_types: Vec<TokenStream>,
}

pub fn field_types_and_indexes(fields: &Fields) -> FieldTypeAndIndexes {
    let field_indexes = field_indexes(fields.len());
    let field_types = field_types(fields);
    FieldTypeAndIndexes {
        field_indexes,
        field_types,
    }
}

pub struct FieldTypeAndIdentAndIndexes {
    pub field_indexes: Vec<usize>,
    pub field_types: Vec<TokenStream>,
    pub field_idents: Vec<TokenStream>,
}

pub fn field_types_idents_and_indexes(fields: &Fields) -> FieldTypeAndIdentAndIndexes {
    let field_indexes = field_indexes(fields.len());
    let field_types = field_types(fields);
    let field_idents = field_idents(fields);
    FieldTypeAndIdentAndIndexes {
        field_indexes,
        field_types,
        field_idents,
    }
}

pub fn field_indexes(len: usize) -> Vec<usize> {
    (0..len).collect::<Vec<_>>()
}

pub fn field_types(fields: &Fields) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| field.ty.to_token_stream())
        .collect::<Vec<_>>()
}

pub fn field_idents(fields: &Fields) -> Vec<TokenStream> {
    let mut index = 0;
    fields
        .iter()
        .map(|field| {
            let result = field
                .ident
                .as_ref()
                .map(|v| v.to_token_stream())
                .unwrap_or_else(|| {
                    Index {
                        index,
                        span: Span::call_site(),
                    }
                    .to_token_stream()
                });
            index += 1;
            result
        })
        .collect::<Vec<_>>()
}

pub fn remove_lifetimes(generics: &mut Generics) {
    generics.params = std::mem::take(&mut generics.params)
        .into_iter()
        .filter(|param| !matches!(param, GenericParam::Lifetime(_)))
        .collect();
    if let Some(where_clause) = &mut generics.where_clause {
        where_clause.predicates = std::mem::take(&mut where_clause.predicates)
            .into_iter()
            .filter(|predicate| !matches!(predicate, WherePredicate::Lifetime(_)))
            .collect()
    }
}
