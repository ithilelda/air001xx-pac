#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<IcscrSpec>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<IcscrSpec>;
#[doc = "Field `HSI_TRIM` reader - HSI clock trimming"]
pub type HsiTrimR = crate::FieldReader<u16>;
#[doc = "Field `HSI_TRIM` writer - HSI clock trimming"]
pub type HsiTrimW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HSI_FS` reader - HSI frequency selection"]
pub type HsiFsR = crate::FieldReader;
#[doc = "Field `HSI_FS` writer - HSI frequency selection"]
pub type HsiFsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LSI_TRIM` reader - LSI clock trimming"]
pub type LsiTrimR = crate::FieldReader<u16>;
#[doc = "Field `LSI_TRIM` writer - LSI clock trimming"]
pub type LsiTrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LSI_STARTUP` reader - LSI startup time"]
pub type LsiStartupR = crate::FieldReader;
#[doc = "Field `LSI_STARTUP` writer - LSI startup time"]
pub type LsiStartupW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsi_trim(&self) -> HsiTrimR {
        HsiTrimR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    pub fn hsi_fs(&self) -> HsiFsR {
        HsiFsR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    pub fn lsi_trim(&self) -> LsiTrimR {
        LsiTrimR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    pub fn lsi_startup(&self) -> LsiStartupR {
        LsiStartupR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsi_trim(&mut self) -> HsiTrimW<'_, IcscrSpec> {
        HsiTrimW::new(self, 0)
    }
    #[doc = "Bits 13:15 - HSI frequency selection"]
    #[inline(always)]
    pub fn hsi_fs(&mut self) -> HsiFsW<'_, IcscrSpec> {
        HsiFsW::new(self, 13)
    }
    #[doc = "Bits 16:24 - LSI clock trimming"]
    #[inline(always)]
    pub fn lsi_trim(&mut self) -> LsiTrimW<'_, IcscrSpec> {
        LsiTrimW::new(self, 16)
    }
    #[doc = "Bits 26:27 - LSI startup time"]
    #[inline(always)]
    pub fn lsi_startup(&mut self) -> LsiStartupW<'_, IcscrSpec> {
        LsiStartupW::new(self, 26)
    }
}
#[doc = "Internal clock sources calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcscrSpec;
impl crate::RegisterSpec for IcscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for IcscrSpec {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for IcscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICSCR to value 0x1000_0000"]
impl crate::Resettable for IcscrSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
