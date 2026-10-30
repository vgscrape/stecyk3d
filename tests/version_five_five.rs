// SPDX-License-Identifier: MIT
// Bryce 5.5 as a program: a description of version "5.5" is read with 5.0's rules; a sky with only the
// plain colour renders exactly as 5.0, and a sky with its own colours (which carry the moon's disc,
// drawn by 5.5 at half 5.0's angle) is refused, saying why. (docs/manual/substrate.md, "Bryce 5.5")

use stecyk3d::description::describe;
use stecyk3d::render::{OpenChoices, RenderMode, render_lit, render_mode};
use stecyk3d::version::{FIVE_FIVE_MOON_DISC, Program, RenderAs};

/// A small description with no objects, as `version`, with the sky's switch word `flags`.
fn description(version: &str, flags: u32) -> String {
    format!(
        r#"{{"size": [24, 18], "render": {{"version": "{version}", "units": 1.0, "antialias": 0, "preview": false, "switches": 959}},
        "sky": {{"flags": {flags}, "sky_colour": [0.2, 0.2, 1.0], "horizon_colour": [1, 1, 1], "plain_colour": [0.3, 0.4, 0.9]}},
        "sun": {{"colour": [1, 1, 1], "glow_colour": [1, 1, 1], "direction": [0.3, 0.6, -0.7], "intensity": 1.0, "switches": 4195585}},
        "five": {{"sky5": true, "sky5_flags2": 0}},
        "camera": {{"position": [0, 1, -8], "forward": [0, 0, 1], "up": [0, 1, 0], "type": "PERSP", "lens": 40.4,
        "sensor_width": 36, "projection": "perspective", "frame_angle": 0.8377578413281667, "pan": [0, 0], "pixel_aspect": 1}},
        "materials": {{}}, "textures": [], "objects": [], "trees": []}}"#
    )
}

fn render(version: &str, flags: u32) -> Result<String, String> {
    let d = describe(&description(version, flags)).map_err(|e| e.0)?;
    // made-up numbers for the haze table a render as 5.0 may ask for; the comparison needs no Bryce data
    let choices = d.choices(OpenChoices { no_preview: true, haze_five: Some([0.5, 0.5, 0.11]), ..OpenChoices::default() });
    render_lit(&d.scene, d.version, &choices).map(|(p, _)| p.levels_sha256_hex()).map_err(|e| e.to_string())
}

#[test]
fn a_five_five_description_is_read_with_five_rules() {
    let d = describe(&description("5.5", 0)).unwrap_or_else(|e| panic!("{}", e.0));
    assert_eq!(d.version, RenderAs::Bryce50);
    assert!(d.five_five);
    assert!(d.choices(OpenChoices::default()).five_five);
    let d = describe(&description("5.0", 0)).unwrap_or_else(|e| panic!("{}", e.0));
    assert!(!d.five_five && !d.choices(OpenChoices::default()).five_five);
    let refused = describe(&description("5.6", 0)).err().expect("5.6 is no program");
    assert!(refused.0.contains(Program::NAMES), "{}", refused.0);
}

#[test]
fn a_plain_sky_renders_as_five_and_a_coloured_one_is_refused() {
    // the plain colour only: the same picture as 5.0
    let five = render("5.0", 0).expect("the plain sky as 5.0");
    assert_eq!(render("5.5", 0).expect("the plain sky as 5.5"), five);
    // the sky's own colours (bit 0x2): drawn as 5.0, refused as 5.5 with the reason
    let coloured = render("5.0", 0x2).expect("the coloured sky as 5.0");
    assert_ne!(coloured, five, "the switch gives the sky its own colours");
    let refused = render("5.5", 0x2).expect_err("refused as 5.5");
    assert!(refused.contains(FIVE_FIVE_MOON_DISC), "{refused}");
}

#[test]
fn a_render_mode_draws_no_sky_and_is_not_refused() {
    let mask = |version: &str| {
        let d = describe(&description(version, 0x2)).unwrap_or_else(|e| panic!("{}", e.0));
        let choices = d.choices(OpenChoices { no_preview: true, haze_five: Some([0.5, 0.5, 0.11]), ..OpenChoices::default() });
        let mode = RenderMode::parse("mask").expect("the mask mode");
        render_mode(&d.scene, d.version, &choices, mode).map(|(p, _)| p.levels_sha256_hex()).map_err(|e| e.to_string())
    };
    assert_eq!(mask("5.5").expect("the mask as 5.5"), mask("5.0").expect("the mask as 5.0"));
}
