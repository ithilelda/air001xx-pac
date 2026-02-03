#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OtyperSpec>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OtyperSpec>;
#[doc = "Field `OT0` reader - Port x configuration bits (y = 0..15)"]
pub type Ot0R = crate::BitReader;
#[doc = "Field `OT0` writer - Port x configuration bits (y = 0..15)"]
pub type Ot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT1` reader - Port x configuration bits (y = 0..15)"]
pub type Ot1R = crate::BitReader;
#[doc = "Field `OT1` writer - Port x configuration bits (y = 0..15)"]
pub type Ot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT2` reader - Port x configuration bits (y = 0..15)"]
pub type Ot2R = crate::BitReader;
#[doc = "Field `OT2` writer - Port x configuration bits (y = 0..15)"]
pub type Ot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub type Ot3R = crate::BitReader;
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub type Ot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT4` reader - Port x configuration bits (y = 0..15)"]
pub type Ot4R = crate::BitReader;
#[doc = "Field `OT4` writer - Port x configuration bits (y = 0..15)"]
pub type Ot4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT5` reader - Port x configuration bits (y = 0..15)"]
pub type Ot5R = crate::BitReader;
#[doc = "Field `OT5` writer - Port x configuration bits (y = 0..15)"]
pub type Ot5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT6` reader - Port x configuration bits (y = 0..15)"]
pub type Ot6R = crate::BitReader;
#[doc = "Field `OT6` writer - Port x configuration bits (y = 0..15)"]
pub type Ot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT7` reader - Port x configuration bits (y = 0..15)"]
pub type Ot7R = crate::BitReader;
#[doc = "Field `OT7` writer - Port x configuration bits (y = 0..15)"]
pub type Ot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OT8` reader - Port x configuration bits (y = 0..15)"]
pub type Ot8R = crate::BitReader;
#[doc = "Field `OT8` writer - Port x configuration bits (y = 0..15)"]
pub type Ot8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&self) -> Ot0R {
        Ot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&self) -> Ot1R {
        Ot1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&self) -> Ot2R {
        Ot2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> Ot3R {
        Ot3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&self) -> Ot4R {
        Ot4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&self) -> Ot5R {
        Ot5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&self) -> Ot6R {
        Ot6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&self) -> Ot7R {
        Ot7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&self) -> Ot8R {
        Ot8R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot0(&mut self) -> Ot0W<'_, OtyperSpec> {
        Ot0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot1(&mut self) -> Ot1W<'_, OtyperSpec> {
        Ot1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot2(&mut self) -> Ot2W<'_, OtyperSpec> {
        Ot2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&mut self) -> Ot3W<'_, OtyperSpec> {
        Ot3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot4(&mut self) -> Ot4W<'_, OtyperSpec> {
        Ot4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot5(&mut self) -> Ot5W<'_, OtyperSpec> {
        Ot5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot6(&mut self) -> Ot6W<'_, OtyperSpec> {
        Ot6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot7(&mut self) -> Ot7W<'_, OtyperSpec> {
        Ot7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot8(&mut self) -> Ot8W<'_, OtyperSpec> {
        Ot8W::new(self, 8)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtyperSpec;
impl crate::RegisterSpec for OtyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OtyperSpec {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OtyperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OtyperSpec {}
