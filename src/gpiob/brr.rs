#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port Reset bit"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port Reset bit"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port Reset bit"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port Reset bit"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port Reset bit"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - Port Reset bit"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BrrSpec> {
        Br5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<'_, BrrSpec> {
        Br6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<'_, BrrSpec> {
        Br7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<'_, BrrSpec> {
        Br8W::new(self, 8)
    }
}
#[doc = "port bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
