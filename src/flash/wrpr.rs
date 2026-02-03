#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WrprSpec>;
#[doc = "Register `WRPR` writer"]
pub type W = crate::W<WrprSpec>;
#[doc = "Field `WRP` reader - WRP address"]
pub type WrpR = crate::FieldReader<u16>;
#[doc = "Field `WRP` writer - WRP address"]
pub type WrpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WRP address"]
    #[inline(always)]
    pub fn wrp(&self) -> WrpR {
        WrpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WRP address"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WrpW<'_, WrprSpec> {
        WrpW::new(self, 0)
    }
}
#[doc = "Flash WRP address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrprSpec;
impl crate::RegisterSpec for WrprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WrprSpec {}
#[doc = "`write(|w| ..)` method takes [`wrpr::W`](W) writer structure"]
impl crate::Writable for WrprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRPR to value 0xffff"]
impl crate::Resettable for WrprSpec {
    const RESET_VALUE: u32 = 0xffff;
}
