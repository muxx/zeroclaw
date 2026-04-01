use super::traits::{Tool, ToolResult};
use async_trait::async_trait;
use chrono::{Local, SecondsFormat, Utc};
use serde_json::json;

pub struct CurrentDateTimeTool;

impl CurrentDateTimeTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CurrentDateTimeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for CurrentDateTimeTool {
    fn name(&self) -> &str {
        "current_datetime"
    }

    fn description(&self) -> &str {
        "Get the current runtime date and time. Returns local and UTC timestamps, timezone, UTC offset, and Unix timestamp. Use this whenever you need the current date/time or to resolve relative dates like today, tomorrow, or last 7 days."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        })
    }

    async fn execute(&self, _args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let local_now = Local::now();
        let utc_now = Utc::now();

        Ok(ToolResult {
            success: true,
            output: json!({
                "local_datetime": local_now.to_rfc3339_opts(SecondsFormat::Secs, true),
                "local_date": local_now.format("%Y-%m-%d").to_string(),
                "local_time": local_now.format("%H:%M:%S").to_string(),
                "timezone": local_now.format("%Z").to_string(),
                "utc_offset": local_now.format("%:z").to_string(),
                "utc_datetime": utc_now.to_rfc3339_opts(SecondsFormat::Secs, true),
                "unix_timestamp": utc_now.timestamp(),
            })
            .to_string(),
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_current_datetime() {
        let tool = CurrentDateTimeTool::new();
        assert_eq!(tool.name(), "current_datetime");
    }

    #[test]
    fn schema_is_empty_object() {
        let tool = CurrentDateTimeTool::new();
        let schema = tool.parameters_schema();
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].as_object().unwrap().is_empty());
        assert_eq!(schema["additionalProperties"], false);
    }

    #[tokio::test]
    async fn execute_returns_expected_fields() {
        let tool = CurrentDateTimeTool::new();
        let result = tool.execute(json!({})).await.unwrap();
        assert!(result.success);

        let payload: serde_json::Value = serde_json::from_str(&result.output).unwrap();
        assert!(payload["local_datetime"].is_string());
        assert!(payload["local_date"].is_string());
        assert!(payload["local_time"].is_string());
        assert!(payload["timezone"].is_string());
        assert!(payload["utc_offset"].is_string());
        assert!(payload["utc_datetime"].is_string());
        assert!(payload["unix_timestamp"].is_i64());
    }
}
