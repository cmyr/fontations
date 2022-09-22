//! raw parsing code

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token, Attribute, Token,
};

pub(crate) struct Items {
    pub(crate) parse_module_path: syn::Path,
    pub(crate) items: Vec<Item>,
}

pub(crate) enum Item {
    Table(Table),
    Record(Record),
    Format(TableFormat),
    GenericGroup(GenericGroup),
    RawEnum(RawEnum),
    Flags(BitFlags),
}

#[derive(Debug, Clone)]
pub(crate) struct Table {
    pub(crate) attrs: TableAttrs,
    name: syn::Ident,
    pub(crate) fields: Fields,
}

impl Table {
    // here for visibility reasons
    pub(crate) fn raw_name(&self) -> &syn::Ident {
        &self.name
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct TableAttrs {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) skip_parse: Option<syn::Path>,
    pub(crate) skip_compile: Option<syn::Path>,
    pub(crate) skip_from_obj: Option<syn::Path>,
    pub(crate) validation_method: Option<Attr<syn::Path>>,
    pub(crate) read_args: Option<Attr<TableReadArgs>>,
    pub(crate) phantom: Option<Attr<syn::Ident>>,
}

#[derive(Debug, Clone)]
pub(crate) struct TableReadArgs {
    pub(crate) args: Vec<TableReadArg>,
}

#[derive(Debug, Clone)]
pub(crate) struct TableReadArg {
    pub(crate) ident: syn::Ident,
    pub(crate) typ: syn::Ident,
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) attrs: TableAttrs,
    pub(crate) lifetime: Option<TokenStream>,
    pub(crate) name: syn::Ident,
    pub(crate) fields: Fields,
}

/// A table with a format; we generate an enum
#[derive(Debug, Clone)]
pub(crate) struct TableFormat {
    pub(crate) attrs: TableAttrs,
    pub(crate) name: syn::Ident,
    pub(crate) format: syn::Ident,
    pub(crate) variants: Vec<FormatVariant>,
}

#[derive(Debug, Clone)]
pub(crate) struct FormatVariant {
    pub(crate) attrs: VariantAttrs,
    pub(crate) name: syn::Ident,
    typ: syn::Ident,
}

/// Generates an enum where each variant has a different generic param to a single type.
///
/// This is used in GPOS/GSUB, allowing us to provide more type information
/// to lookups.
#[derive(Debug, Clone)]
pub(crate) struct GenericGroup {
    pub(crate) attrs: TableAttrs,
    pub(crate) name: syn::Ident,
    /// the inner type, which must accept a generic parameter
    pub(crate) inner_type: syn::Ident,
    /// The field on the inner type that determines the type of the generic param
    pub(crate) inner_field: syn::Ident,
    pub(crate) variants: Vec<GroupVariant>,
}

#[derive(Debug, Clone)]
pub(crate) struct GroupVariant {
    pub(crate) type_id: syn::LitInt,
    pub(crate) name: syn::Ident,
    pub(crate) typ: syn::Ident,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct VariantAttrs {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) match_stmt: Option<Attr<InlineExpr>>,
}

impl FormatVariant {
    pub(crate) fn marker_name(&self) -> syn::Ident {
        quote::format_ident!("{}Marker", &self.typ)
    }

    pub(crate) fn type_name(&self) -> &syn::Ident {
        &self.typ
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Fields {
    // not parsed, but set when the table/record is parsed
    pub(crate) read_args: Option<TableReadArgs>,
    pub(crate) fields: Vec<Field>,
    pub(crate) referenced_fields: ReferencedFields,
}

#[derive(Debug, Clone)]
pub(crate) struct ReferencedFields(HashMap<syn::Ident, NeededWhen>);

#[derive(Debug, Clone, Copy)]
pub(crate) enum NeededWhen {
    Parse,
    Runtime,
    Both,
}

#[derive(Debug, Clone)]
pub(crate) struct Field {
    pub(crate) attrs: FieldAttrs,
    pub(crate) name: syn::Ident,
    pub(crate) typ: FieldType,
    /// `true` if this field is required to be read in order to parse subsequent
    /// fields.
    ///
    /// For instance: in a versioned table, the version must be read to determine
    /// whether to expect version-dependent fields.
    pub(crate) read_at_parse_time: bool,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct FieldAttrs {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) nullable: Option<syn::Path>,
    pub(crate) available: Option<Attr<syn::Expr>>,
    pub(crate) skip_getter: Option<syn::Path>,
    /// if present, we will not try to resolve this offset
    pub(crate) skip_offset_getter: Option<syn::Path>,
    pub(crate) version: Option<syn::Path>,
    pub(crate) format: Option<Attr<syn::LitInt>>,
    pub(crate) count: Option<Attr<Count>>,
    pub(crate) compile: Option<Attr<InlineExpr>>,
    pub(crate) compile_type: Option<Attr<syn::Path>>,
    pub(crate) len: Option<Attr<InlineExpr>>,
    pub(crate) read_with_args: Option<Attr<FieldReadArgs>>,
    pub(crate) read_offset_args: Option<Attr<FieldReadArgs>>,
    pub(crate) to_owned: Option<Attr<InlineExpr>>,
}

#[derive(Debug, Clone)]
pub(crate) struct Attr<T> {
    pub(crate) name: syn::Ident,
    pub(crate) attr: T,
}

impl<T> Attr<T> {
    fn new(name: syn::Ident, attr: T) -> Self {
        Attr { name, attr }
    }

    pub(crate) fn span(&self) -> Span {
        self.name.span()
    }
}

impl<T> std::ops::Deref for Attr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.attr
    }
}

impl<T: ToTokens> ToTokens for Attr<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attr.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FieldReadArgs {
    pub(crate) inputs: Vec<syn::Ident>,
}

/// Annotations for how to calculate the count of an array.
#[derive(Debug, Clone)]
pub(crate) enum Count {
    Field(syn::Ident),
    Expr(InlineExpr),
    All,
}

/// an inline expression used in an attribute
///
/// this has one fancy quality: you can reference fields of the current
/// object by prepending a '$' to the field name, e.g.
///
/// `#[count( $num_items - 1 )]`
#[derive(Debug, Clone)]
pub(crate) struct InlineExpr {
    pub(crate) expr: Box<syn::Expr>,
    // the expression used in a compilation context. This resolves any referenced
    // fields against `self`.
    compile_expr: Option<Box<syn::Expr>>,
    pub(crate) referenced_fields: Vec<syn::Ident>,
}

impl InlineExpr {
    pub(crate) fn compile_expr(&self) -> &syn::Expr {
        self.compile_expr.as_ref().unwrap_or(&self.expr)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum FieldType {
    Offset {
        typ: syn::Ident,
        target: Option<syn::Ident>,
    },
    Scalar {
        typ: syn::Ident,
    },
    Other {
        typ: syn::Path,
    },
    Array {
        inner_typ: Box<FieldType>,
    },
    ComputedArray(ComputedArray),
}

#[derive(Debug, Clone)]
pub(crate) struct ComputedArray {
    span: Span,
    inner: syn::Ident,
    lifetime: Option<syn::Lifetime>,
}

impl ComputedArray {
    pub(crate) fn compile_type(&self) -> TokenStream {
        let inner = &self.inner;
        quote!(Vec<#inner>)
    }

    pub(crate) fn raw_inner_type(&self) -> &syn::Ident {
        &self.inner
    }

    pub(crate) fn type_with_lifetime(&self) -> TokenStream {
        let inner = &self.inner;
        if self.lifetime.is_some() {
            quote!(#inner<'a>)
        } else {
            inner.to_token_stream()
        }
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

/// A raw c-style enum
#[derive(Debug, Clone)]
pub(crate) struct RawEnum {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) name: syn::Ident,
    pub(crate) typ: syn::Ident,
    pub(crate) variants: Vec<RawVariant>,
}

/// A raw scalar variant
#[derive(Debug, Clone)]
pub(crate) struct RawVariant {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) name: syn::Ident,
    pub(crate) value: syn::LitInt,
}

/// A set of bit-flags
#[derive(Debug, Clone)]
pub(crate) struct BitFlags {
    pub(crate) docs: Vec<syn::Attribute>,
    pub(crate) name: syn::Ident,
    pub(crate) typ: syn::Ident,
    pub(crate) variants: Vec<RawVariant>,
}

mod kw {
    syn::custom_keyword!(table);
    syn::custom_keyword!(record);
    syn::custom_keyword!(flags);
    syn::custom_keyword!(format);
    syn::custom_keyword!(group);
}

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut items = Vec::new();
        let parse_module_path = get_parse_module_path(input)?;
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Self {
            items,
            parse_module_path,
        })
    }
}

fn get_parse_module_path(input: ParseStream) -> syn::Result<syn::Path> {
    let attrs = input.call(Attribute::parse_inner)?;
    match attrs.as_slice() {
        [one] if one.path.is_ident("parse_module") => one.parse_args(),
        [one] => Err(syn::Error::new(one.span(), "unexpected attribute")),
        [_, two, ..] => Err(syn::Error::new(
            two.span(),
            "expected at most one top-level attribute",
        )),
        [] => Err(syn::Error::new(
            Span::call_site(),
            "expected #![parse_module(..)] attribute",
        )),
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let peek = input.fork();
        // skip attributes
        while peek.lookahead1().peek(Token![#]) {
            Attribute::parse_outer(&peek)?;
        }

        let lookahead = peek.lookahead1();
        if lookahead.peek(kw::table) {
            Ok(Self::Table(input.parse()?))
        } else if lookahead.peek(kw::record) {
            Ok(Self::Record(input.parse()?))
        } else if lookahead.peek(kw::flags) {
            Ok(Self::Flags(input.parse()?))
        } else if lookahead.peek(kw::format) {
            Ok(Self::Format(input.parse()?))
        } else if lookahead.peek(kw::group) {
            Ok(Self::GenericGroup(input.parse()?))
        } else if lookahead.peek(Token![enum]) {
            Ok(Self::RawEnum(input.parse()?))
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected one of 'table' 'record' 'flags' 'format' 'enum' or 'group'.",
            ))
        }
    }
}

impl Parse for Table {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: TableAttrs = input.parse()?;
        let _table = input.parse::<kw::table>()?;
        let name = input.parse::<syn::Ident>()?;

        let mut fields: Fields = input.parse()?;
        fields.read_args = attrs.read_args.clone().map(|attrs| attrs.attr);
        Ok(Table {
            attrs,
            name,
            fields,
        })
    }
}

impl Parse for Record {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: TableAttrs = input.parse()?;
        let _kw = input.parse::<kw::record>()?;
        let name = input.parse::<syn::Ident>()?;
        let lifetime = input
            .peek(Token![<])
            .then(|| {
                input.parse::<Token![<]>()?;
                input.parse::<syn::Lifetime>()?;
                input.parse::<Token![>]>().map(|_| quote!(<'a>))
            })
            .transpose()?;

        let mut fields: Fields = input.parse()?;
        fields.read_args = attrs.read_args.clone().map(|attrs| attrs.attr);
        Ok(Record {
            attrs,
            lifetime,
            name,
            fields,
        })
    }
}

impl Parse for BitFlags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let docs = get_optional_docs(input)?;
        let _kw = input.parse::<kw::flags>()?;
        let typ = input.parse::<syn::Ident>()?;
        validate_ident(&typ, &["u8", "u16"], "allowed bitflag types: u8, u16")?;
        let name = input.parse::<syn::Ident>()?;

        let content;
        let _ = braced!(content in input);
        let variants = Punctuated::<RawVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(BitFlags {
            docs,
            name,
            typ,
            variants,
        })
    }
}

impl Parse for RawEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let docs = get_optional_docs(input)?;
        let _kw = input.parse::<Token![enum]>()?;
        let typ = input.parse::<syn::Ident>()?;
        validate_ident(&typ, &["u8", "u16"], "allowed enum types: u8, u16")?;
        let name = input.parse::<syn::Ident>()?;
        let content;
        let _ = braced!(content in input);
        let variants = Punctuated::<RawVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(RawEnum {
            docs,
            name,
            typ,
            variants,
        })
    }
}

impl Parse for TableFormat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs: TableAttrs = input.parse()?;
        let _kw = input.parse::<kw::format>()?;
        let format: syn::Ident = input.parse()?;
        validate_ident(&format, &["u16", "i16"], "unexpected format type")?;
        let name = input.parse::<syn::Ident>()?;

        let content;
        let _ = braced!(content in input);
        let variants = Punctuated::<FormatVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(TableFormat {
            attrs,
            format,
            name,
            variants,
        })
    }
}

impl Parse for GenericGroup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.parse::<TableAttrs>()?;
        let _kw = input.parse::<kw::group>()?;
        let name = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let inner_type = content.parse()?;
        content.parse::<Token![,]>()?;
        content.parse::<Token![$]>()?;
        let inner_field = content.parse()?;
        let content;
        let _ = braced!(content in input);
        let variants = Punctuated::<GroupVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(GenericGroup {
            attrs,
            name,
            inner_type,
            inner_field,
            variants,
        })
    }
}

impl Parse for GroupVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_id = input.parse()?;
        input.parse::<Token![=>]>()?;
        let name = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let typ = content.parse()?;
        Ok(GroupVariant { type_id, name, typ })
    }
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = braced!(content in input);
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Self::new(fields)
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.parse()?;
        let name = input.parse::<syn::Ident>().unwrap();
        let _ = input.parse::<Token![:]>()?;
        let typ = input.parse()?;
        Ok(Field {
            attrs,
            name,
            typ,
            // computed later
            read_at_parse_time: false,
        })
    }
}

impl Parse for FieldType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.lookahead1().peek(token::Bracket) {
            let content;
            bracketed!(content in input);
            let span = content.span();
            let inner_typ: FieldType = content.parse()?;
            if matches!(inner_typ, FieldType::Array { .. }) {
                return Err(syn::Error::new(span, "nested arrays are invalid"));
            }
            return Ok(FieldType::Array {
                inner_typ: Box::new(inner_typ),
            });
        }

        let path = input.parse::<syn::Path>()?;
        let last = get_single_path_segment(&path)?;

        if last.ident == "ComputedArray" {
            let inner_typ = get_single_generic_type_arg(&last.arguments)?;
            let last = get_single_path_segment(&inner_typ)?;
            let lifetime = get_single_lifetime(&last.arguments)?;
            return Ok(FieldType::ComputedArray(ComputedArray {
                span: last.span(),
                inner: last.ident.clone(),
                lifetime,
            }));
        }

        if last.ident != "BigEndian" {
            return Ok(FieldType::Other { typ: path });
        }

        let inner = get_single_generic_type_arg(&last.arguments)?;
        let last = inner.segments.last().unwrap();
        if ["Offset16", "Offset24", "Offset32"].contains(&last.ident.to_string().as_str()) {
            let target = get_single_generic_type_arg(&last.arguments)
                .ok()
                .map(|p| p.segments.last().unwrap().ident.clone());
            Ok(FieldType::Offset {
                typ: last.ident.clone(),
                target,
            })
        } else if last.arguments.is_empty() {
            Ok(FieldType::Scalar {
                typ: last.ident.clone(),
            })
        } else {
            Err(syn::Error::new(last.span(), "unexpected arguments"))
        }
    }
}

fn get_single_path_segment(path: &syn::Path) -> syn::Result<&syn::PathSegment> {
    if path.segments.len() != 1 {
        return Err(syn::Error::new(path.span(), "expect a single-item path"));
    }
    Ok(path.segments.last().unwrap())
}

impl Parse for FieldReadArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut inputs = Vec::new();
        while !input.is_empty() {
            input.parse::<Token![$]>()?;
            inputs.push(input.parse::<syn::Ident>()?);
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(FieldReadArgs { inputs })
    }
}

impl Parse for RawVariant {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let docs = get_optional_docs(input)?;
        let name = input.parse::<syn::Ident>()?;
        let _ = input.parse::<Token![=]>()?;
        let value: syn::LitInt = input.parse()?;
        Ok(Self { docs, name, value })
    }
}

impl Parse for FormatVariant {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let attrs = input.parse()?;
        let name = input.parse::<syn::Ident>()?;
        let content;
        parenthesized!(content in input);
        let typ = content.parse::<syn::Ident>()?;
        Ok(Self { attrs, name, typ })
    }
}

static MATCH_IF: &str = "match_if";

impl Parse for VariantAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = VariantAttrs::default();
        let attrs = Attribute::parse_outer(input)
            .map_err(|e| syn::Error::new(e.span(), format!("hmm: '{e}'")))?;

        for attr in attrs {
            let ident = attr.path.get_ident().ok_or_else(|| {
                syn::Error::new(attr.path.span(), "attr paths should be a single identifer")
            })?;
            if ident == DOC {
                this.docs.push(attr);
            } else if ident == MATCH_IF {
                this.match_stmt = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown variant attribute {ident}"),
                ));
            }
        }
        Ok(this)
    }
}

static DOC: &str = "doc";
static NULLABLE: &str = "nullable";
static SKIP_GETTER: &str = "skip_getter";
static COUNT: &str = "count";
static LEN: &str = "len_expr";
static AVAILABLE: &str = "available";
static FORMAT: &str = "format";
static VERSION: &str = "version";
static SKIP_OFFSET_GETTER: &str = "skip_offset_getter";
static COMPILE: &str = "compile";
static COMPILE_TYPE: &str = "compile_type";
static READ_WITH: &str = "read_with";
static READ_OFFSET_WITH: &str = "read_offset_with";
static TO_OWNED: &str = "to_owned";

impl Parse for FieldAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = FieldAttrs::default();
        let attrs = Attribute::parse_outer(input)
            .map_err(|e| syn::Error::new(e.span(), format!("hmm: '{e}'")))?;

        for attr in attrs {
            let ident = attr.path.get_ident().ok_or_else(|| {
                syn::Error::new(attr.path.span(), "attr paths should be a single identifer")
            })?;
            if ident == DOC {
                this.docs.push(attr);
            } else if ident == NULLABLE {
                this.nullable = Some(attr.path);
            } else if ident == SKIP_GETTER {
                this.skip_getter = Some(attr.path);
            } else if ident == SKIP_OFFSET_GETTER {
                this.skip_offset_getter = Some(attr.path);
            } else if ident == VERSION {
                this.version = Some(attr.path);
            } else if ident == COUNT {
                this.count = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == COMPILE {
                this.compile = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == COMPILE_TYPE {
                this.compile_type = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == TO_OWNED {
                this.to_owned = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == AVAILABLE {
                this.available = Some(Attr {
                    name: ident.clone(),
                    attr: attr.parse_args()?,
                });
            } else if ident == LEN {
                this.len = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == READ_WITH {
                this.read_with_args = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == READ_OFFSET_WITH {
                this.read_offset_args = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == FORMAT {
                this.format = Some(Attr::new(ident.clone(), parse_attr_eq_value(attr.tokens)?))
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown field attribute {ident}"),
                ));
            }
        }
        Ok(this)
    }
}

static SKIP_PARSE: &str = "skip_parse";
static SKIP_COMPILE: &str = "skip_compile";
static SKIP_FROM_OBJ: &str = "skip_from_obj";
static VALIDATION_METHOD: &str = "validation_method";
static READ_ARGS: &str = "read_args";
static PHANTOM: &str = "phantom";

impl Parse for TableAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = TableAttrs::default();
        let attrs = Attribute::parse_outer(input)
            .map_err(|e| syn::Error::new(e.span(), format!("hmm: '{e}'")))?;

        for attr in attrs {
            let ident = attr.path.get_ident().ok_or_else(|| {
                syn::Error::new(attr.path.span(), "attr paths should be a single identifer")
            })?;
            if ident == DOC {
                this.docs.push(attr);
            } else if ident == SKIP_PARSE {
                this.skip_parse = Some(attr.path);
            } else if ident == SKIP_COMPILE {
                this.skip_compile = Some(attr.path);
            } else if ident == SKIP_FROM_OBJ {
                this.skip_from_obj = Some(attr.path);
            } else if ident == VALIDATION_METHOD {
                this.validation_method = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == READ_ARGS {
                this.read_args = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else if ident == PHANTOM {
                this.phantom = Some(Attr::new(ident.clone(), attr.parse_args()?));
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("unknown table attribute {ident}"),
                ));
            }
        }
        Ok(this)
    }
}

impl Parse for TableReadArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::<TableReadArg, Token![,]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect();
        Ok(TableReadArgs { args })
    }
}

impl Parse for TableReadArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        input.parse::<Token![:]>()?;
        let typ = input.parse()?;
        Ok(TableReadArg { ident, typ })
    }
}

impl Items {
    pub(crate) fn sanity_check(&self) -> syn::Result<()> {
        for item in &self.items {
            item.sanity_check()?;
        }
        Ok(())
    }
}

impl Item {
    fn sanity_check(&self) -> syn::Result<()> {
        match self {
            Item::Table(item) => item.sanity_check(),
            Item::Record(item) => item.sanity_check(),
            Item::Format(_) => Ok(()),
            Item::RawEnum(_) => Ok(()),
            Item::Flags(_) => Ok(()),
            Item::GenericGroup(_) => Ok(()),
        }
    }
}

impl Parse for Count {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if fork.parse::<Token![$]>().is_ok()
            && fork.parse::<syn::Ident>().is_ok()
            && fork.is_empty()
        {
            input.parse::<Token![$]>()?;
            return Ok(Self::Field(input.parse()?));
        }

        if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            Ok(Self::All)
        } else {
            input.parse().map(Self::Expr)
        }
    }
}

impl Count {
    pub(crate) fn iter_referenced_fields(&self) -> impl Iterator<Item = &syn::Ident> {
        let (one, two) = match self {
            Count::Field(ident) => (Some(ident), None),
            Count::All => (None, None),
            Count::Expr(InlineExpr {
                referenced_fields, ..
            }) => (None, Some(referenced_fields.iter())),
        };
        // a trick so we return the exact sample iterator type from both match arms
        one.into_iter().chain(two.into_iter().flatten())
    }

    pub(crate) fn count_expr(&self) -> TokenStream {
        match self {
            Count::Field(field) => quote!(#field as usize),
            Count::Expr(expr) => expr.expr.to_token_stream(),
            Count::All => panic!("count_to annotation is handled before here"),
        }
    }
}

impl Parse for InlineExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        fn parse_inline_expr(tokens: TokenStream) -> syn::Result<InlineExpr> {
            let span = tokens.span();
            let s = tokens.to_string();
            let mut idents = Vec::new();
            let find_dollar_idents = regex::Regex::new(r#"(\$) (\w+)"#).unwrap();
            for ident in find_dollar_idents.captures_iter(&s) {
                let text = ident.get(2).unwrap().as_str();
                let ident = syn::parse_str::<syn::Ident>(text).map_err(|_| {
                    syn::Error::new(tokens.span(), format!("invalid ident '{text}'"))
                })?;
                idents.push(ident);
            }
            let expr: syn::Expr = if idents.is_empty() {
                syn::parse2(tokens)
            } else {
                let new_source = find_dollar_idents.replace_all(&s, "$2");
                syn::parse_str(&new_source)
            }
            .map_err(|_| syn::Error::new(span, "failed to parse expression"))?;

            let compile_expr = (!idents.is_empty())
                .then(|| {
                    let new_source = find_dollar_idents.replace_all(&s, "&self.$2");
                    syn::parse_str::<syn::Expr>(&new_source)
                })
                .transpose()?
                .map(Box::new);

            idents.sort_unstable();
            idents.dedup();

            Ok(InlineExpr {
                expr: expr.into(),
                compile_expr,
                referenced_fields: idents,
            })
        }

        let tokens: TokenStream = input.parse()?;
        parse_inline_expr(tokens)
    }
}

impl NeededWhen {
    fn at_parsetime(&self) -> bool {
        matches!(self, NeededWhen::Parse | NeededWhen::Both)
    }

    fn at_runtime(&self) -> bool {
        matches!(self, NeededWhen::Runtime | NeededWhen::Both)
    }
}

impl ReferencedFields {
    pub(crate) fn needs_at_parsetime(&self, ident: &syn::Ident) -> bool {
        self.0
            .get(ident)
            .map(NeededWhen::at_parsetime)
            .unwrap_or(false)
    }

    pub(crate) fn needs_at_runtime(&self, ident: &syn::Ident) -> bool {
        self.0
            .get(ident)
            .map(NeededWhen::at_runtime)
            .unwrap_or(false)
    }
}

impl FromIterator<(syn::Ident, NeededWhen)> for ReferencedFields {
    fn from_iter<T: IntoIterator<Item = (syn::Ident, NeededWhen)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn parse_attr_eq_value<T: Parse>(tokens: TokenStream) -> syn::Result<T> {
    /// the tokens '= T' where 'T' is any `Parse`
    struct EqualsThing<T>(T);

    impl<T: Parse> Parse for EqualsThing<T> {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            input.parse::<Token![=]>()?;
            input.parse().map(EqualsThing)
        }
    }
    syn::parse2::<EqualsThing<T>>(tokens).map(|t| t.0)
}

fn validate_ident(ident: &syn::Ident, expected: &[&str], error: &str) -> Result<(), syn::Error> {
    if !expected.iter().any(|exp| ident == exp) {
        return Err(syn::Error::new(ident.span(), error));
    }
    Ok(())
}

fn get_optional_docs(input: ParseStream) -> Result<Vec<syn::Attribute>, syn::Error> {
    let mut result = Vec::new();
    while input.lookahead1().peek(Token![#]) {
        result.extend(Attribute::parse_outer(input)?);
    }
    for attr in &result {
        if !attr.path.is_ident("doc") {
            return Err(syn::Error::new(attr.span(), "expected doc comment"));
        }
    }
    Ok(result)
}

fn get_single_generic_type_arg(input: &syn::PathArguments) -> syn::Result<syn::Path> {
    match input {
        syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
            let arg = args.args.last().unwrap();
            if let syn::GenericArgument::Type(syn::Type::Path(path)) = arg {
                if path.qself.is_none() && path.path.segments.len() == 1 {
                    return Ok(path.path.clone());
                }
            }
        }
        _ => (),
    }
    Err(syn::Error::new(
        input.span(),
        "expected single generic type arg",
    ))
}

fn get_single_lifetime(input: &syn::PathArguments) -> syn::Result<Option<syn::Lifetime>> {
    match input {
        syn::PathArguments::None => return Ok(None),
        syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
            let arg = args.args.last().unwrap();
            if let syn::GenericArgument::Lifetime(lifetime) = arg {
                return Ok(Some(lifetime.clone()));
            }
        }
        _ => (),
    }
    Err(syn::Error::new(input.span(), "expected single lifetime"))
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    #[test]
    fn parse_inline_expr_simple() {
        let s = "div_me($hi * 5)";
        let inline = syn::parse_str::<InlineExpr>(s).unwrap();
        assert_eq!(inline.referenced_fields.len(), 1);
        assert_eq!(
            inline.expr.into_token_stream().to_string(),
            "div_me (hi * 5)"
        );
    }

    #[test]
    fn parse_inline_expr_dedup() {
        let s = "div_me($hi * 5 + $hi)";
        let inline = syn::parse_str::<InlineExpr>(s).unwrap();
        assert_eq!(inline.referenced_fields.len(), 1);
        assert_eq!(
            inline.expr.into_token_stream().to_string(),
            "div_me (hi * 5 + hi)"
        );
    }
}
