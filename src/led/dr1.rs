#[doc = "Register `DR1` reader"]
pub type R = crate::R<Dr1Spec>;
#[doc = "Register `DR1` writer"]
pub type W = crate::W<Dr1Spec>;
#[doc = "Field `DATA1_A` reader - 8-bit data register"]
pub type Data1AR = crate::BitReader;
#[doc = "Field `DATA1_A` writer - 8-bit data register"]
pub type Data1AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_B` reader - 8-bit data register"]
pub type Data1BR = crate::BitReader;
#[doc = "Field `DATA1_B` writer - 8-bit data register"]
pub type Data1BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_C` reader - 8-bit data register"]
pub type Data1CR = crate::BitReader;
#[doc = "Field `DATA1_C` writer - 8-bit data register"]
pub type Data1CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_D` reader - 8-bit data register"]
pub type Data1DR = crate::BitReader;
#[doc = "Field `DATA1_D` writer - 8-bit data register"]
pub type Data1DW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_E` reader - 8-bit data register"]
pub type Data1ER = crate::BitReader;
#[doc = "Field `DATA1_E` writer - 8-bit data register"]
pub type Data1EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_F` reader - 8-bit data register"]
pub type Data1FR = crate::BitReader;
#[doc = "Field `DATA1_F` writer - 8-bit data register"]
pub type Data1FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_G` reader - 8-bit data register"]
pub type Data1GR = crate::BitReader;
#[doc = "Field `DATA1_G` writer - 8-bit data register"]
pub type Data1GW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA1_DP` reader - 8-bit data register"]
pub type Data1DpR = crate::BitReader;
#[doc = "Field `DATA1_DP` writer - 8-bit data register"]
pub type Data1DpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_a(&self) -> Data1AR {
        Data1AR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_b(&self) -> Data1BR {
        Data1BR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_c(&self) -> Data1CR {
        Data1CR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_d(&self) -> Data1DR {
        Data1DR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_e(&self) -> Data1ER {
        Data1ER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_f(&self) -> Data1FR {
        Data1FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_g(&self) -> Data1GR {
        Data1GR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_dp(&self) -> Data1DpR {
        Data1DpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_a(&mut self) -> Data1AW<'_, Dr1Spec> {
        Data1AW::new(self, 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_b(&mut self) -> Data1BW<'_, Dr1Spec> {
        Data1BW::new(self, 1)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_c(&mut self) -> Data1CW<'_, Dr1Spec> {
        Data1CW::new(self, 2)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_d(&mut self) -> Data1DW<'_, Dr1Spec> {
        Data1DW::new(self, 3)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_e(&mut self) -> Data1EW<'_, Dr1Spec> {
        Data1EW::new(self, 4)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_f(&mut self) -> Data1FW<'_, Dr1Spec> {
        Data1FW::new(self, 5)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_g(&mut self) -> Data1GW<'_, Dr1Spec> {
        Data1GW::new(self, 6)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data1_dp(&mut self) -> Data1DpW<'_, Dr1Spec> {
        Data1DpW::new(self, 7)
    }
}
#[doc = "Data1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr1Spec;
impl crate::RegisterSpec for Dr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr1::R`](R) reader structure"]
impl crate::Readable for Dr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dr1::W`](W) writer structure"]
impl crate::Writable for Dr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR1 to value 0"]
impl crate::Resettable for Dr1Spec {}
