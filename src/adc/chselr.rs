#[doc = "Register `CHSELR` reader"]
pub type R = crate::R<ChselrSpec>;
#[doc = "Register `CHSELR` writer"]
pub type W = crate::W<ChselrSpec>;
#[doc = "Field `CHSEL0` reader - Channel-0 selection"]
pub type Chsel0R = crate::BitReader;
#[doc = "Field `CHSEL0` writer - Channel-0 selection"]
pub type Chsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL1` reader - Channel-1 selection"]
pub type Chsel1R = crate::BitReader;
#[doc = "Field `CHSEL1` writer - Channel-1 selection"]
pub type Chsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL2` reader - Channel-2 selection"]
pub type Chsel2R = crate::BitReader;
#[doc = "Field `CHSEL2` writer - Channel-2 selection"]
pub type Chsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL3` reader - Channel-3 selection"]
pub type Chsel3R = crate::BitReader;
#[doc = "Field `CHSEL3` writer - Channel-3 selection"]
pub type Chsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL4` reader - Channel-4 selection"]
pub type Chsel4R = crate::BitReader;
#[doc = "Field `CHSEL4` writer - Channel-4 selection"]
pub type Chsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL5` reader - Channel-5 selection"]
pub type Chsel5R = crate::BitReader;
#[doc = "Field `CHSEL5` writer - Channel-5 selection"]
pub type Chsel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL6` reader - Channel-6 selection"]
pub type Chsel6R = crate::BitReader;
#[doc = "Field `CHSEL6` writer - Channel-6 selection"]
pub type Chsel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL7` reader - Channel-7 selection"]
pub type Chsel7R = crate::BitReader;
#[doc = "Field `CHSEL7` writer - Channel-7 selection"]
pub type Chsel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL8` reader - Channel-8 selection"]
pub type Chsel8R = crate::BitReader;
#[doc = "Field `CHSEL8` writer - Channel-8 selection"]
pub type Chsel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL9` reader - Channel-9 selection"]
pub type Chsel9R = crate::BitReader;
#[doc = "Field `CHSEL9` writer - Channel-9 selection"]
pub type Chsel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL11` reader - Channel-11 selection"]
pub type Chsel11R = crate::BitReader;
#[doc = "Field `CHSEL11` writer - Channel-11 selection"]
pub type Chsel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEL12` reader - Channel-12 selection"]
pub type Chsel12R = crate::BitReader;
#[doc = "Field `CHSEL12` writer - Channel-12 selection"]
pub type Chsel12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    pub fn chsel0(&self) -> Chsel0R {
        Chsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    pub fn chsel1(&self) -> Chsel1R {
        Chsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    pub fn chsel2(&self) -> Chsel2R {
        Chsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    pub fn chsel3(&self) -> Chsel3R {
        Chsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    pub fn chsel4(&self) -> Chsel4R {
        Chsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    pub fn chsel5(&self) -> Chsel5R {
        Chsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    pub fn chsel6(&self) -> Chsel6R {
        Chsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    pub fn chsel7(&self) -> Chsel7R {
        Chsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    pub fn chsel8(&self) -> Chsel8R {
        Chsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    pub fn chsel9(&self) -> Chsel9R {
        Chsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel-11 selection"]
    #[inline(always)]
    pub fn chsel11(&self) -> Chsel11R {
        Chsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel-12 selection"]
    #[inline(always)]
    pub fn chsel12(&self) -> Chsel12R {
        Chsel12R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel-0 selection"]
    #[inline(always)]
    pub fn chsel0(&mut self) -> Chsel0W<'_, ChselrSpec> {
        Chsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel-1 selection"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> Chsel1W<'_, ChselrSpec> {
        Chsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel-2 selection"]
    #[inline(always)]
    pub fn chsel2(&mut self) -> Chsel2W<'_, ChselrSpec> {
        Chsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel-3 selection"]
    #[inline(always)]
    pub fn chsel3(&mut self) -> Chsel3W<'_, ChselrSpec> {
        Chsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel-4 selection"]
    #[inline(always)]
    pub fn chsel4(&mut self) -> Chsel4W<'_, ChselrSpec> {
        Chsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel-5 selection"]
    #[inline(always)]
    pub fn chsel5(&mut self) -> Chsel5W<'_, ChselrSpec> {
        Chsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel-6 selection"]
    #[inline(always)]
    pub fn chsel6(&mut self) -> Chsel6W<'_, ChselrSpec> {
        Chsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel-7 selection"]
    #[inline(always)]
    pub fn chsel7(&mut self) -> Chsel7W<'_, ChselrSpec> {
        Chsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel-8 selection"]
    #[inline(always)]
    pub fn chsel8(&mut self) -> Chsel8W<'_, ChselrSpec> {
        Chsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel-9 selection"]
    #[inline(always)]
    pub fn chsel9(&mut self) -> Chsel9W<'_, ChselrSpec> {
        Chsel9W::new(self, 9)
    }
    #[doc = "Bit 11 - Channel-11 selection"]
    #[inline(always)]
    pub fn chsel11(&mut self) -> Chsel11W<'_, ChselrSpec> {
        Chsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel-12 selection"]
    #[inline(always)]
    pub fn chsel12(&mut self) -> Chsel12W<'_, ChselrSpec> {
        Chsel12W::new(self, 12)
    }
}
#[doc = "ADC group regular sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`chselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChselrSpec;
impl crate::RegisterSpec for ChselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chselr::R`](R) reader structure"]
impl crate::Readable for ChselrSpec {}
#[doc = "`write(|w| ..)` method takes [`chselr::W`](W) writer structure"]
impl crate::Writable for ChselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSELR to value 0x0fff_0000"]
impl crate::Resettable for ChselrSpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
