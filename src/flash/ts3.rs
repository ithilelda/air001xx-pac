#[doc = "Register `TS3` reader"]
pub type R = crate::R<Ts3Spec>;
#[doc = "Register `TS3` writer"]
pub type W = crate::W<Ts3Spec>;
#[doc = "Field `TS3` reader - FLash TS3 register"]
pub type Ts3R = crate::FieldReader;
#[doc = "Field `TS3` writer - FLash TS3 register"]
pub type Ts3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FLash TS3 register"]
    #[inline(always)]
    pub fn ts3(&self) -> Ts3R {
        Ts3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS3 register"]
    #[inline(always)]
    pub fn ts3(&mut self) -> Ts3W<'_, Ts3Spec> {
        Ts3W::new(self, 0)
    }
}
#[doc = "Flash TS3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts3Spec;
impl crate::RegisterSpec for Ts3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts3::R`](R) reader structure"]
impl crate::Readable for Ts3Spec {}
#[doc = "`write(|w| ..)` method takes [`ts3::W`](W) writer structure"]
impl crate::Writable for Ts3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS3 to value 0xb4"]
impl crate::Resettable for Ts3Spec {
    const RESET_VALUE: u32 = 0xb4;
}
