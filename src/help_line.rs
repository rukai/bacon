use {
    super::*,
};

pub struct HelpLine {
    quit: String,
    toggle_summary: Option<String>,
    wrap: Option<String>,
    not_wrap: Option<String>,
    toggle_backtrace: Option<String>,
}

impl HelpLine {
    pub fn new(settings: &Settings) -> Self {
        let kb = &settings.keybindings;
        let quit = kb.shortest_internal_key(Internal::Quit)
            .map(|k| format!("Hit *{k}* to quit"))
            .expect("the app to be quittable");
        let toggle_summary = kb.shortest_internal_key(Internal::ToggleSummary)
            .map(|k| format!("*{k}* to toggle summary mode"));
        let wrap = kb.shortest_internal_key(Internal::ToggleWrap)
            .map(|k| format!("*{k}* to wrap lines"));
        let not_wrap = kb.shortest_internal_key(Internal::ToggleWrap)
            .map(|k| format!("*{k}* to not wrap lines"));
        let toggle_backtrace = kb.shortest_internal_key(Internal::ToggleBacktrace)
            .map(|k| format!("*{k}* to toggle backtraces"));
        Self { quit, toggle_summary, wrap, not_wrap, toggle_backtrace }
    }
    pub fn markdown(&self, state: &AppState) -> String {
        let mut parts: Vec<&str> = vec![&self.quit];
        if let CommandResult::Report(report) = &state.cmd_result {
            if report.suggest_backtrace {
                if let Some(s) = &self.toggle_backtrace {
                    parts.push(s);
                }
            } else {
                if let Some(s) = &self.toggle_summary {
                    parts.push(s);
                }
                if state.wrap {
                    if let Some(s) = &self.not_wrap {
                        parts.push(s);
                    }
                } else {
                    if let Some(s) = &self.wrap {
                        parts.push(s);
                    }
                }
            }
        }
        parts.join(", ")
    }
}