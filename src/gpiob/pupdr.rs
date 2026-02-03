#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PupdrSpec>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PupdrSpec>;
#[doc = "Field `PUPD0` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd0R = crate::FieldReader;
#[doc = "Field `PUPD0` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD1` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd1R = crate::FieldReader;
#[doc = "Field `PUPD1` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD2` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd2R = crate::FieldReader;
#[doc = "Field `PUPD2` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD3` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd3R = crate::FieldReader;
#[doc = "Field `PUPD3` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD4` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd4R = crate::FieldReader;
#[doc = "Field `PUPD4` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD5` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd5R = crate::FieldReader;
#[doc = "Field `PUPD5` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD6` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd6R = crate::FieldReader;
#[doc = "Field `PUPD6` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD7` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd7R = crate::FieldReader;
#[doc = "Field `PUPD7` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUPD8` reader - Port x configuration bits (y = 0..15)"]
pub type Pupd8R = crate::FieldReader;
#[doc = "Field `PUPD8` writer - Port x configuration bits (y = 0..15)"]
pub type Pupd8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&self) -> Pupd0R {
        Pupd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&self) -> Pupd1R {
        Pupd1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&self) -> Pupd2R {
        Pupd2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&self) -> Pupd3R {
        Pupd3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&self) -> Pupd4R {
        Pupd4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&self) -> Pupd5R {
        Pupd5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&self) -> Pupd6R {
        Pupd6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&self) -> Pupd7R {
        Pupd7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&self) -> Pupd8R {
        Pupd8R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd0(&mut self) -> Pupd0W<'_, PupdrSpec> {
        Pupd0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd1(&mut self) -> Pupd1W<'_, PupdrSpec> {
        Pupd1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd2(&mut self) -> Pupd2W<'_, PupdrSpec> {
        Pupd2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd3(&mut self) -> Pupd3W<'_, PupdrSpec> {
        Pupd3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd4(&mut self) -> Pupd4W<'_, PupdrSpec> {
        Pupd4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd5(&mut self) -> Pupd5W<'_, PupdrSpec> {
        Pupd5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd6(&mut self) -> Pupd6W<'_, PupdrSpec> {
        Pupd6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd7(&mut self) -> Pupd7W<'_, PupdrSpec> {
        Pupd7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupd8(&mut self) -> Pupd8W<'_, PupdrSpec> {
        Pupd8W::new(self, 16)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PupdrSpec;
impl crate::RegisterSpec for PupdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PupdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PupdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PupdrSpec {}
