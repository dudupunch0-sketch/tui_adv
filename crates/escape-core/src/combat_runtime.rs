use crate::combat_execution::{
    assemble_result, build_execution_log, prepare_context, stable_fingerprint,
    CombatExecutionContext,
};
use crate::combat_resolution::{
    CombatResolutionError, CombatResolutionFrame, CombatResolutionRequest, CombatResolutionResult,
    CombatResolutionState, CombatResolutionStepper,
};
use crate::{
    CombatExecutionError, CombatExecutionResult, CombatOpportunityCatalog,
    CombatOpportunityContext, CombatOpportunityError, CombatOpportunityEvaluation,
    CombatOpportunityInstance, CombatSimulation, CombatSimulationError, CombatTickFrame,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CombatRuntimeFrame {
    pub(crate) execution: CombatTickFrame,
    pub(crate) resolution: CombatResolutionFrame,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CombatRuntimeOpportunityConfig {
    pub(crate) catalog: CombatOpportunityCatalog,
    pub(crate) instances: Vec<CombatOpportunityInstance>,
    pub(crate) context: CombatOpportunityContext,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CombatRuntimeSelectionHistoryEntry {
    pub(crate) segment_index: u32,
    pub(crate) tick: u32,
    pub(crate) instance_id: String,
    pub(crate) opportunity_id: String,
    pub(crate) response_id: String,
}

pub(crate) fn derive_segment_seed(
    base_effective_seed: u64,
    namespace: crate::CombatRngNamespace,
    simulation_version: &crate::CombatSimulationVersion,
    manifest_fingerprint: &str,
    segment_index: u32,
    history: &[CombatRuntimeSelectionHistoryEntry],
) -> Result<u64, CombatRuntimeError> {
    if simulation_version.as_str().trim().is_empty() || manifest_fingerprint.trim().is_empty() {
        return Err(CombatRuntimeError::InvalidInput);
    }
    let mut canonical = history.to_vec();
    for entry in &canonical {
        if entry.segment_index > segment_index
            || entry.instance_id.trim().is_empty()
            || entry.opportunity_id.trim().is_empty()
            || entry.response_id.trim().is_empty()
        {
            return Err(CombatRuntimeError::InvalidInput);
        }
    }
    canonical.sort_by(|a, b| {
        a.segment_index
            .cmp(&b.segment_index)
            .then(a.tick.cmp(&b.tick))
            .then(a.instance_id.cmp(&b.instance_id))
            .then(a.opportunity_id.cmp(&b.opportunity_id))
            .then(a.response_id.cmp(&b.response_id))
    });
    if canonical
        .windows(2)
        .any(|entries| entries[0].segment_index == entries[1].segment_index)
    {
        return Err(CombatRuntimeError::InvalidInput);
    }
    let payload = (
        base_effective_seed,
        namespace.as_str(),
        simulation_version.as_str(),
        manifest_fingerprint,
        segment_index,
        canonical,
    );
    u64::from_str_radix(&stable_fingerprint(&payload), 16)
        .map_err(|_| CombatRuntimeError::InvalidInput)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CombatRuntimePause {
    pub(crate) tick: u32,
    pub(crate) evaluation: CombatOpportunityEvaluation,
    pub(crate) evaluation_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CombatRuntimeAdvance {
    Frame(CombatRuntimeFrame),
    Paused(CombatRuntimePause),
    Complete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CombatRuntimeError {
    Execution(CombatExecutionError),
    Simulation(CombatSimulationError),
    Resolution(CombatResolutionError),
    Opportunity(CombatOpportunityError),
    InvalidInput,
}

impl From<CombatExecutionError> for CombatRuntimeError {
    fn from(error: CombatExecutionError) -> Self {
        Self::Execution(error)
    }
}

impl From<CombatSimulationError> for CombatRuntimeError {
    fn from(error: CombatSimulationError) -> Self {
        Self::Simulation(error)
    }
}

impl From<CombatResolutionError> for CombatRuntimeError {
    fn from(error: CombatResolutionError) -> Self {
        Self::Resolution(error)
    }
}

impl From<CombatOpportunityError> for CombatRuntimeError {
    fn from(error: CombatOpportunityError) -> Self {
        Self::Opportunity(error)
    }
}

impl std::fmt::Display for CombatRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CombatRuntimeError {}

pub(crate) struct CombatRuntime {
    request: CombatResolutionRequest,
    context: CombatExecutionContext,
    simulation: CombatSimulation,
    stepper: CombatResolutionStepper,
    execution_frames: Vec<CombatTickFrame>,
    resolution_frames: Vec<CombatResolutionFrame>,
    opportunities: Option<CombatRuntimeOpportunityState>,
    paused: Option<CombatRuntimePause>,
    segment_index: u32,
    selection_history: Vec<CombatRuntimeSelectionHistoryEntry>,
    next_segment_seed: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CombatRuntimeOpportunityState {
    catalog: CombatOpportunityCatalog,
    instances: Vec<CombatOpportunityInstance>,
    context: CombatOpportunityContext,
}

impl CombatRuntime {
    pub(crate) fn new(request: CombatResolutionRequest) -> Result<Self, CombatRuntimeError> {
        if request.execution.ticks == 0 {
            return Err(CombatRuntimeError::Execution(
                CombatExecutionError::ZeroTicks,
            ));
        }
        if request.execution.ticks > request.execution.input.config.max_ticks {
            return Err(CombatRuntimeError::Simulation(
                CombatSimulationError::MaxTicksExceeded,
            ));
        }
        let (context, simulation) = prepare_context(&request.execution)?;
        let execution_metadata = assemble_result(&context, vec![], vec![]);
        let stepper = CombatResolutionStepper::new(&request, execution_metadata)?;
        Ok(Self {
            request,
            context,
            simulation,
            stepper,
            execution_frames: vec![],
            resolution_frames: vec![],
            opportunities: None,
            paused: None,
            segment_index: 0,
            selection_history: vec![],
            next_segment_seed: None,
        })
    }

    pub(crate) fn with_opportunities(
        request: CombatResolutionRequest,
        config: CombatRuntimeOpportunityConfig,
    ) -> Result<Self, CombatRuntimeError> {
        config.catalog.validate()?;
        config.context.budget.validate()?;
        let mut runtime = Self::new(request)?;
        runtime.opportunities = Some(CombatRuntimeOpportunityState {
            catalog: config.catalog,
            instances: config.instances,
            context: config.context,
        });
        Ok(runtime)
    }

    pub(crate) fn advance_tick(
        &mut self,
    ) -> Result<Option<CombatRuntimeFrame>, CombatRuntimeError> {
        if self.execution_frames.len() as u32 >= self.request.execution.ticks {
            return Ok(None);
        }
        let execution = self.simulation.advance_tick()?;
        let resolution = self.stepper.step(&execution)?;
        let health_hundredths = resolution
            .combatants
            .iter()
            .map(|combatant| (combatant.id.clone(), combatant.current_health_hundredths))
            .collect();
        self.simulation
            .sync_active_from_health(&health_hundredths)?;
        self.execution_frames.push(execution.clone());
        self.resolution_frames.push(resolution.clone());
        Ok(Some(CombatRuntimeFrame {
            execution,
            resolution,
        }))
    }

    pub(crate) fn advance_with_opportunities(
        &mut self,
    ) -> Result<CombatRuntimeAdvance, CombatRuntimeError> {
        if let Some(pause) = &self.paused {
            return Ok(CombatRuntimeAdvance::Paused(pause.clone()));
        }
        let Some(frame) = self.advance_tick()? else {
            return Ok(CombatRuntimeAdvance::Complete);
        };
        let Some(state) = &mut self.opportunities else {
            return Ok(CombatRuntimeAdvance::Frame(frame));
        };
        let mut context = state.context.clone();
        context.current_tick = frame.resolution.tick;
        let evaluation = state.catalog.evaluate(&state.instances, &context)?;
        state.context.current_tick = frame.resolution.tick;
        state.context.budget = evaluation.budget.clone();
        let Some(candidate) = &evaluation.candidate else {
            return Ok(CombatRuntimeAdvance::Frame(frame));
        };
        state
            .context
            .presented_instance_ids
            .insert(candidate.instance_id.clone());
        let evaluation_fingerprint = evaluation.fingerprint()?;
        let pause = CombatRuntimePause {
            tick: frame.resolution.tick,
            evaluation,
            evaluation_fingerprint,
        };
        self.paused = Some(pause.clone());
        Ok(CombatRuntimeAdvance::Paused(pause))
    }

    pub(crate) fn resume_with_response(
        &mut self,
        response_id: &str,
    ) -> Result<u64, CombatRuntimeError> {
        if response_id.trim().is_empty() {
            return Err(CombatRuntimeError::InvalidInput);
        }
        let pause = self
            .paused
            .as_ref()
            .ok_or(CombatRuntimeError::InvalidInput)?;
        let candidate = pause
            .evaluation
            .candidate
            .as_ref()
            .ok_or(CombatRuntimeError::InvalidInput)?;
        if !candidate
            .options
            .iter()
            .any(|option| option.id == response_id)
        {
            return Err(CombatRuntimeError::InvalidInput);
        }
        let next_segment_index = self
            .segment_index
            .checked_add(1)
            .ok_or(CombatRuntimeError::InvalidInput)?;
        let entry = CombatRuntimeSelectionHistoryEntry {
            segment_index: self.segment_index,
            tick: pause.tick,
            instance_id: candidate.instance_id.clone(),
            opportunity_id: candidate.opportunity_id.clone(),
            response_id: response_id.to_owned(),
        };
        let mut history = self.selection_history.clone();
        history.push(entry);
        let next_seed = derive_segment_seed(
            self.context.effective_seed,
            self.context.namespace,
            &self.context.provenance.simulation_version,
            &self.context.provenance.manifest_fingerprint,
            next_segment_index,
            &history,
        )?;
        self.selection_history = history;
        self.segment_index = next_segment_index;
        self.next_segment_seed = Some(next_seed);
        self.paused = None;
        Ok(next_seed)
    }

    pub(crate) fn resume_no_intervention(&mut self) -> Result<(), CombatRuntimeError> {
        self.resume_with_response("no_intervention").map(|_| ())
    }

    pub(crate) fn finish(self) -> Result<CombatResolutionResult, CombatRuntimeError> {
        if self.execution_frames.len() as u32 != self.request.execution.ticks {
            return Err(CombatRuntimeError::InvalidInput);
        }
        let execution_log = build_execution_log(&self.execution_frames);
        let execution: CombatExecutionResult =
            assemble_result(&self.context, self.execution_frames, execution_log);
        let full_log = self.stepper.full_log().to_vec();
        let state: CombatResolutionState = self.stepper.finish();
        let core_log = full_log
            .iter()
            .filter(|event| event.importance >= crate::CombatLogImportance::Important)
            .cloned()
            .collect();
        let fingerprint = stable_fingerprint(&(
            execution.fingerprint.clone(),
            &self.resolution_frames,
            &state,
            &full_log,
        ));
        Ok(CombatResolutionResult {
            execution,
            frames: self.resolution_frames,
            state,
            full_log,
            core_log,
            fingerprint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn make_request(ticks: u32) -> CombatResolutionRequest {
        let combatant = |id: &str| crate::CombatantState {
            id: id.into(),
            current_health: 100,
            maximum_health: 100,
            current_breath: 1,
            maximum_breath: 1,
            balance: 100,
            maximum_balance: 100,
            fear: 0,
            anger: 0,
            posture: crate::Posture::Neutral,
            weapon_control: crate::WeaponControl::Stable,
            relationship_refs: vec![],
            environment_refs: vec![],
            team_refs: vec![],
            persistent_status_ids: vec![],
            combat_effect_ids: vec![],
        };
        let participant = |id: &str, side: crate::CombatSide, position: crate::HexCoord| {
            crate::CombatSimulationParticipant {
                id: id.into(),
                side,
                position,
                facing: crate::HexCoord { q: 1, r: 0 },
                speed_per_tick: 1,
                move_speed_hundredths: None,
                collision_radius: 1,
                attack_range: 2,
                support_range: 2,
                role_id: "r".into(),
                target_policy_id: None,
                active: true,
                occupies: vec![],
            }
        };
        CombatResolutionRequest {
            execution: crate::CombatExecutionRequest {
                input: crate::CombatSimulationInput {
                    manifest: crate::CombatManifest {
                        simulation_version: crate::CombatSimulationVersion::new(
                            crate::CURRENT_SIMULATION_VERSION,
                        )
                        .unwrap(),
                        actual_seed: 7,
                        world_state_fingerprint: "w".into(),
                        applied_effects: vec![],
                        suppressed_effects: vec![],
                        combatant_ids: vec!["a".into(), "e".into()],
                        placement_ids: vec![],
                        environment_ids: vec![],
                        team_ids: vec![],
                        rule_ids: vec![],
                        public_info_ids: vec![],
                    },
                    state: crate::CombatState {
                        battle_id: "b".into(),
                        combatants: vec![combatant("a"), combatant("e")],
                        persistent_statuses: vec![],
                        active_effects: vec![],
                        environment_refs: vec![],
                        team_refs: vec![],
                        team_formations: vec![],
                        relationships: vec![],
                        environment_states: vec![],
                        manifest_fingerprint: "fp".into(),
                    },
                    seed: 7,
                    config: crate::CombatSimulationConfig {
                        tick_millis: 100,
                        max_ticks: ticks,
                    },
                    participants: vec![
                        participant("a", crate::CombatSide::Ally, crate::HexCoord { q: 0, r: 0 }),
                        participant(
                            "e",
                            crate::CombatSide::Enemy,
                            crate::HexCoord { q: 1, r: 0 },
                        ),
                    ],
                    roles: vec![crate::CombatRolePreset {
                        id: "r".into(),
                        weights: crate::CombatRoleWeights {
                            preferred_distance: 1,
                            aggression: 1,
                            formation_maintenance: 0,
                            pursuit_range: 1,
                            protect_priority: 0,
                            target_priority: 0,
                            risk_tolerance: 0,
                            ability_priority: 0,
                        },
                    }],
                    policies: vec![],
                },
                mode: crate::CombatRunMode::Actual,
                presentation: crate::CombatPresentationSpeed::OneX,
                ticks,
            },
            attacks: vec![],
            defenses: vec![
                crate::CombatDefenseProfile {
                    combatant_id: "a".into(),
                    defense_hundredths: 0,
                    balance_resistance_hundredths: 0,
                },
                crate::CombatDefenseProfile {
                    combatant_id: "e".into(),
                    defense_hundredths: 0,
                    balance_resistance_hundredths: 0,
                },
            ],
            catalog: crate::CombatEffectCatalog { effects: vec![] },
        }
    }

    fn opportunity_config() -> CombatRuntimeOpportunityConfig {
        CombatRuntimeOpportunityConfig {
            catalog: crate::CombatOpportunityCatalog {
                opportunities: vec![crate::CombatOpportunityDefinition {
                    id: "danger".into(),
                    trigger_tags: vec!["danger_tag".into()],
                    required_condition_ids: vec![],
                    thresholds: crate::CombatDetectionThresholds {
                        detected: 0,
                        interpreted: 1,
                        insightful: 2,
                    },
                    expiry_tick: None,
                    dedupe: true,
                    scripted: false,
                    defeat_risk: true,
                    battlefield_impact: false,
                    unique_response: false,
                    tactical_priority: 1,
                    free_alert_id: None,
                }],
                responses: vec![crate::CombatResponseDefinition {
                    id: "intervene".into(),
                    opportunity_id: "danger".into(),
                    minimum_detection: crate::CombatDetectionLevel::Detected,
                    required_capability_ids: vec![],
                    required_condition_ids: vec![],
                    executor_selector: "observer".into(),
                    target_selector: "self".into(),
                    cost_tags: vec![],
                    resolution_kind: "effect".into(),
                    success_effect_ids: vec!["noop_effect".into()],
                    failure_effect_ids: vec!["noop_effect".into()],
                    unique: false,
                    tactical_priority: 1,
                }],
                effect_catalog: crate::CombatEffectCatalog {
                    effects: vec![crate::CombatEffectDefinition {
                        id: "noop_effect".into(),
                        source: "test".into(),
                        category: crate::CombatEffectCategory::State,
                        target_selector: "target".into(),
                        parameters: Default::default(),
                        conditions: vec![],
                        phase: crate::EffectPhase::CombatStart,
                        lifetime: crate::EffectLifetime::Persistent,
                        stacking: crate::EffectStacking::Unique,
                        stacking_group: "noop".into(),
                        stacking_cap: None,
                        priority: 0,
                        visibility: crate::EffectVisibility::Public,
                        tags: vec![],
                    }],
                },
            },
            instances: vec![crate::CombatOpportunityInstance {
                id: "danger_instance".into(),
                definition_id: "danger".into(),
            }],
            context: crate::CombatOpportunityContext {
                current_tick: 0,
                active_tag_ids: BTreeSet::from(["danger_tag".into()]),
                active_condition_ids: BTreeSet::new(),
                presented_instance_ids: BTreeSet::new(),
                observers: vec![crate::CombatObserver {
                    id: "observer".into(),
                    detection_score: 10,
                    capability_ids: vec![],
                    can_observe: true,
                    can_act: true,
                }],
                budget: crate::CombatInterventionBudget {
                    maximum: 1,
                    consumed: 0,
                },
                manifest_fingerprint: "manifest".into(),
            },
        }
    }

    #[test]
    fn runtime_interleaves_one_tick_and_matches_batch_result() {
        let request = make_request(2);
        let expected = crate::resolve_combat(request.clone()).unwrap();
        let mut runtime = CombatRuntime::new(request).unwrap();
        let frame = runtime.advance_tick().unwrap().unwrap();
        assert_eq!(frame.execution.tick, 1);
        assert_eq!(frame.execution.tick, frame.resolution.tick);
        let second = runtime.advance_tick().unwrap().unwrap();
        assert_eq!(second.execution.tick, 2);
        assert!(runtime.advance_tick().unwrap().is_none());
        let actual = runtime.finish().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn lethal_tick_syncs_inactive_roster_before_next_tick() {
        let mut request = make_request(2);
        request.attacks.push(crate::CombatAttackDefinition {
            id: "lethal".into(),
            actor_id: "a".into(),
            power_hundredths: 24_000,
            ability_multiplier_hundredths: 100,
            accuracy_percent: 100,
            attack_range: 2,
            penetration_hundredths: 0,
            collision_balance_hundredths: 0,
            balance_power_hundredths: 0,
            attack_speed_hundredths: None,
            effects: vec![],
        });
        let mut runtime = CombatRuntime::new(request).unwrap();
        let first = runtime.advance_tick().unwrap().unwrap();
        assert!(first.resolution.outcomes.iter().any(|outcome| outcome.hit));
        assert_eq!(
            first
                .resolution
                .combatants
                .iter()
                .find(|combatant| combatant.id == "e")
                .unwrap()
                .current_health_hundredths,
            0
        );

        let second = runtime.advance_tick().unwrap().unwrap();
        assert!(!second
            .execution
            .moves
            .iter()
            .any(|intent| intent.actor_id == "e"));
        assert_eq!(
            second
                .execution
                .moves
                .iter()
                .find(|intent| intent.actor_id == "a")
                .and_then(|intent| intent.target_id.clone()),
            None
        );
        assert!(second.resolution.outcomes.is_empty());
    }

    #[test]
    fn opportunity_candidate_pauses_at_exact_tick_and_dedupes_after_resume() {
        let request = make_request(2);
        let mut runtime = CombatRuntime::with_opportunities(request, opportunity_config()).unwrap();
        let paused = match runtime.advance_with_opportunities().unwrap() {
            CombatRuntimeAdvance::Paused(pause) => pause,
            other => panic!("expected pause, got {other:?}"),
        };
        assert_eq!(paused.tick, 1);
        assert_eq!(paused.evaluation.budget.consumed, 1);
        assert_eq!(
            paused.evaluation.candidate.as_ref().unwrap().instance_id,
            "danger_instance"
        );
        assert_eq!(
            paused.evaluation_fingerprint,
            paused.evaluation.fingerprint().unwrap()
        );
        assert_eq!(
            runtime.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(paused.clone())
        );

        runtime.resume_no_intervention().unwrap();
        match runtime.advance_with_opportunities().unwrap() {
            CombatRuntimeAdvance::Frame(frame) => assert_eq!(frame.execution.tick, 2),
            other => panic!("expected resumed frame, got {other:?}"),
        }
        assert_eq!(
            runtime.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Complete
        );
        assert!(matches!(
            runtime.resume_no_intervention(),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }

    #[test]
    fn response_selection_records_history_and_derives_deterministic_segment_seed() {
        let request = make_request(2);
        let mut first =
            CombatRuntime::with_opportunities(request.clone(), opportunity_config()).unwrap();
        let mut second = CombatRuntime::with_opportunities(request, opportunity_config()).unwrap();
        assert!(matches!(
            first.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(_)
        ));
        assert!(matches!(
            second.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(_)
        ));

        let first_seed = first.resume_with_response("intervene").unwrap();
        let second_seed = second.resume_with_response("intervene").unwrap();
        assert_eq!(first_seed, second_seed);
        assert_eq!(first.segment_index, 1);
        assert_eq!(first.selection_history, second.selection_history);
        assert_eq!(
            stable_fingerprint(&first.selection_history),
            stable_fingerprint(&second.selection_history)
        );
        assert_eq!(first.next_segment_seed, Some(first_seed));
        assert!(first.paused.is_none());
        assert_eq!(first.selection_history[0].segment_index, 0);
        assert_eq!(first.selection_history[0].tick, 1);
        assert_eq!(first.selection_history[0].instance_id, "danger_instance");
        assert_eq!(first.selection_history[0].opportunity_id, "danger");
        assert_eq!(first.selection_history[0].response_id, "intervene");
    }

    #[test]
    fn response_selection_distinguishes_no_intervention_and_rejects_invalid_input() {
        let mut no_intervention =
            CombatRuntime::with_opportunities(make_request(2), opportunity_config()).unwrap();
        let mut actionable =
            CombatRuntime::with_opportunities(make_request(2), opportunity_config()).unwrap();
        assert!(matches!(
            no_intervention.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(_)
        ));
        assert!(matches!(
            actionable.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(_)
        ));
        no_intervention.resume_no_intervention().unwrap();
        let no_intervention_seed = no_intervention.next_segment_seed.unwrap();
        let actionable_seed = actionable.resume_with_response("intervene").unwrap();
        assert_ne!(no_intervention_seed, actionable_seed);
        assert_eq!(
            no_intervention.selection_history[0].response_id,
            "no_intervention"
        );

        let mut invalid =
            CombatRuntime::with_opportunities(make_request(2), opportunity_config()).unwrap();
        assert!(matches!(
            invalid.resume_with_response("intervene"),
            Err(CombatRuntimeError::InvalidInput)
        ));
        assert!(matches!(
            invalid.advance_with_opportunities().unwrap(),
            CombatRuntimeAdvance::Paused(_)
        ));
        assert!(matches!(
            invalid.resume_with_response("unknown"),
            Err(CombatRuntimeError::InvalidInput)
        ));
        assert_eq!(invalid.segment_index, 0);
        assert!(invalid.selection_history.is_empty());
        invalid.resume_with_response("intervene").unwrap();
        assert!(matches!(
            invalid.resume_with_response("intervene"),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }

    #[test]
    fn segment_seed_is_canonical_and_namespace_isolated() {
        let version =
            crate::CombatSimulationVersion::new(crate::CURRENT_SIMULATION_VERSION).unwrap();
        let first = CombatRuntimeSelectionHistoryEntry {
            segment_index: 0,
            tick: 1,
            instance_id: "instance_a".into(),
            opportunity_id: "danger".into(),
            response_id: "no_intervention".into(),
        };
        let second = CombatRuntimeSelectionHistoryEntry {
            segment_index: 1,
            tick: 4,
            instance_id: "instance_b".into(),
            opportunity_id: "rescue".into(),
            response_id: "assist".into(),
        };
        let history = vec![first.clone(), second.clone()];
        let mut reversed = vec![second, first];
        let actual = derive_segment_seed(
            7,
            crate::CombatRngNamespace::ActualCombat,
            &version,
            "manifest",
            1,
            &history,
        )
        .unwrap();
        assert_eq!(
            actual,
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ActualCombat,
                &version,
                "manifest",
                1,
                &reversed,
            )
            .unwrap()
        );
        reversed[0].response_id = "different".into();
        assert_ne!(
            actual,
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ActualCombat,
                &version,
                "manifest",
                1,
                &reversed,
            )
            .unwrap()
        );
        assert_ne!(
            actual,
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ForecastEnsemble,
                &version,
                "manifest",
                1,
                &history,
            )
            .unwrap()
        );
    }

    #[test]
    fn segment_seed_rejects_invalid_history() {
        let version =
            crate::CombatSimulationVersion::new(crate::CURRENT_SIMULATION_VERSION).unwrap();
        let entry = CombatRuntimeSelectionHistoryEntry {
            segment_index: 1,
            tick: 1,
            instance_id: "instance".into(),
            opportunity_id: "danger".into(),
            response_id: "assist".into(),
        };
        assert!(matches!(
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ActualCombat,
                &version,
                "manifest",
                1,
                &[entry.clone(), entry.clone()]
            ),
            Err(CombatRuntimeError::InvalidInput)
        ));
        let mut future = entry.clone();
        future.segment_index = 2;
        assert!(matches!(
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ActualCombat,
                &version,
                "manifest",
                1,
                &[future]
            ),
            Err(CombatRuntimeError::InvalidInput)
        ));
        assert!(matches!(
            derive_segment_seed(
                7,
                crate::CombatRngNamespace::ActualCombat,
                &version,
                "",
                1,
                &[]
            ),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }

    #[test]
    fn runtime_rejects_zero_and_over_max_ticks() {
        let mut request = make_request(1);
        request.execution.ticks = 0;
        assert!(matches!(
            CombatRuntime::new(request),
            Err(CombatRuntimeError::Execution(
                CombatExecutionError::ZeroTicks
            ))
        ));

        let mut request = make_request(1);
        request.execution.ticks = 2;
        request.execution.input.config.max_ticks = 1;
        assert!(matches!(
            CombatRuntime::new(request),
            Err(CombatRuntimeError::Simulation(
                CombatSimulationError::MaxTicksExceeded
            ))
        ));
    }
    #[test]
    fn checkpoint_roundtrip_paused_restore_resume_matches() {
        let request = make_request(2);
        let mut original =
            CombatRuntime::with_opportunities(request.clone(), opportunity_config()).unwrap();
        let _ = original.advance_with_opportunities().unwrap();
        let checkpoint = original.checkpoint().unwrap();
        let json = serde_json::to_string(&checkpoint).unwrap();
        let decoded: CombatRuntimeCheckpoint = serde_json::from_str(&json).unwrap();
        let mut restored = CombatRuntime::restore(decoded).unwrap();
        if original.paused.is_some() {
            original.resume_no_intervention().unwrap();
            restored.resume_no_intervention().unwrap();
        }
        while original.advance_tick().unwrap().is_some() {}
        while restored.advance_tick().unwrap().is_some() {}
        assert_eq!(
            original.finish().unwrap().fingerprint,
            restored.finish().unwrap().fingerprint
        );
    }

    #[test]
    fn checkpoint_restore_rejects_frame_length_mismatch() {
        let mut runtime = CombatRuntime::new(make_request(2)).unwrap();
        runtime.advance_tick().unwrap();
        let mut checkpoint = runtime.checkpoint().unwrap();
        checkpoint.resolution_frames.clear();
        assert!(matches!(
            CombatRuntime::restore(checkpoint),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }

    #[test]
    fn checkpoint_restore_rejects_forged_next_segment_seed() {
        let runtime = CombatRuntime::new(make_request(1)).unwrap();
        let mut checkpoint = runtime.checkpoint().unwrap();
        checkpoint.segment_index = 1;
        checkpoint
            .selection_history
            .push(CombatRuntimeSelectionHistoryEntry {
                segment_index: 0,
                tick: 1,
                instance_id: "i".into(),
                opportunity_id: "o".into(),
                response_id: "r".into(),
            });
        checkpoint.next_segment_seed = Some(0);
        assert!(matches!(
            CombatRuntime::restore(checkpoint),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }
    #[test]
    fn response_checkpoint_roundtrip_preserves_finish() {
        let request = make_request(2);
        let mut original =
            CombatRuntime::with_opportunities(request.clone(), opportunity_config()).unwrap();
        let _ = original.advance_with_opportunities().unwrap();
        original.resume_with_response("intervene").unwrap();
        let checkpoint = original.checkpoint().unwrap();
        let encoded = serde_json::to_string(&checkpoint).unwrap();
        let decoded: CombatRuntimeCheckpoint = serde_json::from_str(&encoded).unwrap();
        let mut restored = CombatRuntime::restore(decoded).unwrap();
        while original.advance_tick().unwrap().is_some() {}
        while restored.advance_tick().unwrap().is_some() {}
        assert_eq!(
            original.finish().unwrap().fingerprint,
            restored.finish().unwrap().fingerprint
        );
    }

    #[test]
    fn checkpoint_save_envelope_roundtrip_preserves_public_payload() {
        let mut runtime = CombatRuntime::new(make_request(1)).unwrap();
        runtime.advance_tick().unwrap();
        let checkpoint = runtime.checkpoint().unwrap();
        let envelope = crate::SaveEnvelope {
            schema_version: crate::SAVE_SCHEMA_VERSION,
            state: crate::new_game(7),
            combat_checkpoint: Some(checkpoint.clone()),
        };
        let encoded = serde_json::to_string(&envelope).unwrap();
        let decoded: crate::SaveEnvelope = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.combat_checkpoint, Some(checkpoint));
    }

    #[test]
    fn checkpoint_restore_rejects_schema_and_provenance_mismatch() {
        let mut runtime = CombatRuntime::new(make_request(1)).unwrap();
        runtime.advance_tick().unwrap();
        let checkpoint = runtime.checkpoint().unwrap();
        let mut wrong_schema = checkpoint.clone();
        wrong_schema.schema_version += 1;
        assert!(matches!(
            CombatRuntime::restore(wrong_schema),
            Err(CombatRuntimeError::InvalidInput)
        ));
        let mut wrong_manifest = checkpoint;
        wrong_manifest.manifest_fingerprint = "different_manifest".into();
        assert!(matches!(
            CombatRuntime::restore(wrong_manifest),
            Err(CombatRuntimeError::InvalidInput)
        ));
    }
}

pub const COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatRuntimeCheckpoint {
    pub schema_version: u32,
    pub simulation_version: crate::CombatSimulationVersion,
    pub manifest_fingerprint: String,
    pub effective_seed: u64,
    pub namespace: crate::CombatRngNamespace,
    pub(crate) request: CombatResolutionRequest,
    pub(crate) execution_frames: Vec<CombatTickFrame>,
    pub(crate) resolution_frames: Vec<CombatResolutionFrame>,
    pub(crate) opportunities: Option<CombatRuntimeOpportunityState>,
    pub(crate) paused: Option<CombatRuntimePause>,
    pub(crate) segment_index: u32,
    pub(crate) selection_history: Vec<CombatRuntimeSelectionHistoryEntry>,
    pub(crate) next_segment_seed: Option<u64>,
}
impl CombatRuntime {
    pub(crate) fn checkpoint(&self) -> Result<CombatRuntimeCheckpoint, CombatRuntimeError> {
        if self.execution_frames.len() != self.resolution_frames.len() {
            return Err(CombatRuntimeError::InvalidInput);
        }
        for (i, (e, r)) in self
            .execution_frames
            .iter()
            .zip(&self.resolution_frames)
            .enumerate()
        {
            if e.tick != (i as u32 + 1) || r.tick != e.tick {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        if let Some(p) = &self.paused {
            if self.resolution_frames.last().map(|f| f.tick) != Some(p.tick)
                || p.evaluation.candidate.is_none()
            {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        derive_segment_seed(
            self.context.effective_seed,
            self.context.namespace,
            &self.context.provenance.simulation_version,
            &self.context.provenance.manifest_fingerprint,
            self.segment_index,
            &self.selection_history,
        )?;
        Ok(CombatRuntimeCheckpoint {
            schema_version: COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION,
            simulation_version: self.context.provenance.simulation_version.clone(),
            manifest_fingerprint: self.context.provenance.manifest_fingerprint.clone(),
            effective_seed: self.context.effective_seed,
            namespace: self.context.namespace,
            request: self.request.clone(),
            execution_frames: self.execution_frames.clone(),
            resolution_frames: self.resolution_frames.clone(),
            opportunities: self.opportunities.clone(),
            paused: self.paused.clone(),
            segment_index: self.segment_index,
            selection_history: self.selection_history.clone(),
            next_segment_seed: self.next_segment_seed,
        })
    }
    pub(crate) fn restore(checkpoint: CombatRuntimeCheckpoint) -> Result<Self, CombatRuntimeError> {
        if checkpoint.schema_version != COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION
            || checkpoint.execution_frames.len() != checkpoint.resolution_frames.len()
            || checkpoint.request.execution.ticks == 0
            || checkpoint.execution_frames.len() as u32 > checkpoint.request.execution.ticks
            || checkpoint.request.execution.ticks
                > checkpoint.request.execution.input.config.max_ticks
        {
            return Err(CombatRuntimeError::InvalidInput);
        }
        for (i, (execution, resolution)) in checkpoint
            .execution_frames
            .iter()
            .zip(&checkpoint.resolution_frames)
            .enumerate()
        {
            let expected_tick = i as u32 + 1;
            if execution.tick != expected_tick || resolution.tick != expected_tick {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        if let Some(pause) = &checkpoint.paused {
            if checkpoint.resolution_frames.last().map(|f| f.tick) != Some(pause.tick)
                || pause.evaluation.candidate.is_none()
            {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        let mut runtime = Self::new(checkpoint.request.clone())?;
        if checkpoint.simulation_version != runtime.context.provenance.simulation_version
            || checkpoint.manifest_fingerprint != runtime.context.provenance.manifest_fingerprint
            || checkpoint.effective_seed != runtime.context.effective_seed
            || checkpoint.namespace != runtime.context.namespace
        {
            return Err(CombatRuntimeError::InvalidInput);
        }
        let derived_seed = derive_segment_seed(
            runtime.context.effective_seed,
            runtime.context.namespace,
            &runtime.context.provenance.simulation_version,
            &runtime.context.provenance.manifest_fingerprint,
            checkpoint.segment_index,
            &checkpoint.selection_history,
        )?;
        if checkpoint.selection_history.len() as u32 != checkpoint.segment_index {
            return Err(CombatRuntimeError::InvalidInput);
        }
        if (checkpoint.segment_index == 0 && checkpoint.next_segment_seed.is_some())
            || (checkpoint.segment_index > 0 && checkpoint.next_segment_seed != Some(derived_seed))
        {
            return Err(CombatRuntimeError::InvalidInput);
        }
        if let Some(opportunities) = &checkpoint.opportunities {
            opportunities.catalog.validate()?;
            opportunities.context.budget.validate()?;
        }
        if let Some(pause) = &checkpoint.paused {
            let opportunities = checkpoint
                .opportunities
                .as_ref()
                .ok_or(CombatRuntimeError::InvalidInput)?;
            let candidate = pause
                .evaluation
                .candidate
                .as_ref()
                .ok_or(CombatRuntimeError::InvalidInput)?;
            if opportunities.context.current_tick != pause.tick
                || !opportunities
                    .context
                    .presented_instance_ids
                    .contains(&candidate.instance_id)
                || pause.evaluation_fingerprint != pause.evaluation.fingerprint()?
            {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        for i in 0..checkpoint.execution_frames.len() {
            let Some(frame) = runtime.advance_tick()? else {
                return Err(CombatRuntimeError::InvalidInput);
            };
            if frame.execution != checkpoint.execution_frames[i]
                || frame.resolution != checkpoint.resolution_frames[i]
            {
                return Err(CombatRuntimeError::InvalidInput);
            }
        }
        runtime.opportunities = checkpoint.opportunities;
        runtime.paused = checkpoint.paused;
        runtime.segment_index = checkpoint.segment_index;
        runtime.selection_history = checkpoint.selection_history;
        runtime.next_segment_seed = checkpoint.next_segment_seed;
        Ok(runtime)
    }
}
