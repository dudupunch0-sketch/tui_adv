pub(crate) const RESOURCE_HEALTH: &str = "health";
pub(crate) const RESOURCE_SANITY: &str = "sanity";
pub(crate) const RESOURCE_BATTERY: &str = "battery";
pub(crate) const RESOURCE_HUNGER: &str = "hunger";
pub(crate) const RESOURCE_THIRST: &str = "thirst";

pub(crate) const RESOURCE_IDS: [&str; 5] = [
    RESOURCE_HEALTH,
    RESOURCE_SANITY,
    RESOURCE_BATTERY,
    RESOURCE_HUNGER,
    RESOURCE_THIRST,
];

pub(crate) const ACTION_PREFIX_CHOICE: &str = "choice:";
pub(crate) const ACTION_PREFIX_MOVE: &str = "move:";
pub(crate) const ACTION_PREFIX_USE: &str = "use:";
pub(crate) const ACTION_PREFIX_TRAIN: &str = "train:";
