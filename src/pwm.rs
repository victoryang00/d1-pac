#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM IRQ Enable Register"]
    pub pier: crate::Reg<pier::PIER_SPEC>,
    #[doc = "0x04 - PWM IRQ Status Register"]
    pub pisr: crate::Reg<pisr::PISR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Capture IRQ Enable Register"]
    pub cier: crate::Reg<cier::CIER_SPEC>,
    #[doc = "0x14 - Capture IRQ Status Register"]
    pub cisr: crate::Reg<cisr::CISR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x20 - PWM01 Clock Configuration Register"]
    pub pccr01: crate::Reg<pccr01::PCCR01_SPEC>,
    #[doc = "0x24 - PWM23 Clock Configuration Register"]
    pub pccr23: crate::Reg<pccr23::PCCR23_SPEC>,
    #[doc = "0x28 - PWM45 Clock Configuration Register"]
    pub pccr45: crate::Reg<pccr45::PCCR45_SPEC>,
    #[doc = "0x2c - PWM67 Clock Configuration Register"]
    pub pccr67: crate::Reg<pccr67::PCCR67_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - PWM Clock Gating Register"]
    pub pcgr: crate::Reg<pcgr::PCGR_SPEC>,
    _reserved9: [u8; 0x1c],
    #[doc = "0x60 - PWM01 Dead Zone Control Register"]
    pub pdzcr01: crate::Reg<pdzcr01::PDZCR01_SPEC>,
    #[doc = "0x64 - PWM23 Dead Zone Control Register"]
    pub pdzcr23: crate::Reg<pdzcr23::PDZCR23_SPEC>,
    #[doc = "0x68 - PWM45 Dead Zone Control Register"]
    pub pdzcr45: crate::Reg<pdzcr45::PDZCR45_SPEC>,
    #[doc = "0x6c - PWM67 Dead Zone Control Register"]
    pub pdzcr67: crate::Reg<pdzcr67::PDZCR67_SPEC>,
    _reserved13: [u8; 0x10],
    #[doc = "0x80 - PWM Enable Register"]
    pub per: crate::Reg<per::PER_SPEC>,
    _reserved14: [u8; 0x0c],
    #[doc = "0x90 - PWM Group0 Register"]
    pub pgr0: crate::Reg<pgr0::PGR0_SPEC>,
    #[doc = "0x94 - PWM Group1 Register"]
    pub pgr1: crate::Reg<pgr1::PGR1_SPEC>,
    #[doc = "0x98 - PWM Group2 Register"]
    pub pgr2: crate::Reg<pgr2::PGR2_SPEC>,
    #[doc = "0x9c - PWM Group3 Register"]
    pub pgr3: crate::Reg<pgr3::PGR3_SPEC>,
    _reserved18: [u8; 0x20],
    #[doc = "0xc0 - Capture Enable Register"]
    pub cer: crate::Reg<cer::CER_SPEC>,
    _reserved19: [u8; 0x3c],
    #[doc = "0x100 - PWM Control Register"]
    pub pcr0: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x104 - PWM Period Register"]
    pub ppr0: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x108 - PWM Count Register"]
    pub pcntr0: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x10c - PWM Pulse Count Register"]
    pub ppcntr0: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x110 - Capture Control Register"]
    pub ccr0: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x114 - Capture Rise Lock Register"]
    pub crlr0: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x118 - Capture Fall Lock Register"]
    pub cflr0: crate::Reg<cflr::CFLR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x120 - PWM Control Register"]
    pub pcr1: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x124 - PWM Period Register"]
    pub ppr1: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x128 - PWM Count Register"]
    pub pcntr1: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x12c - PWM Pulse Count Register"]
    pub ppcntr1: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x130 - Capture Control Register"]
    pub ccr1: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x134 - Capture Rise Lock Register"]
    pub crlr1: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x138 - Capture Fall Lock Register"]
    pub cflr1: crate::Reg<cflr::CFLR_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x140 - PWM Control Register"]
    pub pcr2: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x144 - PWM Period Register"]
    pub ppr2: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x148 - PWM Count Register"]
    pub pcntr2: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x14c - PWM Pulse Count Register"]
    pub ppcntr2: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x150 - Capture Control Register"]
    pub ccr2: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x154 - Capture Rise Lock Register"]
    pub crlr2: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x158 - Capture Fall Lock Register"]
    pub cflr2: crate::Reg<cflr::CFLR_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0x160 - PWM Control Register"]
    pub pcr3: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x164 - PWM Period Register"]
    pub ppr3: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x168 - PWM Count Register"]
    pub pcntr3: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x16c - PWM Pulse Count Register"]
    pub ppcntr3: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x170 - Capture Control Register"]
    pub ccr3: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x174 - Capture Rise Lock Register"]
    pub crlr3: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x178 - Capture Fall Lock Register"]
    pub cflr3: crate::Reg<cflr::CFLR_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0x180 - PWM Control Register"]
    pub pcr4: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x184 - PWM Period Register"]
    pub ppr4: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x188 - PWM Count Register"]
    pub pcntr4: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x18c - PWM Pulse Count Register"]
    pub ppcntr4: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x190 - Capture Control Register"]
    pub ccr4: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x194 - Capture Rise Lock Register"]
    pub crlr4: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x198 - Capture Fall Lock Register"]
    pub cflr4: crate::Reg<cflr::CFLR_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x1a0 - PWM Control Register"]
    pub pcr5: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x1a4 - PWM Period Register"]
    pub ppr5: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x1a8 - PWM Count Register"]
    pub pcntr5: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x1ac - PWM Pulse Count Register"]
    pub ppcntr5: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x1b0 - Capture Control Register"]
    pub ccr5: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x1b4 - Capture Rise Lock Register"]
    pub crlr5: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x1b8 - Capture Fall Lock Register"]
    pub cflr5: crate::Reg<cflr::CFLR_SPEC>,
    _reserved61: [u8; 0x04],
    #[doc = "0x1c0 - PWM Control Register"]
    pub pcr6: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x1c4 - PWM Period Register"]
    pub ppr6: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x1c8 - PWM Count Register"]
    pub pcntr6: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x1cc - PWM Pulse Count Register"]
    pub ppcntr6: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x1d0 - Capture Control Register"]
    pub ccr6: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x1d4 - Capture Rise Lock Register"]
    pub crlr6: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x1d8 - Capture Fall Lock Register"]
    pub cflr6: crate::Reg<cflr::CFLR_SPEC>,
    _reserved68: [u8; 0x04],
    #[doc = "0x1e0 - PWM Control Register"]
    pub pcr7: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x1e4 - PWM Period Register"]
    pub ppr7: crate::Reg<ppr::PPR_SPEC>,
    #[doc = "0x1e8 - PWM Count Register"]
    pub pcntr7: crate::Reg<pcntr::PCNTR_SPEC>,
    #[doc = "0x1ec - PWM Pulse Count Register"]
    pub ppcntr7: crate::Reg<ppcntr::PPCNTR_SPEC>,
    #[doc = "0x1f0 - Capture Control Register"]
    pub ccr7: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x1f4 - Capture Rise Lock Register"]
    pub crlr7: crate::Reg<crlr::CRLR_SPEC>,
    #[doc = "0x1f8 - Capture Fall Lock Register"]
    pub cflr7: crate::Reg<cflr::CFLR_SPEC>,
}
#[doc = "PIER register accessor: an alias for `Reg<PIER_SPEC>`"]
pub type PIER = crate::Reg<pier::PIER_SPEC>;
#[doc = "PWM IRQ Enable Register"]
pub mod pier;
#[doc = "PISR register accessor: an alias for `Reg<PISR_SPEC>`"]
pub type PISR = crate::Reg<pisr::PISR_SPEC>;
#[doc = "PWM IRQ Status Register"]
pub mod pisr;
#[doc = "CIER register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = "Capture IRQ Enable Register"]
pub mod cier;
#[doc = "CISR register accessor: an alias for `Reg<CISR_SPEC>`"]
pub type CISR = crate::Reg<cisr::CISR_SPEC>;
#[doc = "Capture IRQ Status Register"]
pub mod cisr;
#[doc = "PCCR01 register accessor: an alias for `Reg<PCCR01_SPEC>`"]
pub type PCCR01 = crate::Reg<pccr01::PCCR01_SPEC>;
#[doc = "PWM01 Clock Configuration Register"]
pub mod pccr01;
#[doc = "PCCR23 register accessor: an alias for `Reg<PCCR23_SPEC>`"]
pub type PCCR23 = crate::Reg<pccr23::PCCR23_SPEC>;
#[doc = "PWM23 Clock Configuration Register"]
pub mod pccr23;
#[doc = "PCCR45 register accessor: an alias for `Reg<PCCR45_SPEC>`"]
pub type PCCR45 = crate::Reg<pccr45::PCCR45_SPEC>;
#[doc = "PWM45 Clock Configuration Register"]
pub mod pccr45;
#[doc = "PCCR67 register accessor: an alias for `Reg<PCCR67_SPEC>`"]
pub type PCCR67 = crate::Reg<pccr67::PCCR67_SPEC>;
#[doc = "PWM67 Clock Configuration Register"]
pub mod pccr67;
#[doc = "PCGR register accessor: an alias for `Reg<PCGR_SPEC>`"]
pub type PCGR = crate::Reg<pcgr::PCGR_SPEC>;
#[doc = "PWM Clock Gating Register"]
pub mod pcgr;
#[doc = "PDZCR01 register accessor: an alias for `Reg<PDZCR01_SPEC>`"]
pub type PDZCR01 = crate::Reg<pdzcr01::PDZCR01_SPEC>;
#[doc = "PWM01 Dead Zone Control Register"]
pub mod pdzcr01;
#[doc = "PDZCR23 register accessor: an alias for `Reg<PDZCR23_SPEC>`"]
pub type PDZCR23 = crate::Reg<pdzcr23::PDZCR23_SPEC>;
#[doc = "PWM23 Dead Zone Control Register"]
pub mod pdzcr23;
#[doc = "PDZCR45 register accessor: an alias for `Reg<PDZCR45_SPEC>`"]
pub type PDZCR45 = crate::Reg<pdzcr45::PDZCR45_SPEC>;
#[doc = "PWM45 Dead Zone Control Register"]
pub mod pdzcr45;
#[doc = "PDZCR67 register accessor: an alias for `Reg<PDZCR67_SPEC>`"]
pub type PDZCR67 = crate::Reg<pdzcr67::PDZCR67_SPEC>;
#[doc = "PWM67 Dead Zone Control Register"]
pub mod pdzcr67;
#[doc = "PER register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "PWM Enable Register"]
pub mod per;
#[doc = "PGR0 register accessor: an alias for `Reg<PGR0_SPEC>`"]
pub type PGR0 = crate::Reg<pgr0::PGR0_SPEC>;
#[doc = "PWM Group0 Register"]
pub mod pgr0;
#[doc = "PGR1 register accessor: an alias for `Reg<PGR1_SPEC>`"]
pub type PGR1 = crate::Reg<pgr1::PGR1_SPEC>;
#[doc = "PWM Group1 Register"]
pub mod pgr1;
#[doc = "PGR2 register accessor: an alias for `Reg<PGR2_SPEC>`"]
pub type PGR2 = crate::Reg<pgr2::PGR2_SPEC>;
#[doc = "PWM Group2 Register"]
pub mod pgr2;
#[doc = "PGR3 register accessor: an alias for `Reg<PGR3_SPEC>`"]
pub type PGR3 = crate::Reg<pgr3::PGR3_SPEC>;
#[doc = "PWM Group3 Register"]
pub mod pgr3;
#[doc = "CER register accessor: an alias for `Reg<CER_SPEC>`"]
pub type CER = crate::Reg<cer::CER_SPEC>;
#[doc = "Capture Enable Register"]
pub mod cer;
#[doc = "pcr register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PWM Control Register"]
pub mod pcr;
#[doc = "ppr register accessor: an alias for `Reg<PPR_SPEC>`"]
pub type PPR = crate::Reg<ppr::PPR_SPEC>;
#[doc = "PWM Period Register"]
pub mod ppr;
#[doc = "pcntr register accessor: an alias for `Reg<PCNTR_SPEC>`"]
pub type PCNTR = crate::Reg<pcntr::PCNTR_SPEC>;
#[doc = "PWM Count Register"]
pub mod pcntr;
#[doc = "ppcntr register accessor: an alias for `Reg<PPCNTR_SPEC>`"]
pub type PPCNTR = crate::Reg<ppcntr::PPCNTR_SPEC>;
#[doc = "PWM Pulse Count Register"]
pub mod ppcntr;
#[doc = "ccr register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Capture Control Register"]
pub mod ccr;
#[doc = "crlr register accessor: an alias for `Reg<CRLR_SPEC>`"]
pub type CRLR = crate::Reg<crlr::CRLR_SPEC>;
#[doc = "Capture Rise Lock Register"]
pub mod crlr;
#[doc = "cflr register accessor: an alias for `Reg<CFLR_SPEC>`"]
pub type CFLR = crate::Reg<cflr::CFLR_SPEC>;
#[doc = "Capture Fall Lock Register"]
pub mod cflr;
