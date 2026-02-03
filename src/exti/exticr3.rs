#[doc = "Register `EXTICR3` reader"]
pub type R = crate::R<Exticr3Spec>;
#[doc = "Register `EXTICR3` writer"]
pub type W = crate::W<Exticr3Spec>;
#[doc = "Field `EXTI8` reader - GPIO port selection"]
pub type Exti8R = crate::BitReader;
#[doc = "Field `EXTI8` writer - GPIO port selection"]
pub type Exti8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8(&self) -> Exti8R {
        Exti8R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8(&mut self) -> Exti8W<'_, Exticr3Spec> {
        Exti8W::new(self, 0)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exticr3Spec;
impl crate::RegisterSpec for Exticr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr3::R`](R) reader structure"]
impl crate::Readable for Exticr3Spec {}
#[doc = "`write(|w| ..)` method takes [`exticr3::W`](W) writer structure"]
impl crate::Writable for Exticr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for Exticr3Spec {}
