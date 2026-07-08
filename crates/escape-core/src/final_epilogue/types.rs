#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FinalResult {
    BattleLoss,
    CorruptedVictory,
    TrueRouteVictory,
    MumyeongUnsavedVictory,
    MeaningfulVictory,
    IncompleteVictory,
    BasicVictory,
}

impl FinalResult {
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::BattleLoss => "battle_loss",
            Self::CorruptedVictory => "corrupted_victory",
            Self::TrueRouteVictory => "true_route_victory",
            Self::MumyeongUnsavedVictory => "mumyeong_unsaved_victory",
            Self::MeaningfulVictory => "meaningful_victory",
            Self::IncompleteVictory => "incomplete_victory",
            Self::BasicVictory => "basic_victory",
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::BattleLoss => "패배 결산",
            Self::CorruptedVictory => "침식 승리 결산",
            Self::TrueRouteVictory => "계산식 밖의 승리",
            Self::MumyeongUnsavedVictory => "무명 비구원 승리",
            Self::MeaningfulVictory => "의미 있는 승리",
            Self::IncompleteVictory => "불완전한 승리",
            Self::BasicVictory => "기본 승리",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MainEndingType {
    BattleLoss,
    Returnee,
    MurimOutsider,
    CheongryuDivineSword,
    WhitePathPrison,
    BlackNightGentleman,
    DebtorOfAllUnderHeaven,
}

impl MainEndingType {
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::BattleLoss => "battle_loss",
            Self::Returnee => "returnee",
            Self::MurimOutsider => "murim_outsider",
            Self::CheongryuDivineSword => "cheongryu_divine_sword",
            Self::WhitePathPrison => "white_path_prison",
            Self::BlackNightGentleman => "black_night_gentleman",
            Self::DebtorOfAllUnderHeaven => "debtor_of_all_under_heaven",
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::BattleLoss => "패배 결산",
            Self::Returnee => "귀환자",
            Self::MurimOutsider => "무림 외지인",
            Self::CheongryuDivineSword => "청류 신검",
            Self::WhitePathPrison => "백도의 굴레",
            Self::BlackNightGentleman => "흑야의 협객",
            Self::DebtorOfAllUnderHeaven => "천하의 채무자",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct CardCandidate {
    pub(super) id: &'static str,
    pub(super) variant: &'static str,
    pub(super) group: &'static str,
    pub(super) consumed_seeds: Vec<String>,
    pub(super) body: &'static str,
}

#[derive(Clone, Debug)]
pub(super) struct SuppressedCard {
    pub(super) id: &'static str,
    pub(super) suppressed_by: &'static str,
    pub(super) consumed_seeds: Vec<String>,
}
