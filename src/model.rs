mod stats;

use serde::{Deserialize, Serialize};
use stats::{Attributes, Equipment, Interactions, Resistances};

#[derive(Serialize, Deserialize, Default, Debug, Hash)]
pub struct Unit {
    attributes: Attributes,
    equipment: Equipment,
    resistances: Resistances,
    interactions: Interactions,
}
