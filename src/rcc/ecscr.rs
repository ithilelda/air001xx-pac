#[doc = "Register `ECSCR` reader"]
pub type R = crate::R<EcscrSpec>;
#[doc = "Register `ECSCR` writer"]
pub type W = crate::W<EcscrSpec>;
#[doc = "Field `HSE_FREQ` reader - HSE clock freqency selection"]
pub type HseFreqR = crate::FieldReader;
#[doc = "Field `HSE_FREQ` writer - HSE clock freqency selection"]
pub type HseFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LSE_DRIVER` reader - LSE clock driver selection"]
pub type LseDriverR = crate::FieldReader;
#[doc = "Field `LSE_DRIVER` writer - LSE clock driver selection"]
pub type LseDriverW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    pub fn hse_freq(&self) -> HseFreqR {
        HseFreqR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    pub fn lse_driver(&self) -> LseDriverR {
        LseDriverR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - HSE clock freqency selection"]
    #[inline(always)]
    pub fn hse_freq(&mut self) -> HseFreqW<'_, EcscrSpec> {
        HseFreqW::new(self, 2)
    }
    #[doc = "Bits 16:17 - LSE clock driver selection"]
    #[inline(always)]
    pub fn lse_driver(&mut self) -> LseDriverW<'_, EcscrSpec> {
        LseDriverW::new(self, 16)
    }
}
#[doc = "External clock source control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcscrSpec;
impl crate::RegisterSpec for EcscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecscr::R`](R) reader structure"]
impl crate::Readable for EcscrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecscr::W`](W) writer structure"]
impl crate::Writable for EcscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECSCR to value 0"]
impl crate::Resettable for EcscrSpec {}
