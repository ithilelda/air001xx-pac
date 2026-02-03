#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LEDON` reader - LED enable"]
pub type LedonR = crate::BitReader;
#[doc = "Field `LEDON` writer - LED enable"]
pub type LedonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LED_COM_SEL` reader - LED COM Selection"]
pub type LedComSelR = crate::FieldReader;
#[doc = "Field `LED_COM_SEL` writer - LED COM Selection"]
pub type LedComSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IE` reader - LED interrupt enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - LED interrupt enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHS` reader - Light control"]
pub type EhsR = crate::FieldReader;
#[doc = "Field `EHS` writer - Light control"]
pub type EhsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    pub fn ledon(&self) -> LedonR {
        LedonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    pub fn led_com_sel(&self) -> LedComSelR {
        LedComSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    pub fn ehs(&self) -> EhsR {
        EhsR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LED enable"]
    #[inline(always)]
    pub fn ledon(&mut self) -> LedonW<'_, CrSpec> {
        LedonW::new(self, 0)
    }
    #[doc = "Bits 1:2 - LED COM Selection"]
    #[inline(always)]
    pub fn led_com_sel(&mut self) -> LedComSelW<'_, CrSpec> {
        LedComSelW::new(self, 1)
    }
    #[doc = "Bit 3 - LED interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CrSpec> {
        IeW::new(self, 3)
    }
    #[doc = "Bits 12:13 - Light control"]
    #[inline(always)]
    pub fn ehs(&mut self) -> EhsW<'_, CrSpec> {
        EhsW::new(self, 12)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
