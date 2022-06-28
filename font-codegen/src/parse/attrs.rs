use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Lit};

use super::{ArrayField, CustomField, SingleField};

/// All of the attrs that can be applied to a field.
///
/// These are not validated, and do not all make sense on all fields;
/// rather they are just collected here.
#[derive(Default)]
pub struct FieldAttrs {
    docs: Vec<syn::Attribute>,
    hidden: Option<syn::Path>,
    no_getter: Option<syn::Path>,
    count: Option<Count>,
    pub(crate) read: Option<ArgList>,
    variable_size: Option<syn::Path>,
    compute: Option<Compute>,
    compile_type: Option<syn::Path>,
    to_owned: Option<syn::Expr>,
    nullable: Option<syn::Path>,
    skip_offset_getter: Option<syn::Path>,
}

/// Annotations for how to calculate the count of an array.
#[derive(Debug, Clone)]
pub enum Count {
    Field(syn::Ident),
    Literal(syn::LitInt),
    All(syn::Path),
    Function {
        fn_: syn::Path,
        args: Vec<syn::Ident>,
    },
}

/// A list of arguments contained in an attribute
#[derive(Debug, Clone)]
pub struct ArgList {
    attr: syn::Path,
    pub args: Vec<syn::Ident>,
}

/// Annotations for how to calculate certain fields
#[derive(Debug, Clone)]
pub enum Compute {
    /// computed from length of a given collection
    Len(syn::Ident),
    /// Any expression that resolves to the appropriate type (constant, function, etc)
    Expr(syn::Expr),
}

#[derive(Default)]
pub struct VariantAttrs {
    pub docs: Vec<syn::Attribute>,
    pub version: Option<Version>,
}

/// Used to specify the version/format specifier for an enum variant
pub enum Version {
    Lit(syn::LitInt),
    /// A path to a constant to be matched against
    Const(syn::Path),
    /// a path to a method which should return `true` for the first match
    With(syn::Path),
}

#[derive(Default)]
pub struct ItemAttrs {
    pub docs: Vec<syn::Attribute>,
    pub format: Option<syn::Ident>,
    pub generate_getters: Option<syn::Path>,
    pub offset_host: Option<syn::Path>,
    pub init: Vec<(syn::Ident, syn::Type)>,
    pub repr: Option<syn::Ident>,
    pub flags: Option<syn::Ident>,
    pub no_compile: Option<syn::Path>,
    pub skip_to_owned: Option<syn::Path>,
}

static COMPUTE_LEN: &str = "compute_count";
static COMPUTE: &str = "compute";
const NO_GETTER: &str = "no_getter";
const READ_WITH: &str = "read_with";
static COMPILE_TYPE: &str = "compile_type";
static TO_OWNED: &str = "to_owned";
static SKIP_OFFSET_GETTER: &str = "skip_offset_getter";
static NULLABLE: &str = "nullable";

impl FieldAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<FieldAttrs, syn::Error> {
        let mut result = FieldAttrs::default();
        for attr in attrs {
            if attr.path.is_ident(COMPILE_TYPE) {
                result.compile_type = Some(attr.parse_args()?);
                continue;
            } else if attr.path.is_ident(COMPUTE) {
                result.compute = Some(Compute::Expr(attr.parse_args()?));
                continue;
            } else if attr.path.is_ident(TO_OWNED) {
                result.to_owned = Some(attr.parse_args()?);
                continue;
            }
            match attr.parse_meta()? {
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::Path(path) if path.is_ident("hidden") => {
                    result.hidden = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident(NULLABLE) => {
                    result.nullable = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident(NO_GETTER) => {
                    result.no_getter = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident(SKIP_OFFSET_GETTER) => {
                    result.skip_offset_getter = Some(path.clone())
                }

                syn::Meta::Path(path) if path.is_ident("variable_size") => {
                    result.variable_size = Some(path.clone())
                }
                syn::Meta::Path(path) if path.is_ident("count_all") => {
                    result.count = Some(Count::All(path.clone()));
                }

                syn::Meta::List(list) if list.path.is_ident("count") => {
                    let inner = expect_single_item_list(&list)?;
                    match inner {
                        syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.get_ident().is_some() => {
                            result.count = Some(Count::Field(p.get_ident().unwrap().clone()));
                        }
                        syn::NestedMeta::Lit(syn::Lit::Int(int)) => {
                            result.count = Some(Count::Literal(int));
                        }
                        _ => return Err(syn::Error::new(
                            list.path.span(),
                            "count attribute should have format #[count(field)] or #[count(123)]",
                        )),
                    }
                }
                syn::Meta::List(list) if list.path.is_ident("count_with") => {
                    let mut items = list.nested.iter();
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = items.next() {
                        let args = items.map(expect_ident).collect::<Result<_, _>>()?;
                        assert!(result.count.is_none(), "I ONLY COUNT ONCE");
                        result.count = Some(Count::Function {
                            fn_: path.to_owned(),
                            args,
                        });
                        continue;
                    }
                    return Err(syn::Error::new(
                        list.path.span(),
                        "count_with attribute should have format count_with(path::to::fn, arg1, arg2)",
                    ));
                }
                syn::Meta::List(list) if list.path.is_ident(READ_WITH) => {
                    let args = list
                        .nested
                        .iter()
                        .map(expect_ident)
                        .collect::<Result<_, _>>()?;
                    result.read = Some(ArgList {
                        attr: list.path.clone(),
                        args,
                    })
                }
                syn::Meta::List(list) if list.path.is_ident(COMPUTE_LEN) => {
                    let inner = expect_single_item_list(&list)?;
                    result.compute = Some(Compute::Len(expect_ident(&inner)?));
                }
                other => {
                    return Err(syn::Error::new(other.span(), "unknown attribute"));
                }
            }
        }
        Ok(result)
    }

    pub fn into_array(
        self,
        name: syn::Ident,
        mut inner_typ: super::FieldType,
        inner_lifetime: Option<syn::Lifetime>,
    ) -> Result<ArrayField, syn::Error> {
        if let Some(path) = &self.hidden {
            return Err(syn::Error::new(
                path.span(),
                "'hidden' is only valid on scalar fields",
            ));
        }
        if let Some(read) = &self.read {
            if !matches!(inner_typ, super::FieldType::Offset { .. }) {
                return Err(syn::Error::new(
                    read.attr.span(),
                    "'read_with' only valid on arrays of offsets",
                ));
            }
        }
        if self.compute.is_some() {
            return Err(syn::Error::new(
                name.span(),
                "value/compute not valid on arrays",
            ));
        }
        let count = self.count.ok_or_else(|| {
            syn::Error::new(
                name.span(),
                "array types require 'count' or 'count_with' attribute",
            )
        })?;
        if self.nullable.is_some() {
            match &mut inner_typ {
                super::FieldType::Offset { nullable, .. } => *nullable = self.nullable,
                _ => {
                    return Err(syn::Error::new(
                        self.nullable.unwrap().span(),
                        "'nullable' only valid for offsets or arrays of offsets",
                    ))
                }
            }
        }
        Ok(ArrayField {
            docs: self.docs,
            name,
            inner_typ,
            inner_lifetime,
            count,
            variable_size: self.variable_size,
            no_getter: self.no_getter,
            to_owned: self.to_owned,
            read: self.read,
            skip_offset_getter: self.skip_offset_getter,
        })
    }

    pub fn into_single(
        self,
        name: syn::Ident,
        mut typ: super::FieldType,
    ) -> Result<SingleField, syn::Error> {
        if let Some(span) = self.count.as_ref().map(Count::span) {
            if self.read.is_none() {
                return Err(syn::Error::new(
                    span,
                    "count/count_with attribute not valid on scalar fields",
                ));
            }
        }
        if let Some(token) = self.variable_size {
            return Err(syn::Error::new(token.span(), "not valid on scalar fields"));
        }

        if let Some(read) = &self.read {
            if !matches!(typ, super::FieldType::Offset { .. }) {
                return Err(syn::Error::new(
                    read.attr.span(),
                    "'read_with' only valid on offsets or custom types",
                ));
            }
        }
        if let Some(path) = self.nullable {
            match &mut typ {
                super::FieldType::Offset { nullable, .. } => *nullable = Some(path),
                _ => return Err(syn::Error::new(path.span(), "only valid on offsets")),
            }
        }
        Ok(SingleField {
            docs: self.docs,
            name,
            typ,
            hidden: self.hidden,
            compute: self.compute,
            compile_type: self.compile_type,
            to_owned: self.to_owned,
            read: self.read,
        })
    }

    pub fn into_custom(
        self,
        name: syn::Ident,
        typ: syn::Path,
        lifetime: Option<syn::Lifetime>,
    ) -> Result<CustomField, syn::Error> {
        Ok(CustomField {
            docs: self.docs,
            name,
            typ,
            read: self.read.expect("hi"),
            count: self.count,
            compile_type: self.compile_type,
            inner_lifetime: lifetime,
        })
    }
}

impl Count {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Count::All(path) => path.span(),
            Count::Field(ident) => ident.span(),
            Count::Function { fn_, .. } => fn_.span(),
            Count::Literal(lit) => lit.span(),
        }
    }

    pub fn tokens(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Count::Field(name) => {
                let span = name.span();
                let resolved_value = crate::make_resolved_ident(name);
                Some(quote_spanned!(span=> #resolved_value))
            }
            Count::Literal(lit) => {
                let span = lit.span();
                Some(quote_spanned!(span=> #lit))
            }
            Count::Function { fn_, args } => {
                let span = fn_.span();
                let args = args.iter().map(crate::make_resolved_ident);
                Some(quote_spanned!(span=> #fn_( #( #args ),* )))
            }
            Count::All(_) => None,
        }
    }

    pub fn iter_input_fields(&self) -> impl Iterator<Item = &syn::Ident> {
        let fn_fields = match self {
            Count::Function { args, .. } => args.as_slice(),
            _ => &[],
        };

        let field = match self {
            Count::Field(ident) => Some(ident),
            _ => None,
        };

        field.into_iter().chain(fn_fields)
    }
}

static VERSION: &str = "version";
static VERSION_WITH: &str = "version_with";

impl VariantAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<VariantAttrs, syn::Error> {
        let mut result = VariantAttrs::default();
        for attr in attrs {
            match attr.parse_meta()? {
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::List(list) if list.path.is_ident(VERSION) => {
                    let item = expect_single_item_list(&list)?;
                    result.version = match item {
                        syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                            Some(Version::Const(p.clone()))
                        }
                        syn::NestedMeta::Lit(syn::Lit::Int(lit)) => Some(Version::Lit(lit)),
                        _ => {
                            return Err(syn::Error::new(
                                list.path.span(),
                                "expected integer literal or path to constant",
                            ))
                        }
                    };
                }
                syn::Meta::List(list) if list.path.is_ident(VERSION_WITH) => {
                    let inner = expect_single_item_list(&list)?;
                    if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = inner {
                        result.version = Some(Version::With(path));
                    } else {
                        return Err(syn::Error::new(inner.span(), "expected path to method"));
                    }
                }
                other => return Err(syn::Error::new(other.span(), "unknown attribute")),
            }
        }
        Ok(result)
    }
}

static FORMAT: &str = "format";
static REPR: &str = "repr";
static FLAGS: &str = "flags";
static OFFSET_HOST: &str = "offset_host";
static GENERATE_GETTERS: &str = "generate_getters";
static READ_ARGS: &str = "read_args";
static NO_COMPILE: &str = "no_compile";
static SKIP_TO_OWNED: &str = "skip_to_owned";

impl ItemAttrs {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<ItemAttrs, syn::Error> {
        let mut result = ItemAttrs::default();
        for attr in attrs {
            match attr.parse_meta()? {
                syn::Meta::Path(path) if path.is_ident(OFFSET_HOST) => {
                    result.offset_host = Some(path)
                }
                syn::Meta::Path(path) if path.is_ident(NO_COMPILE) => {
                    result.no_compile = Some(path);
                }
                syn::Meta::Path(path) if path.is_ident(SKIP_TO_OWNED) => {
                    result.skip_to_owned = Some(path);
                }
                syn::Meta::Path(path) if path.is_ident(GENERATE_GETTERS) => {
                    result.generate_getters = Some(path)
                }
                syn::Meta::NameValue(value) if value.path.is_ident("doc") => {
                    result.docs.push(attr.clone());
                }
                syn::Meta::List(list) if list.path.is_ident(REPR) => {
                    let item = expect_single_item_list(&list)?;
                    result.repr = Some(expect_ident(&item)?);
                }
                syn::Meta::List(list) if list.path.is_ident(FLAGS) => {
                    let item = expect_single_item_list(&list)?;
                    result.flags = Some(expect_ident(&item)?);
                }
                syn::Meta::List(list) if list.path.is_ident(FORMAT) => {
                    let item = expect_single_item_list(&list)?;
                    result.format = Some(expect_ident(&item)?);
                }
                syn::Meta::List(list) if list.path.is_ident(READ_ARGS) => {
                    result.init = list
                        .nested
                        .iter()
                        .map(expect_init_arg)
                        .collect::<Result<_, _>>()?;
                }
                other => return Err(syn::Error::new(other.span(), "unknown attribute")),
            }
        }
        Ok(result)
    }
}

fn expect_single_item_list(meta: &syn::MetaList) -> Result<syn::NestedMeta, syn::Error> {
    match meta.nested.first() {
        Some(item) if meta.nested.len() == 1 => Ok(item.clone()),
        _ => Err(syn::Error::new(meta.span(), "expected single item list")),
    }
}

fn expect_init_arg(meta: &syn::NestedMeta) -> Result<(syn::Ident, syn::Type), syn::Error> {
    match meta {
        syn::NestedMeta::Meta(syn::Meta::NameValue(namevalue))
            if namevalue.path.get_ident().is_some() =>
        {
            let name = namevalue.path.get_ident().unwrap();
            if let Lit::Str(s) = &namevalue.lit {
                let typ: syn::Type = syn::parse_str(s.value().trim_matches('"'))?;
                Ok((name.clone(), typ))
            } else {
                Err(syn::Error::new(
                    namevalue.lit.span(),
                    "type must be a string literal (e.g.: 'name = \"usize\"')",
                ))
            }
        }
        _ => Err(syn::Error::new(meta.span(), "expected 'name = type'")),
    }
}

fn expect_ident(meta: &syn::NestedMeta) -> Result<syn::Ident, syn::Error> {
    match meta {
        syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.get_ident().is_some() => {
            Ok(p.get_ident().unwrap().clone())
        }
        _ => Err(syn::Error::new(meta.span(), "expected ident")),
    }
}

impl ArgList {
    pub(crate) fn for_read_with_args(&self) -> proc_macro2::TokenStream {
        match self.args.as_slice() {
            [arg] => quote!(&self.#arg()),
            args => {
                quote!( &( #(self.#args()),* ) )
            }
        }
    }
}

impl Version {
    pub fn const_version_tokens(&self) -> Option<&syn::Path> {
        match self {
            Version::Const(path) => Some(path),
            _ => None,
        }
    }
}

impl ToTokens for Version {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        match self {
            Version::Lit(lit) => lit.to_tokens(stream),
            Version::Const(path) => path.to_tokens(stream),
            Version::With(path) => {
                let span = path.span();
                stream.extend(quote_spanned!(span=> v if #path(v)))
            }
        }
    }
}
