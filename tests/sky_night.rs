// SPDX-License-Identifier: MIT
// A night sky of Bryce 5 with Custom colours (a moonlit lake): the rules the manual states for it
// (docs/manual/blender.md, "Sun, moon and stars"), checked through the sky's own colours as 5.0. They are
// properties of the entry's formulas, not new numbers: the sun-side factor 0.7 + 0.5·a of step 4, the
// moon's strength e^(−10·Zmax)·Gm of step 5, the disc's lift Dm·β of step 6 and the stars' visibility Ψ.

use stecyk3d::sky::colours::SunInputs;
use stecyk3d::sky::sky_colours::{FiveColourSettings, SkyColours};

/// The sun 13.5 degrees under the horizon behind the camera, so the moon is 13.5 degrees up ahead.
const SUN: [f64; 3] = [0.0, -0.233_445_363_855_905_4, -0.972_369_920_397_676_6];
const MOON: [f64; 3] = [0.0, 0.233_445_363_855_905_4, 0.972_369_920_397_676_6];
/// W: Glow look 1 and Custom colours; no haze (bit 0x1), so the band and the stars' haze fade are 0.
const CUSTOM: u32 = 0x2 | 0x8;
/// β, what a disc strength of 1 adds to each channel inside the disc.
const BETA: f64 = 0.3051804379339;

fn sky(zenith: [f32; 3], glow_and_disc: [f64; 4]) -> SkyColours {
    let sun = SunInputs { direction: SUN, colour: [1.0, 1.0, 1.0], glow_colour: [1.0, 1.0, 1.0], intensity: 1.0 };
    let mut discs = [0.0; 15];
    discs[0] = 1.0;
    let settings = FiveColourSettings {
        switches: CUSTOM,
        sky_colour: zenith,
        horizon_colour: [0.06, 0.58, 0.88],
        haze: 0.0,
        moon: [220.0, 0.0, -30.0, 1.0],
        discs,
        glow_and_disc,
    };
    SkyColours::new_bryce5(&sun, &settings).expect("a sky with colours of its own")
}

/// A unit direction `degrees` away from the moon, turned about the x axis.
fn off_moon(degrees: f64) -> [f64; 3] {
    let (s, c) = degrees.to_radians().sin_cos();
    [0.0, MOON[1] * c + MOON[2] * s, MOON[2] * c - MOON[1] * s]
}

const LAKE: [f32; 3] = [0.05, 0.56, 0.90];

// Step 4: with the sun's and the moon's strengths 0 nothing else touches the colour, so a ray sees the zenith
// colour times 0.7 + 0.5·a, a = (d·S + 1)/2: 0.7 away from the sun, 0.95 square to it, 1.2 toward it.
#[test]
fn custom_colours_are_seen_times_the_sun_side_factor() {
    let none = sky(LAKE, [0.0; 4]);
    for (d, factor) in [(off_moon(5.0), 0.700_951), ([1.0, 0.0, 0.0], 0.95), (SUN, 1.2)] {
        let c = none.colour(d);
        for k in 0..3 {
            let ratio = f64::from(c[k]) / f64::from(LAKE[k]);
            assert!((ratio - factor).abs() < 1e-6, "{d:?} channel {k}: {ratio}, not {factor}");
        }
    }
}

// Step 5: the moon's strength ν is e^(−10·Zmax) times Moon glow size. Under the lake's zenith (Zmax 0.9) a
// glow size of 20 moves no level next to the disc; under a dark zenith the same change is large.
#[test]
fn a_bright_zenith_hides_the_moon_glow() {
    let near = off_moon(3.0);
    let level = |c: [f32; 3]| c.map(|x| (f64::from(x) * 255.0).round());
    let bright = (sky(LAKE, [1.0, 1.0, 1.0, 1.0]).colour(near), sky(LAKE, [1.0, 20.0, 1.0, 1.0]).colour(near));
    assert_eq!(level(bright.0), level(bright.1), "{bright:?}");
    let dark_zenith = [0.005, 0.056, 0.09];
    let dark = (sky(dark_zenith, [1.0, 1.0, 1.0, 1.0]).colour(near), sky(dark_zenith, [1.0, 20.0, 1.0, 1.0]).colour(near));
    assert!(f64::from(dark.1[2] - dark.0[2]) > 0.1, "{dark:?}");
}

// Step 6: inside the moon's disc Moon disc adds Dm·β to each channel (brightness, never size).
#[test]
fn moon_disc_lifts_the_disc_by_beta() {
    let one = sky(LAKE, [1.0, 1.0, 1.0, 1.0]).colour(MOON);
    let two = sky(LAKE, [1.0, 1.0, 1.0, 2.0]).colour(MOON);
    for k in 0..3 {
        assert!((f64::from(two[k] - one[k]) - BETA).abs() < 1e-6, "channel {k}: {one:?} {two:?}");
    }
    // The disc keeps its size: 2 degrees off the moon is outside it whatever Dm.
    let edge = off_moon(2.0);
    assert_eq!(sky(LAKE, [1.0, 1.0, 1.0, 1.0]).colour(edge), sky(LAKE, [1.0, 1.0, 1.0, 5.0]).colour(edge));
}

// Ψ: with no haze, a ray's stars are laid with weight 1 − 3.8·(the sky's brightest channel), held to [0, 1], so a
// sky whose brightest channel reaches 1/3.8 shows no star whatever their intensity.
#[test]
fn a_bright_sky_hides_the_stars() {
    let d = [1.0, 0.0, 0.0];
    let (c, seen) = sky(LAKE, [1.0, 1.0, 1.0, 1.0]).colour_seen(d);
    assert!(c[2] > 0.27 && seen == 0.0, "{c:?} {seen}");
    let (c, seen) = sky([0.01, 0.05, 0.1], [1.0, 1.0, 1.0, 1.0]).colour_seen(d);
    let brightest = c.iter().map(|&x| f64::from(x)).fold(0.0, f64::max);
    assert!((seen - (1.0 - 3.8 * brightest)).abs() < 1e-12 && seen > 0.5, "{c:?} {seen}");
}
