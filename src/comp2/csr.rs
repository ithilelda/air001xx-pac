#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `COMP_EN` reader - COMP enable bit"]
pub type CompEnR = crate::BitReader;
#[doc = "Field `COMP_EN` writer - COMP enable bit"]
pub type CompEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INMSEL` reader - Comparator signal selector for inverting input INM"]
pub type InmselR = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator signal selector for inverting input INM"]
pub type InmselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL` reader - Comparator signal selector for non-inverting input"]
pub type InpselR = crate::FieldReader;
#[doc = "Field `INPSEL` writer - Comparator signal selector for non-inverting input"]
pub type InpselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WINMODE` reader - Comparator non-inverting input selector for window mode"]
pub type WinmodeR = crate::BitReader;
#[doc = "Field `WINMODE` writer - Comparator non-inverting input selector for window mode"]
pub type WinmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - Comparator polarity selector"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - Comparator polarity selector"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRMODE` reader - Comparator power mode selector"]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Comparator power mode selector"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP_OUT` reader - Comparator output status"]
pub type CompOutR = crate::BitReader;
#[doc = "Field `COMP_OUT` writer - Comparator output status"]
pub type CompOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - CSR register lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - CSR register lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    pub fn comp_en(&self) -> CompEnR {
        CompEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&self) -> InmselR {
        InmselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    pub fn comp_out(&self) -> CompOutR {
        CompOutR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CSR register lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP enable bit"]
    #[inline(always)]
    pub fn comp_en(&mut self) -> CompEnW<'_, CsrSpec> {
        CompEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Comparator signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> InmselW<'_, CsrSpec> {
        InmselW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Comparator signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<'_, CsrSpec> {
        InpselW::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WinmodeW<'_, CsrSpec> {
        WinmodeW::new(self, 11)
    }
    #[doc = "Bit 15 - Comparator polarity selector"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<'_, CsrSpec> {
        PolarityW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Comparator power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<'_, CsrSpec> {
        PwrmodeW::new(self, 18)
    }
    #[doc = "Bit 30 - Comparator output status"]
    #[inline(always)]
    pub fn comp_out(&mut self) -> CompOutW<'_, CsrSpec> {
        CompOutW::new(self, 30)
    }
    #[doc = "Bit 31 - CSR register lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CsrSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "COMP control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
