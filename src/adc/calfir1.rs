#[doc = "Register `CALFIR1` reader"]
pub type R = crate::R<Calfir1Spec>;
#[doc = "Register `CALFIR1` writer"]
pub type W = crate::W<Calfir1Spec>;
#[doc = "Field `CALC4IO` reader - Calibration C4 factor input"]
pub type Calc4ioR = crate::FieldReader;
#[doc = "Field `CALC4IO` writer - Calibration C4 factor input"]
pub type Calc4ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALC5IO` reader - Calibration C5 factor input"]
pub type Calc5ioR = crate::FieldReader;
#[doc = "Field `CALC5IO` writer - Calibration C5 factor input"]
pub type Calc5ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALBIO` reader - Calibration offset factor input"]
pub type CalbioR = crate::FieldReader;
#[doc = "Field `CALBIO` writer - Calibration offset factor input"]
pub type CalbioW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    pub fn calc4io(&self) -> Calc4ioR {
        Calc4ioR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Calibration C5 factor input"]
    #[inline(always)]
    pub fn calc5io(&self) -> Calc5ioR {
        Calc5ioR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Calibration offset factor input"]
    #[inline(always)]
    pub fn calbio(&self) -> CalbioR {
        CalbioR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C4 factor input"]
    #[inline(always)]
    pub fn calc4io(&mut self) -> Calc4ioW<'_, Calfir1Spec> {
        Calc4ioW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Calibration C5 factor input"]
    #[inline(always)]
    pub fn calc5io(&mut self) -> Calc5ioW<'_, Calfir1Spec> {
        Calc5ioW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Calibration offset factor input"]
    #[inline(always)]
    pub fn calbio(&mut self) -> CalbioW<'_, Calfir1Spec> {
        CalbioW::new(self, 16)
    }
}
#[doc = "ADC calibration factor input register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`calfir1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfir1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calfir1Spec;
impl crate::RegisterSpec for Calfir1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfir1::R`](R) reader structure"]
impl crate::Readable for Calfir1Spec {}
#[doc = "`write(|w| ..)` method takes [`calfir1::W`](W) writer structure"]
impl crate::Writable for Calfir1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALFIR1 to value 0"]
impl crate::Resettable for Calfir1Spec {}
