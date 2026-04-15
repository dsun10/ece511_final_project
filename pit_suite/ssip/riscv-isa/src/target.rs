// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! RISC-V target description.
//!
//! This module contains types for describing a RISC-V [`Target`], meaning
//! an implementation of the RISC-V ISA supporting some number of modes and
//! extensions.

use std::fmt::{self, Display};
use std::str::FromStr;

/// RISC-V target configuration.
///
/// # Example
///
/// ```rust
/// # use std::str::FromStr;
/// # use riscv_isa::{Target, Xlen};
/// let target = Target::from_str("RV32IMADCZifencei_Zicsr").unwrap();
///
/// assert_eq!(target.xlen(), Xlen::Rv32);
/// assert!(target.has_ext("I"));
/// assert!(target.has_ext("M"));
/// assert!(target.has_ext("A"));
/// assert!(target.has_ext("F")); // Implied by `D`.
/// assert!(target.has_ext("D"));
/// assert!(target.has_ext("C"));
/// assert!(target.has_ext("Zca")); // Sub-extension of `C`.
/// assert!(target.has_ext("Zifencei"));
/// assert!(target.has_ext("Zicsr"));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Target {
    /// Register width.
    pub(crate) xlen: Xlen,
    /// Instruction set base.
    pub(crate) base: BaseIsa,
    /// Whether instructions from the privileged spec are supported.
    pub privileged: bool,
    /// Whether S-mode (supervisor) instructions are supported.
    pub supervisor_mode: bool,
    /// M standard extension (integer multiply and divide).
    pub(crate) m: bool,
    /// A standard extension (atomics).
    pub(crate) a: bool,
    /// F standard extension (32-bit floating point).
    pub(crate) f: bool,
    /// D standard extension (64-bit (double) floating point).
    pub(crate) d: bool,
    /// Q standard extension (128-bit (quad) floating point).
    pub(crate) q: bool,
    /// Zicsr standard extension.
    pub(crate) zicsr: bool,
    /// Zifencei standard extension.
    pub(crate) zifencei: bool,
    /// Zawrs standard extension.
    pub(crate) zawrs: bool,
    /// Zfh standard extension (16-bit (half) floating point).
    pub(crate) zfh: bool,
    /// Zca standard extension (16-bit compressed instructions).
    pub(crate) zca: bool,
    /// Zcf standard extension (compressed float instructions, RV32 only).
    pub(crate) zcf: bool,
    /// Zcd standard extension (compressed double instructions).
    pub(crate) zcd: bool,
    /// Zba standard extension (bitmanip).
    pub(crate) zba: bool,
    /// Zbb standard extension (bitmanip).
    pub(crate) zbb: bool,
    /// Zbc standard extension (bitmanip).
    pub(crate) zbc: bool,
    /// Zbkb standard extension (bitmanip for cryptography).
    pub(crate) zbkb: bool,
    /// Zbs standard extension (bitmanip).
    pub(crate) zbs: bool,
}

/// Errors when parsing RISC-V target strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseTargetError {
    /// Unknown prefix (expecting `RV` or `RISCV`).
    Prefix,
    /// Unknown register width (expecting `32` or `64`).
    Xlen,
    /// Missing or unknown base (expecting `I` or `E`).
    Base,
    /// Unknown extension string (expecting e.g. `M` or `Zicsr`).
    UnknownExt,
    /// String is parseable, but does not match the target string layout.
    ///
    /// The strict layout requires:
    ///
    /// * The string begins `RV32` or `RV64`.
    /// * Single-letter extensions must be in the order given by the spec.
    /// * Z-extensions must come at the end.
    /// * Z-extensions are first ordered into groups by their second character,
    ///   then alphabetically.
    /// * Multi-character extensions must be separated by underscores.
    Strict,
    /// An extension is incompatible with another extension or the XLEN.
    Incompatible,
}

impl FromStr for Target {
    type Err = ParseTargetError;

    /// Parse a RISC-V target from a target string, e.g.
    /// `RV64IMAFDZicsr_Zifencei`
    ///
    /// Target strings are case-insensitive. The non-standard `riscv` prefix is
    /// accepted due to its use in target tuples/triples.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        let rest = s
            .strip_prefix("rv")
            .or_else(|| s.strip_prefix("riscv"))
            .ok_or(ParseTargetError::Prefix)?;

        let xlen = match &rest.get(..2) {
            Some("32") => Xlen::Rv32,
            Some("64") => Xlen::Rv64,
            _ => return Err(ParseTargetError::Xlen),
        };
        let rest = &rest[2..];

        let Some((base, rest)) = rest.split_at_checked(1) else {
            return Err(ParseTargetError::Base);
        };
        let base = match base {
            "i" => BaseIsa::I,
            "e" => BaseIsa::E,
            _ => return Err(ParseTargetError::Base),
        };

        let mut target = Target { xlen, base, ..Default::default() };

        let mut c_enabled = false;

        let mut extensions = rest;
        while !extensions.is_empty() {
            let (ext, rest) = extension(extensions);
            extensions = rest;

            match ext {
                "i" if base == BaseIsa::E => return Err(ParseTargetError::Incompatible),
                "e" if base == BaseIsa::I => return Err(ParseTargetError::Incompatible),
                "m" => target.m = true,
                "a" => target.a = true,
                "f" => {
                    target.f = true;
                    target.zicsr = true;
                }
                "d" => {
                    target.f = true;
                    target.d = true;
                    target.zicsr = true;
                }
                "q" => {
                    target.f = true;
                    target.d = true;
                    target.q = true;
                    target.zicsr = true;
                }
                "c" => c_enabled = true,
                "b" => {
                    target.zba = true;
                    target.zbb = true;
                    target.zbs = true;
                }
                "g" => {
                    target.m = true;
                    target.a = true;
                    target.f = true;
                    target.d = true;
                    target.zicsr = true;
                    target.zifencei = true;
                }
                "zicsr" => target.zicsr = true,
                "zifencei" => target.zifencei = true,
                "zawrs" => target.zawrs = true,
                "zfh" => {
                    target.f = true;
                    target.zicsr = true;
                    target.zfh = true;
                }
                "zca" => target.zca = true,
                "zcf" => {
                    if target.xlen != Xlen::Rv32 {
                        return Err(ParseTargetError::Incompatible);
                    }
                    target.zca = true;
                    target.zcf = true;
                }
                "zcd" => {
                    target.zca = true;
                    target.zcd = true;
                }
                "zba" => target.zba = true,
                "zbb" => target.zbb = true,
                "zbc" => target.zbc = true,
                "zbkb" => target.zbkb = true,
                "zbs" => target.zbs = true,
                _ => return Err(ParseTargetError::UnknownExt),
            }
        }

        if c_enabled {
            target.zca = true;
            if target.f && target.xlen == Xlen::Rv32 {
                target.zcf = true;
            }
            if target.d {
                target.zcd = true;
            }
        }

        Ok(target)
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.xlen {
            Xlen::Rv32 => f.write_str("RV32")?,
            Xlen::Rv64 => f.write_str("RV64")?,
        }

        let g = self.m && self.a && self.f && self.d && self.zicsr && self.zifencei;
        let c =
            self.zca && (!self.f || self.zcf || self.xlen != Xlen::Rv32) && (!self.d || self.zcd);
        let b = self.zba && self.zbb && self.zbs;
        let extensions = [
            (self.base == BaseIsa::I, "I"),
            (self.base == BaseIsa::E, "E"),
            (self.m && !g, "M"),
            (self.a && !g, "A"),
            (self.f && !g, "F"),
            (self.d && !g, "D"),
            (g, "G"),
            (self.q, "Q"),
            (c, "C"),
            (b, "B"),
            (self.zicsr && !g, "Zicsr"),
            (self.zifencei && !g, "Zifencei"),
            (self.zawrs, "Zawrs"),
            (self.zfh, "Zfh"),
            (self.zca && !c, "Zca"),
            (self.zcf && !c, "Zcf"),
            (self.zcd && !c, "Zcd"),
            (self.zba && !b, "Zba"),
            (self.zbb && !b, "Zbb"),
            (self.zbc, "Zbc"),
            (self.zbkb, "Zbkb"),
            (self.zbs && !b, "Zbs"),
        ];

        let mut underscore = false;
        for extension in extensions.into_iter().filter_map(|(en, s)| en.then_some(s)) {
            // Enable underscores after the first multi-character extension has printed.
            if underscore {
                f.write_str("_")?;
            }
            if extension.len() > 1 {
                underscore = true;
            }

            f.write_str(extension)?;
        }

        Ok(())
    }
}

impl Target {
    /// Get the supported register width (XLEN) of the target.
    pub fn xlen(&self) -> Xlen {
        self.xlen
    }

    /// Get the supported base integer instruction set (`I` or `E`).
    pub fn base(&self) -> BaseIsa {
        self.base
    }

    /// Create a new `Target` with privileged instructions enabled or disabled.
    pub fn with_privileged(mut self, enabled: bool) -> Self {
        self.privileged = enabled;
        self
    }

    /// Create a new `Target` with supervisor (S) mode enabled or disabled.
    pub fn with_s_mode(mut self, enabled: bool) -> Self {
        self.supervisor_mode = enabled;
        self
    }

    /// Check whether this target supports an extension.
    ///
    /// The given extension is case-insensitive and must not be multiple extensions.
    /// Both supersets and subsets of extension groups can be used, i.e. if
    /// `target.has_ext("G")` is true then `targets.has_ext("M")` will also be true.
    ///
    /// Register widths can be given as either `X`, `rvX`, or `rvXi` (e.g. `32`,
    /// `rv32`, `rv32i`).
    ///
    /// Examples:
    ///
    /// ```rust
    /// # use std::str::FromStr;
    /// # use riscv_isa::Target;
    /// let target = Target::from_str("RV32IMACZba_Zbb_Zbs").unwrap();
    ///
    /// // Single-letter extensions:
    /// assert_eq!(target.has_ext("M"), true);
    /// assert_eq!(target.has_ext("F"), false);
    ///
    /// // Multi-letter extensions:
    /// assert_eq!(target.has_ext("Zifencei"), false);
    ///
    /// // Supersets and subsets of extensions:
    /// assert_eq!(target.has_ext("B"), true);
    /// assert_eq!(target.has_ext("Zba"), true);
    ///
    /// // Register widths:
    /// assert_eq!(target.has_ext("RV32"), true);
    /// assert_eq!(target.has_ext("RV64"), false);
    /// ```
    pub fn has_ext(&self, extension: &str) -> bool {
        match extension.to_ascii_lowercase().as_str() {
            "rv32" | "32" => self.xlen == Xlen::Rv32,
            "rv64" | "64" => self.xlen == Xlen::Rv64,
            "i" => self.base == BaseIsa::I,
            "e" => self.base == BaseIsa::E,
            "m" => self.m,
            "a" => self.a,
            "f" => self.f,
            "d" => self.d,
            "g" => self.m && self.a && self.f && self.d && self.zicsr && self.zifencei,
            "q" => self.q,
            "c" => {
                let zcf = !self.f || self.zcf || self.xlen != Xlen::Rv32;
                let zcd = !self.d || self.zcd;
                self.zca && zcf && zcd
            }
            "b" => self.zba && self.zbb && self.zbs,
            "zicsr" => self.zicsr,
            "zifencei" => self.zifencei,
            "zawrs" => self.zawrs,
            "zfh" => self.zfh,
            "zca" => self.zca,
            "zcf" => self.zcf,
            "zcd" => self.zcd,
            "zba" => self.zba,
            "zbb" => self.zbb,
            "zbc" => self.zbc,
            "zbkb" => self.zbkb,
            "zbs" => self.zbs,
            _ => false,
        }
    }

    /// Check whether this target contains another, i.e. it is a superset of the other.
    ///
    /// All features of `other` must be supported by `self` for this to be true.
    ///
    /// Note that two targets with different `XLEN`s will always return `false` because
    /// instruction encodings will have strict differences between the two.
    pub fn contains(&self, other: &Target) -> bool {
        (other.xlen == self.xlen)
            && (other.base == self.base)
            && (!other.privileged || self.privileged)
            && (!other.supervisor_mode || self.supervisor_mode)
            && (!other.m || self.m)
            && (!other.a || self.a)
            && (!other.f || self.f)
            && (!other.d || self.d)
            && (!other.q || self.q)
            && (!other.zicsr || self.zicsr)
            && (!other.zifencei || self.zifencei)
            && (!other.zawrs || self.zawrs)
            && (!other.zfh || self.zfh)
            && (!other.zca || self.zca)
            && (!other.zcf || self.zcf)
            && (!other.zcd || self.zcd)
            && (!other.zba || self.zba)
            && (!other.zbb || self.zbb)
            && (!other.zbc || self.zbc)
            && (!other.zbkb || self.zbkb)
            && (!other.zbs || self.zbs)
    }

    /// Check whether this target supports any compressed instructions.
    ///
    /// This can be used to decide whether or not to consider unaligned 16-bit
    /// instructions when decoding.
    pub fn compressed(&self) -> bool {
        self.zca || self.zcf || self.zcd
    }

    /// Parse a target string, only accepting those which match the spec.
    ///
    /// The strict layout requires:
    ///
    /// * The string begins `RV32` or `RV64`.
    /// * Single-letter extensions must be in the order given by the spec.
    /// * Z-extensions must come at the end.
    /// * Z-extensions are first ordered into groups by their second character,
    ///   then alphabetically.
    /// * Multi-character extensions must be separated by underscores.
    pub fn from_str_strict(s: &str) -> Result<Self, ParseTargetError> {
        let target = Target::from_str(s)?;

        if s != target.to_string() {
            return Err(ParseTargetError::Strict);
        }

        Ok(target)
    }
}

/// Supported integer register widths.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Xlen {
    /// 32-bit registers.
    #[default]
    Rv32,
    /// 64-bit registers.
    Rv64,
}

impl Xlen {
    /// Width of integer registers in bits.
    pub fn bits(&self) -> usize {
        match self {
            Xlen::Rv32 => 32,
            Xlen::Rv64 => 64,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BaseIsa {
    /// Base RISC-V ISA with 32 integer registers.
    #[default]
    I,
    /// Base RISC-V ISA with 16 integer registers.
    E,
}

impl BaseIsa {
    /// Number of integer registers available.
    pub fn registers(&self) -> usize {
        match self {
            BaseIsa::I => 32,
            BaseIsa::E => 16,
        }
    }
}

/// Parse a single extension from the start of the given string.
///
/// Returns the parsed extension and the rest of the string still to parse.
fn extension(s: &str) -> (&str, &str) {
    let len = if s.starts_with('z')
        || s.starts_with('x')
        || s.starts_with("sv")
        || s.starts_with("ss")
        || s.starts_with("sh")
        || s.starts_with("sm")
    {
        s.find('_').unwrap_or(s.len())
    } else {
        1
    };

    let mut next_index = len;
    if s[len..].starts_with('_') {
        next_index += 1;
    }
    let rest = &s[next_index..];

    (&s[..len], rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extensions() {
        assert_eq!(extension("a"), ("a", ""));
        assert_eq!(extension("zifencei"), ("zifencei", ""));
        assert_eq!(extension("a_"), ("a", ""));
        assert_eq!(extension("zifencei_"), ("zifencei", ""));
        assert_eq!(extension("am"), ("a", "m"));
        assert_eq!(extension("a_m"), ("a", "m"));
        assert_eq!(extension("a_zifencei"), ("a", "zifencei"));
        assert_eq!(extension("zifencei_a"), ("zifencei", "a"));
        assert_eq!(extension("zifencei_zicsr"), ("zifencei", "zicsr"));
    }

    #[test]
    fn from_str() {
        #[track_caller]
        fn check(s: &str, expected: Target) {
            let got = Target::from_str(s).unwrap();
            assert_eq!(expected, got);
        }
        #[track_caller]
        fn check_err(s: &str, error: ParseTargetError) {
            let got = Target::from_str(s);
            assert_eq!(Err(error), got);
        }
        let def = Target::default();

        // Prefix and xlen:
        check("RV32I", Target { xlen: Xlen::Rv32, ..def });
        check("RV64I", Target { xlen: Xlen::Rv64, ..def });
        check("RISCV32I", Target { xlen: Xlen::Rv32, ..def });
        check("RISCV64I", Target { xlen: Xlen::Rv64, ..def });
        check(
            "RV32E",
            Target { xlen: Xlen::Rv32, base: BaseIsa::E, ..def },
        );
        check(
            "RV64E",
            Target { xlen: Xlen::Rv64, base: BaseIsa::E, ..def },
        );

        // Check some common extensions individually:
        check("RV32IM", Target { m: true, ..def });
        check("RV32IA", Target { a: true, ..def });
        check("RV32IF", Target { f: true, zicsr: true, ..def });
        check("RV32ID", Target { f: true, d: true, zicsr: true, ..def });
        check("RV32IZifencei", Target { zifencei: true, ..def });
        check("RV32IZicsr", Target { zicsr: true, ..def });

        // Multiple extensions:
        check("RV32IMA", Target { m: true, a: true, ..def });
        check("RV32IZicsr", Target { zicsr: true, ..def });
        check("RV32IMZicsr", Target { m: true, zicsr: true, ..def });
        check("RV32IM_Zicsr", Target { m: true, zicsr: true, ..def });
        check("RV32IZicsr_M", Target { m: true, zicsr: true, ..def });
        check(
            "RV32IZicsr_Zifencei",
            Target { zicsr: true, zifencei: true, ..def },
        );

        // Group of extensions:
        check("RV32IB", Target { zba: true, zbb: true, zbs: true, ..def });

        // Errors:
        check_err("RV32", ParseTargetError::Base);
        check_err("RV64", ParseTargetError::Base);
        check_err("RV32IE", ParseTargetError::Incompatible);
        check_err("RV64IE", ParseTargetError::Incompatible);
        check_err("RV64IZcf", ParseTargetError::Incompatible);
    }

    #[test]
    fn display() {
        #[track_caller]
        fn check_becomes(s: &str, expected: &str) {
            let got = Target::from_str(s).unwrap().to_string();
            assert_eq!(got, expected);
        }
        #[track_caller]
        fn check(s: &str) {
            check_becomes(s, s);
        }

        check("RV32I");
        check("RV64IMAC");

        // Multi-character extensions:
        check("RV32IMACZicsr");
        check("RV32IMACZicsr_Zba_Zbkb");

        // Check that the three `B` sub-extensions collapse into one:
        check_becomes("RV32IZba", "RV32IZba");
        check_becomes("RV32IZba_Zbb", "RV32IZba_Zbb");
        check_becomes("RV32IZba_Zbb_Zbs", "RV32IB");
    }

    #[test]
    fn parse_strict() {
        #[track_caller]
        fn check(s: &str, accept: bool) {
            let got = Target::from_str_strict(s);
            let not_ = not(accept);
            assert!(got.is_ok() == accept, "expected '{s}' {not_}to be accepted");
        }

        check("RV32I", true);
        check("RV64I", true);
        check("RV32", false); // Missing I.
        check("RV32M", false); // Missing I.
        check("RV256I", false); // Unsupported XLEN.

        check("RV64IMAC", true);
        check("RV64IMCA", false); // Wrong order.

        check("RV32IMZba", true);
        check("RV32IMZba_Zbb", true);
        check("RV32IMZicsr_Zba", true);
        check("RV32IMZbb_Zba", false); // Wrong order.
        check("RV32IMZba_Zicsr", false); // Wrong order.
        check("RV32IMZbaZbb", false); // Missing underscore.
    }

    #[test]
    fn supports() {
        #[track_caller]
        fn check(target: &str, ext: &str, accept: bool) {
            let target = Target::from_str(target).unwrap();
            let not_ = not(accept);
            assert!(
                target.has_ext(ext) == accept,
                "expected '{target}' {not_}to support '{ext}'"
            );
        }

        check("RV32I", "RV32", true);
        check("RV32I", "rv32", true);
        check("RV32I", "32", true);
        check("RV32I", "I", true);
        check("RV64I", "RV64", true);
        check("RV64I", "rv64", true);
        check("RV64I", "64", true);
        check("RV64I", "I", true);

        check("RV64I", "RV32", false);
        check("RV32I", "RV64", false);

        check("RV32IMAC", "I", true);
        check("RV32IMAC", "M", true);
        check("RV32IMAC", "A", true);
        check("RV32IMAC", "C", true);
        check("RV32IZifencei", "Zifencei", true);
        check("RV32IZifencei", "zIfEnCeI", true);

        check("RV32IMAC", "D", false);
        check("RV32IMAC", "Zifencei", false);

        check("RV32IB", "Zba", true);
        check("RV32IB", "Zbb", true);
        check("RV32IB", "Zbc", false);
        check("RV32IB", "F", false);

        check("RV32IC", "C", true);
        check("RV32IC", "Zca", true);
        check("RV32IC", "Zcf", false);
        check("RV32IC", "Zcd", false);
        check("RV32IFC", "Zcf", true);
        check("RV32IFC", "Zcd", false);
        check("RV32IFDC", "Zcd", true);
        check("RV64IC", "Zca", true);
        check("RV64IC", "Zcf", false);
        check("RV64IFC", "Zcf", false);
        check("RV64IFDC", "Zcf", false);
        check("RV64IFDC", "Zcd", true);
    }

    #[test]
    fn contains() {
        #[track_caller]
        fn check(lhs: &str, rhs: &str, accept: bool) {
            let lhs = Target::from_str(lhs).unwrap();
            let rhs = Target::from_str(rhs).unwrap();
            let not_ = not(accept);
            assert!(
                lhs.contains(&rhs) == accept,
                "expected {lhs}.contains({rhs}) {not_}to be accepted"
            );
        }

        check("RV32I", "RV32I", true);
        check("RV32IM", "RV32I", true);
        check("RV32IMA", "RV32I", true);
        check("RV32IA", "RV32I", true);
        check("RV32IMAFDQCZicsr_Zifencei", "RV32I", true);

        check("RV32IMAC", "RV32I", true);
        check("RV32IMAC", "RV32IM", true);
        check("RV32IMAC", "RV32IC", true);
        check("RV32IMAC", "RV32IMC", true);
        check("RV32IMAC", "RV32IMAC", true);

        check("RV32I", "RV32IM", false);
        check("RV32I", "RV32IMAC", false);
        check("RV32IMAC", "RV32ID", false);
        check("RV32IMAC", "RV32IMACD", false);

        check("RV32I", "RV64I", false);
        check("RV32IMAC", "RV64I", false);
    }

    /// Helper for inserting `not ` in an expectation string.
    fn not(expectation: bool) -> &'static str {
        match expectation {
            true => "",
            false => "not ",
        }
    }
}
