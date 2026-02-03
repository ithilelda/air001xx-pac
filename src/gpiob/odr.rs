#[doc = "Register `ODR` reader"]
pub type R = crate::R<OdrSpec>;
#[doc = "Register `ODR` writer"]
pub type W = crate::W<OdrSpec>;
#[doc = "Field `OD0` reader - Port output data (y = 0..15)"]
pub type Od0R = crate::BitReader;
#[doc = "Field `OD0` writer - Port output data (y = 0..15)"]
pub type Od0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD1` reader - Port output data (y = 0..15)"]
pub type Od1R = crate::BitReader;
#[doc = "Field `OD1` writer - Port output data (y = 0..15)"]
pub type Od1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD2` reader - Port output data (y = 0..15)"]
pub type Od2R = crate::BitReader;
#[doc = "Field `OD2` writer - Port output data (y = 0..15)"]
pub type Od2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD3` reader - Port output data (y = 0..15)"]
pub type Od3R = crate::BitReader;
#[doc = "Field `OD3` writer - Port output data (y = 0..15)"]
pub type Od3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD4` reader - Port output data (y = 0..15)"]
pub type Od4R = crate::BitReader;
#[doc = "Field `OD4` writer - Port output data (y = 0..15)"]
pub type Od4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD5` reader - Port output data (y = 0..15)"]
pub type Od5R = crate::BitReader;
#[doc = "Field `OD5` writer - Port output data (y = 0..15)"]
pub type Od5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD6` reader - Port output data (y = 0..15)"]
pub type Od6R = crate::BitReader;
#[doc = "Field `OD6` writer - Port output data (y = 0..15)"]
pub type Od6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD7` reader - Port output data (y = 0..15)"]
pub type Od7R = crate::BitReader;
#[doc = "Field `OD7` writer - Port output data (y = 0..15)"]
pub type Od7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD8` reader - Port output data (y = 0..15)"]
pub type Od8R = crate::BitReader;
#[doc = "Field `OD8` writer - Port output data (y = 0..15)"]
pub type Od8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&self) -> Od0R {
        Od0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&self) -> Od1R {
        Od1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&self) -> Od2R {
        Od2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&self) -> Od3R {
        Od3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&self) -> Od4R {
        Od4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&self) -> Od5R {
        Od5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&self) -> Od6R {
        Od6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&self) -> Od7R {
        Od7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&self) -> Od8R {
        Od8R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od0(&mut self) -> Od0W<'_, OdrSpec> {
        Od0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od1(&mut self) -> Od1W<'_, OdrSpec> {
        Od1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od2(&mut self) -> Od2W<'_, OdrSpec> {
        Od2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od3(&mut self) -> Od3W<'_, OdrSpec> {
        Od3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od4(&mut self) -> Od4W<'_, OdrSpec> {
        Od4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od5(&mut self) -> Od5W<'_, OdrSpec> {
        Od5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od6(&mut self) -> Od6W<'_, OdrSpec> {
        Od6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od7(&mut self) -> Od7W<'_, OdrSpec> {
        Od7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data (y = 0..15)"]
    #[inline(always)]
    pub fn od8(&mut self) -> Od8W<'_, OdrSpec> {
        Od8W::new(self, 8)
    }
}
#[doc = "GPIO port output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrSpec;
impl crate::RegisterSpec for OdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odr::R`](R) reader structure"]
impl crate::Readable for OdrSpec {}
#[doc = "`write(|w| ..)` method takes [`odr::W`](W) writer structure"]
impl crate::Writable for OdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODR to value 0"]
impl crate::Resettable for OdrSpec {}
