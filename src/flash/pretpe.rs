#[doc = "Register `PRETPE` reader"]
pub type R = crate::R<PretpeSpec>;
#[doc = "Register `PRETPE` writer"]
pub type W = crate::W<PretpeSpec>;
#[doc = "Field `PRETPE` reader - FLash PRETPE register"]
pub type PretpeR = crate::FieldReader<u16>;
#[doc = "Field `PRETPE` writer - FLash PRETPE register"]
pub type PretpeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    pub fn pretpe(&self) -> PretpeR {
        PretpeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - FLash PRETPE register"]
    #[inline(always)]
    pub fn pretpe(&mut self) -> PretpeW<'_, PretpeSpec> {
        PretpeW::new(self, 0)
    }
}
#[doc = "Flash PRETPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`pretpe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pretpe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PretpeSpec;
impl crate::RegisterSpec for PretpeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pretpe::R`](R) reader structure"]
impl crate::Readable for PretpeSpec {}
#[doc = "`write(|w| ..)` method takes [`pretpe::W`](W) writer structure"]
impl crate::Writable for PretpeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRETPE to value 0x12c0"]
impl crate::Resettable for PretpeSpec {
    const RESET_VALUE: u32 = 0x12c0;
}
