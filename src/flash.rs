#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    _reserved1: [u8; 0x04],
    keyr: Keyr,
    optkeyr: Optkeyr,
    sr: Sr,
    cr: Cr,
    _reserved5: [u8; 0x08],
    optr: Optr,
    sdkr: Sdkr,
    _reserved7: [u8; 0x04],
    wrpr: Wrpr,
    _reserved8: [u8; 0x60],
    stcr: Stcr,
    _reserved9: [u8; 0x6c],
    ts0: Ts0,
    ts1: Ts1,
    ts2p: Ts2p,
    tps3: Tps3,
    ts3: Ts3,
    pertpe: Pertpe,
    smertpe: Smertpe,
    prgtpe: Prgtpe,
    pretpe: Pretpe,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x08 - Flash key register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &Keyr {
        &self.keyr
    }
    #[doc = "0x0c - Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x20 - Flash option register"]
    #[inline(always)]
    pub const fn optr(&self) -> &Optr {
        &self.optr
    }
    #[doc = "0x24 - Flash SDK address register"]
    #[inline(always)]
    pub const fn sdkr(&self) -> &Sdkr {
        &self.sdkr
    }
    #[doc = "0x2c - Flash WRP address register"]
    #[inline(always)]
    pub const fn wrpr(&self) -> &Wrpr {
        &self.wrpr
    }
    #[doc = "0x90 - Flash sleep time config register"]
    #[inline(always)]
    pub const fn stcr(&self) -> &Stcr {
        &self.stcr
    }
    #[doc = "0x100 - Flash TS0 register"]
    #[inline(always)]
    pub const fn ts0(&self) -> &Ts0 {
        &self.ts0
    }
    #[doc = "0x104 - Flash TS1 register"]
    #[inline(always)]
    pub const fn ts1(&self) -> &Ts1 {
        &self.ts1
    }
    #[doc = "0x108 - Flash TS2P register"]
    #[inline(always)]
    pub const fn ts2p(&self) -> &Ts2p {
        &self.ts2p
    }
    #[doc = "0x10c - Flash TPS3 register"]
    #[inline(always)]
    pub const fn tps3(&self) -> &Tps3 {
        &self.tps3
    }
    #[doc = "0x110 - Flash TS3 register"]
    #[inline(always)]
    pub const fn ts3(&self) -> &Ts3 {
        &self.ts3
    }
    #[doc = "0x114 - Flash PERTPE register"]
    #[inline(always)]
    pub const fn pertpe(&self) -> &Pertpe {
        &self.pertpe
    }
    #[doc = "0x118 - Flash SMERTPE register"]
    #[inline(always)]
    pub const fn smertpe(&self) -> &Smertpe {
        &self.smertpe
    }
    #[doc = "0x11c - Flash PRGTPE register"]
    #[inline(always)]
    pub const fn prgtpe(&self) -> &Prgtpe {
        &self.prgtpe
    }
    #[doc = "0x120 - Flash PRETPE register"]
    #[inline(always)]
    pub const fn pretpe(&self) -> &Pretpe {
        &self.pretpe
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`] module"]
#[doc(alias = "KEYR")]
pub type Keyr = crate::Reg<keyr::KeyrSpec>;
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "OPTR (rw) register accessor: Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optr`] module"]
#[doc(alias = "OPTR")]
pub type Optr = crate::Reg<optr::OptrSpec>;
#[doc = "Flash option register"]
pub mod optr;
#[doc = "SDKR (rw) register accessor: Flash SDK address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdkr`] module"]
#[doc(alias = "SDKR")]
pub type Sdkr = crate::Reg<sdkr::SdkrSpec>;
#[doc = "Flash SDK address register"]
pub mod sdkr;
#[doc = "WRPR (rw) register accessor: Flash WRP address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`] module"]
#[doc(alias = "WRPR")]
pub type Wrpr = crate::Reg<wrpr::WrprSpec>;
#[doc = "Flash WRP address register"]
pub mod wrpr;
#[doc = "STCR (rw) register accessor: Flash sleep time config register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcr`] module"]
#[doc(alias = "STCR")]
pub type Stcr = crate::Reg<stcr::StcrSpec>;
#[doc = "Flash sleep time config register"]
pub mod stcr;
#[doc = "TS0 (rw) register accessor: Flash TS0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts0`] module"]
#[doc(alias = "TS0")]
pub type Ts0 = crate::Reg<ts0::Ts0Spec>;
#[doc = "Flash TS0 register"]
pub mod ts0;
#[doc = "TS1 (rw) register accessor: Flash TS1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts1`] module"]
#[doc(alias = "TS1")]
pub type Ts1 = crate::Reg<ts1::Ts1Spec>;
#[doc = "Flash TS1 register"]
pub mod ts1;
#[doc = "TS2P (rw) register accessor: Flash TS2P register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts2p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts2p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts2p`] module"]
#[doc(alias = "TS2P")]
pub type Ts2p = crate::Reg<ts2p::Ts2pSpec>;
#[doc = "Flash TS2P register"]
pub mod ts2p;
#[doc = "TPS3 (rw) register accessor: Flash TPS3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tps3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tps3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tps3`] module"]
#[doc(alias = "TPS3")]
pub type Tps3 = crate::Reg<tps3::Tps3Spec>;
#[doc = "Flash TPS3 register"]
pub mod tps3;
#[doc = "TS3 (rw) register accessor: Flash TS3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts3`] module"]
#[doc(alias = "TS3")]
pub type Ts3 = crate::Reg<ts3::Ts3Spec>;
#[doc = "Flash TS3 register"]
pub mod ts3;
#[doc = "PERTPE (rw) register accessor: Flash PERTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`pertpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pertpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pertpe`] module"]
#[doc(alias = "PERTPE")]
pub type Pertpe = crate::Reg<pertpe::PertpeSpec>;
#[doc = "Flash PERTPE register"]
pub mod pertpe;
#[doc = "SMERTPE (rw) register accessor: Flash SMERTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`smertpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smertpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smertpe`] module"]
#[doc(alias = "SMERTPE")]
pub type Smertpe = crate::Reg<smertpe::SmertpeSpec>;
#[doc = "Flash SMERTPE register"]
pub mod smertpe;
#[doc = "PRGTPE (rw) register accessor: Flash PRGTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`prgtpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgtpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prgtpe`] module"]
#[doc(alias = "PRGTPE")]
pub type Prgtpe = crate::Reg<prgtpe::PrgtpeSpec>;
#[doc = "Flash PRGTPE register"]
pub mod prgtpe;
#[doc = "PRETPE (rw) register accessor: Flash PRETPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`pretpe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pretpe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pretpe`] module"]
#[doc(alias = "PRETPE")]
pub type Pretpe = crate::Reg<pretpe::PretpeSpec>;
#[doc = "Flash PRETPE register"]
pub mod pretpe;
