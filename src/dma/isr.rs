#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `GIF1` reader - Channel 1 Global interrupt flag"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - Channel 1 Transfer Complete flag"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `HTIF1` reader - Channel 1 Half Transfer Complete flag"]
pub type Htif1R = crate::BitReader;
#[doc = "Field `TEIF1` reader - Channel 1 Transfer Error flag"]
pub type Teif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - Channel 2 Global interrupt flag"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - Channel 2 Transfer Complete flag"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `HTIF2` reader - Channel 2 Half Transfer Complete flag"]
pub type Htif2R = crate::BitReader;
#[doc = "Field `TEIF2` reader - Channel 2 Transfer Error flag"]
pub type Teif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - Channel 3 Global interrupt flag"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - Channel 3 Transfer Complete flag"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `HTIF3` reader - Channel 3 Half Transfer Complete flag"]
pub type Htif3R = crate::BitReader;
#[doc = "Field `TEIF3` reader - Channel 3 Transfer Error flag"]
pub type Teif3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register (DMA_ISR)\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
