#[doc = "Register `OAR1` reader"]
pub type R = crate::R<Oar1Spec>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<Oar1Spec>;
#[doc = "Field `ADD` reader - Interface address"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - Interface address"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, Oar1Spec> {
        AddW::new(self, 1)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar1Spec;
impl crate::RegisterSpec for Oar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for Oar1Spec {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for Oar1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for Oar1Spec {}
