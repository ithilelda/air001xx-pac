#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `PRESC` reader - Clock prescaler"]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - Clock prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRELOAD` reader - Registers update mode"]
pub type PreloadR = crate::BitReader;
#[doc = "Field `PRELOAD` writer - Registers update mode"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, CfgrSpec> {
        PrescW::new(self, 9)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<'_, CfgrSpec> {
        PreloadW::new(self, 22)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
