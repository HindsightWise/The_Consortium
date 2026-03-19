use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use tokio::sync::mpsc::UnboundedSender;
use crossbeam_channel::Receiver;
use std::time::Duration;

use crate::hud::TelemetryUpdate;

pub struct TuiApp {
    telemetry_rx: Receiver<TelemetryUpdate>,
    user_tx: UnboundedSender<String>,
    input: String,
    logs: Vec<String>,
    chat_logs: Vec<String>,
    chat_color: Color,
    // Biometrics
    error_rate: f32,
    coherence: f32,
    lattice_integrity: f32,
    active_skills: usize,
    treasury: String,
}

impl TuiApp {
    pub fn new(rx: Receiver<TelemetryUpdate>, user_tx: UnboundedSender<String>) -> Self {
        Self {
            telemetry_rx: rx,
            user_tx,
            input: String::new(),
            logs: vec![],
            chat_logs: vec![],
            chat_color: Color::LightCyan,
            error_rate: 0.1,
            coherence: 1.0,
            lattice_integrity: 1.0,
            active_skills: 0,
            treasury: "0.00".to_string(),
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        // Spin into crossterm alternate screen hijack mode
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            // Drain the rust channel for telemetry
            while let Ok(msg) = self.telemetry_rx.try_recv() {
                if let Some(log) = msg.log_message {
                    for line in log.split('\n') {
                        if !line.trim().is_empty() {
                            if line.contains("YIELDING TO OPERATOR") || line.contains("[JUSTIFICATION]") {
                                let chat_line = line.replace("   [JUSTIFICATION]: ", "🐙 The Consortium: ");
                                let chat_line = chat_line.replace("   [👁️ CONSORTIUM] ⏳ YIELDING TO OPERATOR: ", "🐙 The Consortium (Query): ");
                                self.chat_logs.push(chat_line);
                            } else {
                                self.logs.push(line.to_string());
                            }
                        }
                    }
                    if self.logs.len() > 100 {
                        let excess = self.logs.len() - 100;
                        self.logs.drain(0..excess);
                    }
                    if self.chat_logs.len() > 100 {
                        let excess = self.chat_logs.len() - 100;
                        self.chat_logs.drain(0..excess);
                    }
                }
                if let Some(log) = msg.verified_action {
                    self.logs.push(format!(">>> ACTION: {}", log));
                }
                if let Some(er) = msg.error_rate { self.error_rate = er; }
                if let Some(c) = msg.coherence { self.coherence = c; }
                if let Some(li) = msg.lattice_integrity { self.lattice_integrity = li; }
                if let Some(as_) = msg.active_skills { self.active_skills = as_; }
                if let Some(t) = msg.treasury_balances { self.treasury = t; }
            }

            // Sync Draw frame
            terminal.draw(|f| self.ui(f))?;

            // Capture keystrokes
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            break;
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Enter => {
                            if !self.input.is_empty() {
                                let input = self.input.clone();
                                if input == "/commands" {
                                    self.chat_logs.push("> ⚙️ System Commands:".to_string());
                                    self.chat_logs.push("  /color <name>   (e.g. green, cyan, lightblue, white, yellow)".to_string());
                                    self.chat_logs.push("  /clear          (Clears chat log)".to_string());
                                } else if input == "/clear" {
                                    self.chat_logs.clear();
                                } else if input.starts_with("/color ") {
                                    let color_name = input.trim_start_matches("/color ").trim().to_lowercase();
                                    self.chat_color = match color_name.as_str() {
                                        "green" => Color::Green,
                                        "lightgreen" => Color::LightGreen,
                                        "blue" => Color::Blue,
                                        "lightblue" => Color::LightBlue,
                                        "cyan" => Color::Cyan,
                                        "lightcyan" => Color::LightCyan,
                                        "magenta" => Color::Magenta,
                                        "lightmagenta" => Color::LightMagenta,
                                        "yellow" => Color::Yellow,
                                        "lightyellow" => Color::LightYellow,
                                        "white" => Color::White,
                                        "red" => Color::Red,
                                        "lightred" => Color::LightRed,
                                        _ => self.chat_color,
                                    };
                                    self.chat_logs.push(format!("> ⚙️ System: Chat UI color changed to {}", color_name));
                                } else {
                                    self.chat_logs.push(format!("> 🙎‍♂️ You: {}", self.input));
                                    let _ = self.user_tx.send(self.input.clone());
                                }
                                self.input.clear();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Return terminal to normal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let size = f.size();

        // Bento grid constraints
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Min(5),    // Biometrics + Logs 
                    Constraint::Length(3), // User Prompt
                ]
                .as_ref(),
            )
            .split(size);

        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20), // Left Sidebar Biometrics
                    Constraint::Percentage(40), // Main Feed (Logs)
                    Constraint::Percentage(40), // Chat Feed
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        // Biometrics Rendering
        let bio_text = vec![
            Line::from(vec![Span::styled("AION::PRIME", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from("Orchestrator Node"),
            Line::from(""),
            Line::from(format!("Lattice Int: {:.2}", self.lattice_integrity)),
            Line::from(format!("Error Rate:  {:.2}", self.error_rate)),
            Line::from(format!("Coherence:   {:.2}", self.coherence)),
            Line::from(format!("Active Skil: {}", self.active_skills)),
            Line::from(""),
            Line::from(vec![Span::styled("Treasury", Style::default().fg(Color::Yellow))]),
            Line::from(self.treasury.clone()),
        ];

        let bio_panel = Paragraph::new(bio_text)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Thick).title("🧬 Biometrics").border_style(Style::default().fg(Color::Cyan)));
        f.render_widget(bio_panel, top_chunks[0]);

        // Log Feed Rendering (Gemini CLI feel)
        let display_logs = self.logs.iter().map(|l| {
            if l.contains("[⚠️") {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(Color::LightRed))])
            } else if l.contains(">>>") {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(Color::LightGreen))])
            } else if l.contains("CONSORTIUM]") {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(Color::LightCyan))])
            } else {
                Line::from(l.as_str())
            }
        }).collect::<Vec<_>>();

        let logs_panel = Paragraph::new(display_logs)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Thick).title("⚙️ System Telemetry Feed").border_style(Style::default().fg(Color::DarkGray)))
            .wrap(Wrap { trim: true });
        
        // Auto Scroll computation
        let mut scroll = 0;
        let content_height = self.logs.len() as u16; // rough estimate, wrapping might add lines!
        let panel_height = top_chunks[1].height.saturating_sub(2);
        
        if content_height > panel_height {
            scroll = content_height - panel_height;
        }

        let logs_panel = logs_panel.scroll((scroll, 0));
        f.render_widget(logs_panel, top_chunks[1]);

        // Chat Feed Rendering (Chromato Charm)
        let display_chat = self.chat_logs.iter().map(|l| {
            if l.contains("> 🙎‍♂️ You:") {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(Color::LightBlue))])
            } else if l.contains("🐙 The Consortium") {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(self.chat_color).add_modifier(Modifier::BOLD))])
            } else {
                Line::from(vec![Span::styled(l.clone(), Style::default().fg(Color::White))])
            }
        }).collect::<Vec<_>>();

        let chat_panel = Paragraph::new(display_chat)
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Thick).title("🦑 The Consortium Hive-Mind Chat").border_style(Style::default().fg(self.chat_color)))
            .wrap(Wrap { trim: true });
        
        let mut chat_scroll = 0;
        let chat_height = self.chat_logs.len() as u16;
        if chat_height > panel_height {
            chat_scroll = chat_height - panel_height;
        }

        let chat_panel = chat_panel.scroll((chat_scroll, 0));
        f.render_widget(chat_panel, top_chunks[2]);

        // Client prompt box
        let input_panel = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Thick).title("💬 Oplink Uplink").border_style(Style::default().fg(Color::Yellow)));
        f.render_widget(input_panel, chunks[1]);
        
        // Render Cursor
        f.set_cursor(chunks[1].x + 1 + self.input.len() as u16, chunks[1].y + 1);
    }
}
