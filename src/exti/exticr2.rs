#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<Exticr2Spec>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<Exticr2Spec>;
#[doc = "Field `EXTI4` reader - GPIO port selection"]
pub type Exti4R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - GPIO port selection"]
pub type Exti4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTI5` reader - GPIO port selection"]
pub type Exti5R = crate::BitReader;
#[doc = "Field `EXTI5` writer - GPIO port selection"]
pub type Exti5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6` reader - GPIO port selection"]
pub type Exti6R = crate::BitReader;
#[doc = "Field `EXTI6` writer - GPIO port selection"]
pub type Exti6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7` reader - GPIO port selection"]
pub type Exti7R = crate::BitReader;
#[doc = "Field `EXTI7` writer - GPIO port selection"]
pub type Exti7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti4(&self) -> Exti4R {
        Exti4R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - GPIO port selection"]
    #[inline(always)]
    pub fn exti5(&self) -> Exti5R {
        Exti5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO port selection"]
    #[inline(always)]
    pub fn exti6(&self) -> Exti6R {
        Exti6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO port selection"]
    #[inline(always)]
    pub fn exti7(&self) -> Exti7R {
        Exti7R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO port selection"]
    #[inline(always)]
    pub fn exti4(&mut self) -> Exti4W<'_, Exticr2Spec> {
        Exti4W::new(self, 0)
    }
    #[doc = "Bit 8 - GPIO port selection"]
    #[inline(always)]
    pub fn exti5(&mut self) -> Exti5W<'_, Exticr2Spec> {
        Exti5W::new(self, 8)
    }
    #[doc = "Bit 16 - GPIO port selection"]
    #[inline(always)]
    pub fn exti6(&mut self) -> Exti6W<'_, Exticr2Spec> {
        Exti6W::new(self, 16)
    }
    #[doc = "Bit 24 - GPIO port selection"]
    #[inline(always)]
    pub fn exti7(&mut self) -> Exti7W<'_, Exticr2Spec> {
        Exti7W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exticr2Spec;
impl crate::RegisterSpec for Exticr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for Exticr2Spec {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for Exticr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for Exticr2Spec {}
