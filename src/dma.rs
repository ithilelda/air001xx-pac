#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr1: Ccr1,
    cndtr1: Cndtr1,
    cpar1: Cpar1,
    cmar1: Cmar1,
    _reserved6: [u8; 0x04],
    ccr2: Ccr2,
    cndtr2: Cndtr2,
    cpar2: Cpar2,
    cmar2: Cmar2,
    _reserved10: [u8; 0x04],
    ccr3: Ccr3,
    cndtr3: Cndtr3,
    cpar3: Cpar3,
    cmar3: Cmar3,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x08 - DMA channel configuration register (DMA_CCR)"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x0c - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &Cndtr1 {
        &self.cndtr1
    }
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &Cpar1 {
        &self.cpar1
    }
    #[doc = "0x14 - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &Cmar1 {
        &self.cmar1
    }
    #[doc = "0x1c - DMA channel configuration register (DMA_CCR)"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x20 - DMA channel 2 number of data register"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &Cndtr2 {
        &self.cndtr2
    }
    #[doc = "0x24 - DMA channel 2 peripheral address register"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &Cpar2 {
        &self.cpar2
    }
    #[doc = "0x28 - DMA channel 2 memory address register"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &Cmar2 {
        &self.cmar2
    }
    #[doc = "0x30 - DMA channel configuration register (DMA_CCR)"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x34 - DMA channel 3 number of data register"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &Cndtr3 {
        &self.cndtr3
    }
    #[doc = "0x38 - DMA channel 3 peripheral address register"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &Cpar3 {
        &self.cpar3
    }
    #[doc = "0x3c - DMA channel 3 memory address register"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &Cmar3 {
        &self.cmar3
    }
}
#[doc = "ISR (r) register accessor: DMA interrupt status register (DMA_ISR)\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register (DMA_IFCR)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr1;
#[doc = "CNDTR1 (rw) register accessor: DMA channel 1 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`] module"]
#[doc(alias = "CNDTR1")]
pub type Cndtr1 = crate::Reg<cndtr1::Cndtr1Spec>;
#[doc = "DMA channel 1 number of data register"]
pub mod cndtr1;
#[doc = "CPAR1 (rw) register accessor: DMA channel 1 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`] module"]
#[doc(alias = "CPAR1")]
pub type Cpar1 = crate::Reg<cpar1::Cpar1Spec>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod cpar1;
#[doc = "CMAR1 (rw) register accessor: DMA channel 1 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`] module"]
#[doc(alias = "CMAR1")]
pub type Cmar1 = crate::Reg<cmar1::Cmar1Spec>;
#[doc = "DMA channel 1 memory address register"]
pub mod cmar1;
#[doc = "CCR2 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr2;
#[doc = "CNDTR2 (rw) register accessor: DMA channel 2 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`] module"]
#[doc(alias = "CNDTR2")]
pub type Cndtr2 = crate::Reg<cndtr2::Cndtr2Spec>;
#[doc = "DMA channel 2 number of data register"]
pub mod cndtr2;
#[doc = "CPAR2 (rw) register accessor: DMA channel 2 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`] module"]
#[doc(alias = "CPAR2")]
pub type Cpar2 = crate::Reg<cpar2::Cpar2Spec>;
#[doc = "DMA channel 2 peripheral address register"]
pub mod cpar2;
#[doc = "CMAR2 (rw) register accessor: DMA channel 2 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`] module"]
#[doc(alias = "CMAR2")]
pub type Cmar2 = crate::Reg<cmar2::Cmar2Spec>;
#[doc = "DMA channel 2 memory address register"]
pub mod cmar2;
#[doc = "CCR3 (rw) register accessor: DMA channel configuration register (DMA_CCR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "DMA channel configuration register (DMA_CCR)"]
pub mod ccr3;
#[doc = "CNDTR3 (rw) register accessor: DMA channel 3 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`] module"]
#[doc(alias = "CNDTR3")]
pub type Cndtr3 = crate::Reg<cndtr3::Cndtr3Spec>;
#[doc = "DMA channel 3 number of data register"]
pub mod cndtr3;
#[doc = "CPAR3 (rw) register accessor: DMA channel 3 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`] module"]
#[doc(alias = "CPAR3")]
pub type Cpar3 = crate::Reg<cpar3::Cpar3Spec>;
#[doc = "DMA channel 3 peripheral address register"]
pub mod cpar3;
#[doc = "CMAR3 (rw) register accessor: DMA channel 3 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`] module"]
#[doc(alias = "CMAR3")]
pub type Cmar3 = crate::Reg<cmar3::Cmar3Spec>;
#[doc = "DMA channel 3 memory address register"]
pub mod cmar3;
