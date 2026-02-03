#[doc = "Register `DR0` reader"]
pub type R = crate::R<Dr0Spec>;
#[doc = "Register `DR0` writer"]
pub type W = crate::W<Dr0Spec>;
#[doc = "Field `DATA0_A` reader - 8-bit data register"]
pub type Data0AR = crate::BitReader;
#[doc = "Field `DATA0_A` writer - 8-bit data register"]
pub type Data0AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_B` reader - 8-bit data register"]
pub type Data0BR = crate::BitReader;
#[doc = "Field `DATA0_B` writer - 8-bit data register"]
pub type Data0BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_C` reader - 8-bit data register"]
pub type Data0CR = crate::BitReader;
#[doc = "Field `DATA0_C` writer - 8-bit data register"]
pub type Data0CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_D` reader - 8-bit data register"]
pub type Data0DR = crate::BitReader;
#[doc = "Field `DATA0_D` writer - 8-bit data register"]
pub type Data0DW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_E` reader - 8-bit data register"]
pub type Data0ER = crate::BitReader;
#[doc = "Field `DATA0_E` writer - 8-bit data register"]
pub type Data0EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_F` reader - 8-bit data register"]
pub type Data0FR = crate::BitReader;
#[doc = "Field `DATA0_F` writer - 8-bit data register"]
pub type Data0FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_G` reader - 8-bit data register"]
pub type Data0GR = crate::BitReader;
#[doc = "Field `DATA0_G` writer - 8-bit data register"]
pub type Data0GW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA0_DP` reader - 8-bit data register"]
pub type Data0DpR = crate::BitReader;
#[doc = "Field `DATA0_DP` writer - 8-bit data register"]
pub type Data0DpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_a(&self) -> Data0AR {
        Data0AR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_b(&self) -> Data0BR {
        Data0BR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_c(&self) -> Data0CR {
        Data0CR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_d(&self) -> Data0DR {
        Data0DR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_e(&self) -> Data0ER {
        Data0ER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_f(&self) -> Data0FR {
        Data0FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_g(&self) -> Data0GR {
        Data0GR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_dp(&self) -> Data0DpR {
        Data0DpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_a(&mut self) -> Data0AW<'_, Dr0Spec> {
        Data0AW::new(self, 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_b(&mut self) -> Data0BW<'_, Dr0Spec> {
        Data0BW::new(self, 1)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_c(&mut self) -> Data0CW<'_, Dr0Spec> {
        Data0CW::new(self, 2)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_d(&mut self) -> Data0DW<'_, Dr0Spec> {
        Data0DW::new(self, 3)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_e(&mut self) -> Data0EW<'_, Dr0Spec> {
        Data0EW::new(self, 4)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_f(&mut self) -> Data0FW<'_, Dr0Spec> {
        Data0FW::new(self, 5)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_g(&mut self) -> Data0GW<'_, Dr0Spec> {
        Data0GW::new(self, 6)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data0_dp(&mut self) -> Data0DpW<'_, Dr0Spec> {
        Data0DpW::new(self, 7)
    }
}
#[doc = "Data0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr0Spec;
impl crate::RegisterSpec for Dr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for Dr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dr0::W`](W) writer structure"]
impl crate::Writable for Dr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for Dr0Spec {}
