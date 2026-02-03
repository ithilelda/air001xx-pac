#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    cr: Cr,
    apb_fz1: ApbFz1,
    apb_fz2: ApbFz2,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - Debug MCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x08 - APB Freeze Register1"]
    #[inline(always)]
    pub const fn apb_fz1(&self) -> &ApbFz1 {
        &self.apb_fz1
    }
    #[doc = "0x0c - APB Freeze Register2"]
    #[inline(always)]
    pub const fn apb_fz2(&self) -> &ApbFz2 {
        &self.apb_fz2
    }
}
#[doc = "IDCODE (r) register accessor: MCU Device ID Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: Debug MCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB_FZ1 (rw) register accessor: APB Freeze Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_fz1`] module"]
#[doc(alias = "APB_FZ1")]
pub type ApbFz1 = crate::Reg<apb_fz1::ApbFz1Spec>;
#[doc = "APB Freeze Register1"]
pub mod apb_fz1;
#[doc = "APB_FZ2 (rw) register accessor: APB Freeze Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb_fz2`] module"]
#[doc(alias = "APB_FZ2")]
pub type ApbFz2 = crate::Reg<apb_fz2::ApbFz2Spec>;
#[doc = "APB Freeze Register2"]
pub mod apb_fz2;
