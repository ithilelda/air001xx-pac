#[doc = "Register `CCIPR` reader"]
pub type R = crate::R<CciprSpec>;
#[doc = "Register `CCIPR` writer"]
pub type W = crate::W<CciprSpec>;
#[doc = "Field `PVDSEL` reader - PVD detect clock source selection"]
pub type PvdselR = crate::BitReader;
#[doc = "Field `PVDSEL` writer - PVD detect clock source selection"]
pub type PvdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1SEL` reader - COMP1 clock source selection"]
pub type Comp1selR = crate::BitReader;
#[doc = "Field `COMP1SEL` writer - COMP1 clock source selection"]
pub type Comp1selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2SEL` reader - COMP2 clock source selection"]
pub type Comp2selR = crate::BitReader;
#[doc = "Field `COMP2SEL` writer - COMP2 clock source selection"]
pub type Comp2selW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 clock source selection"]
pub type Lptim1selR = crate::FieldReader;
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 clock source selection"]
pub type Lptim1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    pub fn pvdsel(&self) -> PvdselR {
        PvdselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    pub fn comp1sel(&self) -> Comp1selR {
        Comp1selR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    pub fn comp2sel(&self) -> Comp2selR {
        Comp2selR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> Lptim1selR {
        Lptim1selR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - PVD detect clock source selection"]
    #[inline(always)]
    pub fn pvdsel(&mut self) -> PvdselW<'_, CciprSpec> {
        PvdselW::new(self, 7)
    }
    #[doc = "Bit 8 - COMP1 clock source selection"]
    #[inline(always)]
    pub fn comp1sel(&mut self) -> Comp1selW<'_, CciprSpec> {
        Comp1selW::new(self, 8)
    }
    #[doc = "Bit 9 - COMP2 clock source selection"]
    #[inline(always)]
    pub fn comp2sel(&mut self) -> Comp2selW<'_, CciprSpec> {
        Comp2selW::new(self, 9)
    }
    #[doc = "Bits 18:19 - LPTIM1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> Lptim1selW<'_, CciprSpec> {
        Lptim1selW::new(self, 18)
    }
}
#[doc = "Peripherals independent clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciprSpec;
impl crate::RegisterSpec for CciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr::R`](R) reader structure"]
impl crate::Readable for CciprSpec {}
#[doc = "`write(|w| ..)` method takes [`ccipr::W`](W) writer structure"]
impl crate::Writable for CciprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CciprSpec {}
