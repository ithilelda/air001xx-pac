#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `BIAS_CR` reader - MR Bias current"]
pub type BiasCrR = crate::FieldReader;
#[doc = "Field `BIAS_CR` writer - MR Bias current"]
pub type BiasCrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIAS_CR_SEL` reader - MR Bias current selection"]
pub type BiasCrSelR = crate::BitReader;
#[doc = "Field `BIAS_CR_SEL` writer - MR Bias current selection"]
pub type BiasCrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DbpR = crate::BitReader;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VosR = crate::BitReader;
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRRDY_TIME` reader - Time selection wakeup from LP to VR"]
pub type MrrdyTimeR = crate::FieldReader;
#[doc = "Field `MRRDY_TIME` writer - Time selection wakeup from LP to VR"]
pub type MrrdyTimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLS_SLPTIME` reader - Flash wait time after wakeup from the stop mode"]
pub type FlsSlptimeR = crate::FieldReader;
#[doc = "Field `FLS_SLPTIME` writer - Flash wait time after wakeup from the stop mode"]
pub type FlsSlptimeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPR` reader - Low-power run"]
pub type LprR = crate::BitReader;
#[doc = "Field `LPR` writer - Low-power run"]
pub type LprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_RETV` reader - SRAM retention voltage control"]
pub type SramRetvR = crate::FieldReader;
#[doc = "Field `SRAM_RETV` writer - SRAM retention voltage control"]
pub type SramRetvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSION_CTRL` reader - HSI open time control"]
pub type HsionCtrlR = crate::BitReader;
#[doc = "Field `HSION_CTRL` writer - HSI open time control"]
pub type HsionCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - MR Bias current"]
    #[inline(always)]
    pub fn bias_cr(&self) -> BiasCrR {
        BiasCrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MR Bias current selection"]
    #[inline(always)]
    pub fn bias_cr_sel(&self) -> BiasCrSelR {
        BiasCrSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DbpR {
        DbpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VosR {
        VosR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Time selection wakeup from LP to VR"]
    #[inline(always)]
    pub fn mrrdy_time(&self) -> MrrdyTimeR {
        MrrdyTimeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Flash wait time after wakeup from the stop mode"]
    #[inline(always)]
    pub fn fls_slptime(&self) -> FlsSlptimeR {
        FlsSlptimeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LprR {
        LprR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SRAM retention voltage control"]
    #[inline(always)]
    pub fn sram_retv(&self) -> SramRetvR {
        SramRetvR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - HSI open time control"]
    #[inline(always)]
    pub fn hsion_ctrl(&self) -> HsionCtrlR {
        HsionCtrlR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MR Bias current"]
    #[inline(always)]
    pub fn bias_cr(&mut self) -> BiasCrW<'_, Cr1Spec> {
        BiasCrW::new(self, 0)
    }
    #[doc = "Bit 4 - MR Bias current selection"]
    #[inline(always)]
    pub fn bias_cr_sel(&mut self) -> BiasCrSelW<'_, Cr1Spec> {
        BiasCrSelW::new(self, 4)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DbpW<'_, Cr1Spec> {
        DbpW::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&mut self) -> VosW<'_, Cr1Spec> {
        VosW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Time selection wakeup from LP to VR"]
    #[inline(always)]
    pub fn mrrdy_time(&mut self) -> MrrdyTimeW<'_, Cr1Spec> {
        MrrdyTimeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Flash wait time after wakeup from the stop mode"]
    #[inline(always)]
    pub fn fls_slptime(&mut self) -> FlsSlptimeW<'_, Cr1Spec> {
        FlsSlptimeW::new(self, 12)
    }
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&mut self) -> LprW<'_, Cr1Spec> {
        LprW::new(self, 14)
    }
    #[doc = "Bits 16:18 - SRAM retention voltage control"]
    #[inline(always)]
    pub fn sram_retv(&mut self) -> SramRetvW<'_, Cr1Spec> {
        SramRetvW::new(self, 16)
    }
    #[doc = "Bit 19 - HSI open time control"]
    #[inline(always)]
    pub fn hsion_ctrl(&mut self) -> HsionCtrlW<'_, Cr1Spec> {
        HsionCtrlW::new(self, 19)
    }
}
#[doc = "Power control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0x0003_0000"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
