use anyhow::Result;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use uuid::Uuid;

/// Execution strategy for a plan.
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionModel {
    Sequential,
    Parallel,
    Dag,
}

impl ExecutionModel {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "parallel" => ExecutionModel::Parallel,
            "dag" => ExecutionModel::Dag,
            _ => ExecutionModel::Sequential,
        }
    }
}

/// A single executable step within a plan.
#[derive(Clone, Debug)]
pub struct ActionStep {
    pub id: Uuid,
    pub original_id: String,
    pub action_name: String,
    pub parameters: String,
    pub dependencies: Vec<Uuid>,
}

/// A multi-step plan to achieve a goal.
#[derive(Clone, Debug)]
pub struct ActionPlan {
    pub id: Uuid,
    pub goal: String,
    pub steps: Vec<ActionStep>,
    pub execution_model: ExecutionModel,
}

pub struct PlanningService;

impl PlanningService {
    /// Calculate execution order for DAG topological sorting
    pub fn dag_execution_order(steps: &[ActionStep]) -> Result<Vec<usize>> {
        let mut deps_remaining: HashMap<Uuid, usize> = HashMap::new();
        let mut dependents: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut index_by_id: HashMap<Uuid, usize> = HashMap::new();
        
        for (idx, step) in steps.iter().enumerate() {
            deps_remaining.insert(step.id, step.dependencies.len());
            index_by_id.insert(step.id, idx);
            for dep in &step.dependencies {
                dependents
                    .entry(*dep)
                    .or_default()
                    .push(step.id);
            }
        }

        let mut ready: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for (idx, step) in steps.iter().enumerate() {
            if step.dependencies.is_empty() {
                ready.push(Reverse(idx));
            }
        }

        let mut order: Vec<usize> = Vec::with_capacity(steps.len());
        while let Some(Reverse(idx)) = ready.pop() {
            order.push(idx);
            let step_id = steps[idx].id;
            if let Some(next_steps) = dependents.get(&step_id) {
                for next_id in next_steps {
                    if let Some(remaining) = deps_remaining.get_mut(next_id) {
                        if *remaining > 0 {
                            *remaining -= 1;
                            if *remaining == 0 {
                                if let Some(next_idx) = index_by_id.get(next_id) {
                                    ready.push(Reverse(*next_idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        if order.len() != steps.len() {
            anyhow::bail!("No steps ready to execute - possible circular dependency in DAG");
        }

        Ok(order)
    }

    /// Parse an LLM XML output into an ActionPlan
    pub fn parse_plan_from_xml(xml: &str, fallback_goal: &str) -> Result<ActionPlan> {
        // Extract basic fields
        let goal_re = Regex::new(r"(?s)<goal>(.*?)</goal>")?;
        let exec_model_re = Regex::new(r"(?s)<execution_model>(.*?)</execution_model>")?;
        
        let goal = goal_re.captures(xml)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| fallback_goal.to_string());
            
        let execution_model = exec_model_re.captures(xml)
            .and_then(|c| c.get(1))
            .map(|m| ExecutionModel::from_str(m.as_str()))
            .unwrap_or(ExecutionModel::Sequential);

        // Extract steps
        let step_re = Regex::new(r"(?s)<step>(.*?)</step>")?;
        let id_re = Regex::new(r"(?s)<id>(.*?)</id>")?;
        let action_re = Regex::new(r"(?s)<action>(.*?)</action>")?;
        let params_re = Regex::new(r"(?s)<parameters>(.*?)</parameters>")?;
        let deps_re = Regex::new(r"(?s)<dependencies>(.*?)</dependencies>")?;

        let mut steps: Vec<ActionStep> = Vec::new();
        let mut id_map: HashMap<String, Uuid> = HashMap::new();
        let mut dep_strings: HashMap<Uuid, Vec<String>> = HashMap::new();

        for cap in step_re.captures_iter(xml) {
            let block = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            
            let orig_id = id_re.captures(block).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string());
            let action = action_re.captures(block).and_then(|c| c.get(1)).map(|m| m.as_str().trim().to_string());
            
            if orig_id.is_none() || action.is_none() { continue; }
            
            let orig_id = orig_id.ok_or_else(|| anyhow::anyhow!("Missing orig_id"))?;
            let action = action.ok_or_else(|| anyhow::anyhow!("Missing action"))?;
            let actual_id = Uuid::new_v4();
            
            id_map.insert(orig_id.clone(), actual_id);

            let parameters = params_re.captures(block)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();

            let deps_raw = deps_re.captures(block)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
                
            // Very simple JSON array parsing for dependencies like ["step_1", "step_2"]
            let mut deps = Vec::new();
            if deps_raw.starts_with('[') && deps_raw.ends_with(']') {
                let cleaned = deps_raw[1..deps_raw.len()-1].to_string();
                for d in cleaned.split(',') {
                    let d = d.trim().replace("\"", "");
                    if !d.is_empty() {
                        deps.push(d);
                    }
                }
            }

            dep_strings.insert(actual_id, deps);

            steps.push(ActionStep {
                id: actual_id,
                original_id: orig_id,
                action_name: action,
                parameters,
                dependencies: Vec::new(),
            });
        }

        // Resolve dependencies
        for step in steps.iter_mut() {
            let deps = dep_strings.get(&step.id).cloned().unwrap_or_default();
            let mut resolved: Vec<Uuid> = Vec::new();
            for d in deps {
                if let Some(id) = id_map.get(&d) {
                    resolved.push(*id);
                }
            }
            step.dependencies = resolved;
        }

        Ok(ActionPlan {
            id: Uuid::new_v4(),
            goal,
            steps,
            execution_model,
        })
    }
}
