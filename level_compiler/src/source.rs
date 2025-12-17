#[derive(Debug)]
pub struct LevelSource {
    pub header: LevelHeader,
    pub layers: Vec<LayerSource>,
    pub entities: Vec<EntitySource>,
    pub triggers: Vec<TriggerSource>,
}

#[derive(Debug)]
pub struct LevelHeader {
    pub version: u32,
    pub name: String,
    pub author: String,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub gravity: f32,
    pub background: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LayerSource {
    pub name: String,
    pub collision: bool,
    pub rows: Vec<String>,
}

#[derive(Debug)]
pub struct EntitySource {
    pub x: i32,
    pub y: i32,
    pub gravity_multiplier: f32,
    pub kind: EntityKindSource,
}

#[derive(Debug)]
pub enum EntityKindSource {
    PlayerStart,
    Enemy {
        enemy_kind: String,
        patrol_min: i32,
        patrol_max: i32,
    },
    Pickup {
        pickup_kind: String,
        value: i32,
    },
    MovingPlatform {
        platform_kind: String, // "horizontal" | "vertical"
        size: i32,             // tiles (width if horizontal, height if vertical)
        speed: i32,            // small int for now
        min: i32,              // bound in tiles (x or y depending on kind)
        max: i32,              // bound in tiles (x or y depending on kind)
    },
}

#[derive(Debug)]
pub struct TriggerSource {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub kind: TriggerKindSource,
}

#[derive(Debug)]
pub enum TriggerKindSource {
    LevelExit { target: String },
    Message { text_id: String },
}
