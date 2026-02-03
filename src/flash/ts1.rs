#[doc = "Register `TS1` reader"]
pub type R = crate::R<Ts1Spec>;
#[doc = "Register `TS1` writer"]
pub type W = crate::W<Ts1Spec>;
#[doc = "Field `TS1` reader - FLash TS1 register"]
pub type Ts1R = crate::FieldReader<u16>;
#[doc = "Field `TS1` writer - FLash TS1 register"]
pub type Ts1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - FLash TS1 register"]
    #[inline(always)]
    pub fn ts1(&self) -> Ts1R {
        Ts1R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FLash TS1 register"]
    #[inline(always)]
    pub fn ts1(&mut self) -> Ts1W<'_, Ts1Spec> {
        Ts1W::new(self, 0)
    }
}
#[doc = "Flash TS1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts1Spec;
impl crate::RegisterSpec for Ts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts1::R`](R) reader structure"]
impl crate::Readable for Ts1Spec {}
#[doc = "`write(|w| ..)` method takes [`ts1::W`](W) writer structure"]
impl crate::Writable for Ts1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS1 to value 0x01b0"]
impl crate::Resettable for Ts1Spec {
    const RESET_VALUE: u32 = 0x01b0;
}
