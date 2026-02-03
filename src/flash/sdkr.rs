#[doc = "Register `SDKR` reader"]
pub type R = crate::R<SdkrSpec>;
#[doc = "Register `SDKR` writer"]
pub type W = crate::W<SdkrSpec>;
#[doc = "Field `SDK_STRT` reader - SDK area start address"]
pub type SdkStrtR = crate::FieldReader;
#[doc = "Field `SDK_STRT` writer - SDK area start address"]
pub type SdkStrtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDK_END` reader - SDK area end address"]
pub type SdkEndR = crate::FieldReader;
#[doc = "Field `SDK_END` writer - SDK area end address"]
pub type SdkEndW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - SDK area start address"]
    #[inline(always)]
    pub fn sdk_strt(&self) -> SdkStrtR {
        SdkStrtR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - SDK area end address"]
    #[inline(always)]
    pub fn sdk_end(&self) -> SdkEndR {
        SdkEndR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SDK area start address"]
    #[inline(always)]
    pub fn sdk_strt(&mut self) -> SdkStrtW<'_, SdkrSpec> {
        SdkStrtW::new(self, 0)
    }
    #[doc = "Bits 8:12 - SDK area end address"]
    #[inline(always)]
    pub fn sdk_end(&mut self) -> SdkEndW<'_, SdkrSpec> {
        SdkEndW::new(self, 8)
    }
}
#[doc = "Flash SDK address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdkrSpec;
impl crate::RegisterSpec for SdkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdkr::R`](R) reader structure"]
impl crate::Readable for SdkrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdkr::W`](W) writer structure"]
impl crate::Writable for SdkrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDKR to value 0xffe0_001f"]
impl crate::Resettable for SdkrSpec {
    const RESET_VALUE: u32 = 0xffe0_001f;
}
