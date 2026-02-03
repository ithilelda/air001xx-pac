#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr1: Cfgr1,
    _reserved1: [u8; 0x14],
    cfgr2: Cfgr2,
    cfgr3: Cfgr3,
}
impl RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x18 - SYSCFG configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x1c - SYSCFG configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(&self) -> &Cfgr3 {
        &self.cfgr3
    }
}
#[doc = "CFGR1 (rw) register accessor: SYSCFG configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: SYSCFG configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "CFGR3 (rw) register accessor: SYSCFG configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr3`] module"]
#[doc(alias = "CFGR3")]
pub type Cfgr3 = crate::Reg<cfgr3::Cfgr3Spec>;
#[doc = "SYSCFG configuration register 3"]
pub mod cfgr3;
