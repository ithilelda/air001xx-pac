#[doc = "Register `MODER` reader"]
pub type R = crate::R<ModerSpec>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<ModerSpec>;
#[doc = "Field `MODE0` reader - Port x configuration bits (y = 0..15)"]
pub type Mode0R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Port x configuration bits (y = 0..15)"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE1` reader - Port x configuration bits (y = 0..15)"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Port x configuration bits (y = 0..15)"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE2` reader - Port x configuration bits (y = 0..15)"]
pub type Mode2R = crate::FieldReader;
#[doc = "Field `MODE2` writer - Port x configuration bits (y = 0..15)"]
pub type Mode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE3` reader - Port x configuration bits (y = 0..15)"]
pub type Mode3R = crate::FieldReader;
#[doc = "Field `MODE3` writer - Port x configuration bits (y = 0..15)"]
pub type Mode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE4` reader - Port x configuration bits (y = 0..15)"]
pub type Mode4R = crate::FieldReader;
#[doc = "Field `MODE4` writer - Port x configuration bits (y = 0..15)"]
pub type Mode4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE5` reader - Port x configuration bits (y = 0..15)"]
pub type Mode5R = crate::FieldReader;
#[doc = "Field `MODE5` writer - Port x configuration bits (y = 0..15)"]
pub type Mode5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE6` reader - Port x configuration bits (y = 0..15)"]
pub type Mode6R = crate::FieldReader;
#[doc = "Field `MODE6` writer - Port x configuration bits (y = 0..15)"]
pub type Mode6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE7` reader - Port x configuration bits (y = 0..15)"]
pub type Mode7R = crate::FieldReader;
#[doc = "Field `MODE7` writer - Port x configuration bits (y = 0..15)"]
pub type Mode7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE8` reader - Port x configuration bits (y = 0..15)"]
pub type Mode8R = crate::FieldReader;
#[doc = "Field `MODE8` writer - Port x configuration bits (y = 0..15)"]
pub type Mode8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode2(&self) -> Mode2R {
        Mode2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode4(&self) -> Mode4R {
        Mode4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode5(&self) -> Mode5R {
        Mode5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode6(&self) -> Mode6R {
        Mode6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode7(&self) -> Mode7R {
        Mode7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<'_, ModerSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<'_, ModerSpec> {
        Mode1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode2(&mut self) -> Mode2W<'_, ModerSpec> {
        Mode2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<'_, ModerSpec> {
        Mode3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode4(&mut self) -> Mode4W<'_, ModerSpec> {
        Mode4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode5(&mut self) -> Mode5W<'_, ModerSpec> {
        Mode5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode6(&mut self) -> Mode6W<'_, ModerSpec> {
        Mode6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode7(&mut self) -> Mode7W<'_, ModerSpec> {
        Mode7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn mode8(&mut self) -> Mode8W<'_, ModerSpec> {
        Mode8W::new(self, 16)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModerSpec;
impl crate::RegisterSpec for ModerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for ModerSpec {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for ModerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER to value 0xffff_ffff"]
impl crate::Resettable for ModerSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
