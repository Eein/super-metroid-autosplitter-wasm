#![feature(type_alias_impl_trait, const_async_blocks)]

use asr::{
    future::next_tick,
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
    Process,
};
mod settings;
use settings::Settings;
use std::collections::HashMap;

asr::async_main!(nightly);

async fn main() {
    let room_id_enum = HashMap::from([
        ("landingSite", 0x91F8),
        ("crateriaPowerBombRoom", 0x93AA),
        ("westOcean", 0x93FE),
        ("elevatorToMaridia", 0x94CC),
        ("crateriaMoat", 0x95FF),
        ("elevatorToCaterpillar", 0x962A),
        ("gauntletETankRoom", 0x965B),
        ("climb", 0x96BA),
        ("pitRoom", 0x975C),
        ("elevatorToMorphBall", 0x97B5),
        ("bombTorizo", 0x9804),
        ("terminator", 0x990D),
        ("elevatorToGreenBrinstar", 0x9938),
        ("greenPirateShaft", 0x99BD),
        ("crateriaSupersRoom", 0x99F9),
        ("theFinalMissile", 0x9A90),
        ("greenBrinstarMainShaft", 0x9AD9),
        ("sporeSpawnSuper", 0x9B5B),
        ("earlySupers", 0x9BC8),
        ("brinstarReserveRoom", 0x9C07),
        ("bigPink", 0x9D19),
        ("sporeSpawnKeyhunter", 0x9D9C),
        ("sporeSpawn", 0x9DC7),
        ("pinkBrinstarPowerBombRoom", 0x9E11),
        ("greenHills", 0x9E52),
        ("noobBridge", 0x9FBA),
        ("morphBall", 0x9E9F),
        ("blueBrinstarETankRoom", 0x9F64),
        ("etacoonETankRoom", 0xA011),
        ("etacoonSuperRoom", 0xA051),
        ("waterway", 0xA0D2),
        ("alphaMissileRoom", 0xA107),
        ("hopperETankRoom", 0xA15B),
        ("billyMays", 0xA1D8),
        ("redTower", 0xA253),
        ("xRay", 0xA2CE),
        ("caterpillar", 0xA322),
        ("betaPowerBombRoom", 0xA37C),
        ("alphaPowerBombsRoom", 0xA3AE),
        ("bat", 0xA3DD),
        ("spazer", 0xA447),
        ("warehouseETankRoom", 0xA4B1),
        ("warehouseZeela", 0xA471),
        ("warehouseKiHunters", 0xA4DA),
        ("kraidEyeDoor", 0xA56B),
        ("kraid", 0xA59F),
        ("statuesHallway", 0xA5ED),
        ("statues", 0xA66A),
        ("warehouseEntrance", 0xA6A1),
        ("varia", 0xA6E2),
        ("cathedral", 0xA788),
        ("businessCenter", 0xA7DE),
        ("iceBeam", 0xA890),
        ("crumbleShaft", 0xA8F8),
        ("crocomireSpeedway", 0xA923),
        ("crocomire", 0xA98D),
        ("hiJump", 0xA9E5),
        ("crocomireEscape", 0xAA0E),
        ("hiJumpShaft", 0xAA41),
        ("postCrocomirePowerBombRoom", 0xAADE),
        ("cosineRoom", 0xAB3B),
        ("preGrapple", 0xAB8F),
        ("grapple", 0xAC2B),
        ("norfairReserveRoom", 0xAC5A),
        ("greenBubblesRoom", 0xAC83),
        ("bubbleMountain", 0xACB3),
        ("speedBoostHall", 0xACF0),
        ("speedBooster", 0xAD1B),
        ("singleChamber", 0xAD5E), // Exit room from Lower Norfair, also on the path to Wave
        ("doubleChamber", 0xADAD),
        ("waveBeam", 0xADDE),
        ("volcano", 0xAE32),
        ("kronicBoost", 0xAE74),
        ("magdolliteTunnel", 0xAEB4),
        ("lowerNorfairElevator", 0xAF3F),
        ("risingTide", 0xAFA3),
        ("spikyAcidSnakes", 0xAFFB),
        ("acidStatue", 0xB1E5),
        ("mainHall", 0xB236), // First room in Lower Norfair
        ("goldenTorizo", 0xB283),
        ("ridley", 0xB32E),
        ("lowerNorfairFarming", 0xB37A),
        ("mickeyMouse", 0xB40A),
        ("pillars", 0xB457),
        ("writg", 0xB4AD),
        ("amphitheatre", 0xB4E5),
        ("lowerNorfairSpringMaze", 0xB510),
        ("lowerNorfairEscapePowerBombRoom", 0xB55A),
        ("redKiShaft", 0xB585),
        ("wasteland", 0xB5D5),
        ("metalPirates", 0xB62B),
        ("threeMusketeers", 0xB656),
        ("ridleyETankRoom", 0xB698),
        ("screwAttack", 0xB6C1),
        ("lowerNorfairFireflea", 0xB6EE),
        ("bowling", 0xC98E),
        ("wreckedShipEntrance", 0xCA08),
        ("attic", 0xCA52),
        ("atticWorkerRobotRoom", 0xCAAE),
        ("wreckedShipMainShaft", 0xCAF6),
        ("wreckedShipETankRoom", 0xCC27),
        ("basement", 0xCC6F), // Basement of Wrecked Ship
        ("phantoon", 0xCD13),
        ("wreckedShipLeftSuperRoom", 0xCDA8),
        ("wreckedShipRightSuperRoom", 0xCDF1),
        ("gravity", 0xCE40),
        ("glassTunnel", 0xCEFB),
        ("mainStreet", 0xCFC9),
        ("mamaTurtle", 0xD055),
        ("wateringHole", 0xD13B),
        ("beach", 0xD1DD),
        ("plasmaBeam", 0xD2AA),
        ("maridiaElevator", 0xD30B),
        ("plasmaSpark", 0xD340),
        ("toiletBowl", 0xD408),
        ("oasis", 0xD48E),
        ("leftSandPit", 0xD4EF),
        ("rightSandPit", 0xD51E),
        ("aqueduct", 0xD5A7),
        ("butterflyRoom", 0xD5EC),
        ("botwoonHallway", 0xD617),
        ("springBall", 0xD6D0),
        ("precious", 0xD78F),
        ("botwoonETankRoom", 0xD7E4),
        ("botwoon", 0xD95E),
        ("spaceJump", 0xD9AA),
        ("westCactusAlley", 0xD9FE),
        ("draygon", 0xDA60),
        ("tourianElevator", 0xDAAE),
        ("metroidOne", 0xDAE1),
        ("metroidTwo", 0xDB31),
        ("metroidThree", 0xDB7D),
        ("metroidFour", 0xDBCD),
        ("dustTorizo", 0xDC65),
        ("tourianHopper", 0xDC19),
        ("tourianEyeDoor", 0xDDC4),
        ("bigBoy", 0xDCB1),
        ("motherBrain", 0xDD58),
        ("rinkaShaft", 0xDDF3),
        ("tourianEscape4", 0xDEDE),
        ("ceresElevator", 0xDF45),
        ("flatRoom", 0xE06B), // Placeholder name for the flat room in Ceres Station
        ("ceresRidley", 0xE0B5),
    ]);

    let _map_in_use_enum = HashMap::from([
        ("crateria", 0x0),
        ("brinstar", 0x1),
        ("norfair", 0x2),
        ("wreckedShip", 0x3),
        ("maridia", 0x4),
        ("tourian", 0x5),
        ("ceres", 0x6),
    ]);

    let game_state_enum = HashMap::from([
        ("normalGameplay", 0x8),
        ("doorTransition", 0xB),
        ("startOfCeresCutscene", 0x20),
        ("preEndCutscene", 0x26), // briefly at this value during the black screen transition after the ship fades out
        ("endCutscene", 0x27),
    ]);

    let unlock_flag_enum = HashMap::from([
        // First item byte
        ("variaSuit", 0x1),
        ("springBall", 0x2),
        ("morphBall", 0x4),
        ("screwAttack", 0x8),
        ("gravSuit", 0x20),
        // Second item byte
        ("hiJump", 0x1),
        ("spaceJump", 0x2),
        ("bomb", 0x10),
        ("speedBooster", 0x20),
        ("grapple", 0x40),
        ("xray", 0x80),
        // Beams
        ("wave", 0x1),
        ("ice", 0x2),
        ("spazer", 0x4),
        ("plasma", 0x8),
        // Charge
        ("chargeBeam", 0x10),
    ]);

    let mother_brain_max_hp_enum = HashMap::from([
        ("phase1", 0xBB8),  // 3000
        ("phase2", 0x4650), // 18000
        ("phase3", 0x8CA0), // 36000
    ]);

    let event_flag_enum = HashMap::from([("zebesAblaze", 0x40), ("tubeBroken", 0x8)]);

    let boss_flag_enum = HashMap::from([
        // Crateria
        ("bombTorizo", 0x4),
        // Brinstar
        ("sporeSpawn", 0x2),
        ("kraid", 0x1),
        // Norfair
        ("ridley", 0x1),
        ("crocomire", 0x2),
        ("goldenTorizo", 0x4),
        // Wrecked Ship
        ("phantoon", 0x1),
        // Maridia
        ("draygon", 0x1),
        ("botwoon", 0x2),
        // Tourian
        ("motherBrain", 0x2),
        // Ceres
        ("ceresRidley", 0x1),
    ]);

    let mut picked_up_spore_spawn_super = false;
    let mut picked_up_hundredth_missile = false;
    let _frame_rate = 60.0;

    // TODO: Set up some general state and settings.
    // let mut splits = HashSet::<String>::new();
    let mut settings = Settings::register();

    let mut room_id_watcher = Watcher::<u16>::new();
    let mut map_in_use_watcher = Watcher::<u8>::new();
    let mut game_state_watcher = Watcher::<u8>::new();
    let mut unlocked_equips_2_watcher = Watcher::<u8>::new();
    let mut unlocked_equips_watcher = Watcher::<u8>::new();
    let mut unlocked_beams_watcher = Watcher::<u8>::new();
    let mut unlocked_charge_watcher = Watcher::<u8>::new();
    let mut max_energy_watcher = Watcher::<u16>::new();
    let mut max_missiles_watcher = Watcher::<u8>::new();
    let mut max_supers_watcher = Watcher::<u8>::new();
    let mut max_power_bombs_watcher = Watcher::<u8>::new();
    let mut max_reserve_watcher = Watcher::<u16>::new();
    let mut igt_frames_watcher = Watcher::<u8>::new();
    let mut igt_seconds_watcher = Watcher::<u8>::new();
    let mut igt_minutes_watcher = Watcher::<u8>::new();
    let mut igt_hours_watcher = Watcher::<u8>::new();
    let mut player_state_watcher = Watcher::<u8>::new();
    let mut enemy_hp_watcher = Watcher::<u16>::new();
    let mut ship_ai_watcher = Watcher::<u16>::new();
    let mut mother_brain_hp_watcher = Watcher::<u16>::new();
    let mut event_flags_watcher = Watcher::<u8>::new();
    let mut crateria_bosses_watcher = Watcher::<u8>::new();
    let mut brinstar_bosses_watcher = Watcher::<u8>::new();
    let mut norfair_bosses_watcher = Watcher::<u8>::new();
    let mut wrecked_ship_bosses_watcher = Watcher::<u8>::new();
    let mut maridia_bosses_watcher = Watcher::<u8>::new();
    let mut tourian_bosses_watcher = Watcher::<u8>::new();
    let mut ceres_bosses_watcher = Watcher::<u8>::new();
    let mut crateria_items_watcher = Watcher::<u8>::new();
    let mut brinteria_items_watcher = Watcher::<u8>::new();
    let mut brinstar_items_2_watcher = Watcher::<u8>::new();
    let mut brinstar_items_3_watcher = Watcher::<u8>::new();
    let mut brinstar_items_4_watcher = Watcher::<u8>::new();
    let mut brinstar_items_5_watcher = Watcher::<u8>::new();
    let mut norfair_items_1_watcher = Watcher::<u8>::new();
    let mut norfair_items_2_watcher = Watcher::<u8>::new();
    let mut norfair_items_3_watcher = Watcher::<u8>::new();
    let mut norfair_items_4_watcher = Watcher::<u8>::new();
    let mut norfair_items_5_watcher = Watcher::<u8>::new();
    let mut wrecked_ship_items_watcher = Watcher::<u8>::new();
    let mut maridia_items_1_watcher = Watcher::<u8>::new();
    let mut maridia_items_2_watcher = Watcher::<u8>::new();
    let mut maridia_items_3_watcher = Watcher::<u8>::new();

    loop {
        let process = Process::wait_attach("bsnes.exe").await;
        let main_module_base = 0xB16D7C; // bsnes v115

        process
            .until_closes(async {
                // TODO: Load some initial information from the process.
                loop {
                    settings.update();

                    let room_id_value = match process.read(main_module_base + 0x079B as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let room_id = room_id_watcher.update(room_id_value).unwrap();

                    let game_state_value = match process.read(main_module_base + 0x0998 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let game_state = game_state_watcher.update(game_state_value).unwrap();

                    let map_in_use_value = match process.read(main_module_base + 0x079F as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let map_in_use = map_in_use_watcher.update(map_in_use_value).unwrap();

                    let unlocked_equips_2_value =
                        match process.read(main_module_base + 0x09A4 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let unlocked_equips_2 = unlocked_equips_2_watcher
                        .update(unlocked_equips_2_value)
                        .unwrap();

                    let unlocked_equips_value = match process.read(main_module_base + 0x09A5 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let unlocked_equips = unlocked_equips_watcher
                        .update(unlocked_equips_value)
                        .unwrap();

                    let unlocked_beams_value = match process.read(main_module_base + 0x09A8 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let unlocked_beams =
                        unlocked_beams_watcher.update(unlocked_beams_value).unwrap();

                    let unlocked_charge_value = match process.read(main_module_base + 0x09A9 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let unlocked_charge = unlocked_charge_watcher
                        .update(unlocked_charge_value)
                        .unwrap();

                    let max_energy_value = match process.read(main_module_base + 0x09C4 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let max_energy = max_energy_watcher.update(max_energy_value).unwrap();

                    let max_missiles_value = match process.read(main_module_base + 0x09C8 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let max_missiles = max_missiles_watcher.update(max_missiles_value).unwrap();

                    let max_supers_value = match process.read(main_module_base + 0x09CC as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let max_supers = max_supers_watcher.update(max_supers_value).unwrap();

                    let max_power_bombs_value = match process.read(main_module_base + 0x09D0 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let max_power_bombs = max_power_bombs_watcher
                        .update(max_power_bombs_value)
                        .unwrap();

                    let max_reserve_value = match process.read(main_module_base + 0x09D4 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let max_reserve = max_reserve_watcher.update(max_reserve_value).unwrap();

                    let igt_frames_value = match process.read(main_module_base + 0x09DA as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let igt_frames = igt_frames_watcher.update(igt_frames_value).unwrap();

                    let igt_seconds_value = match process.read(main_module_base + 0x09DC as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let igt_seconds = igt_seconds_watcher.update(igt_seconds_value).unwrap();

                    let igt_minutes_value = match process.read(main_module_base + 0x09DE as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let igt_minutes = igt_minutes_watcher.update(igt_minutes_value).unwrap();

                    let igt_hours_value = match process.read(main_module_base + 0x09E0 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let igt_hours = igt_hours_watcher.update(igt_hours_value).unwrap();

                    let player_state_value = match process.read(main_module_base + 0x0A28 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let player_state = player_state_watcher.update(player_state_value).unwrap();

                    let enemy_hp_value = match process.read(main_module_base + 0x0F8C as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let enemy_hp = enemy_hp_watcher.update(enemy_hp_value).unwrap();

                    let ship_ai_value = match process.read(main_module_base + 0x0FB2 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let ship_ai = ship_ai_watcher.update(ship_ai_value).unwrap();

                    let mother_brain_hp_value = match process.read(main_module_base + 0x0FCC as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let mother_brain_hp = mother_brain_hp_watcher
                        .update(mother_brain_hp_value)
                        .unwrap();

                    let event_flags_value = match process.read(main_module_base + 0x0D821 as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let event_flags = event_flags_watcher.update(event_flags_value).unwrap();

                    let crateria_bosses_value = match process.read(main_module_base + 0xD828 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let crateria_bosses = crateria_bosses_watcher
                        .update(crateria_bosses_value)
                        .unwrap();

                    let brinstar_bosses_value = match process.read(main_module_base + 0xD829 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let brinstar_bosses = brinstar_bosses_watcher
                        .update(brinstar_bosses_value)
                        .unwrap();

                    let norfair_bosses_value = match process.read(main_module_base + 0xD82A as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_bosses =
                        norfair_bosses_watcher.update(norfair_bosses_value).unwrap();

                    let wrecked_ship_bosses_value =
                        match process.read(main_module_base + 0xD82B as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let wrecked_ship_bosses = wrecked_ship_bosses_watcher
                        .update(wrecked_ship_bosses_value)
                        .unwrap();

                    let maridia_bosses_value = match process.read(main_module_base + 0xD82C as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let maridia_bosses =
                        maridia_bosses_watcher.update(maridia_bosses_value).unwrap();

                    let tourian_bosses_value = match process.read(main_module_base + 0xD82D as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let tourian_bosses =
                        tourian_bosses_watcher.update(tourian_bosses_value).unwrap();

                    let ceres_bosses_value = match process.read(main_module_base + 0xD82E as u32) {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let ceres_bosses = ceres_bosses_watcher.update(ceres_bosses_value).unwrap();

                    let crateria_items_value = match process.read(main_module_base + 0xD870 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let crateria_items =
                        crateria_items_watcher.update(crateria_items_value).unwrap();

                    let brinteria_items_value = match process.read(main_module_base + 0xD871 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let brinteria_items = brinteria_items_watcher
                        .update(brinteria_items_value)
                        .unwrap();

                    let brinstar_items_2_value =
                        match process.read(main_module_base + 0xD872 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let brinstar_items_2 = brinstar_items_2_watcher
                        .update(brinstar_items_2_value)
                        .unwrap();

                    let brinstar_items_3_value =
                        match process.read(main_module_base + 0xD873 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let brinstar_items_3 = brinstar_items_3_watcher
                        .update(brinstar_items_3_value)
                        .unwrap();

                    let brinstar_items_4_value =
                        match process.read(main_module_base + 0xD874 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let brinstar_items_4 = brinstar_items_4_watcher
                        .update(brinstar_items_4_value)
                        .unwrap();

                    let brinstar_items_5_value =
                        match process.read(main_module_base + 0xD875 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let brinstar_items_5 = brinstar_items_5_watcher
                        .update(brinstar_items_5_value)
                        .unwrap();

                    let norfair_items_1_value = match process.read(main_module_base + 0xD876 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_items_1 = norfair_items_1_watcher
                        .update(norfair_items_1_value)
                        .unwrap();

                    let norfair_items_2_value = match process.read(main_module_base + 0xD877 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_items_2 = norfair_items_2_watcher
                        .update(norfair_items_2_value)
                        .unwrap();

                    let norfair_items_3_value = match process.read(main_module_base + 0xD878 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_items_3 = norfair_items_3_watcher
                        .update(norfair_items_3_value)
                        .unwrap();

                    let norfair_items_4_value = match process.read(main_module_base + 0xD879 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_items_4 = norfair_items_4_watcher
                        .update(norfair_items_4_value)
                        .unwrap();

                    let norfair_items_5_value = match process.read(main_module_base + 0xD87A as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let norfair_items_5 = norfair_items_5_watcher
                        .update(norfair_items_5_value)
                        .unwrap();
                    let wrecked_ship_items_value =
                        match process.read(main_module_base + 0xD880 as u32) {
                            Ok(val) => Some(val),
                            Err(_e) => None,
                        };
                    let wrecked_ship_items = wrecked_ship_items_watcher
                        .update(wrecked_ship_items_value)
                        .unwrap();

                    let maridia_items_1_value = match process.read(main_module_base + 0xD881 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let maridia_items_1 = maridia_items_1_watcher
                        .update(maridia_items_1_value)
                        .unwrap();

                    let maridia_items_2_value = match process.read(main_module_base + 0xD882 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let maridia_items_2 = maridia_items_2_watcher
                        .update(maridia_items_2_value)
                        .unwrap();

                    let maridia_items_3_value = match process.read(main_module_base + 0xD883 as u32)
                    {
                        Ok(val) => Some(val),
                        Err(_e) => None,
                    };
                    let maridia_items_3 = maridia_items_3_watcher
                        .update(maridia_items_3_value)
                        .unwrap();

                    timer::set_variable_int("GameState", game_state.current as u8);
                    timer::set_variable_int("room_id", room_id.current as u8);
                    timer::set_variable_int("map_in_use", map_in_use.current as u8);
                    timer::set_variable_int("game_state", game_state.current as u8);
                    timer::set_variable_int("unlocked_equips_2", unlocked_equips_2.current as u8);
                    timer::set_variable_int("unlocked_equips", unlocked_equips.current as u8);
                    timer::set_variable_int("unlocked_beams", unlocked_beams.current as u8);
                    timer::set_variable_int("unlocked_charge", unlocked_charge.current as u8);
                    timer::set_variable_int("max_energy", max_energy.current as u8);
                    timer::set_variable_int("max_missiles", max_missiles.current as u8);
                    timer::set_variable_int("max_supers", max_supers.current as u8);
                    timer::set_variable_int("max_power_bombs", max_power_bombs.current as u8);
                    timer::set_variable_int("max_reserve", max_reserve.current as u8);
                    timer::set_variable_int("igt_frames", igt_frames.current as u8);
                    timer::set_variable_int("igt_seconds", igt_seconds.current as u8);
                    timer::set_variable_int("igt_minutes", igt_minutes.current as u8);
                    timer::set_variable_int("igt_hours", igt_hours.current as u8);
                    timer::set_variable_int("player_state", player_state.current as u8);
                    timer::set_variable_int("enemy_hp", enemy_hp.current as u16);
                    timer::set_variable_int("ship_ai", ship_ai.current as u16);
                    timer::set_variable_int("mother_brain_hp", mother_brain_hp.current as u16);
                    timer::set_variable_int("event_flags", event_flags.current as u8);
                    timer::set_variable_int("crateria_bosses", crateria_bosses.current as u8);
                    timer::set_variable_int("brinstar_bosses", brinstar_bosses.current as u8);
                    timer::set_variable_int("norfair_bosses", norfair_bosses.current as u8);
                    timer::set_variable_int(
                        "wrecked_ship_bosses",
                        wrecked_ship_bosses.current as u8,
                    );
                    timer::set_variable_int("maridia_bosses", maridia_bosses.current as u8);
                    timer::set_variable_int("tourian_bosses", tourian_bosses.current as u8);
                    timer::set_variable_int("ceres_bosses", ceres_bosses.current as u8);
                    timer::set_variable_int("crateria_items", crateria_items.current as u8);
                    timer::set_variable_int("brinteria_items", brinteria_items.current as u8);
                    timer::set_variable_int("brinstar_items_2", brinstar_items_2.current as u8);
                    timer::set_variable_int("brinstar_items_3", brinstar_items_3.current as u8);
                    timer::set_variable_int("brinstar_items_4", brinstar_items_4.current as u8);
                    timer::set_variable_int("brinstar_items_5", brinstar_items_5.current as u8);
                    timer::set_variable_int("norfair_items_1", norfair_items_1.current as u8);
                    timer::set_variable_int("norfair_items_2", norfair_items_2.current as u8);
                    timer::set_variable_int("norfair_items_3", norfair_items_3.current as u8);
                    timer::set_variable_int("norfair_items_4", norfair_items_4.current as u8);
                    timer::set_variable_int("norfair_items5", norfair_items_5.current as u8);
                    timer::set_variable_int("wrecked_ship_items", wrecked_ship_items.current as u8);
                    timer::set_variable_int("maridia_items_1", maridia_items_1.current as u8);
                    timer::set_variable_int("maridia_items_2", maridia_items_2.current as u8);
                    timer::set_variable_int("maridia_items3", maridia_items_3.current as u8);

                    match timer::state() {
                        TimerState::NotRunning => {
                            if settings.reset_timer && room_id.old != 0 && room_id.current == 0 {
                                timer::reset();
                            }

                            // Normal Start
                            if settings.start && game_state.old == 2 && game_state.current == 0x1F {
                                // splits = HashSet::<String>::new();
                                //
                                // picked_up_spore_spawn_super = false;
                                // picked_up_hundredth_missile = false;
                                timer::start();
                            }
                        }
                        TimerState::Running => {
                            // Ammo pickup section
                            let first_missile = settings.first_missile
                                && max_missiles.old == 0
                                && max_missiles.current == 5;
                            let all_missiles = settings.all_missiles
                                && (max_missiles.old + 5) == (max_missiles.current);
                            let ocean_bottom_missiles = settings.ocean_bottom_missiles
                                && room_id.current == *room_id_enum.get("westOcean").unwrap()
                                && (crateria_items.old + 2) == (crateria_items.current);
                            let ocean_top_missiles = settings.ocean_top_missiles
                                && room_id.current == *room_id_enum.get("westOcean").unwrap()
                                && (crateria_items.old + 4) == (crateria_items.current);
                            let ocean_middle_missiles = settings.ocean_middle_missiles
                                && room_id.current == *room_id_enum.get("westOcean").unwrap()
                                && (crateria_items.old + 8) == (crateria_items.current);
                            let moat_missiles = settings.moat_missiles
                                && room_id.current == *room_id_enum.get("crateriaMoat").unwrap()
                                && (crateria_items.old + 16) == (crateria_items.current);
                            let old_tourian_missiles = settings.old_tourian_missiles
                                && room_id.current == *room_id_enum.get("pitRoom").unwrap()
                                && (crateria_items.old + 64) == (crateria_items.current);
                            let gauntlet_right_missiles = settings.gauntlet_right_missiles
                                && room_id.current
                                    == *room_id_enum.get("greenPirateShaft").unwrap()
                                && (brinteria_items.old + 2) == (brinteria_items.current);
                            let gauntlet_left_missiles = settings.gauntlet_left_missiles
                                && room_id.current
                                    == *room_id_enum.get("greenPirateShaft").unwrap()
                                && (brinteria_items.old + 4) == (brinteria_items.current);
                            let dental_plan = settings.dental_plan
                                && room_id.current == *room_id_enum.get("theFinalMissile").unwrap()
                                && (brinteria_items.old + 16) == (brinteria_items.current);
                            let early_super_bridge_missiles = settings.early_super_bridge_missiles
                                && room_id.current == *room_id_enum.get("earlySupers").unwrap()
                                && (brinteria_items.old + 128) == (brinteria_items.current);
                            let green_brinstar_reserve_missiles = settings
                                .green_brinstar_reserve_missiles
                                && room_id.current
                                    == *room_id_enum.get("brinstarReserveRoom").unwrap()
                                && (brinstar_items_2.old + 8) == (brinstar_items_2.current);
                            let green_brinstar_extra_reserve_missiles = settings
                                .green_brinstar_extra_reserve_missiles
                                && room_id.current
                                    == *room_id_enum.get("brinstarReserveRoom").unwrap()
                                && (brinstar_items_2.old + 4) == (brinstar_items_2.current);
                            let big_pink_top_missiles = settings.big_pink_top_missiles
                                && room_id.current == *room_id_enum.get("bigPink").unwrap()
                                && (brinstar_items_2.old + 32) == (brinstar_items_2.current);
                            let charge_missiles = settings.charge_missiles
                                && room_id.current == *room_id_enum.get("bigPink").unwrap()
                                && (brinstar_items_2.old + 64) == (brinstar_items_2.current);
                            let green_hills_missiles = settings.green_hills_missiles
                                && room_id.current == *room_id_enum.get("greenHills").unwrap()
                                && (brinstar_items_3.old + 2) == (brinstar_items_3.current);
                            let blue_brinstar_e_tank_missiles = settings
                                .blue_brinstar_e_tank_missiles
                                && room_id.current
                                    == *room_id_enum.get("blueBrinstarETankRoom").unwrap()
                                && (brinstar_items_3.old + 16) == (brinstar_items_3.current);
                            let alpha_missiles = settings.alpha_missiles
                                && room_id.current
                                    == *room_id_enum.get("alphaMissileRoom").unwrap()
                                && (brinstar_items_4.old + 4) == (brinstar_items_4.current);
                            let billy_mays_missiles = settings.billy_mays_missiles
                                && room_id.current == *room_id_enum.get("billyMays").unwrap()
                                && (brinstar_items_4.old + 16) == (brinstar_items_4.current);
                            let but_wait_theres_more_missiles = settings
                                .but_wait_theres_more_missiles
                                && room_id.current == *room_id_enum.get("billyMays").unwrap()
                                && (brinstar_items_4.old + 32) == (brinstar_items_4.current);
                            let red_brinstar_missiles = settings.red_brinstar_missiles
                                && room_id.current
                                    == *room_id_enum.get("alphaPowerBombsRoom").unwrap()
                                && (brinstar_items_5.old + 2) == (brinstar_items_5.current);
                            let warehouse_missiles = settings.warehouse_missiles
                                && room_id.current
                                    == *room_id_enum.get("warehouseKiHunters").unwrap()
                                && (brinstar_items_5.old + 16) == (brinstar_items_5.current);
                            let cathedral_missiles = settings.cathedral_missiles
                                && room_id.current == *room_id_enum.get("cathedral").unwrap()
                                && (norfair_items_1.old + 2) == (norfair_items_1.current);
                            let crumble_shaft_missiles = settings.crumble_shaft_missiles
                                && room_id.current == *room_id_enum.get("crumbleShaft").unwrap()
                                && (norfair_items_1.old + 8) == (norfair_items_1.current);
                            let crocomire_escape_missiles = settings.crocomire_escape_missiles
                                && room_id.current == *room_id_enum.get("crocomireEscape").unwrap()
                                && (norfair_items_1.old + 64) == (norfair_items_1.current);
                            let hi_jump_missiles = settings.hi_jump_missiles
                                && room_id.current == *room_id_enum.get("hiJumpShaft").unwrap()
                                && (norfair_items_1.old + 128) == (norfair_items_1.current);
                            let post_crocomire_missiles = settings.post_crocomire_missiles
                                && room_id.current == *room_id_enum.get("cosineRoom").unwrap()
                                && (norfair_items_2.old + 4) == (norfair_items_2.current);
                            let grapple_missiles = settings.grapple_missiles
                                && room_id.current == *room_id_enum.get("preGrapple").unwrap()
                                && (norfair_items_2.old + 8) == (norfair_items_2.current);
                            let norfair_reserve_missiles = settings.norfair_reserve_missiles
                                && room_id.current
                                    == *room_id_enum.get("norfairReserveRoom").unwrap()
                                && (norfair_items_2.old + 64) == (norfair_items_2.current);
                            let green_bubbles_missiles = settings.green_bubbles_missiles
                                && room_id.current
                                    == *room_id_enum.get("greenBubblesRoom").unwrap()
                                && (norfair_items_2.old + 128) == (norfair_items_2.current);
                            let bubble_mountain_missiles = settings.bubble_mountain_missiles
                                && room_id.current == *room_id_enum.get("bubbleMountain").unwrap()
                                && (norfair_items_3.old + 1) == (norfair_items_3.current);
                            let speed_boost_missiles = settings.speed_boost_missiles
                                && room_id.current == *room_id_enum.get("speedBoostHall").unwrap()
                                && (norfair_items_3.old + 2) == (norfair_items_3.current);
                            let wave_missiles = settings.wave_missiles
                                && room_id.current == *room_id_enum.get("doubleChamber").unwrap()
                                && (norfair_items_3.old + 8) == (norfair_items_3.current);
                            let gold_torizo_missiles = settings.gold_torizo_missiles
                                && room_id.current == *room_id_enum.get("goldenTorizo").unwrap()
                                && (norfair_items_3.old + 64) == (norfair_items_3.current);
                            let mickey_mouse_missiles = settings.mickey_mouse_missiles
                                && room_id.current == *room_id_enum.get("mickeyMouse").unwrap()
                                && (norfair_items_4.old + 2) == (norfair_items_4.current);
                            let lower_norfair_spring_maze_missiles = settings
                                .lower_norfair_spring_maze_missiles
                                && room_id.current
                                    == *room_id_enum.get("lowerNorfairSpringMaze").unwrap()
                                && (norfair_items_4.old + 4) == (norfair_items_4.current);
                            let three_musketeers_missiles = settings.three_musketeers_missiles
                                && room_id.current == *room_id_enum.get("threeMusketeers").unwrap()
                                && (norfair_items_4.old + 32) == (norfair_items_4.current);
                            let wrecked_ship_main_shaft_missiles = settings
                                .wrecked_ship_main_shaft_missiles
                                && room_id.current
                                    == *room_id_enum.get("wreckedShipMainShaft").unwrap()
                                && (wrecked_ship_items.old + 1) == (wrecked_ship_items.current);
                            let bowling_missiles = settings.bowling_missiles
                                && room_id.current == *room_id_enum.get("bowling").unwrap()
                                && (wrecked_ship_items.old + 4) == (wrecked_ship_items.current);
                            let attic_missiles = settings.attic_missiles
                                && room_id.current
                                    == *room_id_enum.get("atticWorkerRobotRoom").unwrap()
                                && (wrecked_ship_items.old + 8) == (wrecked_ship_items.current);
                            let main_street_missiles = settings.main_street_missiles
                                && room_id.current == *room_id_enum.get("mainStreet").unwrap()
                                && (maridia_items_1.old + 1) == (maridia_items_1.current);
                            let mama_turtle_missiles = settings.mama_turtle_missiles
                                && room_id.current == *room_id_enum.get("mamaTurtle").unwrap()
                                && (maridia_items_1.old + 8) == (maridia_items_1.current);
                            let watering_hole_missiles = settings.watering_hole_missiles
                                && room_id.current == *room_id_enum.get("wateringHole").unwrap()
                                && (maridia_items_1.old + 32) == (maridia_items_1.current);
                            let beach_missiles = settings.beach_missiles
                                && room_id.current == *room_id_enum.get("beach").unwrap()
                                && (maridia_items_1.old + 64) == (maridia_items_1.current);
                            let left_sand_pit_missiles = settings.left_sand_pit_missiles
                                && room_id.current == *room_id_enum.get("leftSandPit").unwrap()
                                && (maridia_items_2.old + 1) == (maridia_items_2.current);
                            let right_sand_pit_missiles = settings.right_sand_pit_missiles
                                && room_id.current == *room_id_enum.get("rightSandPit").unwrap()
                                && (maridia_items_2.old + 4) == (maridia_items_2.current);
                            let aqueduct_missiles = settings.aqueduct_missiles
                                && room_id.current == *room_id_enum.get("aqueduct").unwrap()
                                && (maridia_items_2.old + 16) == (maridia_items_2.current);
                            let pre_draygon_missiles = settings.pre_draygon_missiles
                                && room_id.current == *room_id_enum.get("precious").unwrap()
                                && (maridia_items_2.old + 128) == (maridia_items_2.current);
                            let first_super = settings.first_super
                                && max_supers.old == 0
                                && max_supers.current == 5;
                            let all_supers =
                                settings.all_supers && (max_supers.old + 5) == (max_supers.current);
                            let climb_supers = settings.climb_supers
                                && room_id.current
                                    == *room_id_enum.get("crateriaSupersRoom").unwrap()
                                && (brinteria_items.old + 8) == (brinteria_items.current);
                            let spore_spawn_supers = settings.spore_spawn_supers
                                && room_id.current == *room_id_enum.get("sporeSpawnSuper").unwrap()
                                && (brinteria_items.old + 64) == (brinteria_items.current);
                            let early_supers = settings.early_supers
                                && room_id.current == *room_id_enum.get("earlySupers").unwrap()
                                && (brinstar_items_2.old + 1) == (brinstar_items_2.current);
                            let etacoon_supers = settings.etacoon_supers
                                && room_id.current
                                    == *room_id_enum.get("etacoonSuperRoom").unwrap()
                                && (brinstar_items_3.old + 128) == (brinstar_items_3.current);
                            let gold_torizo_supers = settings.gold_torizo_supers
                                && room_id.current == *room_id_enum.get("goldenTorizo").unwrap()
                                && (norfair_items_3.old + 128) == (norfair_items_3.current);
                            let wrecked_ship_left_supers = settings.wrecked_ship_left_supers
                                && room_id.current
                                    == *room_id_enum.get("wreckedShipLeftSuperRoom").unwrap()
                                && (wrecked_ship_items.old + 32) == (wrecked_ship_items.current);
                            let wrecked_ship_right_supers = settings.wrecked_ship_right_supers
                                && room_id.current
                                    == *room_id_enum.get("wreckedShipRightSuperRoom").unwrap()
                                && (wrecked_ship_items.old + 64) == (wrecked_ship_items.current);
                            let crab_supers = settings.crab_supers
                                && room_id.current == *room_id_enum.get("mainStreet").unwrap()
                                && (maridia_items_1.old + 2) == (maridia_items_1.current);
                            let watering_hole_supers = settings.watering_hole_supers
                                && room_id.current == *room_id_enum.get("wateringHole").unwrap()
                                && (maridia_items_1.old + 16) == (maridia_items_1.current);
                            let aqueduct_supers = settings.aqueduct_supers
                                && room_id.current == *room_id_enum.get("aqueduct").unwrap()
                                && (maridia_items_2.old + 32) == (maridia_items_2.current);
                            let first_power_bomb = settings.first_power_bomb
                                && max_power_bombs.old == 0
                                && max_power_bombs.current == 5;
                            let all_power_bombs = settings.all_power_bombs
                                && (max_power_bombs.old + 5) == (max_power_bombs.current);
                            let landing_site_bombs = settings.landing_site_bombs
                                && room_id.current
                                    == *room_id_enum.get("crateriaPowerBombRoom").unwrap()
                                && (crateria_items.old + 1) == (crateria_items.current);
                            let etacoon_bombs = settings.etacoon_bombs
                                && room_id.current
                                    == *room_id_enum.get("greenBrinstarMainShaft").unwrap()
                                && (brinteria_items.old + 32) == (brinteria_items.current);
                            let pink_brinstar_bombs = settings.pink_brinstar_bombs
                                && room_id.current
                                    == *room_id_enum.get("pinkBrinstarPowerBombRoom").unwrap()
                                && (brinstar_items_3.old + 1) == (brinstar_items_3.current);
                            let blue_brinstar_bombs = settings.blue_brinstar_bombs
                                && room_id.current == *room_id_enum.get("morphBall").unwrap()
                                && (brinstar_items_3.old + 8) == (brinstar_items_3.current);
                            let alpha_bombs = settings.alpha_bombs
                                && room_id.current
                                    == *room_id_enum.get("alphaPowerBombsRoom").unwrap()
                                && (brinstar_items_5.old + 1) == (brinstar_items_5.current);
                            let beta_bombs = settings.beta_bombs
                                && room_id.current
                                    == *room_id_enum.get("betaPowerBombRoom").unwrap()
                                && (brinstar_items_4.old + 128) == (brinstar_items_4.current);
                            let crocomire_bombs = settings.crocomire_bombs
                                && room_id.current
                                    == *room_id_enum.get("postCrocomirePowerBombRoom").unwrap()
                                && (norfair_items_2.old + 2) == (norfair_items_2.current);
                            let lower_norfair_escape_bombs = settings.lower_norfair_escape_bombs
                                && room_id.current
                                    == *room_id_enum
                                        .get("lowerNorfairEscapePowerBombRoom")
                                        .unwrap()
                                && (norfair_items_4.old + 8) == (norfair_items_4.current);
                            let shame_bombs = settings.shame_bombs
                                && room_id.current == *room_id_enum.get("wasteland").unwrap()
                                && (norfair_items_4.old + 16) == (norfair_items_4.current);
                            let right_sand_pit_bombs = settings.right_sand_pit_bombs
                                && room_id.current == *room_id_enum.get("rightSandPit").unwrap()
                                && (maridia_items_2.old + 8) == (maridia_items_2.current);

                            let pickup = first_missile
                                || all_missiles
                                || ocean_bottom_missiles
                                || ocean_top_missiles
                                || ocean_middle_missiles
                                || moat_missiles
                                || old_tourian_missiles
                                || gauntlet_right_missiles
                                || gauntlet_left_missiles
                                || dental_plan
                                || early_super_bridge_missiles
                                || green_brinstar_reserve_missiles
                                || green_brinstar_extra_reserve_missiles
                                || big_pink_top_missiles
                                || charge_missiles
                                || green_hills_missiles
                                || blue_brinstar_e_tank_missiles
                                || alpha_missiles
                                || billy_mays_missiles
                                || but_wait_theres_more_missiles
                                || red_brinstar_missiles
                                || warehouse_missiles
                                || cathedral_missiles
                                || crumble_shaft_missiles
                                || crocomire_escape_missiles
                                || hi_jump_missiles
                                || post_crocomire_missiles
                                || grapple_missiles
                                || norfair_reserve_missiles
                                || green_bubbles_missiles
                                || bubble_mountain_missiles
                                || speed_boost_missiles
                                || wave_missiles
                                || gold_torizo_missiles
                                || mickey_mouse_missiles
                                || lower_norfair_spring_maze_missiles
                                || three_musketeers_missiles
                                || wrecked_ship_main_shaft_missiles
                                || bowling_missiles
                                || attic_missiles
                                || main_street_missiles
                                || mama_turtle_missiles
                                || watering_hole_missiles
                                || beach_missiles
                                || left_sand_pit_missiles
                                || right_sand_pit_missiles
                                || aqueduct_missiles
                                || pre_draygon_missiles
                                || first_super
                                || all_supers
                                || climb_supers
                                || spore_spawn_supers
                                || early_supers
                                || etacoon_supers
                                || gold_torizo_supers
                                || wrecked_ship_left_supers
                                || wrecked_ship_right_supers
                                || crab_supers
                                || watering_hole_supers
                                || aqueduct_supers
                                || first_power_bomb
                                || all_power_bombs
                                || landing_site_bombs
                                || etacoon_bombs
                                || pink_brinstar_bombs
                                || blue_brinstar_bombs
                                || alpha_bombs
                                || beta_bombs
                                || crocomire_bombs
                                || lower_norfair_escape_bombs
                                || shame_bombs
                                || right_sand_pit_bombs;

                            // Item unlock section
                            let varia = settings.varia_suit
                                && room_id.current == *room_id_enum.get("varia").unwrap()
                                && (unlocked_equips_2.old
                                    & unlock_flag_enum.get("variaSuit").unwrap())
                                    == 0
                                && (unlocked_equips_2.current
                                    & unlock_flag_enum.get("variaSuit").unwrap())
                                    > 0;
                            let spring_ball = settings.spring_ball
                                && room_id.current == *room_id_enum.get("springBall").unwrap()
                                && (unlocked_equips_2.old
                                    & unlock_flag_enum.get("springBall").unwrap())
                                    == 0
                                && (unlocked_equips_2.current
                                    & unlock_flag_enum.get("springBall").unwrap())
                                    > 0;

                            let morph_ball = settings.morph_ball
                                && room_id.current == *room_id_enum.get("morphBall").unwrap()
                                && (unlocked_equips_2.old
                                    & unlock_flag_enum.get("morphBall").unwrap())
                                    == 0
                                && (unlocked_equips_2.current
                                    & unlock_flag_enum.get("morphBall").unwrap())
                                    > 0;

                            let screw_attack = settings.screw_attack
                                && room_id.current == *room_id_enum.get("screwAttack").unwrap()
                                && (unlocked_equips_2.old
                                    & unlock_flag_enum.get("screwAttack").unwrap())
                                    == 0
                                && (unlocked_equips_2.current
                                    & unlock_flag_enum.get("screwAttack").unwrap())
                                    > 0;
                            let grav_suit = settings.grav_suit
                                && room_id.current == *room_id_enum.get("gravity").unwrap()
                                && (unlocked_equips_2.old
                                    & unlock_flag_enum.get("gravSuit").unwrap())
                                    == 0
                                && (unlocked_equips_2.current
                                    & unlock_flag_enum.get("gravSuit").unwrap())
                                    > 0;
                            let hi_jump = settings.hi_jump
                                && room_id.current == *room_id_enum.get("hiJump").unwrap()
                                && (unlocked_equips.old & unlock_flag_enum.get("hiJump").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("hiJump").unwrap())
                                    > 0;
                            let space_jump = settings.space_jump
                                && room_id.current == *room_id_enum.get("spaceJump").unwrap()
                                && (unlocked_equips.old
                                    & unlock_flag_enum.get("spaceJump").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("spaceJump").unwrap())
                                    > 0;
                            let bomb = settings.bomb
                                && room_id.current == *room_id_enum.get("bombTorizo").unwrap()
                                && (unlocked_equips.old & unlock_flag_enum.get("bomb").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("bomb").unwrap())
                                    > 0;
                            let speed_booster = settings.speed_booster
                                && room_id.current == *room_id_enum.get("speedBooster").unwrap()
                                && (unlocked_equips.old
                                    & unlock_flag_enum.get("speedBooster").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("speedBooster").unwrap())
                                    > 0;
                            let grapple = settings.grapple
                                && room_id.current == *room_id_enum.get("grapple").unwrap()
                                && (unlocked_equips.old & unlock_flag_enum.get("grapple").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("grapple").unwrap())
                                    > 0;
                            let xray = settings.xray
                                && room_id.current == *room_id_enum.get("xRay").unwrap()
                                && (unlocked_equips.old & unlock_flag_enum.get("xray").unwrap())
                                    == 0
                                && (unlocked_equips.current
                                    & unlock_flag_enum.get("xray").unwrap())
                                    > 0;
                            let unlock = varia
                                || spring_ball
                                || morph_ball
                                || screw_attack
                                || grav_suit
                                || hi_jump
                                || space_jump
                                || bomb
                                || speed_booster
                                || grapple
                                || xray;

                            // Beam unlock section
                            let wave = settings.wave
                                && room_id.current == *room_id_enum.get("waveBeam").unwrap()
                                && (unlocked_beams.old & *unlock_flag_enum.get("wave").unwrap())
                                    == 0
                                && (unlocked_beams.current
                                    & *unlock_flag_enum.get("wave").unwrap())
                                    > 0;
                            let ice = settings.ice
                                && room_id.current == *room_id_enum.get("iceBeam").unwrap()
                                && (unlocked_beams.old & *unlock_flag_enum.get("ice").unwrap())
                                    == 0
                                && (unlocked_beams.current & *unlock_flag_enum.get("ice").unwrap())
                                    > 0;
                            let spazer = settings.spazer
                                && room_id.current == *room_id_enum.get("spazer").unwrap()
                                && (unlocked_beams.old & *unlock_flag_enum.get("spazer").unwrap())
                                    == 0
                                && (unlocked_beams.current
                                    & *unlock_flag_enum.get("spazer").unwrap())
                                    > 0;
                            let plasma = settings.plasma
                                && room_id.current == *room_id_enum.get("plasmaBeam").unwrap()
                                && (unlocked_beams.old & *unlock_flag_enum.get("plasma").unwrap())
                                    == 0
                                && (unlocked_beams.current
                                    & *unlock_flag_enum.get("plasma").unwrap())
                                    > 0;
                            let charge_beam = settings.charge_beam
                                && room_id.current == *room_id_enum.get("bigPink").unwrap()
                                && (unlocked_charge.old
                                    & *unlock_flag_enum.get("chargeBeam").unwrap())
                                    == 0
                                && (unlocked_charge.current
                                    & *unlock_flag_enum.get("chargeBeam").unwrap())
                                    > 0;
                            let beam = wave || ice || spazer || plasma || charge_beam;

                            // E-tanks and reserve tanks
                            let first_e_tank = settings.first_e_tank
                                && max_energy.old == 99
                                && max_energy.current == 199;
                            let all_e_tanks = settings.all_e_tanks
                                && (max_energy.old + 100) == (max_energy.current);
                            let gauntlet_e_tank = settings.gauntlet_e_tank
                                && room_id.current
                                    == *room_id_enum.get("gauntletETankRoom").unwrap()
                                && (crateria_items.old + 32) == (crateria_items.current);
                            let terminator_e_tank = settings.terminator_e_tank
                                && room_id.current == *room_id_enum.get("terminator").unwrap()
                                && (brinteria_items.old + 1) == (brinteria_items.current);
                            let ceiling_e_tank = settings.ceiling_e_tank
                                && room_id.current
                                    == *room_id_enum.get("blueBrinstarETankRoom").unwrap()
                                && (brinstar_items_3.old + 32) == (brinstar_items_3.current);
                            let etecoons_e_tank = settings.etecoons_e_tank
                                && room_id.current
                                    == *room_id_enum.get("etacoonETankRoom").unwrap()
                                && (brinstar_items_3.old + 64) == (brinstar_items_3.current);
                            let waterway_e_tank = settings.waterway_e_tank
                                && room_id.current == *room_id_enum.get("waterway").unwrap()
                                && (brinstar_items_4.old + 2) == (brinstar_items_4.current);
                            let wave_gate_e_tank = settings.wave_gate_e_tank
                                && room_id.current == *room_id_enum.get("hopperETankRoom").unwrap()
                                && (brinstar_items_4.old + 8) == (brinstar_items_4.current);
                            let kraid_e_tank = settings.kraid_e_tank
                                && room_id.current
                                    == *room_id_enum.get("warehouseETankRoom").unwrap()
                                && (brinstar_items_5.old + 8) == (brinstar_items_5.current);
                            let crocomire_e_tank = settings.crocomire_e_tank
                                && room_id.current == *room_id_enum.get("crocomire").unwrap()
                                && (norfair_items_1.old + 16) == (norfair_items_1.current);
                            let hi_jump_e_tank = settings.hi_jump_e_tank
                                && room_id.current == *room_id_enum.get("hiJumpShaft").unwrap()
                                && (norfair_items_2.old + 1) == (norfair_items_2.current);
                            let ridley_e_tank = settings.ridley_e_tank
                                && room_id.current == *room_id_enum.get("ridleyETankRoom").unwrap()
                                && (norfair_items_4.old + 64) == (norfair_items_4.current);
                            let fireflea_e_tank = settings.fireflea_e_tank
                                && room_id.current
                                    == *room_id_enum.get("lowerNorfairFireflea").unwrap()
                                && (norfair_items_5.old + 1) == (norfair_items_5.current);
                            let wrecked_ship_e_tank = settings.wrecked_ship_e_tank
                                && room_id.current
                                    == *room_id_enum.get("wreckedShipETankRoom").unwrap()
                                && (wrecked_ship_items.old + 16) == (wrecked_ship_items.current);
                            let tatori_e_tank = settings.tatori_e_tank
                                && room_id.current == *room_id_enum.get("mamaTurtle").unwrap()
                                && (maridia_items_1.old + 4) == (maridia_items_1.current);
                            let botwoon_e_tank = settings.botwoon_e_tank
                                && room_id.current
                                    == *room_id_enum.get("botwoonETankRoom").unwrap()
                                && (maridia_items_3.old + 1) == (maridia_items_3.current);
                            let reserve_tanks = settings.reserve_tanks
                                && (max_reserve.old + 100) == (max_reserve.current);
                            let brinstar_reserve = settings.brinstar_reserve
                                && room_id.current
                                    == *room_id_enum.get("brinstarReserveRoom").unwrap()
                                && (brinstar_items_2.old + 2) == (brinstar_items_2.current);
                            let norfair_reserve = settings.norfair_reserve
                                && room_id.current
                                    == *room_id_enum.get("norfairReserveRoom").unwrap()
                                && (norfair_items_2.old + 32) == (norfair_items_2.current);
                            let wrecked_ship_reserve = settings.wrecked_ship_reserve
                                && room_id.current == *room_id_enum.get("bowling").unwrap()
                                && (wrecked_ship_items.old + 2) == (wrecked_ship_items.current);
                            let maridia_reserve = settings.maridia_reserve
                                && room_id.current == *room_id_enum.get("leftSandPit").unwrap()
                                && (maridia_items_2.old + 2) == (maridia_items_2.current);
                            let energy_upgrade = first_e_tank
                                || all_e_tanks
                                || gauntlet_e_tank
                                || terminator_e_tank
                                || ceiling_e_tank
                                || etecoons_e_tank
                                || waterway_e_tank
                                || wave_gate_e_tank
                                || kraid_e_tank
                                || crocomire_e_tank
                                || hi_jump_e_tank
                                || ridley_e_tank
                                || fireflea_e_tank
                                || wrecked_ship_e_tank
                                || tatori_e_tank
                                || botwoon_e_tank
                                || reserve_tanks
                                || brinstar_reserve
                                || norfair_reserve
                                || wrecked_ship_reserve
                                || maridia_reserve;

                            // Miniboss room transitions
                            let mut mini_boss_rooms = false;
                            if settings.mini_boss_rooms {
                                let ceres_ridley_room = room_id.old
                                    == *room_id_enum.get("flatRoom").unwrap()
                                    && room_id.current == *room_id_enum.get("ceresRidley").unwrap();
                                let spore_spawn_room = room_id.old
                                    == *room_id_enum.get("sporeSpawnKeyhunter").unwrap()
                                    && room_id.current == *room_id_enum.get("sporeSpawn").unwrap();
                                let crocomire_room = room_id.old
                                    == *room_id_enum.get("crocomireSpeedway").unwrap()
                                    && room_id.current == *room_id_enum.get("crocomire").unwrap();
                                let botwoon_room = room_id.old
                                    == *room_id_enum.get("botwoonHallway").unwrap()
                                    && room_id.current == *room_id_enum.get("botwoon").unwrap();
                                // Allow either vanilla or GGG entry
                                let golden_torizo_room = (room_id.old
                                    == *room_id_enum.get("acidStatue").unwrap()
                                    || room_id.old == *room_id_enum.get("screwAttack").unwrap())
                                    && room_id.current
                                        == *room_id_enum.get("goldenTorizo").unwrap();
                                mini_boss_rooms = ceres_ridley_room
                                    || spore_spawn_room
                                    || crocomire_room
                                    || botwoon_room
                                    || golden_torizo_room;
                            }

                            // Boss room transitions
                            let mut boss_rooms = false;
                            if settings.boss_rooms {
                                let kraid_room = room_id.old
                                    == *room_id_enum.get("kraidEyeDoor").unwrap()
                                    && room_id.current == *room_id_enum.get("kraid").unwrap();
                                let phantoon_room = room_id.old
                                    == *room_id_enum.get("basement").unwrap()
                                    && room_id.current == *room_id_enum.get("phantoon").unwrap();
                                let draygon_room = room_id.old
                                    == *room_id_enum.get("precious").unwrap()
                                    && room_id.current == *room_id_enum.get("draygon").unwrap();
                                let ridley_room = room_id.old
                                    == *room_id_enum.get("lowerNorfairFarming").unwrap()
                                    && room_id.current == *room_id_enum.get("ridley").unwrap();
                                let mother_brain_room = room_id.old
                                    == *room_id_enum.get("rinkaShaft").unwrap()
                                    && room_id.current == *room_id_enum.get("motherBrain").unwrap();
                                boss_rooms = kraid_room
                                    || phantoon_room
                                    || draygon_room
                                    || ridley_room
                                    || mother_brain_room;
                            }

                            // Elevator transitions between areas
                            let mut elevator_transitions = false;
                            if settings.elevator_transitions {
                                let blue_brinstar = (room_id.old
                                    == *room_id_enum.get("elevatorToMorphBall").unwrap()
                                    && room_id.current == *room_id_enum.get("morphBall").unwrap())
                                    || (room_id.old == *room_id_enum.get("morphBall").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("elevatorToMorphBall").unwrap());
                                let green_brinstar = (room_id.old
                                    == *room_id_enum.get("elevatorToGreenBrinstar").unwrap()
                                    && room_id.current
                                        == *room_id_enum.get("greenBrinstarMainShaft").unwrap())
                                    || (room_id.old
                                        == *room_id_enum.get("greenBrinstarMainShaft").unwrap()
                                        && room_id.current
                                            == *room_id_enum
                                                .get("elevatorToGreenBrinstar")
                                                .unwrap());
                                let business_center = (room_id.old
                                    == *room_id_enum.get("warehouseEntrance").unwrap()
                                    && room_id.current
                                        == *room_id_enum.get("businessCenter").unwrap())
                                    || (room_id.old
                                        == *room_id_enum.get("businessCenter").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("warehouseEntrance").unwrap());
                                let caterpillar = (room_id.old
                                    == *room_id_enum.get("elevatorToCaterpillar").unwrap()
                                    && room_id.current
                                        == *room_id_enum.get("caterpillar").unwrap())
                                    || (room_id.old == *room_id_enum.get("caterpillar").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("elevatorToCaterpillar").unwrap());
                                let maridia_elevator = (room_id.old
                                    == *room_id_enum.get("elevatorToMaridia").unwrap()
                                    && room_id.current
                                        == *room_id_enum.get("maridiaElevator").unwrap())
                                    || (room_id.old
                                        == *room_id_enum.get("maridiaElevator").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("elevatorToMaridia").unwrap());
                                elevator_transitions = blue_brinstar
                                    || green_brinstar
                                    || business_center
                                    || caterpillar
                                    || maridia_elevator;
                            }

                            // Room transitions
                            let ceres_escape = settings.ceres_escape
                                && room_id.current == *room_id_enum.get("ceresElevator").unwrap()
                                && game_state.old
                                    == *game_state_enum.get("normalGameplay").unwrap()
                                && game_state.current
                                    == *game_state_enum.get("startOfCeresCutscene").unwrap();
                            let wrecked_ship_entrance = settings.wrecked_ship_entrance
                                && room_id.old == *room_id_enum.get("westOcean").unwrap()
                                && room_id.current
                                    == *room_id_enum.get("wreckedShipEntrance").unwrap();
                            let red_tower_middle_entrance = settings.red_tower_middle_entrance
                                && room_id.old == *room_id_enum.get("noobBridge").unwrap()
                                && room_id.current == *room_id_enum.get("redTower").unwrap();
                            let red_tower_bottom_entrance = settings.red_tower_bottom_entrance
                                && room_id.old == *room_id_enum.get("bat").unwrap()
                                && room_id.current == *room_id_enum.get("redTower").unwrap();
                            let kraids_lair = settings.kraids_lair
                                && room_id.old == *room_id_enum.get("warehouseEntrance").unwrap()
                                && room_id.current == *room_id_enum.get("warehouseZeela").unwrap();
                            let rising_tide_entrance = settings.rising_tide_entrance
                                && room_id.old == *room_id_enum.get("cathedral").unwrap()
                                && room_id.current == *room_id_enum.get("risingTide").unwrap();
                            let attic_exit = settings.attic_exit
                                && room_id.old == *room_id_enum.get("attic").unwrap()
                                && room_id.current == *room_id_enum.get("westOcean").unwrap();
                            let tube_broken = settings.tube_broken
                                && room_id.current == *room_id_enum.get("glassTunnel").unwrap()
                                && (event_flags.old & event_flag_enum.get("tubeBroken").unwrap())
                                    == 0
                                && (event_flags.current
                                    & event_flag_enum.get("tubeBroken").unwrap())
                                    > 0;
                            let cac_exit = settings.cac_exit
                                && room_id.old == *room_id_enum.get("westCactusAlley").unwrap()
                                && room_id.current == *room_id_enum.get("butterflyRoom").unwrap();
                            let toilet = settings.toilet
                                && (room_id.old == *room_id_enum.get("plasmaSpark").unwrap()
                                    && room_id.current == *room_id_enum.get("toiletBowl").unwrap()
                                    || room_id.old == *room_id_enum.get("oasis").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("toiletBowl").unwrap());
                            let kronic_boost = settings.kronic_boost
                                && (room_id.old == *room_id_enum.get("magdolliteTunnel").unwrap()
                                    && room_id.current
                                        == *room_id_enum.get("kronicBoost").unwrap()
                                    || room_id.old
                                        == *room_id_enum.get("spikyAcidSnakes").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("kronicBoost").unwrap()
                                    || room_id.old == *room_id_enum.get("volcano").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("kronicBoost").unwrap());
                            let lower_norfair_entrance = settings.lower_norfair_entrance
                                && room_id.old
                                    == *room_id_enum.get("lowerNorfairElevator").unwrap()
                                && room_id.current == *room_id_enum.get("mainHall").unwrap();
                            let writg = settings.writg
                                && room_id.old == *room_id_enum.get("pillars").unwrap()
                                && room_id.current == *room_id_enum.get("writg").unwrap();
                            let red_ki_shaft = settings.red_ki_shaft
                                && (room_id.old == *room_id_enum.get("amphitheatre").unwrap()
                                    && room_id.current == *room_id_enum.get("redKiShaft").unwrap()
                                    || room_id.old == *room_id_enum.get("wasteland").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("redKiShaft").unwrap());
                            let metal_pirates = settings.metal_pirates
                                && room_id.old == *room_id_enum.get("wasteland").unwrap()
                                && room_id.current == *room_id_enum.get("metalPirates").unwrap();
                            let lower_norfair_spring_maze = settings.lower_norfair_spring_maze
                                && room_id.old
                                    == *room_id_enum.get("lowerNorfairFireflea").unwrap()
                                && room_id.current
                                    == *room_id_enum.get("lowerNorfairSpringMaze").unwrap();
                            let lower_norfair_exit = settings.lower_norfair_exit
                                && room_id.old == *room_id_enum.get("threeMusketeers").unwrap()
                                && room_id.current == *room_id_enum.get("singleChamber").unwrap();
                            let all_bosses_finished = (brinstar_bosses.current
                                & boss_flag_enum.get("kraid").unwrap())
                                > 0
                                && (wrecked_ship_bosses.current
                                    & boss_flag_enum.get("phantoon").unwrap())
                                    > 0
                                && (maridia_bosses.current
                                    & boss_flag_enum.get("draygon").unwrap())
                                    > 0
                                && (norfair_bosses.current & boss_flag_enum.get("ridley").unwrap())
                                    > 0;
                            let golden_four = settings.golden_four
                                && room_id.old == *room_id_enum.get("statuesHallway").unwrap()
                                && room_id.current == *room_id_enum.get("statues").unwrap()
                                && all_bosses_finished;
                            let tourian_entrance = settings.tourian_entrance
                                && room_id.old == *room_id_enum.get("statues").unwrap()
                                && room_id.current == *room_id_enum.get("tourianElevator").unwrap();
                            let metroids = settings.metroids
                                && (room_id.old == *room_id_enum.get("metroidOne").unwrap()
                                    && room_id.current == *room_id_enum.get("metroidTwo").unwrap()
                                    || room_id.old == *room_id_enum.get("metroidTwo").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("metroidThree").unwrap()
                                    || room_id.old == *room_id_enum.get("metroidThree").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("metroidFour").unwrap()
                                    || room_id.old == *room_id_enum.get("metroidFour").unwrap()
                                        && room_id.current
                                            == *room_id_enum.get("tourianHopper").unwrap());
                            let baby_metroid_room = settings.baby_metroid_room
                                && room_id.old == *room_id_enum.get("dustTorizo").unwrap()
                                && room_id.current == *room_id_enum.get("bigBoy").unwrap();
                            let escape_climb = settings.escape_climb
                                && room_id.old == *room_id_enum.get("tourianEscape4").unwrap()
                                && room_id.current == *room_id_enum.get("climb").unwrap();
                            let room_transitions = mini_boss_rooms
                                || boss_rooms
                                || elevator_transitions
                                || ceres_escape
                                || wrecked_ship_entrance
                                || red_tower_middle_entrance
                                || red_tower_bottom_entrance
                                || kraids_lair
                                || rising_tide_entrance
                                || attic_exit
                                || tube_broken
                                || cac_exit
                                || toilet
                                || kronic_boost
                                || lower_norfair_entrance
                                || writg
                                || red_ki_shaft
                                || metal_pirates
                                || lower_norfair_spring_maze
                                || lower_norfair_exit
                                || tourian_entrance
                                || golden_four
                                || metroids
                                || baby_metroid_room
                                || escape_climb;

                            // Minibosses
                            let ceres_ridley = settings.ceres_ridley
                                && (ceres_bosses.old & boss_flag_enum.get("ceresRidley").unwrap())
                                    == 0
                                && (ceres_bosses.current
                                    & boss_flag_enum.get("ceresRidley").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("ceresRidley").unwrap();
                            let bomb_torizo = settings.bomb_torizo
                                && (crateria_bosses.old
                                    & boss_flag_enum.get("bombTorizo").unwrap())
                                    == 0
                                && (crateria_bosses.current
                                    & boss_flag_enum.get("bombTorizo").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("bombTorizo").unwrap();
                            let spore_spawn = settings.spore_spawn
                                && (brinstar_bosses.old
                                    & boss_flag_enum.get("sporeSpawn").unwrap())
                                    == 0
                                && (brinstar_bosses.current
                                    & boss_flag_enum.get("sporeSpawn").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("sporeSpawn").unwrap();
                            let crocomire = settings.crocomire
                                && (norfair_bosses.old & boss_flag_enum.get("crocomire").unwrap())
                                    == 0
                                && (norfair_bosses.current
                                    & boss_flag_enum.get("crocomire").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("crocomire").unwrap();
                            let botwoon = settings.botwoon
                                && (maridia_bosses.old & boss_flag_enum.get("botwoon").unwrap())
                                    == 0
                                && (maridia_bosses.current
                                    & boss_flag_enum.get("botwoon").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("botwoon").unwrap();
                            let golden_torizo = settings.golden_torizo
                                && (norfair_bosses.old
                                    & boss_flag_enum.get("goldenTorizo").unwrap())
                                    == 0
                                && (norfair_bosses.current
                                    & boss_flag_enum.get("goldenTorizo").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("goldenTorizo").unwrap();
                            let miniboss_defeat = ceres_ridley
                                || bomb_torizo
                                || spore_spawn
                                || crocomire
                                || botwoon
                                || golden_torizo;

                            // Bosses
                            let kraid = settings.kraid
                                && (brinstar_bosses.old & boss_flag_enum.get("kraid").unwrap())
                                    == 0
                                && (brinstar_bosses.current & boss_flag_enum.get("kraid").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("kraid").unwrap();
                            let phantoon = settings.phantoon
                                && (wrecked_ship_bosses.old
                                    & boss_flag_enum.get("phantoon").unwrap())
                                    == 0
                                && (wrecked_ship_bosses.current
                                    & boss_flag_enum.get("phantoon").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("phantoon").unwrap();
                            let draygon = settings.draygon
                                && (maridia_bosses.old & boss_flag_enum.get("draygon").unwrap())
                                    == 0
                                && (maridia_bosses.current
                                    & boss_flag_enum.get("draygon").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("draygon").unwrap();
                            let ridley = settings.ridley
                                && (norfair_bosses.old & boss_flag_enum.get("ridley").unwrap())
                                    == 0
                                && (norfair_bosses.current & boss_flag_enum.get("ridley").unwrap())
                                    > 0
                                && room_id.current == *room_id_enum.get("ridley").unwrap();
                            // Mother Brain phases
                            let in_mother_brain_room =
                                room_id.current == *room_id_enum.get("motherBrain").unwrap();
                            let mb1 = settings.mb1
                                && in_mother_brain_room
                                && game_state.current
                                    == *game_state_enum.get("normalGameplay").unwrap()
                                && mother_brain_hp.old == 0
                                && mother_brain_hp.current
                                    == (*mother_brain_max_hp_enum.get("phase2").unwrap());
                            let mb2 = settings.mb2
                                && in_mother_brain_room
                                && game_state.current
                                    == *game_state_enum.get("normalGameplay").unwrap()
                                && mother_brain_hp.old == 0
                                && mother_brain_hp.current
                                    == (*mother_brain_max_hp_enum.get("phase3").unwrap());
                            let mb3 = settings.mb3
                                && in_mother_brain_room
                                && (tourian_bosses.old
                                    & boss_flag_enum.get("motherBrain").unwrap())
                                    == 0
                                && (tourian_bosses.current
                                    & boss_flag_enum.get("motherBrain").unwrap())
                                    > 0;
                            let boss_defeat =
                                kraid || phantoon || draygon || ridley || mb1 || mb2 || mb3;

                            // Run-ending splits
                            let escape = settings.rta_finish
                                && (event_flags.current
                                    & event_flag_enum.get("zebesAblaze").unwrap())
                                    > 0
                                && ship_ai.old != 0xaa4f
                                && ship_ai.current == 0xaa4f;

                            let takeoff = settings.igt_finish
                                && room_id.current == *room_id_enum.get("landingSite").unwrap()
                                && game_state.old
                                    == *game_state_enum.get("preEndCutscene").unwrap()
                                && game_state.current
                                    == *game_state_enum.get("endCutscene").unwrap();

                            let mut spore_spawn_rta_finish = false;
                            if settings.spore_spawn_rta_finish {
                                if picked_up_spore_spawn_super {
                                    if igt_frames.old != igt_frames.current {
                                        spore_spawn_rta_finish = true;
                                        picked_up_spore_spawn_super = false;
                                    }
                                } else {
                                    picked_up_spore_spawn_super = room_id.current
                                        == *room_id_enum.get("sporeSpawnSuper").unwrap()
                                        && (max_supers.old + 5) == (max_supers.current)
                                        && (brinstar_bosses.current
                                            & boss_flag_enum.get("sporeSpawn").unwrap())
                                            > 0;
                                }
                            }

                            let mut hundred_missile_rta_finish = false;
                            if settings.hundred_missile_rta_finish {
                                if picked_up_hundredth_missile {
                                    if igt_frames.old != igt_frames.current {
                                        hundred_missile_rta_finish = true;
                                        picked_up_hundredth_missile = false;
                                    }
                                } else {
                                    picked_up_hundredth_missile =
                                        max_missiles.old == 95 && max_missiles.current == 100;
                                }
                            }

                            let non_standard_category_finish =
                                spore_spawn_rta_finish || hundred_missile_rta_finish;

                            if pickup
                                || unlock
                                || beam
                                || energy_upgrade
                                || room_transitions
                                || miniboss_defeat
                                || boss_defeat
                                || escape
                                || takeoff
                                || non_standard_category_finish {
                                timer::split();
                            }
                        }
                        _ => {}
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}
