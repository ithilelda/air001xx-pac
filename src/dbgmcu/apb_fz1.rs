#[doc = "Register `APB_FZ1` reader"]
pub type R = crate::R<ApbFz1Spec>;
#[doc = "Register `APB_FZ1` writer"]
pub type W = crate::W<ApbFz1Spec>;
#[doc = "Field `DBG_TIMER3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DbgTimer3StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DbgTimer3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopR = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM_STOP` reader - Debug LPTIM stopped when Core is halted"]
pub type DbgLptimStopR = crate::BitReader;
#[doc = "Field `DBG_LPTIM_STOP` writer - Debug LPTIM stopped when Core is halted"]
pub type DbgLptimStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer3_stop(&self) -> DbgTimer3StopR {
        DbgTimer3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&self) -> DbgLptimStopR {
        DbgLptimStopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer3_stop(&mut self) -> DbgTimer3StopW<'_, ApbFz1Spec> {
        DbgTimer3StopW::new(self, 1)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<'_, ApbFz1Spec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<'_, ApbFz1Spec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<'_, ApbFz1Spec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 31 - Debug LPTIM stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim_stop(&mut self) -> DbgLptimStopW<'_, ApbFz1Spec> {
        DbgLptimStopW::new(self, 31)
    }
}
#[doc = "APB Freeze Register1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbFz1Spec;
impl crate::RegisterSpec for ApbFz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_fz1::R`](R) reader structure"]
impl crate::Readable for ApbFz1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_fz1::W`](W) writer structure"]
impl crate::Writable for ApbFz1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB_FZ1 to value 0"]
impl crate::Resettable for ApbFz1Spec {}
