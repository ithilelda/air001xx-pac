#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `LSION` reader - LSI oscillator enable"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - LSI oscillator enable"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - LSI oscillator ready"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSIRDY` writer - LSI oscillator ready"]
pub type LsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVF` reader - Remove reset flags"]
pub type RmvfR = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flags"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub type OblrstfR = crate::BitReader;
#[doc = "Field `OBLRSTF` writer - Option byte loader reset flag"]
pub type OblrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - Pin reset flag"]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` writer - Pin reset flag"]
pub type PinrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRSTF` reader - BOR or POR/PDR flag"]
pub type PwrrstfR = crate::BitReader;
#[doc = "Field `PWRRSTF` writer - BOR or POR/PDR flag"]
pub type PwrrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub type SftrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDGRSTF` reader - Independent window watchdog reset flag"]
pub type IwdgrstfR = crate::BitReader;
#[doc = "Field `IWDGRSTF` writer - Independent window watchdog reset flag"]
pub type IwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WwdgrstfR = crate::BitReader;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub type WwdgrstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flags"]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OblrstfR {
        OblrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR or POR/PDR flag"]
    #[inline(always)]
    pub fn pwrrstf(&self) -> PwrrstfR {
        PwrrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IwdgrstfR {
        IwdgrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WwdgrstfR {
        WwdgrstfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<'_, CsrSpec> {
        LsionW::new(self, 0)
    }
    #[doc = "Bit 1 - LSI oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&mut self) -> LsirdyW<'_, CsrSpec> {
        LsirdyW::new(self, 1)
    }
    #[doc = "Bit 23 - Remove reset flags"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<'_, CsrSpec> {
        RmvfW::new(self, 23)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OblrstfW<'_, CsrSpec> {
        OblrstfW::new(self, 25)
    }
    #[doc = "Bit 26 - Pin reset flag"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PinrstfW<'_, CsrSpec> {
        PinrstfW::new(self, 26)
    }
    #[doc = "Bit 27 - BOR or POR/PDR flag"]
    #[inline(always)]
    pub fn pwrrstf(&mut self) -> PwrrstfW<'_, CsrSpec> {
        PwrrstfW::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SftrstfW<'_, CsrSpec> {
        SftrstfW::new(self, 28)
    }
    #[doc = "Bit 29 - Independent window watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IwdgrstfW<'_, CsrSpec> {
        IwdgrstfW::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WwdgrstfW<'_, CsrSpec> {
        WwdgrstfW::new(self, 30)
    }
}
#[doc = "Control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
