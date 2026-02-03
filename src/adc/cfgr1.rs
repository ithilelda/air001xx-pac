#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `DMAEN` reader - ADC DMA transfer enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - ADC DMA transfer enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - ADC DMA transfer configuration"]
pub type DmacfgR = crate::BitReader;
#[doc = "Field `DMACFG` writer - ADC DMA transfer configuration"]
pub type DmacfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANDIR` reader - Scan sequence direction"]
pub type ScandirR = crate::BitReader;
#[doc = "Field `SCANDIR` writer - Scan sequence direction"]
pub type ScandirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESSEL` reader - ADC data resolution"]
pub type ResselR = crate::FieldReader;
#[doc = "Field `RESSEL` writer - ADC data resolution"]
pub type ResselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - ADC data alignement"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - ADC data alignement"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - ADC group regular external trigger source"]
pub type ExtselR = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - ADC group regular external trigger source"]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTEN` reader - ADC group regular external trigger polarity"]
pub type ExtenR = crate::FieldReader;
#[doc = "Field `EXTEN` writer - ADC group regular external trigger polarity"]
pub type ExtenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - ADC group regular overrun configuration"]
pub type OvrmodR = crate::BitReader;
#[doc = "Field `OVRMOD` writer - ADC group regular overrun configuration"]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - ADC group regular continuous conversion mode"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - ADC group regular continuous conversion mode"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT` reader - Wait conversion mode"]
pub type WaitR = crate::BitReader;
#[doc = "Field `WAIT` writer - Wait conversion mode"]
pub type WaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - ADC group regular sequencer discontinuous mode"]
pub type DiscenR = crate::BitReader;
#[doc = "Field `DISCEN` writer - ADC group regular sequencer discontinuous mode"]
pub type DiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDSGL` reader - ADC analog watchdog monitoring a single channel or all channels"]
pub type AwdsglR = crate::BitReader;
#[doc = "Field `AWDSGL` writer - ADC analog watchdog monitoring a single channel or all channels"]
pub type AwdsglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDEN` reader - ADC analog watchdog enable on scope ADC group regular"]
pub type AwdenR = crate::BitReader;
#[doc = "Field `AWDEN` writer - ADC analog watchdog enable on scope ADC group regular"]
pub type AwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDCH` reader - ADC analog watchdog monitored channel selection"]
pub type AwdchR = crate::FieldReader;
#[doc = "Field `AWDCH` writer - ADC analog watchdog monitored channel selection"]
pub type AwdchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC DMA transfer configuration"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    pub fn scandir(&self) -> ScandirR {
        ScandirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - ADC data resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> ResselR {
        ResselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ADC data alignement"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wait conversion mode"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DiscenR {
        DiscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC analog watchdog monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AwdsglR {
        AwdsglR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC analog watchdog enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awden(&self) -> AwdenR {
        AwdenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:29 - ADC analog watchdog monitored channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AwdchR {
        AwdchR::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Cfgr1Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC DMA transfer configuration"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DmacfgW<'_, Cfgr1Spec> {
        DmacfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Scan sequence direction"]
    #[inline(always)]
    pub fn scandir(&mut self) -> ScandirW<'_, Cfgr1Spec> {
        ScandirW::new(self, 2)
    }
    #[doc = "Bits 3:4 - ADC data resolution"]
    #[inline(always)]
    pub fn ressel(&mut self) -> ResselW<'_, Cfgr1Spec> {
        ResselW::new(self, 3)
    }
    #[doc = "Bit 5 - ADC data alignement"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<'_, Cfgr1Spec> {
        AlignW::new(self, 5)
    }
    #[doc = "Bits 6:8 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&mut self) -> ExtselW<'_, Cfgr1Spec> {
        ExtselW::new(self, 6)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&mut self) -> ExtenW<'_, Cfgr1Spec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OvrmodW<'_, Cfgr1Spec> {
        OvrmodW::new(self, 12)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, Cfgr1Spec> {
        ContW::new(self, 13)
    }
    #[doc = "Bit 14 - Wait conversion mode"]
    #[inline(always)]
    pub fn wait(&mut self) -> WaitW<'_, Cfgr1Spec> {
        WaitW::new(self, 14)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&mut self) -> DiscenW<'_, Cfgr1Spec> {
        DiscenW::new(self, 16)
    }
    #[doc = "Bit 22 - ADC analog watchdog monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AwdsglW<'_, Cfgr1Spec> {
        AwdsglW::new(self, 22)
    }
    #[doc = "Bit 23 - ADC analog watchdog enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awden(&mut self) -> AwdenW<'_, Cfgr1Spec> {
        AwdenW::new(self, 23)
    }
    #[doc = "Bits 26:29 - ADC analog watchdog monitored channel selection"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AwdchW<'_, Cfgr1Spec> {
        AwdchW::new(self, 26)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {}
