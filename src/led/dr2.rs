#[doc = "Register `DR2` reader"]
pub type R = crate::R<Dr2Spec>;
#[doc = "Register `DR2` writer"]
pub type W = crate::W<Dr2Spec>;
#[doc = "Field `DATA2_A` reader - 8-bit data register"]
pub type Data2AR = crate::BitReader;
#[doc = "Field `DATA2_A` writer - 8-bit data register"]
pub type Data2AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_B` reader - 8-bit data register"]
pub type Data2BR = crate::BitReader;
#[doc = "Field `DATA2_B` writer - 8-bit data register"]
pub type Data2BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_C` reader - 8-bit data register"]
pub type Data2CR = crate::BitReader;
#[doc = "Field `DATA2_C` writer - 8-bit data register"]
pub type Data2CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_D` reader - 8-bit data register"]
pub type Data2DR = crate::BitReader;
#[doc = "Field `DATA2_D` writer - 8-bit data register"]
pub type Data2DW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_E` reader - 8-bit data register"]
pub type Data2ER = crate::BitReader;
#[doc = "Field `DATA2_E` writer - 8-bit data register"]
pub type Data2EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_F` reader - 8-bit data register"]
pub type Data2FR = crate::BitReader;
#[doc = "Field `DATA2_F` writer - 8-bit data register"]
pub type Data2FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_G` reader - 8-bit data register"]
pub type Data2GR = crate::BitReader;
#[doc = "Field `DATA2_G` writer - 8-bit data register"]
pub type Data2GW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA2_DP` reader - 8-bit data register"]
pub type Data2DpR = crate::BitReader;
#[doc = "Field `DATA2_DP` writer - 8-bit data register"]
pub type Data2DpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_a(&self) -> Data2AR {
        Data2AR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_b(&self) -> Data2BR {
        Data2BR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_c(&self) -> Data2CR {
        Data2CR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_d(&self) -> Data2DR {
        Data2DR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_e(&self) -> Data2ER {
        Data2ER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_f(&self) -> Data2FR {
        Data2FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_g(&self) -> Data2GR {
        Data2GR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_dp(&self) -> Data2DpR {
        Data2DpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_a(&mut self) -> Data2AW<'_, Dr2Spec> {
        Data2AW::new(self, 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_b(&mut self) -> Data2BW<'_, Dr2Spec> {
        Data2BW::new(self, 1)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_c(&mut self) -> Data2CW<'_, Dr2Spec> {
        Data2CW::new(self, 2)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_d(&mut self) -> Data2DW<'_, Dr2Spec> {
        Data2DW::new(self, 3)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_e(&mut self) -> Data2EW<'_, Dr2Spec> {
        Data2EW::new(self, 4)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_f(&mut self) -> Data2FW<'_, Dr2Spec> {
        Data2FW::new(self, 5)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_g(&mut self) -> Data2GW<'_, Dr2Spec> {
        Data2GW::new(self, 6)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data2_dp(&mut self) -> Data2DpW<'_, Dr2Spec> {
        Data2DpW::new(self, 7)
    }
}
#[doc = "Data2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr2Spec;
impl crate::RegisterSpec for Dr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr2::R`](R) reader structure"]
impl crate::Readable for Dr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dr2::W`](W) writer structure"]
impl crate::Writable for Dr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR2 to value 0"]
impl crate::Resettable for Dr2Spec {}
