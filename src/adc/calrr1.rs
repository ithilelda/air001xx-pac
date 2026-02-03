#[doc = "Register `CALRR1` reader"]
pub type R = crate::R<Calrr1Spec>;
#[doc = "Field `CALC4OUT` reader - C4 result"]
pub type Calc4outR = crate::FieldReader;
#[doc = "Field `CALC5OUT` reader - C5 result"]
pub type Calc5outR = crate::FieldReader;
#[doc = "Field `CALBOUT` reader - offset result"]
pub type CalboutR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - C4 result"]
    #[inline(always)]
    pub fn calc4out(&self) -> Calc4outR {
        Calc4outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - C5 result"]
    #[inline(always)]
    pub fn calc5out(&self) -> Calc5outR {
        Calc5outR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - offset result"]
    #[inline(always)]
    pub fn calbout(&self) -> CalboutR {
        CalboutR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "ADC calibration result register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`calrr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calrr1Spec;
impl crate::RegisterSpec for Calrr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calrr1::R`](R) reader structure"]
impl crate::Readable for Calrr1Spec {}
#[doc = "`reset()` method sets CALRR1 to value 0"]
impl crate::Resettable for Calrr1Spec {}
