#![allow(non_camel_case_types)]

use serde::*;

type Boolean = bool;

#[derive(Clone, Debug)]
pub enum __DirectiveLocation {
    QUERY,
    MUTATION,
    SUBSCRIPTION,
    FIELD,
    FRAGMENT_DEFINITION,
    FRAGMENT_SPREAD,
    INLINE_FRAGMENT,
    SCHEMA,
    SCALAR,
    OBJECT,
    FIELD_DEFINITION,
    ARGUMENT_DEFINITION,
    INTERFACE,
    UNION,
    ENUM,
    ENUM_VALUE,
    INPUT_OBJECT,
    INPUT_FIELD_DEFINITION,
    Other(String),
}

impl ::serde::Serialize for __DirectiveLocation {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            __DirectiveLocation::QUERY => "QUERY",
            __DirectiveLocation::MUTATION => "MUTATION",
            __DirectiveLocation::SUBSCRIPTION => "SUBSCRIPTION",
            __DirectiveLocation::FIELD => "FIELD",
            __DirectiveLocation::FRAGMENT_DEFINITION => "FRAGMENT_DEFINITION",
            __DirectiveLocation::FRAGMENT_SPREAD => "FRAGMENT_SPREAD",
            __DirectiveLocation::INLINE_FRAGMENT => "INLINE_FRAGMENT",
            __DirectiveLocation::SCHEMA => "SCHEMA",
            __DirectiveLocation::SCALAR => "SCALAR",
            __DirectiveLocation::OBJECT => "OBJECT",
            __DirectiveLocation::FIELD_DEFINITION => "FIELD_DEFINITION",
            __DirectiveLocation::ARGUMENT_DEFINITION => "ARGUMENT_DEFINITION",
            __DirectiveLocation::INTERFACE => "INTERFACE",
            __DirectiveLocation::UNION => "UNION",
            __DirectiveLocation::ENUM => "ENUM",
            __DirectiveLocation::ENUM_VALUE => "ENUM_VALUE",
            __DirectiveLocation::INPUT_OBJECT => "INPUT_OBJECT",
            __DirectiveLocation::INPUT_FIELD_DEFINITION => "INPUT_FIELD_DEFINITION",
            __DirectiveLocation::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for __DirectiveLocation {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "QUERY" => Ok(__DirectiveLocation::QUERY),
            "MUTATION" => Ok(__DirectiveLocation::MUTATION),
            "SUBSCRIPTION" => Ok(__DirectiveLocation::SUBSCRIPTION),
            "FIELD" => Ok(__DirectiveLocation::FIELD),
            "FRAGMENT_DEFINITION" => Ok(__DirectiveLocation::FRAGMENT_DEFINITION),
            "FRAGMENT_SPREAD" => Ok(__DirectiveLocation::FRAGMENT_SPREAD),
            "INLINE_FRAGMENT" => Ok(__DirectiveLocation::INLINE_FRAGMENT),
            "SCHEMA" => Ok(__DirectiveLocation::SCHEMA),
            "SCALAR" => Ok(__DirectiveLocation::SCALAR),
            "OBJECT" => Ok(__DirectiveLocation::OBJECT),
            "FIELD_DEFINITION" => Ok(__DirectiveLocation::FIELD_DEFINITION),
            "ARGUMENT_DEFINITION" => Ok(__DirectiveLocation::ARGUMENT_DEFINITION),
            "INTERFACE" => Ok(__DirectiveLocation::INTERFACE),
            "UNION" => Ok(__DirectiveLocation::UNION),
            "ENUM" => Ok(__DirectiveLocation::ENUM),
            "ENUM_VALUE" => Ok(__DirectiveLocation::ENUM_VALUE),
            "INPUT_OBJECT" => Ok(__DirectiveLocation::INPUT_OBJECT),
            "INPUT_FIELD_DEFINITION" => Ok(__DirectiveLocation::INPUT_FIELD_DEFINITION),
            _ => Ok(__DirectiveLocation::Other(s.to_string())),
        }
    }
}

#[derive(Clone, Debug)]
pub enum __TypeKind {
    SCALAR,
    OBJECT,
    INTERFACE,
    UNION,
    ENUM,
    INPUT_OBJECT,
    LIST,
    NON_NULL,
    Other(String),
}

impl ::serde::Serialize for __TypeKind {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            __TypeKind::SCALAR => "SCALAR",
            __TypeKind::OBJECT => "OBJECT",
            __TypeKind::INTERFACE => "INTERFACE",
            __TypeKind::UNION => "UNION",
            __TypeKind::ENUM => "ENUM",
            __TypeKind::INPUT_OBJECT => "INPUT_OBJECT",
            __TypeKind::LIST => "LIST",
            __TypeKind::NON_NULL => "NON_NULL",
            __TypeKind::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for __TypeKind {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "SCALAR" => Ok(__TypeKind::SCALAR),
            "OBJECT" => Ok(__TypeKind::OBJECT),
            "INTERFACE" => Ok(__TypeKind::INTERFACE),
            "UNION" => Ok(__TypeKind::UNION),
            "ENUM" => Ok(__TypeKind::ENUM),
            "INPUT_OBJECT" => Ok(__TypeKind::INPUT_OBJECT),
            "LIST" => Ok(__TypeKind::LIST),
            "NON_NULL" => Ok(__TypeKind::NON_NULL),
            _ => Ok(__TypeKind::Other(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<Option<FullTypeFields>>>,
    pub input_fields: Option<Vec<Option<FullTypeInputFields>>>,
    pub interfaces: Option<Vec<Option<FullTypeInterfaces>>>,
    pub enum_values: Option<Vec<Option<FullTypeEnumValues>>>,
    pub possible_types: Option<Vec<Option<FullTypePossibleTypes>>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFieldsArgs {
    #[serde(flatten)]
    input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFieldsType {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub args: Option<Vec<Option<FullTypeFieldsArgs>>>,
    #[serde(rename = "type")]
    pub type_: Option<FullTypeFieldsType>,
    pub is_deprecated: Option<Boolean>,
    pub deprecation_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeInputFields {
    #[serde(flatten)]
    pub input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeInterfaces {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeEnumValues {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<Boolean>,
    pub deprecation_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypePossibleTypes {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<InputValueType>,
    pub default_value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValueType {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<Box<TypeRef>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfTypeOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfTypeOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfTypeOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRefOfType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<TypeRefOfTypeOfType>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaQueryType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaMutationType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaSubscriptionType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaTypes {
    #[serde(flatten)]
    pub full_type: FullType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaDirectivesArgs {
    #[serde(flatten)]
    input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchemaDirectives {
    pub name: Option<String>,
    pub description: Option<String>,
    pub locations: Option<Vec<Option<__DirectiveLocation>>>,
    pub args: Option<Vec<Option<RustIntrospectionQuerySchemaDirectivesArgs>>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustIntrospectionQuerySchema {
    pub query_type: Option<RustIntrospectionQuerySchemaQueryType>,
    pub mutation_type: Option<RustIntrospectionQuerySchemaMutationType>,
    pub subscription_type: Option<RustIntrospectionQuerySchemaSubscriptionType>,
    pub types: Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
    directives: Option<Vec<Option<RustIntrospectionQuerySchemaDirectives>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Schema {
    #[serde(rename = "__schema")]
    pub schema: Option<RustIntrospectionQuerySchema>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct FullResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum IntrospectionResponse {
    FullResponse(FullResponse<Schema>),
    Schema(Schema),
}

impl IntrospectionResponse {
    pub(crate) fn as_schema(&self) -> &Schema {
        match self {
            IntrospectionResponse::FullResponse(full_response) => &full_response.data,
            IntrospectionResponse::Schema(schema) => &schema,
        }
    }
}
