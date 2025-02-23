use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseWrapper<T: std::fmt::Debug + Clone> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CooldownSchema {
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub started_at: String,
    pub expiration: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapContentSchema {
    #[serde(rename = "type")]
    pub typ: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapSchema {
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
    pub content: Option<MapContentSchema>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InventorySlot {
    pub slot: i32,
    pub code: String,
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterSchema {
    // Stats //
    pub name: String,
    pub account: String,
    pub skin: String,
    pub level: i32,
    pub xp: i32,
    pub max_xp: i32,
    pub gold: i32,
    // pub speed: i32, // Not implemented
    pub hp: i32,
    pub max_hp: i32,
    pub haste: i32,
    pub critical_strike: i32,
    pub wisdom: i32,
    pub prospecting: i32,
    pub attack_fire: i32,
    pub attack_earth: i32,
    pub attack_water: i32,
    pub attack_air: i32,
    pub dmg: i32,
    pub dmg_fire: i32,
    pub dmg_earth: i32,
    pub dmg_water: i32,
    pub dmg_air: i32,
    pub res_fire: i32,
    pub res_earth: i32,
    pub res_water: i32,
    pub res_air: i32,
    pub x: i32,
    pub y: i32,
    pub cooldown: i32,
    pub cooldown_expiration: String,
    pub weapon_slot: String,
    pub rune_slot: String,
    pub shield_slot: String,
    pub helmet_slot: String,
    pub body_armor_slot: String,
    pub leg_armor_slot: String,
    pub boots_slot: String,
    pub ring1_slot: String,
    pub ring2_slot: String,
    pub amulet_slot: String,
    pub artifact1_slot: String,
    pub artifact2_slot: String,
    pub artifact3_slot: String,
    pub utility1_slot: String,
    pub utility1_slot_quantity: i32,
    pub utility2_slot: String,
    pub utility2_slot_quantity: i32,
    pub bag_slot: String,
    pub task: String,
    pub task_type: String,
    pub task_progress: i32,
    pub task_total: i32,
    pub inventory_max_items: i32,
    pub inventory: Vec<InventorySlot>,
    // Skills //
    // Mining //
    pub mining_level: i32,
    pub mining_xp: i32,
    pub mining_max_xp: i32,
    // Woodcutting //
    pub woodcutting_level: i32,
    pub woodcutting_xp: i32,
    pub woodcutting_max_xp: i32,
    // Fishing //
    pub fishing_level: i32,
    pub fishing_xp: i32,
    pub fishing_max_xp: i32,
    // Weaponcrafting //
    pub weaponcrafting_level: i32,
    pub weaponcrafting_xp: i32,
    pub weaponcrafting_max_xp: i32,
    // Gearcrafting //
    pub gearcrafting_level: i32,
    pub gearcrafting_xp: i32,
    pub gearcrafting_max_xp: i32,
    // Jewelrycrafting //
    pub jewelrycrafting_level: i32,
    pub jewelrycrafting_xp: i32,
    pub jewelrycrafting_max_xp: i32,
    // Cooking //
    pub cooking_level: i32,
    pub cooking_xp: i32,
    pub cooking_max_xp: i32,
    // Alchemy //
    pub alchemy_level: i32,
    pub alchemy_xp: i32,
    pub alchemy_max_xp: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterMovementDataSchema {
    pub cooldown: CooldownSchema,
    pub destination: MapSchema,
    pub character: CharacterSchema,
}
