use std::sync::Arc;

use crate::Result;
use crate::config::{AppConfig, RouteRule};
use crate::discord::DiscordClient;
use crate::events::{IncomingEvent, MessageFormat, render_template};

pub struct Router {
    config: Arc<AppConfig>,
}

impl Router {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }

    pub async fn dispatch(&self, event: &IncomingEvent, discord: &DiscordClient) -> Result<()> {
        let (channel, _format, content) = self.preview(event)?;
        discord.send_message(&channel, &content).await
    }

    pub fn preview(&self, event: &IncomingEvent) -> Result<(String, MessageFormat, String)> {
        let route = self.route_for(event);
        let channel = event
            .channel
            .clone()
            .or_else(|| route.and_then(|route| route.channel.clone()))
            .or_else(|| self.config.defaults.channel.clone())
            .ok_or_else(|| format!("no channel configured for event {}", event.canonical_kind()))?;
        let format = event
            .format
            .clone()
            .or_else(|| route.and_then(|route| route.format.clone()))
            .unwrap_or_else(|| self.config.defaults.format.clone());
        let content = if let Some(template) = event
            .template
            .as_deref()
            .or_else(|| route.and_then(|route| route.template.as_deref()))
        {
            render_template(template, &event.template_context())
        } else {
            event.render_default(&format)?
        };
        let content = match route.and_then(|route| route.mention.as_deref()) {
            Some(mention) if !mention.trim().is_empty() => {
                format!("{} {}", mention.trim(), content)
            }
            _ => content,
        };
        Ok((channel, format, content))
    }

    fn route_for<'a>(&'a self, event: &IncomingEvent) -> Option<&'a RouteRule> {
        let context = event.template_context();
        self.config.routes.iter().find(|route| {
            glob_match(&route.event, event.canonical_kind())
                && route.filter.iter().all(|(key, expected)| {
                    context
                        .get(key)
                        .map(|actual| glob_match(expected, actual))
                        .unwrap_or(false)
                })
        })
    }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    if pattern == value {
        return true;
    }
    if !pattern.contains('*') {
        return false;
    }

    let mut remainder = value;
    let parts: Vec<&str> = pattern.split('*').collect();
    let starts_with_wildcard = pattern.starts_with('*');
    let ends_with_wildcard = pattern.ends_with('*');

    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        if index == 0 && !starts_with_wildcard {
            if !remainder.starts_with(part) {
                return false;
            }
            remainder = &remainder[part.len()..];
            continue;
        }

        if index == parts.len() - 1 && !ends_with_wildcard {
            return remainder.ends_with(part);
        }

        if let Some(position) = remainder.find(part) {
            remainder = &remainder[(position + part.len())..];
        } else {
            return false;
        }
    }

    ends_with_wildcard || remainder.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DefaultsConfig, RouteRule};

    #[test]
    fn preview_uses_filtered_route_overrides() {
        let config = AppConfig {
            defaults: DefaultsConfig {
                channel: Some("default".into()),
                format: MessageFormat::Compact,
            },
            routes: vec![RouteRule {
                event: "tmux.*".into(),
                filter: [("session".to_string(), "issue-*".to_string())]
                    .into_iter()
                    .collect(),
                channel: Some("route".into()),
                mention: None,
                format: Some(MessageFormat::Alert),
                template: None,
            }],
            ..AppConfig::default()
        };
        let router = Router::new(Arc::new(config));
        let event =
            IncomingEvent::tmux_keyword("issue-1440".into(), "error".into(), "boom".into(), None);

        let (channel, format, content) = router.preview(&event).unwrap();
        assert_eq!(channel, "route");
        assert_eq!(format, MessageFormat::Alert);
        assert_eq!(
            content,
            "🚨 tmux session issue-1440 hit keyword 'error': boom"
        );
    }

    #[test]
    fn route_level_mention_is_prepended_for_custom() {
        let config = AppConfig {
            defaults: DefaultsConfig {
                channel: Some("default".into()),
                format: MessageFormat::Compact,
            },
            routes: vec![RouteRule {
                event: "custom".into(),
                filter: Default::default(),
                channel: Some("route".into()),
                mention: Some("<@1465264645320474637>".into()),
                format: Some(MessageFormat::Compact),
                template: None,
            }],
            ..AppConfig::default()
        };
        let router = Router::new(Arc::new(config));
        let event = IncomingEvent::custom(None, "wake up".into());
        let (channel, _, content) = router.preview(&event).unwrap();
        assert_eq!(channel, "route");
        assert_eq!(content, "<@1465264645320474637> wake up");
    }

    #[test]
    fn route_level_mention_is_prepended_for_github_and_tmux() {
        let config = AppConfig {
            defaults: DefaultsConfig {
                channel: Some("default".into()),
                format: MessageFormat::Compact,
            },
            routes: vec![
                RouteRule {
                    event: "github.*".into(),
                    filter: [("repo".to_string(), "clawhip".to_string())]
                        .into_iter()
                        .collect(),
                    channel: Some("gh-route".into()),
                    mention: Some("<@botid>".into()),
                    format: Some(MessageFormat::Alert),
                    template: None,
                },
                RouteRule {
                    event: "tmux.*".into(),
                    filter: [("session".to_string(), "issue-*".to_string())]
                        .into_iter()
                        .collect(),
                    channel: Some("tmux-route".into()),
                    mention: Some("<@botid>".into()),
                    format: Some(MessageFormat::Alert),
                    template: None,
                },
            ],
            ..AppConfig::default()
        };
        let router = Router::new(Arc::new(config));

        let github_event =
            IncomingEvent::github_issue_opened("clawhip".into(), 5, "boom".into(), None);
        let (_, _, github_content) = router.preview(&github_event).unwrap();
        assert!(github_content.starts_with("<@botid> "));
        assert!(github_content.contains("boom"));

        let tmux_event =
            IncomingEvent::tmux_keyword("issue-1440".into(), "error".into(), "failed".into(), None);
        let (_, _, tmux_content) = router.preview(&tmux_event).unwrap();
        assert!(tmux_content.starts_with("<@botid> "));
        assert!(tmux_content.contains("failed"));
    }

    #[test]
    fn filter_can_route_same_event_type_by_repo() {
        let config = AppConfig {
            defaults: DefaultsConfig {
                channel: Some("default".into()),
                format: MessageFormat::Compact,
            },
            routes: vec![
                RouteRule {
                    event: "github.*".into(),
                    filter: [("repo".to_string(), "oh-my-claudecode".to_string())]
                        .into_iter()
                        .collect(),
                    channel: Some("repo-a".into()),
                    mention: None,
                    format: None,
                    template: None,
                },
                RouteRule {
                    event: "github.*".into(),
                    filter: [("repo".to_string(), "clawhip".to_string())]
                        .into_iter()
                        .collect(),
                    channel: Some("repo-b".into()),
                    mention: None,
                    format: None,
                    template: None,
                },
            ],
            ..AppConfig::default()
        };
        let router = Router::new(Arc::new(config));
        let event = IncomingEvent::github_issue_opened("clawhip".into(), 7, "bug".into(), None);
        let (channel, _, _) = router.preview(&event).unwrap();
        assert_eq!(channel, "repo-b");
    }
}
