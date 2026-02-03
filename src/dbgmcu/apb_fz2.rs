#[doc = "Register `APB_FZ2` reader"]
pub type R = crate::R<ApbFz2Spec>;
#[doc = "Register `APB_FZ2` writer"]
pub type W = crate::W<ApbFz2Spec>;
#[doc = "Field `DBG_TIMER1_STOP` reader - Debug Timer 1 stopped when Core is halted"]
pub type DbgTimer1StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER1_STOP` writer - Debug Timer 1 stopped when Core is halted"]
pub type DbgTimer1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER14_STOP` reader - Debug Timer 14 stopped when Core is halted"]
pub type DbgTimer14StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER14_STOP` writer - Debug Timer 14 stopped when Core is halted"]
pub type DbgTimer14StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER16_STOP` reader - Debug Timer 16 stopped when Core is halted"]
pub type DbgTimer16StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER16_STOP` writer - Debug Timer 16 stopped when Core is halted"]
pub type DbgTimer16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER17_STOP` reader - Debug Timer 17 stopped when Core is halted"]
pub type DbgTimer17StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER17_STOP` writer - Debug Timer 17 stopped when Core is halted"]
pub type DbgTimer17StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer1_stop(&self) -> DbgTimer1StopR {
        DbgTimer1StopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&self) -> DbgTimer14StopR {
        DbgTimer14StopR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer16_stop(&self) -> DbgTimer16StopR {
        DbgTimer16StopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer17_stop(&self) -> DbgTimer17StopR {
        DbgTimer17StopR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Debug Timer 1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer1_stop(&mut self) -> DbgTimer1StopW<'_, ApbFz2Spec> {
        DbgTimer1StopW::new(self, 11)
    }
    #[doc = "Bit 15 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&mut self) -> DbgTimer14StopW<'_, ApbFz2Spec> {
        DbgTimer14StopW::new(self, 15)
    }
    #[doc = "Bit 17 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer16_stop(&mut self) -> DbgTimer16StopW<'_, ApbFz2Spec> {
        DbgTimer16StopW::new(self, 17)
    }
    #[doc = "Bit 18 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer17_stop(&mut self) -> DbgTimer17StopW<'_, ApbFz2Spec> {
        DbgTimer17StopW::new(self, 18)
    }
}
#[doc = "APB Freeze Register2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbFz2Spec;
impl crate::RegisterSpec for ApbFz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_fz2::R`](R) reader structure"]
impl crate::Readable for ApbFz2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure"]
impl crate::Writable for ApbFz2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB_FZ2 to value 0"]
impl crate::Resettable for ApbFz2Spec {}
