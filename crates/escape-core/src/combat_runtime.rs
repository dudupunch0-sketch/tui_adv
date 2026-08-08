use crate::combat_execution::{
    assemble_result, build_execution_log, prepare_context, stable_fingerprint,
    CombatExecutionContext,
};
use crate::combat_resolution::{
    CombatResolutionError, CombatResolutionFrame, CombatResolutionRequest, CombatResolutionResult,
    CombatResolutionState, CombatResolutionStepper,
};
use crate::{
    CombatExecutionError, CombatExecutionResult, CombatSimulation, CombatSimulationError,
    CombatTickFrame,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CombatRuntimeFrame {
    pub(crate) execution: CombatTickFrame,
    pub(crate) resolution: CombatResolutionFrame,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CombatRuntimeError {
    Execution(CombatExecutionError),
    Simulation(CombatSimulationError),
    Resolution(CombatResolutionError),
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
        })
    }

    pub(crate) fn advance_tick(
        &mut self,
    ) -> Result<Option<CombatRuntimeFrame>, CombatRuntimeError> {
        if self.execution_frames.len() as u32 >= self.request.execution.ticks {
            return Ok(None);
        }
        let execution = self.simulation.advance_tick()?;
        let resolution = self.stepper.step(&execution)?;
        self.execution_frames.push(execution.clone());
        self.resolution_frames.push(resolution.clone());
        Ok(Some(CombatRuntimeFrame {
            execution,
            resolution,
        }))
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
}
