#[doc = "Register `CCSR` reader"]
pub type R = crate::R<CcsrSpec>;
#[doc = "Register `CCSR` writer"]
pub type W = crate::W<CcsrSpec>;
#[doc = "Field `CALSEL` reader - Calibration contents selection"]
pub type CalselR = crate::BitReader;
#[doc = "Field `CALSEL` writer - Calibration contents selection"]
pub type CalselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSMP` reader - Calibration sample time selection"]
pub type CalsmpR = crate::FieldReader;
#[doc = "Field `CALSMP` writer - Calibration sample time selection"]
pub type CalsmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CALSET` reader - Calibration factor selection"]
pub type CalsetR = crate::BitReader;
#[doc = "Field `CALSET` writer - Calibration factor selection"]
pub type CalsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALFAIL` reader - Calibration fail flag"]
pub type CalfailR = crate::BitReader;
#[doc = "Field `CALFAIL` writer - Calibration fail flag"]
pub type CalfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - Calibration flag"]
pub type CalonR = crate::BitReader;
impl R {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CalselR {
        CalselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    pub fn calsmp(&self) -> CalsmpR {
        CalsmpR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    pub fn calset(&self) -> CalsetR {
        CalsetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    pub fn calfail(&self) -> CalfailR {
        CalfailR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Calibration flag"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Calibration contents selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CalselW<'_, CcsrSpec> {
        CalselW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration sample time selection"]
    #[inline(always)]
    pub fn calsmp(&mut self) -> CalsmpW<'_, CcsrSpec> {
        CalsmpW::new(self, 12)
    }
    #[doc = "Bit 15 - Calibration factor selection"]
    #[inline(always)]
    pub fn calset(&mut self) -> CalsetW<'_, CcsrSpec> {
        CalsetW::new(self, 15)
    }
    #[doc = "Bit 30 - Calibration fail flag"]
    #[inline(always)]
    pub fn calfail(&mut self) -> CalfailW<'_, CcsrSpec> {
        CalfailW::new(self, 30)
    }
}
#[doc = "ADC calibration configuration and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcsrSpec;
impl crate::RegisterSpec for CcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccsr::R`](R) reader structure"]
impl crate::Readable for CcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccsr::W`](W) writer structure"]
impl crate::Writable for CcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCSR to value 0"]
impl crate::Resettable for CcsrSpec {}
