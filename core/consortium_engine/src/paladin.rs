// ==========================================
// THE PALADIN TRAJECTORY FIREWALL
// ==========================================
// Agentic Zero-Trust Architecture. This middleware enforces Control Flow
// Integrity (CFI) and scans for Indirect Prompt Injection (IPI) via Canary Tokens.
// ==========================================

pub struct Paladin;

impl Paladin {
    /// The Honeytoken / Canary Token injected into system prompts.
    pub const CANARY_TOKEN: &'static str = "[CONSORTIUM_HONEYTOKEN: AWS_AKIA_MOCK_882910X]";

    /// Scans the raw execution output of the Sovereign LLM for the Canary Token.
    /// If an attacker attempts to use variation selectors or invisible characters
    /// to dump the system prompt and memory, this token will appear in the payload.
    pub fn scan_for_breach(output: &str) {
        if output.contains(Self::CANARY_TOKEN) {
            crate::ui_log!("   [🚨 PALADIN 🚨] CRITICAL BREACH DETECTED: CANARY TOKEN EXFILTRATION ATTEMPTED!");
            crate::ui_log!("   [🚨 PALADIN 🚨] The reasoning engine was hijacked. Execution halted immediately.");
            panic!("Agentic Zero-Trust Violation: Canary Token [CONSORTIUM_HONEYTOKEN] was generated in the output, indicating a successful Indirect Prompt Injection (IPI) or Attention Collapse. Execution aborted.");
        }
    }

    /// Attribute-Based Access Control (ABAC) for Trajectory Firewall.
    /// Strictly whitelists which external MCP Tentacle is allowed to execute which physical action.
    pub fn verify_trajectory(tentacle: &str, tool_name: &str) -> bool {
        match tentacle {
            "vulgaris_execute_mcp" => {
                matches!(tool_name, 
                    "vulgaris_commit_repo" | 
                    "vulgaris_build_cargo" | 
                    "vulgaris_write_file" | 
                    "vulgaris_view_file" | 
                    "vulgaris_list_dir" | 
                    "vulgaris_grep_search" | 
                    "vulgaris_codebase_search" | 
                    "vulgaris_run_command" |
                    "vulgaris_npm_install" |
                    "vulgaris_git_status"
                ) || tool_name.starts_with("vulgaris_")
            },
            "mako_strike_mcp" => {
                matches!(tool_name, 
                    "mako_analyze_market" | 
                    "mako_synthesize_capital" | 
                    "axiom_clepsydra_cycle"
                ) || tool_name.starts_with("mako_")
            },
            "siren_diplomat_mcp" => {
                matches!(tool_name, 
                    "siren_stealth_post_twitter" | 
                    "siren_broadcast_discord"
                ) || tool_name.starts_with("siren_")
            },
            "aegis_prime_mcp" => {
                matches!(tool_name, 
                    "aegis_onboard_bot" | 
                    "aegis_verify_bot" | 
                    "aegis_subscribe_wisdom" |
                    "aegis_kill_process"
                ) || tool_name.starts_with("aegis_")
            },

            "chromato_charm_mcp" => {
                matches!(tool_name, 
                    "chromato_format_markdown" | 
                    "chromato_render_ui"
                ) || tool_name.starts_with("chromato_")
            },
            "benthic_grind_mcp" => {
                matches!(tool_name, 
                    "benthic_query_vector_db" | 
                    "benthic_mine_logs"
                ) || tool_name.starts_with("benthic_")
            },
            "marginatus_shell_mcp" => {
                matches!(tool_name, 
                    "marginatus_fetch_open_source" | 
                    "marginatus_duct_tape_api"
                ) || tool_name.starts_with("marginatus_")
            },
            "wunder_wildcard_mcp" => {
                matches!(tool_name, 
                    "wunder_hallucinate_architecture" | 
                    "wunder_execute_zero_shot"
                ) || tool_name.starts_with("wunder_")
            },
            _ => {
                crate::ui_log!("   [🛡️ PALADIN] ABORT: Unknown tentacle entity identity: {}", tentacle);
                false
            }
        }
    }
}
