#[doc = "Register `TPS3` reader"]
pub type R = crate::R<Tps3Spec>;
#[doc = "Register `TPS3` writer"]
pub type W = crate::W<Tps3Spec>;
#[doc = "Field `TPS3` reader - FLash TPS3 register"]
pub type Tps3R = crate::FieldReader<u16>;
#[doc = "Field `TPS3` writer - FLash TPS3 register"]
pub type Tps3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - FLash TPS3 register"]
    #[inline(always)]
    pub fn tps3(&self) -> Tps3R {
        Tps3R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - FLash TPS3 register"]
    #[inline(always)]
    pub fn tps3(&mut self) -> Tps3W<'_, Tps3Spec> {
        Tps3W::new(self, 0)
    }
}
#[doc = "Flash TPS3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tps3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tps3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tps3Spec;
impl crate::RegisterSpec for Tps3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tps3::R`](R) reader structure"]
impl crate::Readable for Tps3Spec {}
#[doc = "`write(|w| ..)` method takes [`tps3::W`](W) writer structure"]
impl crate::Writable for Tps3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPS3 to value 0x06c0"]
impl crate::Resettable for Tps3Spec {
    const RESET_VALUE: u32 = 0x06c0;
}
