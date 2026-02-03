#[doc = "Register `WINR` reader"]
pub type R = crate::R<WinrSpec>;
#[doc = "Field `WIN` reader - window counter"]
pub type WinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - window counter"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Window register (IWDG_SR)\n\nYou can [`read`](crate::Reg::read) this register and get [`winr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinrSpec;
impl crate::RegisterSpec for WinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winr::R`](R) reader structure"]
impl crate::Readable for WinrSpec {}
#[doc = "`reset()` method sets WINR to value 0"]
impl crate::Resettable for WinrSpec {}
