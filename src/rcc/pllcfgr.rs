#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PllcfgrSpec>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PllcfgrSpec>;
#[doc = "Field `PLLSRC` reader - PLL clock source selection"]
pub type PllsrcR = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLL clock source selection"]
pub type PllsrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<'_, PllcfgrSpec> {
        PllsrcW::new(self, 0)
    }
}
#[doc = "PLL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgrSpec;
impl crate::RegisterSpec for PllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCFGR to value 0"]
impl crate::Resettable for PllcfgrSpec {}
