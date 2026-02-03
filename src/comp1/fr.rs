#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Field `FLTEN` reader - Filter enable bit"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - Filter enable bit"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCNT` reader - Comparator filter and counter"]
pub type FltcntR = crate::FieldReader<u16>;
#[doc = "Field `FLTCNT` writer - Comparator filter and counter"]
pub type FltcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    pub fn fltcnt(&self) -> FltcntR {
        FltcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Filter enable bit"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, FrSpec> {
        FltenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Comparator filter and counter"]
    #[inline(always)]
    pub fn fltcnt(&mut self) -> FltcntW<'_, FrSpec> {
        FltcntW::new(self, 16)
    }
}
#[doc = "Comparator Filter register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {}
