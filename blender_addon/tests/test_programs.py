# SPDX-License-Identifier: GPL-3.0-or-later
# The program choices: Bryce 5.5 beside 3.0, 3.1 and 5.0, read and shown as 5.0 (docs/manual/substrate.md, "Bryce 5.5").
import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from stecyk3d import convert, defaults, environment, importing, names  # noqa: E402

RENDER = {"antialias": 1, "super_rays": 4, "premium_rays": 8, "tir_depth": 2, "spatial_kind": "TREE",
          "spatial_level": 1, "depth": 6, "units": 1.0}


class Programs(unittest.TestCase):
    def test_the_four_programs(self):
        self.assertEqual(list(names.PROGRAM), ["3.0", "3.1", "5.0", "5.5"])
        self.assertEqual(names.PROGRAM["5.5"], 3)  # a new number: the numbers saved in files stay theirs
        self.assertEqual([p for p in names.PROGRAM if names.is_five(p)], ["5.0", "5.5"])

    def test_five_five_is_described_as_five(self):
        five = convert.render_request(dict(RENDER, program="5.0"), 0)
        five_five = convert.render_request(dict(RENDER, program="5.5"), 0)
        self.assertEqual(five_five["version"], "5.5")
        self.assertEqual({k: v for k, v in five_five.items() if k != "version"},
                         {k: v for k, v in five.items() if k != "version"})
        with self.assertRaises(ValueError):
            convert.render_request(dict(RENDER, program="5.6"), 0)

    def test_five_five_takes_fives_settings(self):
        self.assertEqual(environment.free_bits("5.5"), environment.free_bits("5.0"))
        self.assertEqual(defaults.program_default("sky", "flags", "5.5"), defaults.program_default("sky", "flags", "5.0"))
        # going between 5.0 and 5.5 changes no default; from 3.0 to 5.5 as to 5.0
        self.assertIsNone(defaults.switched_value("sky", "flags", defaults.program_default("sky", "flags", "5.0"), "5.5"))
        three = defaults.program_default("sky", "flags", "3.0")
        self.assertEqual(defaults.switched_value("sky", "flags", three, "5.5"),
                         defaults.switched_value("sky", "flags", three, "5.0"))
        self.assertEqual(importing.reader_index("5.5"), importing.reader_index("5.0"))
        self.assertEqual(importing.reader_index("3.1"), 1)


if __name__ == "__main__":
    unittest.main()
