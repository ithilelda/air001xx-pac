#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPEC` reader - PEC enable"]
pub type EnpecR = crate::BitReader;
#[doc = "Field `ENPEC` writer - PEC enable"]
pub type EnpecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENGC` reader - General call enable"]
pub type EngcR = crate::BitReader;
#[doc = "Field `ENGC` writer - General call enable"]
pub type EngcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)"]
pub type NostretchR = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)"]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge enable"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge enable"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS` reader - Acknowledge/PEC Position (for data reception)"]
pub type PosR = crate::BitReader;
#[doc = "Field `POS` writer - Acknowledge/PEC Position (for data reception)"]
pub type PosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> EnpecR {
        EnpecR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> EngcR {
        EngcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, Cr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&mut self) -> EnpecW<'_, Cr1Spec> {
        EnpecW::new(self, 5)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&mut self) -> EngcW<'_, Cr1Spec> {
        EngcW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NostretchW<'_, Cr1Spec> {
        NostretchW::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Cr1Spec> {
        AckW::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&mut self) -> PosW<'_, Cr1Spec> {
        PosW::new(self, 11)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, Cr1Spec> {
        PecW::new(self, 12)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, Cr1Spec> {
        SwrstW::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
