#[doc = "Register `CALFIR2` reader"]
pub type R = crate::R<Calfir2Spec>;
#[doc = "Register `CALFIR2` writer"]
pub type W = crate::W<Calfir2Spec>;
#[doc = "Field `CALC0IO` reader - Calibration C0 factor input"]
pub type Calc0ioR = crate::FieldReader;
#[doc = "Field `CALC0IO` writer - Calibration C0 factor input"]
pub type Calc0ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALC1IO` reader - Calibration C1 factor input"]
pub type Calc1ioR = crate::FieldReader;
#[doc = "Field `CALC1IO` writer - Calibration C1 factor input"]
pub type Calc1ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALC2IO` reader - Calibration C2 factor input"]
pub type Calc2ioR = crate::FieldReader;
#[doc = "Field `CALC2IO` writer - Calibration C2 factor input"]
pub type Calc2ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CALC3IO` reader - Calibration C3 factor input"]
pub type Calc3ioR = crate::FieldReader;
#[doc = "Field `CALC3IO` writer - Calibration C3 factor input"]
pub type Calc3ioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calc0io(&self) -> Calc0ioR {
        Calc0ioR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc1io(&self) -> Calc1ioR {
        Calc1ioR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc2io(&self) -> Calc2ioR {
        Calc2ioR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc3io(&self) -> Calc3ioR {
        Calc3ioR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration C0 factor input"]
    #[inline(always)]
    pub fn calc0io(&mut self) -> Calc0ioW<'_, Calfir2Spec> {
        Calc0ioW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Calibration C1 factor input"]
    #[inline(always)]
    pub fn calc1io(&mut self) -> Calc1ioW<'_, Calfir2Spec> {
        Calc1ioW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Calibration C2 factor input"]
    #[inline(always)]
    pub fn calc2io(&mut self) -> Calc2ioW<'_, Calfir2Spec> {
        Calc2ioW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Calibration C3 factor input"]
    #[inline(always)]
    pub fn calc3io(&mut self) -> Calc3ioW<'_, Calfir2Spec> {
        Calc3ioW::new(self, 24)
    }
}
#[doc = "ADC calibration factor input register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calfir2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfir2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calfir2Spec;
impl crate::RegisterSpec for Calfir2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfir2::R`](R) reader structure"]
impl crate::Readable for Calfir2Spec {}
#[doc = "`write(|w| ..)` method takes [`calfir2::W`](W) writer structure"]
impl crate::Writable for Calfir2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALFIR2 to value 0"]
impl crate::Resettable for Calfir2Spec {}
