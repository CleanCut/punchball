pub mod arena;
pub mod event;
pub mod gamepad;
pub mod player;
pub mod points;
pub mod setup;

mod prelude {
    /// An alias to show that we're dealing with a player id
    pub type PlayerID = usize;
    /// Radius of the arena circle -- based off of circle radius in the PNG
    pub const ARENA_RADIUS: f32 = 384.0;
    /// The radius of a player sprite, used for collision detection
    pub const COLLISION_RADIUS: f32 = 32.0;
    /// How far a joystick has to move before it's no longer considered neutral
    pub const DEAD_ZONE_THRESHOLD: f32 = 0.2;
    /// How quickly movement should slow to a stop when joystick is neutral
    pub const DRAG: f32 = 0.8;
    /// Z depth for the arena
    pub const LAYER_ARENA: f32 = 0.0;
    /// Z depth for gloves
    pub const LAYER_GLOVE: f32 = 0.2;
    /// Z depth for players (positive Z is towards the viewer, negative Z is into the screen)
    pub const LAYER_PLAYER: f32 = 0.1;
    /// Z depth for points
    pub const LAYER_POINTS: f32 = 0.3;
    /// Maximum velocity a player can move by itself (can be exceeded when punched)
    pub const MAX_VELOCITY: f32 = 6.0;
    /// How fast a player accelerates
    pub const MOVE_SPEED: f32 = 25.0;
    /// How long after being the last to touch someone you will get a point if they leave the arena
    pub const POINT_TOUCH_DURATION: f32 = 5.0;
    /// How far away from the center of the player that the boxing glove rests
    pub const PUNCH_BASE: f32 = 40.0;
    /// Where the boxing glove rests relative to the player as it's parent as an array (convert it to Vec3)
    pub const PUNCH_BASE_ARR3: [f32; 3] = [PUNCH_BASE, 0.0, 0.1];
    /// How long it takes to draw your boxing glove back after a punch
    pub const PUNCH_DRAWBACK_DURATION: f32 = 0.50;
    /// Boxing glove location relative to player parent when fully extended
    pub const PUNCH_EXTENDED_ARR3: [f32; 3] = [PUNCH_BASE + PUNCH_LENGTH, 0.0, LAYER_GLOVE];
    /// How far the boxing glove punches outward
    pub const PUNCH_LENGTH: f32 = 50.0;
    /// How much the punched player gets pushed back
    pub const PUNCH_PUSHBACK_OTHER: f32 = 3.0;
    /// How much the player doing the punching gets pushed back
    pub const PUNCH_PUSHBACK_SELF: f32 = 1.0;
    /// How long it takes to shrink and respawn once you've died
    pub const RESPAWN_DURATION: f32 = 1.5;
    /// Where players 0, 1, 2, and 3 spawn on the screen.
    pub const STARTING_LOCATIONS: [[f32; 3]; 4] = [
        [-100.0, 100.0, LAYER_PLAYER],
        [100.0, 100.0, LAYER_PLAYER],
        [100.0, -100.0, LAYER_PLAYER],
        [-100.0, -100.0, LAYER_PLAYER],
    ];
    /// How fast a player turns
    pub const TURN_SPEED: f32 = 6.0;
    /// How many points it takes to win
    pub const WIN_POINTS: usize = 10;
}
