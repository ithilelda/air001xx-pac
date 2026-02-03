#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub type FlashenR = crate::BitReader;
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAM memory interface clock enable"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM memory interface clock enable"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM memory interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, AhbenrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&mut self) -> FlashenW<'_, AhbenrSpec> {
        FlashenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRAM memory interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SramenW<'_, AhbenrSpec> {
        SramenW::new(self, 9)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, AhbenrSpec> {
        CrcenW::new(self, 12)
    }
}
#[doc = "AHB peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0"]
impl crate::Resettable for AhbenrSpec {}
