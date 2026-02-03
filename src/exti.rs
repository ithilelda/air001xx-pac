#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr: Rtsr,
    ftsr: Ftsr,
    swier: Swier,
    pr: Pr,
    _reserved4: [u8; 0x50],
    exticr1: Exticr1,
    exticr2: Exticr2,
    exticr3: Exticr3,
    _reserved7: [u8; 0x14],
    imr: Imr,
    emr: Emr,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(&self) -> &Rtsr {
        &self.rtsr
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &Ftsr {
        &self.ftsr
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier(&self) -> &Swier {
        &self.swier
    }
    #[doc = "0x0c - EXTI pending register"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x60 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &Exticr1 {
        &self.exticr1
    }
    #[doc = "0x64 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &Exticr2 {
        &self.exticr2
    }
    #[doc = "0x68 - EXTI external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &Exticr3 {
        &self.exticr3
    }
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x84 - EXTI CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
}
#[doc = "RTSR (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr`] module"]
#[doc(alias = "RTSR")]
pub type Rtsr = crate::Reg<rtsr::RtsrSpec>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr`] module"]
#[doc(alias = "FTSR")]
pub type Ftsr = crate::Reg<ftsr::FtsrSpec>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`] module"]
#[doc(alias = "SWIER")]
pub type Swier = crate::Reg<swier::SwierSpec>;
#[doc = "EXTI software interrupt event register"]
pub mod swier;
#[doc = "PR (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "EXTI pending register"]
pub mod pr;
#[doc = "EXTICR1 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`] module"]
#[doc(alias = "EXTICR1")]
pub type Exticr1 = crate::Reg<exticr1::Exticr1Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`] module"]
#[doc(alias = "EXTICR2")]
pub type Exticr2 = crate::Reg<exticr2::Exticr2Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`] module"]
#[doc(alias = "EXTICR3")]
pub type Exticr3 = crate::Reg<exticr3::Exticr3Spec>;
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "IMR (rw) register accessor: EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr;
#[doc = "EMR (rw) register accessor: EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod emr;
