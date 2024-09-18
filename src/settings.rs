use asr::settings::gui::Title;
use asr::settings::Gui;

// #[derive(Gui)]
// enum Category {
//     /// Any%
//     SingleStory,
//     /// Glitchless
//     #[default]
//     TrueEnding,
// }

#[derive(Gui)]
pub struct Settings {
    run_start: Title,
    #[default = true]
    /// Automatic Start on New Game
    pub start: bool,

    // Split on Missiles, Super Missiles, and Power Bombs
    ammo_pickups: Title,

    /// Split on the first Missile pickup
    pub first_missile: bool,
    /// Split on each Missile upgrade
    pub all_missiles: bool,
    /// Split on specific Missile Pack locations
    pub specific_missiles: bool,
    /// Split on Crateria Missile Pack locations
    pub crateria_missiles: bool,
    /// Split on picking up the Missile Pack located at the bottom left of the West Ocean
    pub ocean_bottom_missiles: bool,
    /// Split on picking up the Missile Pack located in the ceiling tile in West Ocean
    pub ocean_top_missiles: bool,
    /// Split on picking up the Missile Pack located in the Morphball maze section of West Ocean
    pub ocean_middle_missiles: bool,
    /// Split on picking up the Missile Pack in The Moat, also known as The Lake
    pub moat_missiles: bool,
    /// Split on picking up the Missile Pack in the Pit Room
    pub old_tourian_missiles: bool,
    /// Split on picking up the right side Missile Pack at the end of Gauntlet(Green Pirates Shaft)
    pub gauntlet_right_missiles: bool,
    /// Split on picking up the left side Missile Pack at the end of Gauntlet(Green Pirates Shaft)
    pub gauntlet_left_missiles: bool,
    /// Split on picking up the Missile Pack located in The Final Missile
    pub dental_plan: bool,
    /// Split on Brinstar Missile Pack locations
    pub brinstar_missiles: bool,
    /// Split on picking up the Missile Pack located below the crumble bridge in the Early Supers Room
    pub early_super_bridge_missiles: bool,
    /// Split on picking up the first Missile Pack behind the Brinstar Reserve Tank
    pub green_brinstar_reserve_missiles: bool,
    /// Split on picking up the second Missile Pack behind the Brinstar Reserve Tank Room
    pub green_brinstar_extra_reserve_missiles: bool,
    /// Split on picking up the Missile Pack located left of center in Big Pink
    pub big_pink_top_missiles: bool,
    /// Split on picking up the Missile Pack located at the bottom left of Big Pink
    pub charge_missiles: bool,
    /// Split on picking up the Missile Pack in Green Hill Zone
    pub green_hills_missiles: bool,
    /// Split on picking up the Missile Pack in the Blue Brinstar Energy Tank Room
    pub blue_brinstar_e_tank_missiles: bool,
    /// Split on picking up the first Missile Pack of the game(First Missile Room)
    pub alpha_missiles: bool,
    /// Split on picking up the Missile Pack located on the pedestal in Billy Mays' Room
    pub billy_mays_missiles: bool,
    /// Split on picking up the Missile Pack located in the floor of Billy Mays' Room
    pub but_wait_theres_more_missiles: bool,
    /// Split on picking up the Missile Pack in the Alpha Power Bombs Room
    pub red_brinstar_missiles: bool,
    /// Split on picking up the Missile Pack in the Warehouse Kihunter Room
    pub warehouse_missiles: bool,
    /// Split on Norfair Missile Pack locations
    pub norfair_missiles: bool,
    /// Split on picking up the Missile Pack in Cathedral
    pub cathedral_missiles: bool,
    /// Split on picking up the Missile Pack in Crumble Shaft
    pub crumble_shaft_missiles: bool,
    /// Split on picking up the Missile Pack in Crocomire Escape
    pub crocomire_escape_missiles: bool,
    /// Split on picking up the Missile Pack in the Hi Jump Energy Tank Room
    pub hi_jump_missiles: bool,
    /// Split on picking up the Missile Pack in the Post Crocomire Missile Room, also known as Cosine Room
    pub post_crocomire_missiles: bool,
    /// Split on picking up the Missile Pack in the Post Crocomire Jump Room
    pub grapple_missiles: bool,
    /// Split on picking up the Missile Pack in the Norfair Reserve Tank Room
    pub norfair_reserve_missiles: bool,
    /// Split on picking up the Missile Pack in the Green Bubbles Missile Room
    pub green_bubbles_missiles: bool,
    /// Split on picking up the Missile Pack in Bubble Mountain
    pub bubble_mountain_missiles: bool,
    /// Split on picking up the Missile Pack in Speed Booster Hall
    pub speed_boost_missiles: bool,
    /// Split on picking up the Wave Missile Pack in Double Chamber
    pub wave_missiles: bool,
    /// Split on picking up the Missile Pack in the Golden Torizo's Room
    pub gold_torizo_missiles: bool,
    /// Split on picking up the Missile Pack in the Mickey Mouse Room
    pub mickey_mouse_missiles: bool,
    /// Split on picking up the Missile Pack in the Lower Norfair Springball Maze Room
    pub lower_norfair_spring_maze_missiles: bool,
    /// Split on picking up the Missile Pack in the The Musketeers' Room
    pub three_musketeers_missiles: bool,
    /// Split on Wrecked Ship Missile Pack locations
    pub wrecked_ship_missiles: bool,
    /// Split on picking up the Missile Pack in Wrecked Ship Main Shaft
    pub wrecked_ship_main_shaft_missiles: bool,
    /// Split on picking up the Missile Pack in Bowling Alley
    pub bowling_missiles: bool,
    /// Split on picking up the Missile Pack in the Wrecked Ship East Missile Room
    pub attic_missiles: bool,
    /// Split on Maridia Missile Pack locations
    pub maridia_missiles: bool,
    /// Split on picking up the Missile Pack in Main Street
    pub main_street_missiles: bool,
    /// Split on picking up the Missile Pack in the Mama Turtle Room
    pub mama_turtle_missiles: bool,
    /// Split on picking up the Missile Pack in Watering Hole
    pub watering_hole_missiles: bool,
    /// Split on picking up the Missile Pack in the Pseudo Plasma Spark Room
    pub beach_missiles: bool,
    /// Split on picking up the Missile Pack in West Sand Hole
    pub left_sand_pit_missiles: bool,
    /// Split on picking up the Missile Pack in East Sand Hole
    pub right_sand_pit_missiles: bool,
    /// Split on picking up the Missile Pack in Aqueduct
    pub aqueduct_missiles: bool,
    /// Split on picking up the Missile Pack in The Precious Room
    pub pre_draygon_missiles: bool,
    /// Split on the first Super Missile pickup
    pub first_super: bool,
    /// Split on each Super Missile upgrade
    pub all_supers: bool,
    /// Split on specific Super Missile Pack locations
    pub specific_supers: bool,
    /// Split on picking up the Super Missile Pack in the Crateria Super Room
    pub climb_supers: bool,
    /// Split on picking up the Super Missile Pack in the Spore Spawn Super Room (NOTE: SSTRA splits when the dialogue box disappears, not on touch. Use Spore Spawn RTA Finish for SSTRA runs.)
    pub spore_spawn_supers: bool,
    /// Split on picking up the Super Missile Pack in the Early Supers Room
    pub early_supers: bool,
    /// Split on picking up the Super Missile Pack in the Etacoon Super Room
    pub etacoon_supers: bool,
    /// Split on picking up the Super Missile Pack in the Golden Torizo's Room
    pub gold_torizo_supers: bool,
    /// Split on picking up the Super Missile Pack in the Wrecked Ship West Super Room
    pub wrecked_ship_left_supers: bool,
    /// Split on picking up the Super Missile Pack in the Wrecked Ship East Super Room
    pub wrecked_ship_right_supers: bool,
    /// Split on picking up the Super Missile Pack in Main Street
    pub crab_supers: bool,
    /// Split on picking up the Super Missile Pack in Watering Hole
    pub watering_hole_supers: bool,
    /// Split on picking up the Super Missile Pack in Aqueduct
    pub aqueduct_supers: bool,
    /// Split on the first Power Bomb pickup
    pub first_power_bomb: bool,
    /// Split on each Power Bomb upgrade
    pub all_power_bombs: bool,
    /// Split on specific Power Bomb Pack locations
    pub specific_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Crateria Power Bomb Room
    pub landing_site_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Etacoon Room section of Green Brinstar Main Shaft
    pub etacoon_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Pink Brinstar Power Bomb Room
    pub pink_brinstar_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Morph Ball Room
    pub blue_brinstar_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Alpha Power Bomb Room
    pub alpha_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Beta Power Bomb Room
    pub beta_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Post Crocomire Power Bomb Room
    pub crocomire_bombs: bool,
    /// Split on picking up the Power Bomb Pack in the Lower Norfair Escape Power Bomb Room
    pub lower_norfair_escape_bombs: bool,
    /// Split on picking up the Power Bomb Pack in Wasteland
    pub shame_bombs: bool,
    /// Split on picking up the Power Bomb Pack in East Sand Hall
    pub right_sand_pit_bombs: bool,

    // Split on Varia and Gravity pickups
    pub suit_upgrades: Title,
    /// Split on picking up the Varia Suit
    pub varia_suit: bool,
    /// Split on picking up the Gravity Suit
    pub grav_suit: bool,

    // Split on beam upgrades
    pub beam_upgrades: Title,
    /// Split on picking up the Charge Beam
    pub charge_beam: bool,
    /// Split on picking up the Spazer
    pub spazer: bool,
    /// Split on picking up the Wave Beam
    pub wave: bool,
    /// Split on picking up the Ice Beam
    pub ice: bool,
    /// Split on picking up the Plasma Beam
    pub plasma: bool,

    // Split on boot upgrades
    boot_upgrades: Title,
    /// Split on picking up the Hi-Jump Boots
    pub hi_jump: bool,
    /// Split on picking up Space Jump
    pub space_jump: bool,
    /// Split on picking up the Speed Booster
    pub speed_booster: bool,

    // Split on Energy Tanks and Reserve Tanks
    energy_upgrades: Title,
    /// Split on picking up the first Energy Tank
    pub first_e_tank: bool,
    /// Split on picking up each Energy Tank
    pub all_e_tanks: bool,
    /// Split on specific Energy Tank locations
    pub specific_e_tanks: bool,
    /// Split on picking up the Energy Tank in the Gauntlet Energy Tank Room
    pub gauntlet_e_tank: bool,
    /// Split on picking up the Energy Tank in the Terminator Room
    pub terminator_e_tank: bool,
    /// Split on picking up the Energy Tank in the Blue Brinstar Energy Tank Room
    pub ceiling_e_tank: bool,
    /// Split on picking up the Energy Tank in the Etacoon Energy Tank Room
    pub etecoons_e_tank: bool,
    /// Split on picking up the Energy Tank in Waterway
    pub waterway_e_tank: bool,
    /// Split on picking up the Energy Tank in the Hopper Energy Tank Room
    pub wave_gate_e_tank: bool,
    /// Split on picking up the Kraid Energy Tank in the Warehouse Energy Tank Room
    pub kraid_e_tank: bool,
    /// Split on picking up the Energy Tank in Crocomire's Room
    pub crocomire_e_tank: bool,
    /// Split on picking up the Energy Tank in the Hi Jump Energy Tank Room
    pub hi_jump_e_tank: bool,
    /// Split on picking up the Energy Tank in the Ridley Tank Room
    pub ridley_e_tank: bool,
    /// Split on picking up the Energy Tank in the Lower Norfair Fireflea Room
    pub fireflea_e_tank: bool,
    /// Split on picking up the Energy Tank in the Wrecked Ship Energy Tank Room
    pub wrecked_ship_e_tank: bool,
    /// Split on picking up the Energy Tank in the Mama Turtle Room
    pub tatori_e_tank: bool,
    /// Split on picking up the Energy Tank in the Botwoon Energy Tank Room
    pub botwoon_e_tank: bool,
    /// Split on picking up each Reserve Tank
    pub reserve_tanks: bool,
    /// Split on specific Reserve Tank locations
    pub specific_r_tanks: bool,
    /// Split on picking up the Reserve Tank in the Brinstar Reserve Tank Room
    pub brinstar_reserve: bool,
    /// Split on picking up the Reserve Tank in the Norfair Reserve Tank Room
    pub norfair_reserve: bool,
    /// Split on picking up the Reserve Tank in Bowling Alley
    pub wrecked_ship_reserve: bool,
    /// Split on picking up the Reserve Tank in West Sand Hole
    pub maridia_reserve: bool,

    /// Split on the miscellaneous upgrades
    misc_upgrades: Title,
    /// Split on picking up the Morphing Ball
    pub morph_ball: bool,
    /// Split on picking up the Bomb
    pub bomb: bool,
    /// Split on picking up the Spring Ball
    pub spring_ball: bool,
    /// Split on picking up the Screw Attack
    pub screw_attack: bool,
    /// Split on picking up the Grapple Beam
    pub grapple: bool,
    /// Split on picking up the X-Ray Scope
    pub xray: bool,

    /// Split on transitions between areas
    pub area_transitions: bool,
    /// Split on entering miniboss rooms (except Bomb Torizo)
    pub mini_boss_rooms: bool,
    /// Split on entering major boss rooms
    pub boss_rooms: bool,
    /// Split on elevator transitions between areas (except Statue Room to Tourian)
    pub elevator_transitions: bool,
    /// Split on leaving Ceres Station
    pub ceres_escape: bool,
    /// Split on entering the Wrecked Ship Entrance from the lower door of West Ocean
    pub wrecked_ship_entrance: bool,
    /// Split on entering Red Tower from Noob Bridge
    pub red_tower_middle_entrance: bool,
    /// Split on entering Red Tower from Skree Boost room
    pub red_tower_bottom_entrance: bool,
    /// Split on entering Kraid's Lair
    pub kraids_lair: bool,
    /// Split on entering Rising Tide from Cathedral
    pub rising_tide_entrance: bool,
    /// Split on exiting Attic
    pub attic_exit: bool,
    /// Split on blowing up the tube to enter Maridia
    pub tube_broken: bool,
    /// Split on exiting West Cacattack Alley
    pub cac_exit: bool,
    /// Split on entering Toilet Bowl from either direction
    pub toilet: bool,
    /// Split on entering Kronic Boost room
    pub kronic_boost: bool,
    /// Split on the elevator down to Lower Norfair
    pub lower_norfair_entrance: bool,
    /// Split on entering Worst Room in the Game
    pub writg: bool,
    /// Split on entering Red Kihunter Shaft from either Amphitheatre or Wastelands (NOTE: will split twice)
    pub red_ki_shaft: bool,
    /// Split on entering Metal Pirates Room from Wasteland
    pub metal_pirates: bool,
    /// Split on entering Lower Norfair Springball Maze Room
    pub lower_norfair_spring_maze: bool,
    /// Split on moving from the Three Musketeers' Room to the Single Chamber
    pub lower_norfair_exit: bool,
    /// Split on entering the Statues Room with all four major bosses defeated
    pub golden_four: bool,
    /// Split on the elevator down to Tourian
    pub tourian_entrance: bool,
    /// Split on exiting each of the Metroid rooms in Tourian
    pub metroids: bool,
    /// Split on moving from the Dust Torizo Room to the Big Boy Room
    pub baby_metroid_room: bool,
    /// Split on moving from Tourian Escape Room 4 to The Climb
    pub escape_climb: bool,

    // Split on defeating minibosses
    mini_bosses: Title,
    /// Split on starting the Ceres Escape
    pub ceres_ridley: bool,
    /// Split on Bomb Torizo's drops appearing
    pub bomb_torizo: bool,
    /// Split on the last hit to Spore Spawn
    pub spore_spawn: bool,
    /// Split on Crocomire's drops appearing
    pub crocomire: bool,
    /// Split on Botwoon's vertical column being fully destroyed
    pub botwoon: bool,
    /// Split on Golden Torizo's drops appearing
    pub golden_torizo: bool,

    // Split on defeating major bosses
    bosses: Title,
    /// Split shortly after Kraid's drops appear
    pub kraid: bool,
    /// Split on Phantoon's drops appearing
    pub phantoon: bool,
    /// Split on Draygon's drops appearing
    pub draygon: bool,
    /// Split on Ridley's drops appearing
    pub ridley: bool,
    /// Split on Mother Brain's head hitting the ground at the end of the first phase
    pub mb1: bool,
    /// Split on the Baby Metroid detaching from Mother Brain's head
    pub mb2: bool,
    /// Split on the start of the Zebes Escape
    pub mb3: bool,

    // Ending Splits
    ending_splits: Title,
    /// Split on facing forward at the end of Zebes Escape
    pub rta_finish: bool,
    /// Split on In-Game Time finalizing, when the end cutscene starts
    pub igt_finish: bool,
    /// Split on the end of a Spore Spawn RTA run, when the text box clears after collecting the Super Missiles
    pub spore_spawn_rta_finish: bool,
    /// Split on the end of a 100 Missile RTA run, when the text box clears after collecting the hundredth missile
    pub hundred_missile_rta_finish: bool,
}
