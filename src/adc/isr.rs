#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `EOSMP` reader - ADC group regular end of sampling flag"]
pub type EosmpR = crate::BitReader;
#[doc = "Field `EOSMP` writer - ADC group regular end of sampling flag"]
pub type EosmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - ADC group regular end of unitary conversion flag"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - ADC group regular end of unitary conversion flag"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSEQ` reader - ADC group regular end of sequence conversions flag"]
pub type EoseqR = crate::BitReader;
#[doc = "Field `EOSEQ` writer - ADC group regular end of sequence conversions flag"]
pub type EoseqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - ADC group regular overrun flag"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - ADC group regular overrun flag"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD` reader - ADC analog watchdog flag"]
pub type AwdR = crate::BitReader;
#[doc = "Field `AWD` writer - ADC analog watchdog flag"]
pub type AwdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&self) -> EosmpR {
        EosmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eoseq(&self) -> EoseqR {
        EoseqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AwdR {
        AwdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EosmpW<'_, IsrSpec> {
        EosmpW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IsrSpec> {
        EocW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eoseq(&mut self) -> EoseqW<'_, IsrSpec> {
        EoseqW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<'_, IsrSpec> {
        OvrW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&mut self) -> AwdW<'_, IsrSpec> {
        AwdW::new(self, 7)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
