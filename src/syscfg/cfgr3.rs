#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<Cfgr3Spec>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<Cfgr3Spec>;
#[doc = "Field `DMA1_MAP` reader - DMA channel1 requeset selection"]
pub type Dma1MapR = crate::FieldReader;
#[doc = "Field `DMA1_MAP` writer - DMA channel1 requeset selection"]
pub type Dma1MapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMA2_MAP` reader - DMA channel2 requeset selection"]
pub type Dma2MapR = crate::FieldReader;
#[doc = "Field `DMA2_MAP` writer - DMA channel2 requeset selection"]
pub type Dma2MapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMA3_MAP` reader - DMA channel3 requeset selection"]
pub type Dma3MapR = crate::FieldReader;
#[doc = "Field `DMA3_MAP` writer - DMA channel3 requeset selection"]
pub type Dma3MapW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    pub fn dma1_map(&self) -> Dma1MapR {
        Dma1MapR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    pub fn dma2_map(&self) -> Dma2MapR {
        Dma2MapR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    pub fn dma3_map(&self) -> Dma3MapR {
        Dma3MapR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA channel1 requeset selection"]
    #[inline(always)]
    pub fn dma1_map(&mut self) -> Dma1MapW<'_, Cfgr3Spec> {
        Dma1MapW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA channel2 requeset selection"]
    #[inline(always)]
    pub fn dma2_map(&mut self) -> Dma2MapW<'_, Cfgr3Spec> {
        Dma2MapW::new(self, 8)
    }
    #[doc = "Bits 16:20 - DMA channel3 requeset selection"]
    #[inline(always)]
    pub fn dma3_map(&mut self) -> Dma3MapW<'_, Cfgr3Spec> {
        Dma3MapW::new(self, 16)
    }
}
#[doc = "SYSCFG configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr3Spec;
impl crate::RegisterSpec for Cfgr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for Cfgr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for Cfgr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for Cfgr3Spec {}
