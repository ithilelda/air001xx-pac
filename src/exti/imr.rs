#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `IM0` reader - CPU wakeup with interrupt mask on event input"]
pub type Im0R = crate::BitReader;
#[doc = "Field `IM0` writer - CPU wakeup with interrupt mask on event input"]
pub type Im0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM1` reader - CPU wakeup with interrupt mask on event input"]
pub type Im1R = crate::BitReader;
#[doc = "Field `IM1` writer - CPU wakeup with interrupt mask on event input"]
pub type Im1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM2` reader - CPU wakeup with interrupt mask on event input"]
pub type Im2R = crate::BitReader;
#[doc = "Field `IM2` writer - CPU wakeup with interrupt mask on event input"]
pub type Im2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM3` reader - CPU wakeup with interrupt mask on event input"]
pub type Im3R = crate::BitReader;
#[doc = "Field `IM3` writer - CPU wakeup with interrupt mask on event input"]
pub type Im3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM4` reader - CPU wakeup with interrupt mask on event input"]
pub type Im4R = crate::BitReader;
#[doc = "Field `IM4` writer - CPU wakeup with interrupt mask on event input"]
pub type Im4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM5` reader - CPU wakeup with interrupt mask on event input"]
pub type Im5R = crate::BitReader;
#[doc = "Field `IM5` writer - CPU wakeup with interrupt mask on event input"]
pub type Im5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM6` reader - CPU wakeup with interrupt mask on event input"]
pub type Im6R = crate::BitReader;
#[doc = "Field `IM6` writer - CPU wakeup with interrupt mask on event input"]
pub type Im6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM7` reader - CPU wakeup with interrupt mask on event input"]
pub type Im7R = crate::BitReader;
#[doc = "Field `IM7` writer - CPU wakeup with interrupt mask on event input"]
pub type Im7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM8` reader - CPU wakeup with interrupt mask on event input"]
pub type Im8R = crate::BitReader;
#[doc = "Field `IM8` writer - CPU wakeup with interrupt mask on event input"]
pub type Im8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM9` reader - CPU wakeup with interrupt mask on event input"]
pub type Im9R = crate::BitReader;
#[doc = "Field `IM9` writer - CPU wakeup with interrupt mask on event input"]
pub type Im9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM10` reader - CPU wakeup with interrupt mask on event input"]
pub type Im10R = crate::BitReader;
#[doc = "Field `IM10` writer - CPU wakeup with interrupt mask on event input"]
pub type Im10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM11` reader - CPU wakeup with interrupt mask on event input"]
pub type Im11R = crate::BitReader;
#[doc = "Field `IM11` writer - CPU wakeup with interrupt mask on event input"]
pub type Im11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM12` reader - CPU wakeup with interrupt mask on event input"]
pub type Im12R = crate::BitReader;
#[doc = "Field `IM12` writer - CPU wakeup with interrupt mask on event input"]
pub type Im12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM13` reader - CPU wakeup with interrupt mask on event input"]
pub type Im13R = crate::BitReader;
#[doc = "Field `IM13` writer - CPU wakeup with interrupt mask on event input"]
pub type Im13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM14` reader - CPU wakeup with interrupt mask on event input"]
pub type Im14R = crate::BitReader;
#[doc = "Field `IM14` writer - CPU wakeup with interrupt mask on event input"]
pub type Im14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM15` reader - CPU wakeup with interrupt mask on event input"]
pub type Im15R = crate::BitReader;
#[doc = "Field `IM15` writer - CPU wakeup with interrupt mask on event input"]
pub type Im15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM16` reader - CPU wakeup with interrupt mask on event input"]
pub type Im16R = crate::BitReader;
#[doc = "Field `IM16` writer - CPU wakeup with interrupt mask on event input"]
pub type Im16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM17` reader - CPU wakeup with interrupt mask on event input"]
pub type Im17R = crate::BitReader;
#[doc = "Field `IM17` writer - CPU wakeup with interrupt mask on event input"]
pub type Im17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM18` reader - CPU wakeup with interrupt mask on event input"]
pub type Im18R = crate::BitReader;
#[doc = "Field `IM18` writer - CPU wakeup with interrupt mask on event input"]
pub type Im18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM19` reader - CPU wakeup with interrupt mask on event input"]
pub type Im19R = crate::BitReader;
#[doc = "Field `IM19` writer - CPU wakeup with interrupt mask on event input"]
pub type Im19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM29` reader - CPU wakeup with interrupt mask on event input"]
pub type Im29R = crate::BitReader;
#[doc = "Field `IM29` writer - CPU wakeup with interrupt mask on event input"]
pub type Im29W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&self) -> Im0R {
        Im0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&self) -> Im1R {
        Im1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&self) -> Im2R {
        Im2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&self) -> Im3R {
        Im3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&self) -> Im4R {
        Im4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&self) -> Im5R {
        Im5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&self) -> Im6R {
        Im6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&self) -> Im7R {
        Im7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&self) -> Im8R {
        Im8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&self) -> Im9R {
        Im9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&self) -> Im10R {
        Im10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&self) -> Im11R {
        Im11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&self) -> Im12R {
        Im12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&self) -> Im13R {
        Im13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&self) -> Im14R {
        Im14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&self) -> Im15R {
        Im15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&self) -> Im16R {
        Im16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&self) -> Im17R {
        Im17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&self) -> Im18R {
        Im18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&self) -> Im19R {
        Im19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&self) -> Im29R {
        Im29R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&mut self) -> Im0W<'_, ImrSpec> {
        Im0W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&mut self) -> Im1W<'_, ImrSpec> {
        Im1W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&mut self) -> Im2W<'_, ImrSpec> {
        Im2W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&mut self) -> Im3W<'_, ImrSpec> {
        Im3W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&mut self) -> Im4W<'_, ImrSpec> {
        Im4W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&mut self) -> Im5W<'_, ImrSpec> {
        Im5W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&mut self) -> Im6W<'_, ImrSpec> {
        Im6W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&mut self) -> Im7W<'_, ImrSpec> {
        Im7W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&mut self) -> Im8W<'_, ImrSpec> {
        Im8W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&mut self) -> Im9W<'_, ImrSpec> {
        Im9W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&mut self) -> Im10W<'_, ImrSpec> {
        Im10W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&mut self) -> Im11W<'_, ImrSpec> {
        Im11W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&mut self) -> Im12W<'_, ImrSpec> {
        Im12W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&mut self) -> Im13W<'_, ImrSpec> {
        Im13W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&mut self) -> Im14W<'_, ImrSpec> {
        Im14W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&mut self) -> Im15W<'_, ImrSpec> {
        Im15W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&mut self) -> Im16W<'_, ImrSpec> {
        Im16W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&mut self) -> Im17W<'_, ImrSpec> {
        Im17W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&mut self) -> Im18W<'_, ImrSpec> {
        Im18W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&mut self) -> Im19W<'_, ImrSpec> {
        Im19W::new(self, 19)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&mut self) -> Im29W<'_, ImrSpec> {
        Im29W::new(self, 29)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0xfff8_0000"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0xfff8_0000;
}
