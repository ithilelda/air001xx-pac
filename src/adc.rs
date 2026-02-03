#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
    smpr: Smpr,
    _reserved6: [u8; 0x08],
    tr: Tr,
    _reserved7: [u8; 0x04],
    chselr: Chselr,
    _reserved8: [u8; 0x14],
    dr: Dr,
    ccsr: Ccsr,
    calrr1: Calrr1,
    calrr2: Calrr2,
    calfir1: Calfir1,
    calfir2: Calfir2,
    _reserved14: [u8; 0x02b0],
    ccr: Ccr,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register"]
    #[inline(always)]
    pub const fn smpr(&self) -> &Smpr {
        &self.smpr
    }
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn tr(&self) -> &Tr {
        &self.tr
    }
    #[doc = "0x28 - ADC group regular sequencer register"]
    #[inline(always)]
    pub const fn chselr(&self) -> &Chselr {
        &self.chselr
    }
    #[doc = "0x40 - ADC group regular data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x44 - ADC calibration configuration and status register"]
    #[inline(always)]
    pub const fn ccsr(&self) -> &Ccsr {
        &self.ccsr
    }
    #[doc = "0x48 - ADC calibration result register 1"]
    #[inline(always)]
    pub const fn calrr1(&self) -> &Calrr1 {
        &self.calrr1
    }
    #[doc = "0x4c - ADC calibration result register 2"]
    #[inline(always)]
    pub const fn calrr2(&self) -> &Calrr2 {
        &self.calrr2
    }
    #[doc = "0x50 - ADC calibration factor input register 1"]
    #[inline(always)]
    pub const fn calfir1(&self) -> &Calfir1 {
        &self.calfir1
    }
    #[doc = "0x54 - ADC calibration factor input register 2"]
    #[inline(always)]
    pub const fn calfir2(&self) -> &Calfir2 {
        &self.calfir2
    }
    #[doc = "0x308 - ADC common configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR1 (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "ADC configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR (rw) register accessor: ADC sampling time register\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr`] module"]
#[doc(alias = "SMPR")]
pub type Smpr = crate::Reg<smpr::SmprSpec>;
#[doc = "ADC sampling time register"]
pub mod smpr;
#[doc = "TR (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`] module"]
#[doc(alias = "TR")]
pub type Tr = crate::Reg<tr::TrSpec>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr;
#[doc = "CHSELR (rw) register accessor: ADC group regular sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chselr`] module"]
#[doc(alias = "CHSELR")]
pub type Chselr = crate::Reg<chselr::ChselrSpec>;
#[doc = "ADC group regular sequencer register"]
pub mod chselr;
#[doc = "DR (r) register accessor: ADC group regular data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "ADC group regular data register"]
pub mod dr;
#[doc = "CCSR (rw) register accessor: ADC calibration configuration and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsr`] module"]
#[doc(alias = "CCSR")]
pub type Ccsr = crate::Reg<ccsr::CcsrSpec>;
#[doc = "ADC calibration configuration and status register"]
pub mod ccsr;
#[doc = "CALRR1 (r) register accessor: ADC calibration result register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`calrr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calrr1`] module"]
#[doc(alias = "CALRR1")]
pub type Calrr1 = crate::Reg<calrr1::Calrr1Spec>;
#[doc = "ADC calibration result register 1"]
pub mod calrr1;
#[doc = "CALRR2 (r) register accessor: ADC calibration result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calrr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calrr2`] module"]
#[doc(alias = "CALRR2")]
pub type Calrr2 = crate::Reg<calrr2::Calrr2Spec>;
#[doc = "ADC calibration result register 2"]
pub mod calrr2;
#[doc = "CALFIR1 (rw) register accessor: ADC calibration factor input register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`calfir1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfir1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfir1`] module"]
#[doc(alias = "CALFIR1")]
pub type Calfir1 = crate::Reg<calfir1::Calfir1Spec>;
#[doc = "ADC calibration factor input register 1"]
pub mod calfir1;
#[doc = "CALFIR2 (rw) register accessor: ADC calibration factor input register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calfir2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfir2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfir2`] module"]
#[doc(alias = "CALFIR2")]
pub type Calfir2 = crate::Reg<calfir2::Calfir2Spec>;
#[doc = "ADC calibration factor input register 2"]
pub mod calfir2;
#[doc = "CCR (rw) register accessor: ADC common configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "ADC common configuration register"]
pub mod ccr;
