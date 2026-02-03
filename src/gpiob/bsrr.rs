#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BS0` writer - Port x set bit y (y= 0..15)"]
pub type Bs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - Port x set bit y (y= 0..15)"]
pub type Bs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS2` writer - Port x set bit y (y= 0..15)"]
pub type Bs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS3` writer - Port x set bit y (y= 0..15)"]
pub type Bs3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS4` writer - Port x set bit y (y= 0..15)"]
pub type Bs4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS5` writer - Port x set bit y (y= 0..15)"]
pub type Bs5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS6` writer - Port x set bit y (y= 0..15)"]
pub type Bs6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS7` writer - Port x set bit y (y= 0..15)"]
pub type Bs7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS8` writer - Port x set bit y (y= 0..15)"]
pub type Bs8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` writer - Port x set bit y (y= 0..15)"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x reset bit y (y = 0..15)"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port x reset bit y (y = 0..15)"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port x reset bit y (y = 0..15)"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port x reset bit y (y = 0..15)"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port x reset bit y (y = 0..15)"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port x reset bit y (y = 0..15)"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port x reset bit y (y = 0..15)"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - Port x reset bit y (y = 0..15)"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs0(&mut self) -> Bs0W<'_, BsrrSpec> {
        Bs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs1(&mut self) -> Bs1W<'_, BsrrSpec> {
        Bs1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs2(&mut self) -> Bs2W<'_, BsrrSpec> {
        Bs2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs3(&mut self) -> Bs3W<'_, BsrrSpec> {
        Bs3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs4(&mut self) -> Bs4W<'_, BsrrSpec> {
        Bs4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs5(&mut self) -> Bs5W<'_, BsrrSpec> {
        Bs5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs6(&mut self) -> Bs6W<'_, BsrrSpec> {
        Bs6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs7(&mut self) -> Bs7W<'_, BsrrSpec> {
        Bs7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn bs8(&mut self) -> Bs8W<'_, BsrrSpec> {
        Bs8W::new(self, 8)
    }
    #[doc = "Bit 16 - Port x set bit y (y= 0..15)"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BsrrSpec> {
        Br0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BsrrSpec> {
        Br1W::new(self, 17)
    }
    #[doc = "Bit 18 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BsrrSpec> {
        Br2W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BsrrSpec> {
        Br3W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BsrrSpec> {
        Br4W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BsrrSpec> {
        Br5W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<'_, BsrrSpec> {
        Br6W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<'_, BsrrSpec> {
        Br7W::new(self, 23)
    }
    #[doc = "Bit 24 - Port x reset bit y (y = 0..15)"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<'_, BsrrSpec> {
        Br8W::new(self, 24)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
