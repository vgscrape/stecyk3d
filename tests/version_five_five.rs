// SPDX-License-Identifier: MIT
// Bryce 5.5 as a program: a description of version "5.5" is read with 5.0's rules; a sky with only the
// plain colour renders exactly as 5.0; a sky with its own colours renders, and differs from 5.0's only in the
// moon's disc, which 5.5 draws at half 5.0's angle (z·1.8·Δ/2). (docs/manual/substrate.md, "Bryce 5.5")

use stecyk3d::description::describe;
use stecyk3d::render::{OpenChoices, RenderMode, render_lit, render_mode};
use stecyk3d::sky::colours::SunInputs;
use stecyk3d::sky::sky_colours::{FiveColourSettings, SkyColours};
use stecyk3d::version::{Program, RenderAs};

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
fn a_plain_sky_renders_as_five_and_a_coloured_one_renders_too() {
    // the plain colour only: the same picture as 5.0
    let five = render("5.0", 0).expect("the plain sky as 5.0");
    assert_eq!(render("5.5", 0).expect("the plain sky as 5.5"), five);
    // the sky's own colours (bit 0x2): drawn as 5.0 and as 5.5; the moon is not in this view, so the same picture
    let coloured = render("5.0", 0x2).expect("the coloured sky as 5.0");
    assert_ne!(coloured, five, "the switch gives the sky its own colours");
    assert_eq!(render("5.5", 0x2).expect("the coloured sky as 5.5"), coloured);
}

/// The sun 13.5 degrees under the horizon, the moon 13.5 degrees up opposite it (a moonlit lake, Custom colours).
const SUN: [f64; 3] = [0.0, -0.233_445_363_855_905_4, -0.972_369_920_397_676_6];
const MOON: [f64; 3] = [0.0, 0.233_445_363_855_905_4, 0.972_369_920_397_676_6];

fn moonlit(five_five: bool, moon_disc: f64) -> SkyColours {
    let sun = SunInputs { direction: SUN, colour: [1.0, 1.0, 1.0], glow_colour: [1.0, 1.0, 1.0], intensity: 1.0 };
    let mut discs = [0.0; 15];
    discs[0] = 1.0;
    let settings = FiveColourSettings {
        switches: 0x2 | 0x8, // Glow look 1, Custom colours
        sky_colour: [0.005, 0.056, 0.09],
        horizon_colour: [0.06, 0.58, 0.88],
        haze: 0.0,
        moon: [220.0, 0.0, -30.0, 1.0],
        discs,
        glow_and_disc: [1.0, 1.0, 1.0, moon_disc],
    };
    SkyColours::new_bryce5_as(five_five, &sun, &settings).expect("a sky with colours of its own")
}

/// A unit direction `degrees` away from the moon.
fn off_moon(degrees: f64) -> [f64; 3] {
    let (s, c) = degrees.to_radians().sin_cos();
    [0.0, MOON[1] * c + MOON[2] * s, MOON[2] * c - MOON[1] * s]
}

/// Whether a direction is in the moon's disc: the disc strength (Moon disc) changes its colour there only.
fn in_disc(five_five: bool, degrees: f64) -> bool {
    moonlit(five_five, 1.0).colour(off_moon(degrees)) != moonlit(five_five, 3.0).colour(off_moon(degrees))
}

#[test]
fn five_five_draws_the_moons_disc_at_half_the_angle() {
    // 5.0's disc reaches 1.8 degrees (size 1 times 1.8), 5.5's 0.9
    assert!(in_disc(false, 1.75) && !in_disc(false, 1.85));
    assert!(in_disc(true, 0.85) && !in_disc(true, 0.95));
    // the same moonlit sky as 5.0 and as 5.5: the same colour everywhere outside 5.0's disc; between the two
    // discs' edges 5.0 draws the disc and 5.5 the sky. (Inside 5.5's disc the red and green edges, a fixed step
    // in the cosine past the disc's, fall elsewhere in the smaller disc, so the colours there may differ too.)
    let (five, five_five) = (moonlit(false, 1.0), moonlit(true, 1.0));
    for step in 0..=3000 {
        let degrees = f64::from(step) * 0.01; // 0 to 30 degrees from the moon
        let d = off_moon(degrees);
        if degrees > 1.85 {
            assert_eq!(five.colour_seen(d), five_five.colour_seen(d), "{degrees} degrees off the moon");
        } else if degrees > 0.95 && degrees < 1.75 {
            assert_ne!(five.colour(d), five_five.colour(d), "{degrees} degrees: in 5.0's disc, not in 5.5's");
            assert_eq!(five_five.colour_seen(d).1, moonlit(true, 3.0).colour_seen(d).1, "5.5 shows the sky there");
        }
    }
    // the sun's disc is 5.0's
    assert_eq!(five.colour_seen(SUN), five_five.colour_seen(SUN));
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
