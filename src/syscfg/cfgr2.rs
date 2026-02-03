#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `LOCKUP_LOCK` reader - Cortex-M0+ LOCKUP bit enable bit"]
pub type LockupLockR = crate::BitReader;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex-M0+ LOCKUP bit enable bit"]
pub type LockupLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PvdLockR = crate::BitReader;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PvdLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_BRK_TIM1` reader - COMP1 is enable to input of TIM1 break"]
pub type Comp1BrkTim1R = crate::BitReader;
#[doc = "Field `COMP1_BRK_TIM1` writer - COMP1 is enable to input of TIM1 break"]
pub type Comp1BrkTim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_BRK_TIM1` reader - COMP2 is enable to input of TIM1 break"]
pub type Comp2BrkTim1R = crate::BitReader;
#[doc = "Field `COMP2_BRK_TIM1` writer - COMP2 is enable to input of TIM1 break"]
pub type Comp2BrkTim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_BRK_TIM16` reader - COMP1 is enable to input of TIM16 break"]
pub type Comp1BrkTim16R = crate::BitReader;
#[doc = "Field `COMP1_BRK_TIM16` writer - COMP1 is enable to input of TIM16 break"]
pub type Comp1BrkTim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_BRK_TIM16` reader - COMP2 is enable to input of TIM16 break"]
pub type Comp2BrkTim16R = crate::BitReader;
#[doc = "Field `COMP2_BRK_TIM16` writer - COMP2 is enable to input of TIM16 break"]
pub type Comp2BrkTim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1_BRK_TIM17` reader - COMP1 is enable to input of TIM17 break"]
pub type Comp1BrkTim17R = crate::BitReader;
#[doc = "Field `COMP1_BRK_TIM17` writer - COMP1 is enable to input of TIM17 break"]
pub type Comp1BrkTim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2_BRK_TIM17` reader - COMP2 is enable to input of TIM17 break"]
pub type Comp2BrkTim17R = crate::BitReader;
#[doc = "Field `COMP2_BRK_TIM17` writer - COMP2 is enable to input of TIM17 break"]
pub type Comp2BrkTim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETR_SRC_TIM1` reader - TIM1 ETR source selection"]
pub type EtrSrcTim1R = crate::FieldReader;
#[doc = "Field `ETR_SRC_TIM1` writer - TIM1 ETR source selection"]
pub type EtrSrcTim1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LockupLockR {
        LockupLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PvdLockR {
        PvdLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP1 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&self) -> Comp1BrkTim1R {
        Comp1BrkTim1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COMP2 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&self) -> Comp2BrkTim1R {
        Comp2BrkTim1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMP1 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&self) -> Comp1BrkTim16R {
        Comp1BrkTim16R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP2 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&self) -> Comp2BrkTim16R {
        Comp2BrkTim16R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COMP1 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&self) -> Comp1BrkTim17R {
        Comp1BrkTim17R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - COMP2 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&self) -> Comp2BrkTim17R {
        Comp2BrkTim17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - TIM1 ETR source selection"]
    #[inline(always)]
    pub fn etr_src_tim1(&self) -> EtrSrcTim1R {
        EtrSrcTim1R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0+ LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LockupLockW<'_, Cfgr2Spec> {
        LockupLockW::new(self, 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&mut self) -> PvdLockW<'_, Cfgr2Spec> {
        PvdLockW::new(self, 2)
    }
    #[doc = "Bit 3 - COMP1 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp1_brk_tim1(&mut self) -> Comp1BrkTim1W<'_, Cfgr2Spec> {
        Comp1BrkTim1W::new(self, 3)
    }
    #[doc = "Bit 4 - COMP2 is enable to input of TIM1 break"]
    #[inline(always)]
    pub fn comp2_brk_tim1(&mut self) -> Comp2BrkTim1W<'_, Cfgr2Spec> {
        Comp2BrkTim1W::new(self, 4)
    }
    #[doc = "Bit 5 - COMP1 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp1_brk_tim16(&mut self) -> Comp1BrkTim16W<'_, Cfgr2Spec> {
        Comp1BrkTim16W::new(self, 5)
    }
    #[doc = "Bit 6 - COMP2 is enable to input of TIM16 break"]
    #[inline(always)]
    pub fn comp2_brk_tim16(&mut self) -> Comp2BrkTim16W<'_, Cfgr2Spec> {
        Comp2BrkTim16W::new(self, 6)
    }
    #[doc = "Bit 7 - COMP1 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp1_brk_tim17(&mut self) -> Comp1BrkTim17W<'_, Cfgr2Spec> {
        Comp1BrkTim17W::new(self, 7)
    }
    #[doc = "Bit 8 - COMP2 is enable to input of TIM17 break"]
    #[inline(always)]
    pub fn comp2_brk_tim17(&mut self) -> Comp2BrkTim17W<'_, Cfgr2Spec> {
        Comp2BrkTim17W::new(self, 8)
    }
    #[doc = "Bits 9:10 - TIM1 ETR source selection"]
    #[inline(always)]
    pub fn etr_src_tim1(&mut self) -> EtrSrcTim1W<'_, Cfgr2Spec> {
        EtrSrcTim1W::new(self, 9)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}
