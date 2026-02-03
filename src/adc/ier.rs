#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
pub type EosmpieR = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub type EosmpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSEQIE` reader - ADC group regular end of sequence conversions interrupt"]
pub type EoseqieR = crate::BitReader;
#[doc = "Field `EOSEQIE` writer - ADC group regular end of sequence conversions interrupt"]
pub type EoseqieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDIE` reader - ADC analog watchdog interrupt"]
pub type AwdieR = crate::BitReader;
#[doc = "Field `AWDIE` writer - ADC analog watchdog interrupt"]
pub type AwdieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EosmpieR {
        EosmpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eoseqie(&self) -> EoseqieR {
        EoseqieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog interrupt"]
    #[inline(always)]
    pub fn awdie(&self) -> AwdieR {
        AwdieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EosmpieW<'_, IerSpec> {
        EosmpieW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<'_, IerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eoseqie(&mut self) -> EoseqieW<'_, IerSpec> {
        EoseqieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<'_, IerSpec> {
        OvrieW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC analog watchdog interrupt"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AwdieW<'_, IerSpec> {
        AwdieW::new(self, 7)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
