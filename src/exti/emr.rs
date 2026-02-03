#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `EM0` reader - CPU wakeup with event mask on event input"]
pub type Em0R = crate::BitReader;
#[doc = "Field `EM0` writer - CPU wakeup with event mask on event input"]
pub type Em0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM1` reader - CPU wakeup with event mask on event input"]
pub type Em1R = crate::BitReader;
#[doc = "Field `EM1` writer - CPU wakeup with event mask on event input"]
pub type Em1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2` reader - CPU wakeup with event mask on event input"]
pub type Em2R = crate::BitReader;
#[doc = "Field `EM2` writer - CPU wakeup with event mask on event input"]
pub type Em2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM3` reader - CPU wakeup with event mask on event input"]
pub type Em3R = crate::BitReader;
#[doc = "Field `EM3` writer - CPU wakeup with event mask on event input"]
pub type Em3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM4` reader - CPU wakeup with event mask on event input"]
pub type Em4R = crate::BitReader;
#[doc = "Field `EM4` writer - CPU wakeup with event mask on event input"]
pub type Em4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM5` reader - CPU wakeup with event mask on event input"]
pub type Em5R = crate::BitReader;
#[doc = "Field `EM5` writer - CPU wakeup with event mask on event input"]
pub type Em5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM6` reader - CPU wakeup with event mask on event input"]
pub type Em6R = crate::BitReader;
#[doc = "Field `EM6` writer - CPU wakeup with event mask on event input"]
pub type Em6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM7` reader - CPU wakeup with event mask on event input"]
pub type Em7R = crate::BitReader;
#[doc = "Field `EM7` writer - CPU wakeup with event mask on event input"]
pub type Em7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM8` reader - CPU wakeup with event mask on event input"]
pub type Em8R = crate::BitReader;
#[doc = "Field `EM8` writer - CPU wakeup with event mask on event input"]
pub type Em8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM9` reader - CPU wakeup with event mask on event input"]
pub type Em9R = crate::BitReader;
#[doc = "Field `EM9` writer - CPU wakeup with event mask on event input"]
pub type Em9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM10` reader - CPU wakeup with event mask on event input"]
pub type Em10R = crate::BitReader;
#[doc = "Field `EM10` writer - CPU wakeup with event mask on event input"]
pub type Em10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM11` reader - CPU wakeup with event mask on event input"]
pub type Em11R = crate::BitReader;
#[doc = "Field `EM11` writer - CPU wakeup with event mask on event input"]
pub type Em11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM12` reader - CPU wakeup with event mask on event input"]
pub type Em12R = crate::BitReader;
#[doc = "Field `EM12` writer - CPU wakeup with event mask on event input"]
pub type Em12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM13` reader - CPU wakeup with event mask on event input"]
pub type Em13R = crate::BitReader;
#[doc = "Field `EM13` writer - CPU wakeup with event mask on event input"]
pub type Em13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM14` reader - CPU wakeup with event mask on event input"]
pub type Em14R = crate::BitReader;
#[doc = "Field `EM14` writer - CPU wakeup with event mask on event input"]
pub type Em14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM15` reader - CPU wakeup with event mask on event input"]
pub type Em15R = crate::BitReader;
#[doc = "Field `EM15` writer - CPU wakeup with event mask on event input"]
pub type Em15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM16` reader - CPU wakeup with event mask on event input"]
pub type Em16R = crate::BitReader;
#[doc = "Field `EM16` writer - CPU wakeup with event mask on event input"]
pub type Em16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM17` reader - CPU wakeup with event mask on event input"]
pub type Em17R = crate::BitReader;
#[doc = "Field `EM17` writer - CPU wakeup with event mask on event input"]
pub type Em17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM18` reader - CPU wakeup with event mask on event input"]
pub type Em18R = crate::BitReader;
#[doc = "Field `EM18` writer - CPU wakeup with event mask on event input"]
pub type Em18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM19` reader - CPU wakeup with event mask on event input"]
pub type Em19R = crate::BitReader;
#[doc = "Field `EM19` writer - CPU wakeup with event mask on event input"]
pub type Em19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM29` reader - CPU wakeup with event mask on event input"]
pub type Em29R = crate::BitReader;
#[doc = "Field `EM29` writer - CPU wakeup with event mask on event input"]
pub type Em29W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em2(&self) -> Em2R {
        Em2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em3(&self) -> Em3R {
        Em3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em4(&self) -> Em4R {
        Em4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em5(&self) -> Em5R {
        Em5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em6(&self) -> Em6R {
        Em6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em7(&self) -> Em7R {
        Em7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em8(&self) -> Em8R {
        Em8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em9(&self) -> Em9R {
        Em9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em10(&self) -> Em10R {
        Em10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em11(&self) -> Em11R {
        Em11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em12(&self) -> Em12R {
        Em12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em13(&self) -> Em13R {
        Em13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em14(&self) -> Em14R {
        Em14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em15(&self) -> Em15R {
        Em15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em16(&self) -> Em16R {
        Em16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em17(&self) -> Em17R {
        Em17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em18(&self) -> Em18R {
        Em18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em19(&self) -> Em19R {
        Em19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em29(&self) -> Em29R {
        Em29R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em0(&mut self) -> Em0W<'_, EmrSpec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em1(&mut self) -> Em1W<'_, EmrSpec> {
        Em1W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em2(&mut self) -> Em2W<'_, EmrSpec> {
        Em2W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em3(&mut self) -> Em3W<'_, EmrSpec> {
        Em3W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em4(&mut self) -> Em4W<'_, EmrSpec> {
        Em4W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em5(&mut self) -> Em5W<'_, EmrSpec> {
        Em5W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em6(&mut self) -> Em6W<'_, EmrSpec> {
        Em6W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em7(&mut self) -> Em7W<'_, EmrSpec> {
        Em7W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em8(&mut self) -> Em8W<'_, EmrSpec> {
        Em8W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em9(&mut self) -> Em9W<'_, EmrSpec> {
        Em9W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em10(&mut self) -> Em10W<'_, EmrSpec> {
        Em10W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em11(&mut self) -> Em11W<'_, EmrSpec> {
        Em11W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em12(&mut self) -> Em12W<'_, EmrSpec> {
        Em12W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em13(&mut self) -> Em13W<'_, EmrSpec> {
        Em13W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em14(&mut self) -> Em14W<'_, EmrSpec> {
        Em14W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em15(&mut self) -> Em15W<'_, EmrSpec> {
        Em15W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em16(&mut self) -> Em16W<'_, EmrSpec> {
        Em16W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em17(&mut self) -> Em17W<'_, EmrSpec> {
        Em17W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em18(&mut self) -> Em18W<'_, EmrSpec> {
        Em18W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em19(&mut self) -> Em19W<'_, EmrSpec> {
        Em19W::new(self, 19)
    }
    #[doc = "Bit 29 - CPU wakeup with event mask on event input"]
    #[inline(always)]
    pub fn em29(&mut self) -> Em29W<'_, EmrSpec> {
        Em29W::new(self, 29)
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
