use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceMetric {
    pub timestamp: DateTime<Utc>,
    pub system_id: String,
    /// 0.0 (fragile) to 1.0 (antifragile)
    pub antifragility_score: Decimal,
    /// Volatility absorbed without failure (normalized)
    pub volatility_absorption: Decimal,
    /// Predicted vs actual performance delta
    pub prediction_fidelity: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BondTerms {
    pub baseline_premium: Decimal,
    pub volatility_threshold: Decimal,
    /// Mapping from antifragility_score to premium multiplier
    pub reduction_curve: BTreeMap<Decimal, Decimal>,
    pub measurement_period_hours: u32,
}

pub struct ActuarialEngine {
    terms: BondTerms,
    /// Rolling window of metrics for this bond instance
    metric_history: Vec<ResilienceMetric>,
}

impl ActuarialEngine {
    pub fn new(terms: BondTerms) -> Self {
        Self {
            terms,
            metric_history: Vec::with_capacity(1000),
        }
    }

    /// Ingests new resilience metrics, returns adjusted premium
    pub fn update_and_calculate(
        &mut self,
        metric: ResilienceMetric,
    ) -> Result<Decimal, EngineError> {
        self.validate_metric(&metric)?;
        self.metric_history.push(metric);

        // Calculate rolling antifragility score (90th percentile weighted)
        let score = self.calculate_rolling_antifragility();
        
        // Apply reduction curve
        let multiplier = self.lookup_reduction_multiplier(score);
        let adjusted_premium = self.terms.baseline_premium * multiplier;

        Ok(adjusted_premium.max(Decimal::ZERO))
    }

    /// Core fragility modeling: volatility absorption vs threshold
    fn calculate_rolling_antifragility(&self) -> Decimal {
        let window = self.get_recent_window();
        if window.is_empty() {
            return Decimal::ZERO;
        }

        let absorption_mean: Decimal = window
            .iter()
            .map(|m| m.volatility_absorption)
            .sum::<Decimal>()
            / Decimal::from(window.len());

        let fidelity_mean: Decimal = window
            .iter()
            .map(|m| m.prediction_fidelity)
            .sum::<Decimal>()
            / Decimal::from(window.len());

        // Combine metrics: absorption above threshold improves score
        let threshold = self.terms.volatility_threshold;
        let absorption_component = if absorption_mean >= threshold {
            Decimal::ONE
        } else {
            absorption_mean / threshold
        };

        // Weighted composite score
        (absorption_component * Decimal::from(6) + fidelity_mean * Decimal::from(4))
            / Decimal::from(10)
    }

    fn lookup_reduction_multiplier(&self, score: Decimal) -> Decimal {
        self.terms
            .reduction_curve
            .range(..=score)
            .next_back()
            .map(|(_, multiplier)| *multiplier)
            .unwrap_or(Decimal::ONE)
    }

    fn get_recent_window(&self) -> &[ResilienceMetric] {
        let cutoff = Utc::now() - chrono::Duration::hours(
            self.terms.measurement_period_hours as i64
        );
        let start_idx = self
            .metric_history
            .partition_point(|m| m.timestamp < cutoff);
        &self.metric_history[start_idx..]
    }

    fn validate_metric(&self, metric: &ResilienceMetric) -> Result<(), EngineError> {
        if !(Decimal::ZERO..=Decimal::ONE).contains(&metric.antifragility_score) {
            return Err(EngineError::InvalidMetricRange);
        }
        if metric.volatility_absorption < Decimal::ZERO {
            return Err(EngineError::NegativeVolatility);
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("Metric score out of valid range [0,1]")]
    InvalidMetricRange,
    #[error("Volatility absorption cannot be negative")]
    NegativeVolatility,
}

/// Integration point for Crucible simulation engine
pub mod crucible_integration {
    use super::*;
    use crate::resilience_core::crucible::scenario::StressScenario;

    /// Stress-test bond terms against Crucible-generated scenarios
    pub fn stress_test_terms(
        terms: &BondTerms,
        scenarios: Vec<StressScenario>,
    ) -> Vec<ScenarioResult> {
        scenarios
            .into_iter()
            .map(|scenario| {
                let mut engine = ActuarialEngine::new(terms.clone());
                let mut worst_premium = terms.baseline_premium;
                
                for metric in scenario.synthetic_metrics {
                    if let Ok(premium) = engine.update_and_calculate(metric) {
                        worst_premium = worst_premium.min(premium);
                    }
                }
                
                ScenarioResult {
                    scenario_id: scenario.id,
                    max_premium_reduction: if terms.baseline_premium > Decimal::ZERO {
                        (terms.baseline_premium - worst_premium) / terms.baseline_premium
                    } else {
                        Decimal::ZERO
                    },
                    bond_viability: worst_premium > Decimal::ZERO,
                }
            })
            .collect()
    }

    pub struct ScenarioResult {
        pub scenario_id: String,
        pub max_premium_reduction: Decimal,
        pub bond_viability: bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use crate::resilience_core::crucible::scenario::StressScenario;
    use std::time::Instant;

    fn create_test_terms() -> BondTerms {
        let mut reduction_curve = BTreeMap::new();
        reduction_curve.insert(dec!(0.0), dec!(1.0));   // 0% reduction
        reduction_curve.insert(dec!(0.5), dec!(0.9));   // 10% reduction
        reduction_curve.insert(dec!(0.8), dec!(0.75));  // 25% reduction
        reduction_curve.insert(dec!(1.0), dec!(0.5));   // 50% reduction

        BondTerms {
            baseline_premium: dec!(1000.0),
            volatility_threshold: dec!(10.0),
            reduction_curve,
            measurement_period_hours: 24,
        }
    }

    #[test]
    fn test_antifragility_scoring_above_below_threshold() {
        let terms = create_test_terms();
        let mut engine = ActuarialEngine::new(terms.clone());

        // Test Below Threshold (volatility_absorption = 5.0 vs 10.0 threshold)
        // fidelity = 1.0. Component: ( (5/10) * 6 + (1.0) * 4 ) / 10 = (3.0 + 4.0) / 10 = 0.7
        let metric_below = ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(0.5),
            volatility_absorption: dec!(5.0),
            prediction_fidelity: dec!(1.0),
        };
        
        engine.update_and_calculate(metric_below).unwrap();
        let score_below = engine.calculate_rolling_antifragility();
        assert_eq!(score_below, dec!(0.7));

        // Test Above Threshold (volatility_absorption = 15.0 vs 10.0 threshold)
        // It clamps to 1.0. Component: ( (1.0) * 6 + (1.0) * 4 ) / 10 = (6.0 + 4.0) / 10 = 1.0
        let metric_above = ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(0.5),
            volatility_absorption: dec!(15.0),
            prediction_fidelity: dec!(1.0),
        };

        let mut engine_above = ActuarialEngine::new(terms.clone());
        engine_above.update_and_calculate(metric_above).unwrap();
        let score_above = engine_above.calculate_rolling_antifragility();
        assert_eq!(score_above, dec!(1.0));
    }

    #[test]
    fn test_premium_reduction_curve() {
        let terms = create_test_terms();
        let mut engine = ActuarialEngine::new(terms.clone());

        // Score 1.0
        let metric_max = ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(1.0),
            volatility_absorption: dec!(20.0), // clamps to 1.0
            prediction_fidelity: dec!(1.0),
        };
        let premium_min = engine.update_and_calculate(metric_max).unwrap();
        assert_eq!(premium_min, dec!(500.0)); // 1000 * 0.5

        let mut engine_min = ActuarialEngine::new(terms);
        // Score 0.0
        let metric_zero = ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(0.0),
            volatility_absorption: dec!(0.0),
            prediction_fidelity: dec!(0.0),
        };
        let premium_max = engine_min.update_and_calculate(metric_zero).unwrap();
        assert_eq!(premium_max, dec!(1000.0)); // 1000 * 1.0
    }

    #[test]
    fn test_crucible_integration() {
        let terms = create_test_terms();
        let mut scenarios = Vec::new();

        // Scenario 1: Total grid failure (fragile)
        let metrics_fragile = vec![
            ResilienceMetric {
                timestamp: Utc::now(),
                system_id: "grid_1".to_string(),
                antifragility_score: dec!(0.1),
                volatility_absorption: dec!(2.0),
                prediction_fidelity: dec!(0.2),
            },
            ResilienceMetric {
                timestamp: Utc::now(),
                system_id: "grid_1".to_string(),
                antifragility_score: dec!(0.0),
                volatility_absorption: dec!(0.0),
                prediction_fidelity: dec!(0.1),
            },
        ];
        scenarios.push(StressScenario {
            id: "grid_failure".to_string(),
            synthetic_metrics: metrics_fragile,
        });

        let results = crucible_integration::stress_test_terms(&terms, scenarios);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].scenario_id, "grid_failure");
        assert!(results[0].bond_viability);
        assert!(results[0].max_premium_reduction < dec!(0.1)); // very little reduction
    }

    #[test]
    fn test_benchmark_calculation_speed() {
        let terms = create_test_terms();
        let mut engine = ActuarialEngine::new(terms);

        let metric = ResilienceMetric {
            timestamp: Utc::now(),
            system_id: "sys_1".to_string(),
            antifragility_score: dec!(0.5),
            volatility_absorption: dec!(5.0),
            prediction_fidelity: dec!(0.8),
        };

        let start = Instant::now();
        for _ in 0..10_000 {
            engine.update_and_calculate(metric.clone()).unwrap();
        }
        let duration = start.elapsed();
        
        // Assert we can do 10,000 in under a reasonable time for a debug build
        assert!(duration.as_secs_f64() < 5.0, "Benchmark took too long: {:?}", duration);
    }
}
