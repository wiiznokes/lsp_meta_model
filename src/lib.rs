//! This is a rust implementation of the [meta model](https://microsoft.github.io/language-server-protocol/specifications/specification-current/#metaModel) of the [language-server-protocol](https://microsoft.github.io/language-server-protocol/overviews/lsp/overview/).
//!
//! This is useful to parse the json meta file and generate rust code for a lsp implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum BaseTypes {
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "DocumentUri")]
    DocumentUri,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "uinteger")]
    Uinteger,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "RegExp")]
    RegExp,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "null")]
    Null,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum TypeKind {
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "tuple")]
    Tuple,
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "stringLiteral")]
    StringLiteral,
    #[serde(rename = "integerLiteral")]
    IntegerLiteral,
    #[serde(rename = "booleanLiteral")]
    BooleanLiteral,
}

/// Indicates in which direction a message is sent in the protocol.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub enum MessageDirection {
    #[serde(rename = "clientToServer")]
    ClientToServer,
    #[serde(rename = "serverToClient")]
    ServerToClient,
    #[serde(rename = "both")]
    Both,
}

/// Represents a base type like `string` or `DocumentUri`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BaseType {
    pub kind: TypeKind,
    pub name: BaseTypes,
}

/// Represents a reference to another type (e.g. `TextDocument`).
/// This is either a `Structure`, a `Enumeration` or a `TypeAlias`
/// in the same meta model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReferenceType {
    pub kind: TypeKind,
    pub name: String,
}

/// Represents an array type (e.g. `TextDocument[]`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArrayType {
    pub kind: TypeKind,
    pub element: Box<Type>,
}

/// Represents a type that can be used as a key in a
/// map type. If a reference type is used then the
/// type must either resolve to a `string` or `integer`
/// type. (e.g. `type ChangeAnnotationIdentifier === string`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum MapKeyType {
    Inner(TypeKind, BaseTypes),
    ReferenceType(ReferenceType),
}

/// Represents a JSON object map
/// (e.g. `interface Map<K extends string | integer, V> { [key: K] => V; }`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MapType {
    pub kind: TypeKind,
    pub key: MapKeyType,
    pub value: Box<Type>,
}

/// Represents an `and`type
/// (e.g. TextDocumentParams & WorkDoneProgressParams`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AndType {
    pub kind: TypeKind,
    pub items: Vec<Type>,
}

/// Represents an `or` type
/// (e.g. `Location | LocationLink`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OrType {
    pub kind: TypeKind,
    pub items: Vec<Type>,
}

/// Represents a `tuple` type
/// (e.g. `[integer, integer]`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TupleType {
    pub kind: TypeKind,
    pub items: Vec<Type>,
}

/// Represents a literal structure
/// (e.g. `property: { start: uinteger; end: uinteger; }`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StructureLiteralType {
    kind: TypeKind,
    value: StructureLiteral,
}

/// Represents a string literal type
/// (e.g. `kind: 'rename'`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StringLiteralType {
    kind: TypeKind,
    value: String,
}

/// Represents an integer literal type
/// (e.g. `kind: 1`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IntegerLiteralType {
    kind: TypeKind,
    value: i32,
}

/// Represents a boolean literal type
/// (e.g. `kind: true`).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BooleanLiteralType {
    kind: TypeKind,
    value: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Type {
    BaseType(BaseTypes),
    ReferenceType(ReferenceType),
    ArrayType(ArrayType),
    MapType(MapType),
    AndType(AndType),
    OrType(OrType),
    TupleType(TupleType),
    StructureLiteralType(StructureLiteralType),
    StringLiteralType(StringLiteralType),
    IntegerLiteralType(IntegerLiteralType),
    BooleanLiteralType(BooleanLiteralType),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Params {
    Single(Type),
    Multiple(Vec<Type>),
}

/// Represents a LSP request
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Request {
    /// The request's method name.
    pub method: String,

    /// The parameter type(s) if any.
    pub params: Option<Params>,

    /// The result type.
    pub result: Type,

    /// Optional partial result type if the request
    /// supports partial result reporting.
    #[serde(rename = "partialResult")]
    pub partial_result: Option<Type>,

    /// An optional error data type.
    #[serde(rename = "errorData")]
    pub error_data: Option<Type>,

    /// Optional a dynamic registration method if it
    /// different from the request's method.
    #[serde(rename = "registrationMethod")]
    pub registration_method: Option<String>,

    /// Optional registration options if the request
    /// supports dynamic registration.
    #[serde(rename = "registrationOptions")]
    pub registration_options: Option<Type>,

    /// The direction in which this request is sent
    /// in the protocol.
    #[serde(rename = "messageDirection")]
    pub message_direction: MessageDirection,

    /// An optional documentation;
    pub documentation: Option<String>,

    /// Since when (release number) this request is
    /// available. Is undefined if not known.
    pub since: Option<String>,

    /// Whether this is a proposed feature. If omitted
    /// the feature is final.
    pub proposed: Option<bool>,

    /// Whether the request is deprecated or not. If deprecated
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Represents a LSP notification
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    /// The request's method name.
    pub method: String,

    /// The parameter type(s) if any.
    pub params: Option<Params>,

    /// Optional a dynamic registration method if it
    /// different from the request's method.
    #[serde(rename = "registrationMethod")]
    pub registration_method: Option<String>,

    /// Optional registration options if the request
    /// supports dynamic registration.
    #[serde(rename = "registrationOptions")]
    pub registration_options: Option<Type>,

    /// The direction in which this request is sent
    /// in the protocol.
    #[serde(rename = "messageDirection")]
    pub message_direction: MessageDirection,

    /// An optional documentation;
    pub documentation: Option<String>,

    /// Since when (release number) this request is
    /// available. Is undefined if not known.
    pub since: Option<String>,

    /// Whether this is a proposed feature. If omitted
    /// the feature is final.
    pub proposed: Option<bool>,

    /// Whether the request is deprecated or not. If deprecated
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Represents an object property.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Property {
    /// The property name.
    pub name: String,

    /// The type of the property.
    pub r#type: Type,

    /// Whether the property is optional.
    /// If omitted, the property is mandatory.
    pub optional: Option<bool>,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this property is available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed property. If omitted, the structure is final.
    pub proposed: Option<bool>,

    /// Whether the property is deprecated or not. If deprecated,
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Defines the structure of an object literal.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Structure {
    /// The name of the structure.
    pub name: String,

    /// Structures extended from. This structures form
    /// a polymorphic type hierarchy.
    pub extends: Option<Vec<Type>>,

    /// Structures to mix in. The properties of these
    /// structures are `copied` into this structure.
    /// Mixins don't form a polymorphic type hierarchy in
    /// LSP.
    pub mixins: Option<Vec<Type>>,

    /// The properties.
    pub properties: Vec<Property>,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this structure is
    /// available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed structure. If omitted,
    /// the structure is final.
    pub proposed: Option<bool>,

    /// Whether the structure is deprecated or not. If deprecated,
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Defines an unnamed structure of an object literal.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StructureLiteral {
    /// The properties.
    pub properties: Vec<Property>,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this structure is
    /// available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed structure. If omitted,
    /// the structure is final.
    pub proposed: Option<bool>,

    /// Whether the literal is deprecated or not. If deprecated,
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Defines a type alias.
/// (e.g. `type Definition = Location | LocationLink`)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TypeAlias {
    /// The name of the type alias.
    pub name: String,

    /// The aliased type.
    pub r#type: Type,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this structure is available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed type alias. If omitted, the type alias is final.
    pub proposed: Option<bool>,

    /// Whether the type alias is deprecated or not. If deprecated, the property contains the deprecation message.
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub enum Value {
    String(String),
    Number(i32),
}

/// Defines an enumeration entry.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EnumerationEntry {
    /// The name of the enum item.
    pub name: String,

    /// The value.
    pub value: Value,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this enumeration entry is available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed enumeration entry. If omitted, the enumeration entry is final.
    pub proposed: Option<bool>,

    /// Whether the enum entry is deprecated or not. If deprecated, the property contains the deprecation message.
    pub deprecated: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EnumerationType {
    kind: TypeKind,
    name: BaseTypes,
}

/// Defines an enumeration.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Enumeration {
    /// The name of the enumeration.
    pub name: String,

    /// The type of the elements.
    pub r#type: EnumerationType,

    /// The enum values.
    pub values: Vec<EnumerationEntry>,

    /// Whether the enumeration supports custom values (e.g. values which are not
    /// part of the set defined in `values`). If omitted no custom values are
    /// supported.
    #[serde(rename = "supportsCustomValues")]
    pub supports_custom_values: Option<bool>,

    /// An optional documentation.
    pub documentation: Option<String>,

    /// Since when (release number) this enumeration is
    /// available. Is None if not known.
    pub since: Option<String>,

    /// Whether this is a proposed enumeration. If omitted,
    /// the enumeration is final.
    pub proposed: Option<bool>,

    /// Whether the enumeration is deprecated or not. If deprecated
    /// the property contains the deprecation message.
    pub deprecated: Option<String>,
}

/// Metadata structure.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MetaData {
    /// The protocol version.
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MetaModel {
    /// Additional metadata.
    #[serde(rename = "metaData")]
    pub meta_data: MetaData,

    /// The requests.
    pub requests: Vec<Request>,

    /// The notifications.
    pub notifications: Vec<Notification>,

    /// The structures.
    pub structures: Vec<Structure>,

    /// The enumerations.
    pub enumerations: Vec<Enumeration>,

    /// The type aliases.
    #[serde(rename = "typeAliases")]
    pub type_aliases: Vec<TypeAlias>,
}
