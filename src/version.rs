// SPDX-License-Identifier: MIT
//! The program version a scene is rendered as.
//!
//! (shared vocabulary). The three versions are those the entries name
//! (for example: "Bryce 3D 3.0, 3.1, or Bryce 5.0").
//!
//! `Program` is what a user names: one of the four programs, Bryce 5.5 among them. Bryce 5.5's
//! renderer is Bryce 5.0's apart from the differences found by comparing the two programs' code
//! (docs/manual/substrate.md, "Bryce 5.5"), so a render as 5.5 uses 5.0's rules (`RenderAs::Bryce50`)
//! with a flag for what 5.5 does differently (`OpenChoices::five_five`: the moon's disc).

/// Which program's behaviour a render reproduces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderAs {
    Bryce30,
    Bryce31,
    Bryce50,
}

/// A program a user may name: Bryce 3D 3.0 or 3.1, Bryce 5.0, or Bryce 5.5.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Program {
    Bryce30,
    Bryce31,
    Bryce50,
    Bryce55,
}

impl Program {
    /// Every program, in the order the choices list them.
    pub const ALL: [Program; 4] = [Program::Bryce30, Program::Bryce31, Program::Bryce50, Program::Bryce55];
    /// The names, for messages: "3.0, 3.1, 5.0 and 5.5".
    pub const NAMES: &'static str = "3.0, 3.1, 5.0 and 5.5";

    /// The program a name ("3.0", "3.1", "5.0", "5.5") stands for.
    pub fn parse(name: &str) -> Option<Program> {
        Program::ALL.into_iter().find(|p| p.name() == name)
    }

    /// The program's name: "3.0", "3.1", "5.0" or "5.5".
    pub fn name(self) -> &'static str {
        match self {
            Program::Bryce30 => "3.0",
            Program::Bryce31 => "3.1",
            Program::Bryce50 => "5.0",
            Program::Bryce55 => "5.5",
        }
    }

    /// The rules a render of the program uses: Bryce 5.5's are Bryce 5.0's.
    pub fn render_as(self) -> RenderAs {
        match self {
            Program::Bryce30 => RenderAs::Bryce30,
            Program::Bryce31 => RenderAs::Bryce31,
            Program::Bryce50 | Program::Bryce55 => RenderAs::Bryce50,
        }
    }

    /// Bryce 5.5, whose known differences from 5.0 a render draws (`OpenChoices::five_five`).
    pub fn is_five_five(self) -> bool {
        self == Program::Bryce55
    }
}

impl From<RenderAs> for Program {
    fn from(v: RenderAs) -> Program {
        match v {
            RenderAs::Bryce30 => Program::Bryce30,
            RenderAs::Bryce31 => Program::Bryce31,
            RenderAs::Bryce50 => Program::Bryce50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn programs_by_name() {
        for p in Program::ALL {
            assert_eq!(Program::parse(p.name()), Some(p));
        }
        assert_eq!(Program::parse("5.5").map(Program::render_as), Some(RenderAs::Bryce50));
        assert_eq!(Program::parse("5"), None);
        assert_eq!(Program::parse("5.50"), None);
        assert_eq!(Program::from(RenderAs::Bryce31), Program::Bryce31);
        assert!(Program::Bryce55.is_five_five() && !Program::Bryce50.is_five_five());
        let names: Vec<&str> = Program::ALL.iter().map(|p| p.name()).collect();
        assert_eq!(Program::NAMES, format!("{}, {}, {} and {}", names[0], names[1], names[2], names[3]));
    }
}
