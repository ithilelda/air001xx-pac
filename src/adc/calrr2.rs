#[doc = "Register `CALRR2` reader"]
pub type R = crate::R<Calrr2Spec>;
#[doc = "Field `CALC0OUT` reader - C0 result"]
pub type Calc0outR = crate::FieldReader;
#[doc = "Field `CALC1OUT` reader - C1 result"]
pub type Calc1outR = crate::FieldReader;
#[doc = "Field `CALC2OUT` reader - C2 result"]
pub type Calc2outR = crate::FieldReader;
#[doc = "Field `CALC3OUT` reader - C3 result"]
pub type Calc3outR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - C0 result"]
    #[inline(always)]
    pub fn calc0out(&self) -> Calc0outR {
        Calc0outR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - C1 result"]
    #[inline(always)]
    pub fn calc1out(&self) -> Calc1outR {
        Calc1outR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - C2 result"]
    #[inline(always)]
    pub fn calc2out(&self) -> Calc2outR {
        Calc2outR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - C3 result"]
    #[inline(always)]
    pub fn calc3out(&self) -> Calc3outR {
        Calc3outR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ADC calibration result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calrr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calrr2Spec;
impl crate::RegisterSpec for Calrr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calrr2::R`](R) reader structure"]
impl crate::Readable for Calrr2Spec {}
#[doc = "`reset()` method sets CALRR2 to value 0"]
impl crate::Resettable for Calrr2Spec {}
