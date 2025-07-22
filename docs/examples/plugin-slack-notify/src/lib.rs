use taskrush_plugin_api::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct SlackMessage {
    text: String,
    channel: Option<String>,
    username: Option<String>,
    icon_emoji: Option<String>,
}

pub struct SlackNotifyPlugin {
    webhook_url: String,
    default_channel: Option<String>,
    username: String,
    icon_emoji: String,
    enabled_events: Vec<TaskEvent>,
}

impl Default for SlackNotifyPlugin {
    fn default() -> Self {
        Self {
            webhook_url: String::new(),
            default_channel: None,
            username: "TaskRush".to_string(),
            icon_emoji: ":rocket:".to_string(),
            enabled_events: vec![TaskEvent::Success, TaskEvent::Failure],
        }
    }
}

#[derive(Debug, Deserialize)]
enum TaskEvent {
    Start,
    Success,
    Failure,
    Timeout,
}

impl TaskRushPlugin for SlackNotifyPlugin {
    fn name(&self) -> &str {
        "slack-notify"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn description(&self) -> &str {
        "Send Slack notifications for task events"
    }

    fn initialize(&mut self, config: &PluginConfig) -> PluginResult<()> {
        // Required configuration
        self.webhook_url = config.get_string("webhook_url")
            .or_else(|| std::env::var("SLACK_WEBHOOK").ok())
            .ok_or_else(|| PluginError::ConfigMissing("webhook_url or SLACK_WEBHOOK".to_string()))?;

        // Optional configuration with defaults
        self.default_channel = config.get_string("channel");
        self.username = config.get_string("username").unwrap_or_else(|| "TaskRush".to_string());
        self.icon_emoji = config.get_string("icon_emoji").unwrap_or_else(|| ":rocket:".to_string());

        // Parse enabled events
        if let Some(events) = config.get_array::<String>("events") {
            self.enabled_events = events.into_iter()
                .filter_map(|e| match e.as_str() {
                    "start" => Some(TaskEvent::Start),
                    "success" => Some(TaskEvent::Success),
                    "failure" => Some(TaskEvent::Failure),
                    "timeout" => Some(TaskEvent::Timeout),
                    _ => None,
                })
                .collect();
        }

        Ok(())
    }

    fn before_task(&mut self, context: &TaskContext) -> PluginResult<()> {
        if self.enabled_events.contains(&TaskEvent::Start) {
            let message = format!("🏃 Starting task: `{}`", context.task_name);
            self.send_notification(&message, context)?;
        }
        Ok(())
    }

    fn after_task(&mut self, context: &TaskContext, result: &TaskResult) -> PluginResult<()> {
        let (event, message) = match result.status {
            TaskStatus::Success => {
                if !self.enabled_events.contains(&TaskEvent::Success) {
                    return Ok(());
                }
                let duration = result.duration.as_secs_f32();
                (TaskEvent::Success, format!("✅ Task `{}` completed successfully in {:.1}s", context.task_name, duration))
            }
            TaskStatus::Failure => {
                if !self.enabled_events.contains(&TaskEvent::Failure) {
                    return Ok(());
                }
                let error_msg = result.error.as_ref()
                    .map(|e| format!("\nError: {}", e))
                    .unwrap_or_default();
                (TaskEvent::Failure, format!("❌ Task `{}` failed{}", context.task_name, error_msg))
            }
            TaskStatus::Timeout => {
                if !self.enabled_events.contains(&TaskEvent::Timeout) {
                    return Ok(());
                }
                (TaskEvent::Timeout, format!("⏰ Task `{}` timed out", context.task_name))
            }
        };

        self.send_notification(&message, context)?;
        Ok(())
    }

    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                name: "slack-test".to_string(),
                description: "Send a test message to Slack".to_string(),
                handler: Box::new(|args| {
                    let message = if args.is_empty() {
                        "🧪 Test message from TaskRush"
                    } else {
                        &args.join(" ")
                    };
                    
                    // This would need access to the plugin instance
                    // Implementation details TBD
                    println!("Would send: {}", message);
                    Ok(())
                }),
            }
        ]
    }
}

impl SlackNotifyPlugin {
    async fn send_notification(&self, message: &str, context: &TaskContext) -> PluginResult<()> {
        let channel = context.plugin_config
            .get("slack-notify")
            .and_then(|config| config.get_string("channel"))
            .or_else(|| self.default_channel.clone());

        let slack_message = SlackMessage {
            text: message.to_string(),
            channel,
            username: Some(self.username.clone()),
            icon_emoji: Some(self.icon_emoji.clone()),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook_url)
            .json(&slack_message)
            .send()
            .await
            .map_err(|e| PluginError::Network(format!("Failed to send Slack message: {}", e)))?;

        if !response.status().is_success() {
            return Err(PluginError::Network(format!(
                "Slack API returned status: {}", 
                response.status()
            )));
        }

        Ok(())
    }
}

// Export the plugin
taskrush_plugin_api::export_plugin!(SlackNotifyPlugin);
