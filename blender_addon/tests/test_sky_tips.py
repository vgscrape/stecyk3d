# SPDX-License-Identifier: GPL-3.0-or-later
"""The sky's tooltips and the Sun and moon switches name real settings and bits of their own."""

import sys
import unittest
from unittest import mock

sys.modules.setdefault("bpy", mock.MagicMock())

from stecyk3d import names  # noqa: E402
from stecyk3d import props  # noqa: E402


class SkyTips(unittest.TestCase):
    def test_every_tip_is_a_sky_setting(self):
        settings = {n for n, _k, _e in props.SKY}
        for name, tip in names.SKY_TIPS.items():
            self.assertIn(name, settings)
            self.assertTrue(tip and not tip.endswith("."), name)

    def test_the_moon_settings_say_what_they_do(self):
        self.assertIn("does not widen", names.SKY_TIPS["sky5_moon_glow_size"])
        self.assertIn("e^(-10*Z)", names.SKY_TIPS["sky5_moon_glow_size"])
        self.assertIn("0.7 + 0.5*a", names.SKY_TIPS["zenith"])
        self.assertIn("0.26", names.SKY_TIPS["sky5_star_intensity"])

    def test_sun_and_moon_switches_are_bits_of_their_own(self):
        flags = [bit for word, _p, rows in names.GROUP_SWITCHES["sky"] if word == "flags" for bit, _l, _d in rows]
        self.assertEqual(len(flags), len(set(flags)))
        for bit, label, description in names.SUN_MOON_SWITCHES:
            self.assertEqual(bit & (bit - 1), 0, label)
            self.assertIn("flag_%x" % bit, names.SWITCH_VIEWS)
        self.assertEqual({b for b, _l, _d in names.SUN_MOON_SWITCHES},
                         {0x4000, 0x10000, 0x20000, 0x40000, 0x100000, 0x2000000})


if __name__ == "__main__":
    unittest.main()
