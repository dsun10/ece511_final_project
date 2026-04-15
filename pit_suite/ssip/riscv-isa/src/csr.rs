// Copyright James Wainwright
//
// SPDX-License-Identifier: MPL-2.0

//! Control and status register definitions.
//!
//! # Example
//!
//! ```rust
//! # use std::str::FromStr;
//! # use riscv_isa::Csr;
//! assert_eq!(Csr::decode(0x342).unwrap().name, "mcause");
//! assert_eq!(Csr::from_str("time").unwrap().number, 0xC01);
//! ```

use std::fmt::{self, Display};
use std::str::FromStr;

use crate::Privilege;

/// Control and status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Csr {
    /// Official name given in the spec.
    pub name: &'static str,
    /// 12-bit ID number.
    pub number: u32,
    /// Minimum privilege mode that the CSR can be accessed from.
    pub mode: Privilege,
    /// Whether tthe CSR can be written to.
    pub write: bool,
}

impl Csr {
    /// Generate a CSR with the given name, ID number, and privilege.
    ///
    /// The privilege string should match what's listed in the spec, e.g.
    /// `urw` or `sro` for "user read/write" and "supervisor read-only".
    const fn new(number: u32, privilege: &str, name: &'static str) -> Csr {
        let mode = match privilege.as_bytes()[0] {
            b'u' => Privilege::User,
            b's' => Privilege::Supervisor,
            b'h' => Privilege::Hypervisor,
            b'm' => Privilege::Machine,
            b'd' => Privilege::Debug,
            _ => panic!("unknown privilege mode"),
        };
        assert!(privilege.as_bytes()[1] == b'r');
        let write = match privilege.as_bytes()[2] {
            b'w' => true,
            b'o' => false,
            _ => panic!("unknown write flag"),
        };

        Csr { name, number, mode, write }
    }

    /// Find a CSR with the given ID number.
    pub fn decode(number: u32) -> Option<Csr> {
        CSR_TABLE.iter().find(|csr| csr.number == number).copied()
    }
}

impl FromStr for Csr {
    type Err = &'static str;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let csr = CSR_TABLE.iter().find(|csr| csr.name == name).copied();
        csr.ok_or("unknown CSR name")
    }
}

impl Display for Csr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

/// List of all known standard CSRs.
///
/// Taken from the privileged ISA spec version `20240411`.
static CSR_TABLE: [Csr; 367] = [
    // Unprivileged floating-point:
    Csr::new(0x001, "urw", "fflags"),
    Csr::new(0x002, "urw", "frm"),
    Csr::new(0x003, "urw", "fcsr"),
    // Unprivileged counters and timers:
    Csr::new(0xC00, "uro", "cycle"),
    Csr::new(0xC01, "uro", "time"),
    Csr::new(0xC02, "uro", "instret"),
    Csr::new(0xC03, "uro", "hpmcounter3"),
    Csr::new(0xC04, "uro", "hpmcounter4"),
    Csr::new(0xC05, "uro", "hpmcounter5"),
    Csr::new(0xC06, "uro", "hpmcounter6"),
    Csr::new(0xC07, "uro", "hpmcounter7"),
    Csr::new(0xC08, "uro", "hpmcounter8"),
    Csr::new(0xC09, "uro", "hpmcounter9"),
    Csr::new(0xC0A, "uro", "hpmcounter10"),
    Csr::new(0xC0B, "uro", "hpmcounter11"),
    Csr::new(0xC0C, "uro", "hpmcounter12"),
    Csr::new(0xC0D, "uro", "hpmcounter13"),
    Csr::new(0xC0E, "uro", "hpmcounter14"),
    Csr::new(0xC0F, "uro", "hpmcounter15"),
    Csr::new(0xC10, "uro", "hpmcounter16"),
    Csr::new(0xC11, "uro", "hpmcounter17"),
    Csr::new(0xC12, "uro", "hpmcounter18"),
    Csr::new(0xC13, "uro", "hpmcounter19"),
    Csr::new(0xC14, "uro", "hpmcounter20"),
    Csr::new(0xC15, "uro", "hpmcounter21"),
    Csr::new(0xC16, "uro", "hpmcounter22"),
    Csr::new(0xC17, "uro", "hpmcounter23"),
    Csr::new(0xC18, "uro", "hpmcounter24"),
    Csr::new(0xC19, "uro", "hpmcounter25"),
    Csr::new(0xC1A, "uro", "hpmcounter26"),
    Csr::new(0xC1B, "uro", "hpmcounter27"),
    Csr::new(0xC1C, "uro", "hpmcounter28"),
    Csr::new(0xC1D, "uro", "hpmcounter29"),
    Csr::new(0xC1E, "uro", "hpmcounter30"),
    Csr::new(0xC1F, "uro", "hpmcounter31"),
    Csr::new(0xC80, "uro", "cycleh"),
    Csr::new(0xC81, "uro", "timeh"),
    Csr::new(0xC82, "uro", "instreth"),
    Csr::new(0xC83, "uro", "hpmcounter3h"),
    Csr::new(0xC84, "uro", "hpmcounter4h"),
    Csr::new(0xC85, "uro", "hpmcounter5h"),
    Csr::new(0xC86, "uro", "hpmcounter6h"),
    Csr::new(0xC87, "uro", "hpmcounter7h"),
    Csr::new(0xC88, "uro", "hpmcounter8h"),
    Csr::new(0xC89, "uro", "hpmcounter9h"),
    Csr::new(0xC8A, "uro", "hpmcounter10h"),
    Csr::new(0xC8B, "uro", "hpmcounter11h"),
    Csr::new(0xC8C, "uro", "hpmcounter12h"),
    Csr::new(0xC8D, "uro", "hpmcounter13h"),
    Csr::new(0xC8E, "uro", "hpmcounter14h"),
    Csr::new(0xC8F, "uro", "hpmcounter15h"),
    Csr::new(0xC90, "uro", "hpmcounter16h"),
    Csr::new(0xC91, "uro", "hpmcounter17h"),
    Csr::new(0xC92, "uro", "hpmcounter18h"),
    Csr::new(0xC93, "uro", "hpmcounter19h"),
    Csr::new(0xC94, "uro", "hpmcounter20h"),
    Csr::new(0xC95, "uro", "hpmcounter21h"),
    Csr::new(0xC96, "uro", "hpmcounter22h"),
    Csr::new(0xC97, "uro", "hpmcounter23h"),
    Csr::new(0xC98, "uro", "hpmcounter24h"),
    Csr::new(0xC99, "uro", "hpmcounter25h"),
    Csr::new(0xC9A, "uro", "hpmcounter26h"),
    Csr::new(0xC9B, "uro", "hpmcounter27h"),
    Csr::new(0xC9C, "uro", "hpmcounter28h"),
    Csr::new(0xC9D, "uro", "hpmcounter29h"),
    Csr::new(0xC9E, "uro", "hpmcounter30h"),
    Csr::new(0xC9F, "uro", "hpmcounter31h"),
    // Supervisor trap setup:
    Csr::new(0x100, "srw", "sstatus"),
    Csr::new(0x104, "srw", "sie"),
    Csr::new(0x105, "srw", "stvec"),
    Csr::new(0x106, "srw", "scounteren"),
    // Supervisor configuration:
    Csr::new(0x10A, "srw", "senvcfg"),
    // Supervisor counter setup:
    Csr::new(0x120, "srw", "scountinhibit"),
    // Supervisor trap handling:
    Csr::new(0x140, "srw", "sscratch"),
    Csr::new(0x141, "srw", "sepc"),
    Csr::new(0x142, "srw", "scause"),
    Csr::new(0x143, "srw", "stval"),
    Csr::new(0x144, "srw", "sip"),
    Csr::new(0xDA0, "sro", "scountovf"),
    // Supervisor protection and translation:
    Csr::new(0x180, "srw", "satp"),
    // Debug/trace registers:
    Csr::new(0x5A8, "srw", "scontext"),
    // Supervisor state enable registers:
    Csr::new(0x10C, "srw", "sstateen0"),
    Csr::new(0x10D, "srw", "sstateen1"),
    Csr::new(0x10E, "srw", "sstateen2"),
    Csr::new(0x10F, "srw", "sstateen3"),
    // Hypervisor trap setup:
    Csr::new(0x600, "hrw", "hstatus"),
    Csr::new(0x602, "hrw", "hedeleg"),
    Csr::new(0x603, "hrw", "hideleg"),
    Csr::new(0x604, "hrw", "hie"),
    Csr::new(0x606, "hrw", "hcounteren"),
    Csr::new(0x607, "hrw", "hgeie"),
    Csr::new(0x612, "hrw", "hedelegh"),
    // Hypervisor trap handling:
    Csr::new(0x643, "hrw", "htval"),
    Csr::new(0x644, "hrw", "hip"),
    Csr::new(0x645, "hrw", "hvip"),
    Csr::new(0x64A, "hrw", "htinst"),
    Csr::new(0xE12, "hro", "hgeip"),
    // Hypervisor configuration:
    Csr::new(0x60A, "hrw", "henvcfg"),
    Csr::new(0x61A, "hrw", "henvcfgh"),
    // Hypervisor protection and translation:
    Csr::new(0x680, "hrw", "hgatp"),
    // Debug/trace registers:
    Csr::new(0x6A8, "hrw", "hcontext"),
    // Hypervisor counter/timer virtualization registers:
    Csr::new(0x605, "hrw", "htimedelta"),
    Csr::new(0x615, "hrw", "htimedeltah"),
    // Hypervisor state enable registers:
    Csr::new(0x60C, "hrw", "hstateen0"),
    Csr::new(0x60D, "hrw", "hstateen1"),
    Csr::new(0x60E, "hrw", "hstateen2"),
    Csr::new(0x60F, "hrw", "hstateen3"),
    Csr::new(0x61C, "hrw", "hstateen0h"),
    Csr::new(0x61D, "hrw", "hstateen1h"),
    Csr::new(0x61E, "hrw", "hstateen2h"),
    Csr::new(0x61F, "hrw", "hstateen3h"),
    // Virtual supervisor registers:
    Csr::new(0x200, "hrw", "vsstatus"),
    Csr::new(0x204, "hrw", "vsie"),
    Csr::new(0x205, "hrw", "vstvec"),
    Csr::new(0x240, "hrw", "vsscratch"),
    Csr::new(0x241, "hrw", "vsepc"),
    Csr::new(0x242, "hrw", "vscause"),
    Csr::new(0x243, "hrw", "vstval"),
    Csr::new(0x244, "hrw", "vsip"),
    Csr::new(0x280, "hrw", "vsatp"),
    // Machine information registers:
    Csr::new(0xF11, "mro", "mvendorid"),
    Csr::new(0xF12, "mro", "marchid"),
    Csr::new(0xF13, "mro", "mimpid"),
    Csr::new(0xF14, "mro", "mhartid"),
    Csr::new(0xF15, "mro", "mconfigptr"),
    // Machine trap setup:
    Csr::new(0x300, "mrw", "mstatus"),
    Csr::new(0x301, "mrw", "misa"),
    Csr::new(0x302, "mrw", "medeleg"),
    Csr::new(0x303, "mrw", "mideleg"),
    Csr::new(0x304, "mrw", "mie"),
    Csr::new(0x305, "mrw", "mtvec"),
    Csr::new(0x306, "mrw", "mcounteren"),
    Csr::new(0x310, "mrw", "mstatush"),
    Csr::new(0x312, "mrw", "medelegh"),
    // Machine trap handling:
    Csr::new(0x340, "mrw", "mscratch"),
    Csr::new(0x341, "mrw", "mepc"),
    Csr::new(0x342, "mrw", "mcause"),
    Csr::new(0x343, "mrw", "mtval"),
    Csr::new(0x344, "mrw", "mip"),
    Csr::new(0x34A, "mrw", "mtinst"),
    Csr::new(0x34B, "mrw", "mtval2"),
    // Machine configuration:
    Csr::new(0x30A, "mrw", "menvcfg"),
    Csr::new(0x31A, "mrw", "menvcfgh"),
    Csr::new(0x747, "mrw", "mseccfg"),
    Csr::new(0x757, "mrw", "mseccfgh"),
    // Machine memory protection:
    Csr::new(0x3A0, "mrw", "pmpcfg0"),
    Csr::new(0x3A1, "mrw", "pmpcfg1"),
    Csr::new(0x3A2, "mrw", "pmpcfg2"),
    Csr::new(0x3A3, "mrw", "pmpcfg3"),
    Csr::new(0x3A4, "mrw", "pmpcfg4"),
    Csr::new(0x3A5, "mrw", "pmpcfg5"),
    Csr::new(0x3A6, "mrw", "pmpcfg6"),
    Csr::new(0x3A7, "mrw", "pmpcfg7"),
    Csr::new(0x3A8, "mrw", "pmpcfg8"),
    Csr::new(0x3A9, "mrw", "pmpcfg9"),
    Csr::new(0x3AA, "mrw", "pmpcfg10"),
    Csr::new(0x3AB, "mrw", "pmpcfg11"),
    Csr::new(0x3AC, "mrw", "pmpcfg12"),
    Csr::new(0x3AD, "mrw", "pmpcfg13"),
    Csr::new(0x3AE, "mrw", "pmpcfg14"),
    Csr::new(0x3AF, "mrw", "pmpcfg15"),
    Csr::new(0x3B0, "mrw", "pmpaddr0"),
    Csr::new(0x3B1, "mrw", "pmpaddr1"),
    Csr::new(0x3B2, "mrw", "pmpaddr2"),
    Csr::new(0x3B3, "mrw", "pmpaddr3"),
    Csr::new(0x3B4, "mrw", "pmpaddr4"),
    Csr::new(0x3B5, "mrw", "pmpaddr5"),
    Csr::new(0x3B6, "mrw", "pmpaddr6"),
    Csr::new(0x3B7, "mrw", "pmpaddr7"),
    Csr::new(0x3B8, "mrw", "pmpaddr8"),
    Csr::new(0x3B9, "mrw", "pmpaddr9"),
    Csr::new(0x3BA, "mrw", "pmpaddr10"),
    Csr::new(0x3BB, "mrw", "pmpaddr11"),
    Csr::new(0x3BC, "mrw", "pmpaddr12"),
    Csr::new(0x3BD, "mrw", "pmpaddr13"),
    Csr::new(0x3BE, "mrw", "pmpaddr14"),
    Csr::new(0x3BF, "mrw", "pmpaddr15"),
    Csr::new(0x3C0, "mrw", "pmpaddr16"),
    Csr::new(0x3C1, "mrw", "pmpaddr17"),
    Csr::new(0x3C2, "mrw", "pmpaddr18"),
    Csr::new(0x3C3, "mrw", "pmpaddr19"),
    Csr::new(0x3C4, "mrw", "pmpaddr20"),
    Csr::new(0x3C5, "mrw", "pmpaddr21"),
    Csr::new(0x3C6, "mrw", "pmpaddr22"),
    Csr::new(0x3C7, "mrw", "pmpaddr23"),
    Csr::new(0x3C8, "mrw", "pmpaddr24"),
    Csr::new(0x3C9, "mrw", "pmpaddr25"),
    Csr::new(0x3CA, "mrw", "pmpaddr26"),
    Csr::new(0x3CB, "mrw", "pmpaddr27"),
    Csr::new(0x3CC, "mrw", "pmpaddr28"),
    Csr::new(0x3CD, "mrw", "pmpaddr29"),
    Csr::new(0x3CE, "mrw", "pmpaddr30"),
    Csr::new(0x3CF, "mrw", "pmpaddr31"),
    Csr::new(0x3D0, "mrw", "pmpaddr32"),
    Csr::new(0x3D1, "mrw", "pmpaddr33"),
    Csr::new(0x3D2, "mrw", "pmpaddr34"),
    Csr::new(0x3D3, "mrw", "pmpaddr35"),
    Csr::new(0x3D4, "mrw", "pmpaddr36"),
    Csr::new(0x3D5, "mrw", "pmpaddr37"),
    Csr::new(0x3D6, "mrw", "pmpaddr38"),
    Csr::new(0x3D7, "mrw", "pmpaddr39"),
    Csr::new(0x3D8, "mrw", "pmpaddr40"),
    Csr::new(0x3D9, "mrw", "pmpaddr41"),
    Csr::new(0x3DA, "mrw", "pmpaddr42"),
    Csr::new(0x3DB, "mrw", "pmpaddr43"),
    Csr::new(0x3DC, "mrw", "pmpaddr44"),
    Csr::new(0x3DD, "mrw", "pmpaddr45"),
    Csr::new(0x3DE, "mrw", "pmpaddr46"),
    Csr::new(0x3DF, "mrw", "pmpaddr47"),
    Csr::new(0x3E0, "mrw", "pmpaddr48"),
    Csr::new(0x3E1, "mrw", "pmpaddr49"),
    Csr::new(0x3E2, "mrw", "pmpaddr50"),
    Csr::new(0x3E3, "mrw", "pmpaddr51"),
    Csr::new(0x3E4, "mrw", "pmpaddr52"),
    Csr::new(0x3E5, "mrw", "pmpaddr53"),
    Csr::new(0x3E6, "mrw", "pmpaddr54"),
    Csr::new(0x3E7, "mrw", "pmpaddr55"),
    Csr::new(0x3E8, "mrw", "pmpaddr56"),
    Csr::new(0x3E9, "mrw", "pmpaddr57"),
    Csr::new(0x3EA, "mrw", "pmpaddr58"),
    Csr::new(0x3EB, "mrw", "pmpaddr59"),
    Csr::new(0x3EC, "mrw", "pmpaddr60"),
    Csr::new(0x3ED, "mrw", "pmpaddr61"),
    Csr::new(0x3EE, "mrw", "pmpaddr62"),
    Csr::new(0x3EF, "mrw", "pmpaddr63"),
    // Machine state enable registers:
    Csr::new(0x30C, "mrw", "mstateen0"),
    Csr::new(0x30D, "mrw", "mstateen1"),
    Csr::new(0x30E, "mrw", "mstateen2"),
    Csr::new(0x30F, "mrw", "mstateen3"),
    Csr::new(0x31C, "mrw", "mstateen0h"),
    Csr::new(0x31D, "mrw", "mstateen1h"),
    Csr::new(0x31E, "mrw", "mstateen2h"),
    Csr::new(0x31F, "mrw", "mstateen3h"),
    // Machine non-maskable interrupt handling:
    Csr::new(0x740, "mrw", "mnscratch"),
    Csr::new(0x741, "mrw", "mnepc"),
    Csr::new(0x742, "mrw", "mncause"),
    Csr::new(0x744, "mrw", "mnstatus"),
    // Machine counter/timers:
    Csr::new(0xB00, "mrw", "mcycle"),
    Csr::new(0xB02, "mrw", "minstret"),
    Csr::new(0xB03, "mrw", "mhpmcounter3"),
    Csr::new(0xB04, "mrw", "mhpmcounter4"),
    Csr::new(0xB05, "mrw", "mhpmcounter5"),
    Csr::new(0xB06, "mrw", "mhpmcounter6"),
    Csr::new(0xB07, "mrw", "mhpmcounter7"),
    Csr::new(0xB08, "mrw", "mhpmcounter8"),
    Csr::new(0xB09, "mrw", "mhpmcounter9"),
    Csr::new(0xB0A, "mrw", "mhpmcounter10"),
    Csr::new(0xB0B, "mrw", "mhpmcounter11"),
    Csr::new(0xB0C, "mrw", "mhpmcounter12"),
    Csr::new(0xB0D, "mrw", "mhpmcounter13"),
    Csr::new(0xB0E, "mrw", "mhpmcounter14"),
    Csr::new(0xB0F, "mrw", "mhpmcounter15"),
    Csr::new(0xB10, "mrw", "mhpmcounter16"),
    Csr::new(0xB11, "mrw", "mhpmcounter17"),
    Csr::new(0xB12, "mrw", "mhpmcounter18"),
    Csr::new(0xB13, "mrw", "mhpmcounter19"),
    Csr::new(0xB14, "mrw", "mhpmcounter20"),
    Csr::new(0xB15, "mrw", "mhpmcounter21"),
    Csr::new(0xB16, "mrw", "mhpmcounter22"),
    Csr::new(0xB17, "mrw", "mhpmcounter23"),
    Csr::new(0xB18, "mrw", "mhpmcounter24"),
    Csr::new(0xB19, "mrw", "mhpmcounter25"),
    Csr::new(0xB1A, "mrw", "mhpmcounter26"),
    Csr::new(0xB1B, "mrw", "mhpmcounter27"),
    Csr::new(0xB1C, "mrw", "mhpmcounter28"),
    Csr::new(0xB1D, "mrw", "mhpmcounter29"),
    Csr::new(0xB1E, "mrw", "mhpmcounter30"),
    Csr::new(0xB1F, "mrw", "mhpmcounter31"),
    Csr::new(0xB80, "mrw", "mcycleh"),
    Csr::new(0xB82, "mrw", "minstreth"),
    Csr::new(0xB83, "mrw", "mhpmcounter3h"),
    Csr::new(0xB84, "mrw", "mhpmcounter4h"),
    Csr::new(0xB85, "mrw", "mhpmcounter5h"),
    Csr::new(0xB86, "mrw", "mhpmcounter6h"),
    Csr::new(0xB87, "mrw", "mhpmcounter7h"),
    Csr::new(0xB88, "mrw", "mhpmcounter8h"),
    Csr::new(0xB89, "mrw", "mhpmcounter9h"),
    Csr::new(0xB8A, "mrw", "mhpmcounter10h"),
    Csr::new(0xB8B, "mrw", "mhpmcounter11h"),
    Csr::new(0xB8C, "mrw", "mhpmcounter12h"),
    Csr::new(0xB8D, "mrw", "mhpmcounter13h"),
    Csr::new(0xB8E, "mrw", "mhpmcounter14h"),
    Csr::new(0xB8F, "mrw", "mhpmcounter15h"),
    Csr::new(0xB90, "mrw", "mhpmcounter16h"),
    Csr::new(0xB91, "mrw", "mhpmcounter17h"),
    Csr::new(0xB92, "mrw", "mhpmcounter18h"),
    Csr::new(0xB93, "mrw", "mhpmcounter19h"),
    Csr::new(0xB94, "mrw", "mhpmcounter20h"),
    Csr::new(0xB95, "mrw", "mhpmcounter21h"),
    Csr::new(0xB96, "mrw", "mhpmcounter22h"),
    Csr::new(0xB97, "mrw", "mhpmcounter23h"),
    Csr::new(0xB98, "mrw", "mhpmcounter24h"),
    Csr::new(0xB99, "mrw", "mhpmcounter25h"),
    Csr::new(0xB9A, "mrw", "mhpmcounter26h"),
    Csr::new(0xB9B, "mrw", "mhpmcounter27h"),
    Csr::new(0xB9C, "mrw", "mhpmcounter28h"),
    Csr::new(0xB9D, "mrw", "mhpmcounter29h"),
    Csr::new(0xB9E, "mrw", "mhpmcounter30h"),
    Csr::new(0xB9F, "mrw", "mhpmcounter31h"),
    // Machine counter setup:
    Csr::new(0x320, "mrw", "mcountinhibit"),
    Csr::new(0x323, "mrw", "mhpmevent3"),
    Csr::new(0x324, "mrw", "mhpmevent4"),
    Csr::new(0x325, "mrw", "mhpmevent5"),
    Csr::new(0x326, "mrw", "mhpmevent6"),
    Csr::new(0x327, "mrw", "mhpmevent7"),
    Csr::new(0x328, "mrw", "mhpmevent8"),
    Csr::new(0x329, "mrw", "mhpmevent9"),
    Csr::new(0x32A, "mrw", "mhpmevent10"),
    Csr::new(0x32B, "mrw", "mhpmevent11"),
    Csr::new(0x32C, "mrw", "mhpmevent12"),
    Csr::new(0x32D, "mrw", "mhpmevent13"),
    Csr::new(0x32E, "mrw", "mhpmevent14"),
    Csr::new(0x32F, "mrw", "mhpmevent15"),
    Csr::new(0x320, "mrw", "mhpmevent16"),
    Csr::new(0x331, "mrw", "mhpmevent17"),
    Csr::new(0x332, "mrw", "mhpmevent18"),
    Csr::new(0x333, "mrw", "mhpmevent19"),
    Csr::new(0x334, "mrw", "mhpmevent20"),
    Csr::new(0x335, "mrw", "mhpmevent21"),
    Csr::new(0x336, "mrw", "mhpmevent22"),
    Csr::new(0x337, "mrw", "mhpmevent23"),
    Csr::new(0x338, "mrw", "mhpmevent24"),
    Csr::new(0x339, "mrw", "mhpmevent25"),
    Csr::new(0x33A, "mrw", "mhpmevent26"),
    Csr::new(0x33B, "mrw", "mhpmevent27"),
    Csr::new(0x33C, "mrw", "mhpmevent28"),
    Csr::new(0x33D, "mrw", "mhpmevent29"),
    Csr::new(0x33E, "mrw", "mhpmevent30"),
    Csr::new(0x33F, "mrw", "mhpmevent31"),
    Csr::new(0x723, "mrw", "mhpmevent3h"),
    Csr::new(0x724, "mrw", "mhpmevent4h"),
    Csr::new(0x725, "mrw", "mhpmevent5h"),
    Csr::new(0x726, "mrw", "mhpmevent6h"),
    Csr::new(0x727, "mrw", "mhpmevent7h"),
    Csr::new(0x728, "mrw", "mhpmevent8h"),
    Csr::new(0x729, "mrw", "mhpmevent9h"),
    Csr::new(0x72A, "mrw", "mhpmevent10h"),
    Csr::new(0x72B, "mrw", "mhpmevent11h"),
    Csr::new(0x72C, "mrw", "mhpmevent12h"),
    Csr::new(0x72D, "mrw", "mhpmevent13h"),
    Csr::new(0x72E, "mrw", "mhpmevent14h"),
    Csr::new(0x72F, "mrw", "mhpmevent15h"),
    Csr::new(0x730, "mrw", "mhpmevent16h"),
    Csr::new(0x731, "mrw", "mhpmevent17h"),
    Csr::new(0x732, "mrw", "mhpmevent18h"),
    Csr::new(0x733, "mrw", "mhpmevent19h"),
    Csr::new(0x734, "mrw", "mhpmevent20h"),
    Csr::new(0x735, "mrw", "mhpmevent21h"),
    Csr::new(0x736, "mrw", "mhpmevent22h"),
    Csr::new(0x737, "mrw", "mhpmevent23h"),
    Csr::new(0x738, "mrw", "mhpmevent24h"),
    Csr::new(0x739, "mrw", "mhpmevent25h"),
    Csr::new(0x73A, "mrw", "mhpmevent26h"),
    Csr::new(0x73B, "mrw", "mhpmevent27h"),
    Csr::new(0x73C, "mrw", "mhpmevent28h"),
    Csr::new(0x73D, "mrw", "mhpmevent29h"),
    Csr::new(0x73E, "mrw", "mhpmevent30h"),
    Csr::new(0x73F, "mrw", "mhpmevent31h"),
    // Debug/trace registers (shared with debug mode):
    Csr::new(0x7A0, "mrw", "tselect"),
    Csr::new(0x7A1, "mrw", "tdata1"),
    Csr::new(0x7A2, "mrw", "tdata2"),
    Csr::new(0x7A3, "mrw", "tdata3"),
    Csr::new(0x7A8, "mrw", "mcontext"),
    // Debug mode registers:
    Csr::new(0x7B0, "drw", "dcsr"),
    Csr::new(0x7B1, "drw", "dpc"),
    Csr::new(0x7B2, "drw", "dscratch0"),
    Csr::new(0x7B3, "drw", "dscratch1"),
];
