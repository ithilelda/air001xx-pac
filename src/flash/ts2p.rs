#[doc = "Register `TS2P` reader"]
pub type R = crate::R<Ts2pSpec>;
#[doc = "Register `TS2P` writer"]
pub type W = crate::W<Ts2pSpec>;
#[doc = "Field `TS2P` reader - FLash TS2P register"]
pub type Ts2pR = crate::FieldReader;
#[doc = "Field `TS2P` writer - FLash TS2P register"]
pub type Ts2pW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FLash TS2P register"]
    #[inline(always)]
    pub fn ts2p(&self) -> Ts2pR {
        Ts2pR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FLash TS2P register"]
    #[inline(always)]
    pub fn ts2p(&mut self) -> Ts2pW<'_, Ts2pSpec> {
        Ts2pW::new(self, 0)
    }
}
#[doc = "Flash TS2P register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts2p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts2p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts2pSpec;
impl crate::RegisterSpec for Ts2pSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts2p::R`](R) reader structure"]
impl crate::Readable for Ts2pSpec {}
#[doc = "`write(|w| ..)` method takes [`ts2p::W`](W) writer structure"]
impl crate::Writable for Ts2pSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS2P to value 0xb4"]
impl crate::Resettable for Ts2pSpec {
    const RESET_VALUE: u32 = 0xb4;
}
