#[doc = "Register `SWIER` reader"]
pub type R = crate::R<SwierSpec>;
#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SwierSpec>;
#[doc = "Field `SWI0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi0R = crate::BitReader;
#[doc = "Field `SWI0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi1R = crate::BitReader;
#[doc = "Field `SWI1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi2R = crate::BitReader;
#[doc = "Field `SWI2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi3R = crate::BitReader;
#[doc = "Field `SWI3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi4R = crate::BitReader;
#[doc = "Field `SWI4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi5R = crate::BitReader;
#[doc = "Field `SWI5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi6R = crate::BitReader;
#[doc = "Field `SWI6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi7R = crate::BitReader;
#[doc = "Field `SWI7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI8` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi8R = crate::BitReader;
#[doc = "Field `SWI8` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI9` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi9R = crate::BitReader;
#[doc = "Field `SWI9` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI10` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi10R = crate::BitReader;
#[doc = "Field `SWI10` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI11` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi11R = crate::BitReader;
#[doc = "Field `SWI11` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI12` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi12R = crate::BitReader;
#[doc = "Field `SWI12` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI13` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi13R = crate::BitReader;
#[doc = "Field `SWI13` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI14` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi14R = crate::BitReader;
#[doc = "Field `SWI14` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI15` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi15R = crate::BitReader;
#[doc = "Field `SWI15` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI16` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi16R = crate::BitReader;
#[doc = "Field `SWI16` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI17` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi17R = crate::BitReader;
#[doc = "Field `SWI17` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI18` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi18R = crate::BitReader;
#[doc = "Field `SWI18` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Swi18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi0(&self) -> Swi0R {
        Swi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi1(&self) -> Swi1R {
        Swi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi2(&self) -> Swi2R {
        Swi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi3(&self) -> Swi3R {
        Swi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi4(&self) -> Swi4R {
        Swi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi5(&self) -> Swi5R {
        Swi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi6(&self) -> Swi6R {
        Swi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi7(&self) -> Swi7R {
        Swi7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi8(&self) -> Swi8R {
        Swi8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi9(&self) -> Swi9R {
        Swi9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi10(&self) -> Swi10R {
        Swi10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi11(&self) -> Swi11R {
        Swi11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi12(&self) -> Swi12R {
        Swi12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi13(&self) -> Swi13R {
        Swi13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi14(&self) -> Swi14R {
        Swi14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi15(&self) -> Swi15R {
        Swi15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi16(&self) -> Swi16R {
        Swi16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi17(&self) -> Swi17R {
        Swi17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi18(&self) -> Swi18R {
        Swi18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi0(&mut self) -> Swi0W<'_, SwierSpec> {
        Swi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi1(&mut self) -> Swi1W<'_, SwierSpec> {
        Swi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi2(&mut self) -> Swi2W<'_, SwierSpec> {
        Swi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi3(&mut self) -> Swi3W<'_, SwierSpec> {
        Swi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi4(&mut self) -> Swi4W<'_, SwierSpec> {
        Swi4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi5(&mut self) -> Swi5W<'_, SwierSpec> {
        Swi5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi6(&mut self) -> Swi6W<'_, SwierSpec> {
        Swi6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi7(&mut self) -> Swi7W<'_, SwierSpec> {
        Swi7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi8(&mut self) -> Swi8W<'_, SwierSpec> {
        Swi8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi9(&mut self) -> Swi9W<'_, SwierSpec> {
        Swi9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi10(&mut self) -> Swi10W<'_, SwierSpec> {
        Swi10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi11(&mut self) -> Swi11W<'_, SwierSpec> {
        Swi11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi12(&mut self) -> Swi12W<'_, SwierSpec> {
        Swi12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi13(&mut self) -> Swi13W<'_, SwierSpec> {
        Swi13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi14(&mut self) -> Swi14W<'_, SwierSpec> {
        Swi14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi15(&mut self) -> Swi15W<'_, SwierSpec> {
        Swi15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi16(&mut self) -> Swi16W<'_, SwierSpec> {
        Swi16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi17(&mut self) -> Swi17W<'_, SwierSpec> {
        Swi17W::new(self, 17)
    }
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn swi18(&mut self) -> Swi18W<'_, SwierSpec> {
        Swi18W::new(self, 18)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwierSpec;
impl crate::RegisterSpec for SwierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier::R`](R) reader structure"]
impl crate::Readable for SwierSpec {}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SwierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SwierSpec {}
