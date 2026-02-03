#[doc = "Register `SMERTPE` reader"]
pub type R = crate::R<SmertpeSpec>;
#[doc = "Register `SMERTPE` writer"]
pub type W = crate::W<SmertpeSpec>;
#[doc = "Field `SMERTPE` reader - FLash SMERTPE register"]
pub type SmertpeR = crate::FieldReader<u32>;
#[doc = "Field `SMERTPE` writer - FLash SMERTPE register"]
pub type SmertpeW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - FLash SMERTPE register"]
    #[inline(always)]
    pub fn smertpe(&self) -> SmertpeR {
        SmertpeR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - FLash SMERTPE register"]
    #[inline(always)]
    pub fn smertpe(&mut self) -> SmertpeW<'_, SmertpeSpec> {
        SmertpeW::new(self, 0)
    }
}
#[doc = "Flash SMERTPE register\n\nYou can [`read`](crate::Reg::read) this register and get [`smertpe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smertpe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmertpeSpec;
impl crate::RegisterSpec for SmertpeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smertpe::R`](R) reader structure"]
impl crate::Readable for SmertpeSpec {}
#[doc = "`write(|w| ..)` method takes [`smertpe::W`](W) writer structure"]
impl crate::Writable for SmertpeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMERTPE to value 0xfd20"]
impl crate::Resettable for SmertpeSpec {
    const RESET_VALUE: u32 = 0xfd20;
}
