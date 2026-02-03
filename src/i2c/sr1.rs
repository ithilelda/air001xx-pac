#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SbR = crate::BitReader;
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type AddrR = crate::BitReader;
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BtfR = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type StopfR = crate::BitReader;
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RxNeR = crate::BitReader;
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TxER = crate::BitReader;
#[doc = "Field `BERR` reader - Bus error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bus error"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ArloR = crate::BitReader;
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ArloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AfR = crate::BitReader;
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BtfR {
        BtfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RxNeR {
        RxNeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TxER {
        TxER::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ArloR {
        ArloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AfR {
        AfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, Sr1Spec> {
        BerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ArloW<'_, Sr1Spec> {
        ArloW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&mut self) -> AfW<'_, Sr1Spec> {
        AfW::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<'_, Sr1Spec> {
        OvrW::new(self, 11)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PecerrW<'_, Sr1Spec> {
        PecerrW::new(self, 12)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {}
