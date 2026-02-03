#[doc = "Register `STCR` reader"]
pub type R = crate::R<StcrSpec>;
#[doc = "Register `STCR` writer"]
pub type W = crate::W<StcrSpec>;
#[doc = "Field `SLEEP_EN` reader - FLash sleep enable"]
pub type SleepEnR = crate::BitReader;
#[doc = "Field `SLEEP_EN` writer - FLash sleep enable"]
pub type SleepEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_TIME` reader - FLash sleep time configuration(counter based on HSI_10M)"]
pub type SleepTimeR = crate::FieldReader;
#[doc = "Field `SLEEP_TIME` writer - FLash sleep time configuration(counter based on HSI_10M)"]
pub type SleepTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - FLash sleep enable"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SleepEnR {
        SleepEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - FLash sleep time configuration(counter based on HSI_10M)"]
    #[inline(always)]
    pub fn sleep_time(&self) -> SleepTimeR {
        SleepTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLash sleep enable"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SleepEnW<'_, StcrSpec> {
        SleepEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - FLash sleep time configuration(counter based on HSI_10M)"]
    #[inline(always)]
    pub fn sleep_time(&mut self) -> SleepTimeW<'_, StcrSpec> {
        SleepTimeW::new(self, 8)
    }
}
#[doc = "Flash sleep time config register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcrSpec;
impl crate::RegisterSpec for StcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcr::R`](R) reader structure"]
impl crate::Readable for StcrSpec {}
#[doc = "`write(|w| ..)` method takes [`stcr::W`](W) writer structure"]
impl crate::Writable for StcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCR to value 0x6400"]
impl crate::Resettable for StcrSpec {
    const RESET_VALUE: u32 = 0x6400;
}
