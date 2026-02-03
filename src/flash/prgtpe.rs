#[doc = "Register `PRGTPE` reader"]
pub type R = crate::R<PrgtpeSpec>;
#[doc = "Register `PRGTPE` writer"]
pub type W = crate::W<PrgtpeSpec>;
#[doc = "Field `PRGTPE` reader - FLash PRGTPE register"]
pub type PrgtpeR = crate::FieldReader<u16>;
#[doc = "Field `PRGTPE` writer - FLash PRGTPE register"]
pub type PrgtpeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    pub fn prgtpe(&self) -> PrgtpeR {
        PrgtpeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FLash PRGTPE register"]
    #[inline(always)]
    pub fn prgtpe(&mut self) -> PrgtpeW<'_, PrgtpeSpec> {
        PrgtpeW::new(self, 0)
    }
}
#[doc = "Flash PRGTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`prgtpe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgtpe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrgtpeSpec;
impl crate::RegisterSpec for PrgtpeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prgtpe::R`](R) reader structure"]
impl crate::Readable for PrgtpeSpec {}
#[doc = "`write(|w| ..)` method takes [`prgtpe::W`](W) writer structure"]
impl crate::Writable for PrgtpeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRGTPE to value 0x8ca0"]
impl crate::Resettable for PrgtpeSpec {
    const RESET_VALUE: u32 = 0x8ca0;
}
