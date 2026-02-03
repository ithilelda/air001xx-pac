#[doc = "Register `CICR` writer"]
pub type W = crate::W<CicrSpec>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - clock secure system interrupt flag clear"]
pub type CsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` writer - LSE clock secure system interrupt flag clear"]
pub type LsecsscW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<'_, CicrSpec> {
        LsirdycW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<'_, CicrSpec> {
        LserdycW::new(self, 1)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<'_, CicrSpec> {
        HsirdycW::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<'_, CicrSpec> {
        HserdycW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<'_, CicrSpec> {
        PllrdycW::new(self, 5)
    }
    #[doc = "Bit 8 - clock secure system interrupt flag clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CsscW<'_, CicrSpec> {
        CsscW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE clock secure system interrupt flag clear"]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LsecsscW<'_, CicrSpec> {
        LsecsscW::new(self, 9)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicrSpec;
impl crate::RegisterSpec for CicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CicrSpec {}
