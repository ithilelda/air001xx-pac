#[doc = "Register `TS0` reader"]
pub type R = crate::R<Ts0Spec>;
#[doc = "Register `TS0` writer"]
pub type W = crate::W<Ts0Spec>;
#[doc = "Field `TS0` reader - FLash TS0 register"]
pub type Ts0R = crate::FieldReader;
#[doc = "Field `TS0` writer - FLash TS0 register"]
pub type Ts0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    pub fn ts0(&self) -> Ts0R {
        Ts0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS0 register"]
    #[inline(always)]
    pub fn ts0(&mut self) -> Ts0W<'_, Ts0Spec> {
        Ts0W::new(self, 0)
    }
}
#[doc = "Flash TS0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts0Spec;
impl crate::RegisterSpec for Ts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts0::R`](R) reader structure"]
impl crate::Readable for Ts0Spec {}
#[doc = "`write(|w| ..)` method takes [`ts0::W`](W) writer structure"]
impl crate::Writable for Ts0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS0 to value 0xb4"]
impl crate::Resettable for Ts0Spec {
    const RESET_VALUE: u32 = 0xb4;
}
