#[doc = "Register `PERTPE` reader"]
pub type R = crate::R<PertpeSpec>;
#[doc = "Register `PERTPE` writer"]
pub type W = crate::W<PertpeSpec>;
#[doc = "Field `PERTPE` reader - FLash PERTPE register"]
pub type PertpeR = crate::FieldReader<u32>;
#[doc = "Field `PERTPE` writer - FLash PERTPE register"]
pub type PertpeW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    pub fn pertpe(&self) -> PertpeR {
        PertpeR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - FLash PERTPE register"]
    #[inline(always)]
    pub fn pertpe(&mut self) -> PertpeW<'_, PertpeSpec> {
        PertpeW::new(self, 0)
    }
}
#[doc = "Flash PERTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`pertpe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pertpe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PertpeSpec;
impl crate::RegisterSpec for PertpeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pertpe::R`](R) reader structure"]
impl crate::Readable for PertpeSpec {}
#[doc = "`write(|w| ..)` method takes [`pertpe::W`](W) writer structure"]
impl crate::Writable for PertpeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERTPE to value 0xea60"]
impl crate::Resettable for PertpeSpec {
    const RESET_VALUE: u32 = 0xea60;
}
