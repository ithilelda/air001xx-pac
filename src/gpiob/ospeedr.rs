#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OspeedrSpec>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OspeedrSpec>;
#[doc = "Field `OSPEED0` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed0R = crate::FieldReader;
#[doc = "Field `OSPEED0` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED1` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed1R = crate::FieldReader;
#[doc = "Field `OSPEED1` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED2` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed2R = crate::FieldReader;
#[doc = "Field `OSPEED2` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED3` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed3R = crate::FieldReader;
#[doc = "Field `OSPEED3` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED4` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed4R = crate::FieldReader;
#[doc = "Field `OSPEED4` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED5` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed5R = crate::FieldReader;
#[doc = "Field `OSPEED5` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED6` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed6R = crate::FieldReader;
#[doc = "Field `OSPEED6` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED7` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed7R = crate::FieldReader;
#[doc = "Field `OSPEED7` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSPEED8` reader - Port x configuration bits (y = 0..15)"]
pub type Ospeed8R = crate::FieldReader;
#[doc = "Field `OSPEED8` writer - Port x configuration bits (y = 0..15)"]
pub type Ospeed8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&self) -> Ospeed0R {
        Ospeed0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&self) -> Ospeed1R {
        Ospeed1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&self) -> Ospeed2R {
        Ospeed2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&self) -> Ospeed3R {
        Ospeed3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&self) -> Ospeed4R {
        Ospeed4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&self) -> Ospeed5R {
        Ospeed5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&self) -> Ospeed6R {
        Ospeed6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&self) -> Ospeed7R {
        Ospeed7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&self) -> Ospeed8R {
        Ospeed8R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed0(&mut self) -> Ospeed0W<'_, OspeedrSpec> {
        Ospeed0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed1(&mut self) -> Ospeed1W<'_, OspeedrSpec> {
        Ospeed1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed2(&mut self) -> Ospeed2W<'_, OspeedrSpec> {
        Ospeed2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed3(&mut self) -> Ospeed3W<'_, OspeedrSpec> {
        Ospeed3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed4(&mut self) -> Ospeed4W<'_, OspeedrSpec> {
        Ospeed4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed5(&mut self) -> Ospeed5W<'_, OspeedrSpec> {
        Ospeed5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed6(&mut self) -> Ospeed6W<'_, OspeedrSpec> {
        Ospeed6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed7(&mut self) -> Ospeed7W<'_, OspeedrSpec> {
        Ospeed7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeed8(&mut self) -> Ospeed8W<'_, OspeedrSpec> {
        Ospeed8W::new(self, 16)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OspeedrSpec;
impl crate::RegisterSpec for OspeedrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OspeedrSpec {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OspeedrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OspeedrSpec {}
