#[cfg(test)]
mod tests {

    #[cfg(all(test, feature = "serde"))]
    use serde::{Deserialize, Serialize};
    #[cfg(all(test, any(feature = "jsonschema")))]
    use serde_json::Value;
    #[cfg(all(
        test,
        any(feature = "typescript", feature = "jsonschema", feature = "zod", feature = "serde")
    ))]
    use tixschema::model_schema;

    #[cfg(all(
        test,
        any(
            feature = "typescript",
            feature = "jsonschema",
            feature = "zod",
            feature = "serde"
        )
    ))]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    #[model_schema()]
    struct BasicUser {
        id: String,
        name: String,
        age: u32,
        height: f32,
        is_active: bool,
    }

    #[test]
    #[cfg(feature = "jsonschema")]
    fn test_basic_struct_json_schema() {
        let schema = BasicUser::json_schema();

        // Check that it's an object
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["additionalProperties"], false);

        // Check properties exist
        let properties = schema["properties"].as_object().unwrap();
        assert!(properties.contains_key("id"));
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("age"));
        assert!(properties.contains_key("height"));
        assert!(properties.contains_key("is_active"));

        // Check property types
        assert_eq!(properties["id"]["type"], "string");
        assert_eq!(properties["name"]["type"], "string");
        assert_eq!(properties["age"]["type"], "integer");
        assert_eq!(properties["height"]["type"], "number");
        assert_eq!(properties["is_active"]["type"], "boolean");

        // Check required fields
        let required = schema["required"].as_array().unwrap();
        assert_eq!(required.len(), 5);
        assert!(required.contains(&Value::String("id".to_string())));
        assert!(required.contains(&Value::String("name".to_string())));
        assert!(required.contains(&Value::String("age".to_string())));
        assert!(required.contains(&Value::String("height".to_string())));
        assert!(required.contains(&Value::String("is_active".to_string())));
    }

    #[test]
    #[cfg(feature = "typescript")]
    fn test_basic_struct_ts_definition() {
        let ts_definition = BasicUser::ts_definition();

        // Check that it contains TypeScript type definition
        assert!(ts_definition.contains("export type BasicUser = {"));
        assert!(ts_definition.contains("id: string;"));
        assert!(ts_definition.contains("name: string;"));
        assert!(ts_definition.contains("age: number;"));
        assert!(ts_definition.contains("height: number;"));
        assert!(ts_definition.contains("is_active: boolean;"));

        // Should NOT contain Zod schema (now separated)
        assert!(!ts_definition.contains("export const BasicUser$Schema"));
        assert!(!ts_definition.contains("z.strictObject"));
        assert!(!ts_definition.contains("z.string()"));
        assert!(!ts_definition.contains("z.number()"));
        assert!(!ts_definition.contains("z.boolean()"));
    }

    #[test]
    #[cfg(feature = "zod")]
    fn test_basic_struct_zod_schema() {
        let zod_schema = BasicUser::zod_schema();

        // Check that it contains Zod schema
        assert!(zod_schema.contains("export const BasicUser$Schema"));
        assert!(zod_schema.contains("z.strictObject({"));
        assert!(zod_schema.contains("id: z.string()"));
        assert!(zod_schema.contains("name: z.string()"));
        assert!(zod_schema.contains("age: z.number().int()"));
        assert!(zod_schema.contains("height: z.number()"));
        assert!(zod_schema.contains("is_active: z.boolean()"));

        // Should NOT contain TypeScript type definition
        assert!(!zod_schema.contains("export type BasicUser"));
        assert!(!zod_schema.contains("id: string;"));
        assert!(!zod_schema.contains("age: number;"));
    }

    #[cfg(all(
        test,
        any(feature = "typescript", feature = "jsonschema", feature = "zod")
    ))]
    // Test struct with optional fields
    #[model_schema()]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    struct UserWithOptionals {
        id: String,
        name: String,
        email: Option<String>,
        age: Option<u32>,
        nickname: Option<String>,
    }

    #[test]
    #[cfg(feature = "jsonschema")]
    fn test_optional_fields_json_schema() {
        let schema = UserWithOptionals::json_schema();

        // Check properties exist
        let properties = schema["properties"].as_object().unwrap();
        assert!(properties.contains_key("id"));
        assert!(properties.contains_key("name"));
        assert!(properties.contains_key("email"));
        assert!(properties.contains_key("age"));
        assert!(properties.contains_key("nickname"));

        // Check required fields (only non-optional ones)
        let required = schema["required"].as_array().unwrap();
        assert_eq!(required.len(), 2);
        assert!(required.contains(&Value::String("id".to_string())));
        assert!(required.contains(&Value::String("name".to_string())));
        assert!(!required.contains(&Value::String("email".to_string())));
        assert!(!required.contains(&Value::String("age".to_string())));
        assert!(!required.contains(&Value::String("nickname".to_string())));
    }

    #[test]
    #[cfg(feature = "typescript")]
    fn test_optional_fields_ts_definition() {
        let ts_definition = UserWithOptionals::ts_definition();

        // Check that optional fields are properly typed
        assert!(ts_definition.contains("id: string;"));
        assert!(ts_definition.contains("name: string;"));
        assert!(ts_definition.contains("email: string | undefined;"));
        assert!(ts_definition.contains("age: number | undefined;"));
        assert!(ts_definition.contains("nickname: string | undefined;"));

        // Should NOT contain Zod schema (now separated)
        assert!(!ts_definition.contains("z.string().or(z.undefined())"));
        assert!(!ts_definition.contains("z.number().int().or(z.undefined())"));
    }

    #[test]
    #[cfg(feature = "zod")]
    fn test_optional_fields_zod_schema() {
        let zod_schema = UserWithOptionals::zod_schema();

        // Check Zod schema has optional fields
        assert!(zod_schema.contains("email: z.string().or(z.undefined())"));
        assert!(zod_schema.contains("age: z.number().int().or(z.undefined())"));
        assert!(zod_schema.contains("nickname: z.string().or(z.undefined())"));

        // Should NOT contain TypeScript type definition
        assert!(!zod_schema.contains("email: string | undefined;"));
        assert!(!zod_schema.contains("age: number | undefined;"));
    }

    #[cfg(all(
        test,
        any(feature = "typescript", feature = "jsonschema", feature = "zod")
    ))]
    // Test empty struct
    #[model_schema()]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, PartialEq)]
    struct EmptyStruct {}

    #[test]
    #[cfg(feature = "jsonschema")]
    fn test_empty_struct_json_schema() {
        let schema = EmptyStruct::json_schema();

        assert_eq!(schema["type"], "object");
        assert_eq!(schema["additionalProperties"], false);

        let properties = schema["properties"].as_object().unwrap();
        assert!(properties.is_empty());

        let required = schema["required"].as_array().unwrap();
        assert!(required.is_empty());
    }

    #[test]
    #[cfg(feature = "typescript")]
    fn test_empty_struct_ts_definition() {
        let ts_definition = EmptyStruct::ts_definition();

        // Should generate Record<string, never> for empty structs
        assert!(ts_definition.contains("export type EmptyStruct = Record<string, never>;"));
    }
}
