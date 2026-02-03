#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCSEL` reader - Power voltage detector volatage selection"]
pub type SrcselR = crate::BitReader;
#[doc = "Field `SRCSEL` writer - Power voltage detector volatage selection"]
pub type SrcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDT` reader - Power voltage detector threshold selection"]
pub type PvdtR = crate::FieldReader;
#[doc = "Field `PVDT` writer - Power voltage detector threshold selection"]
pub type PvdtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLTEN` reader - Digital filter enable"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - Digital filter enable"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT_TIME` reader - Digital filter time configuration"]
pub type FltTimeR = crate::FieldReader;
#[doc = "Field `FLT_TIME` writer - Digital filter time configuration"]
pub type FltTimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    pub fn srcsel(&self) -> SrcselR {
        SrcselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    pub fn pvdt(&self) -> PvdtR {
        PvdtR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    pub fn flt_time(&self) -> FltTimeR {
        FltTimeR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<'_, Cr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bit 2 - Power voltage detector volatage selection"]
    #[inline(always)]
    pub fn srcsel(&mut self) -> SrcselW<'_, Cr2Spec> {
        SrcselW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Power voltage detector threshold selection"]
    #[inline(always)]
    pub fn pvdt(&mut self) -> PvdtW<'_, Cr2Spec> {
        PvdtW::new(self, 4)
    }
    #[doc = "Bit 8 - Digital filter enable"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, Cr2Spec> {
        FltenW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Digital filter time configuration"]
    #[inline(always)]
    pub fn flt_time(&mut self) -> FltTimeW<'_, Cr2Spec> {
        FltTimeW::new(self, 9)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0x0500"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0x0500;
}
