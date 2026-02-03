#[doc = "Register `PRLH` writer"]
pub type W = crate::W<PrlhSpec>;
#[doc = "Field `PRLH` writer - RTC Prescaler Load Register High"]
pub type PrlhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub fn prlh(&mut self) -> PrlhW<'_, PrlhSpec> {
        PrlhW::new(self, 0)
    }
}
#[doc = "RTC Prescaler Load Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prlh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrlhSpec;
impl crate::RegisterSpec for PrlhSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prlh::W`](W) writer structure"]
impl crate::Writable for PrlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRLH to value 0"]
impl crate::Resettable for PrlhSpec {}
