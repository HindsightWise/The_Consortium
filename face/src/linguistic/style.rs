use colored::*;
use regex::Regex;

pub struct TerminalStyle;

impl TerminalStyle {
    /// Renders basic markdown patterns (like **bold**) and common entities (filenames) with colors.
    pub fn render(input: &str) -> String {
        let mut output = input.to_string();

        // 1. Render Bold: **text** -> Bold text
        if let Ok(re_bold) = Regex::new(r"\*\*(?P<content>.*?)\*\*") {
            output = re_bold.replace_all(&output, |caps: &regex::Captures| {
                caps["content"].bold().to_string()
            }).to_string();
        }

        // 2. Render File Paths: Detect common path patterns and color them cyan
        if let Ok(re_path) = Regex::new(r"((?:[a-zA-Z0-9._\-/]+)\.(?:rs|txt|json|html|mjs|py|md|sh))") {
            output = re_path.replace_all(&output, |caps: &regex::Captures| {
                caps[1].cyan().to_string()
            }).to_string();
        }

        // 3. Render Status/Keywords
        output = output.replace("SUCCESS", &"SUCCESS".green().bold().to_string());
        output = output.replace("FAILURE", &"FAILURE".red().bold().to_string());
        output = output.replace("WARNING", &"WARNING".yellow().bold().to_string());
        output = output.replace("CRITICAL", &"CRITICAL".on_red().white().bold().to_string());
        output = output.replace("APPROVED", &"APPROVED".green().bold().to_string());
        output = output.replace("DENIED", &"DENIED".red().bold().to_string());

        output
    }

    /// Specialized agent log styling
    pub fn agent_label(name: &str) -> String {
        match name {
            "CEO" => name.magenta().bold().to_string(),
            "Facilitator" => name.blue().bold().to_string(),
            "Skeptic" => name.yellow().bold().to_string(),
            "Provocateur" => name.red().bold().to_string(),
            "Analyst" => name.green().bold().to_string(),
            "The Operator" => name.cyan().bold().to_string(),
            "The Bastion" => name.white().on_blue().bold().to_string(),
            "QA Engineer" => name.truecolor(255, 165, 0).bold().to_string(), // Orange
            _ => name.bright_black().bold().to_string(),
        }
    }
}
