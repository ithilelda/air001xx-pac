#[doc = "Register `DR3` reader"]
pub type R = crate::R<Dr3Spec>;
#[doc = "Register `DR3` writer"]
pub type W = crate::W<Dr3Spec>;
#[doc = "Field `DATA3_A` reader - 8-bit data register"]
pub type Data3AR = crate::BitReader;
#[doc = "Field `DATA3_A` writer - 8-bit data register"]
pub type Data3AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_B` reader - 8-bit data register"]
pub type Data3BR = crate::BitReader;
#[doc = "Field `DATA3_B` writer - 8-bit data register"]
pub type Data3BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_C` reader - 8-bit data register"]
pub type Data3CR = crate::BitReader;
#[doc = "Field `DATA3_C` writer - 8-bit data register"]
pub type Data3CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_D` reader - 8-bit data register"]
pub type Data3DR = crate::BitReader;
#[doc = "Field `DATA3_D` writer - 8-bit data register"]
pub type Data3DW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_E` reader - 8-bit data register"]
pub type Data3ER = crate::BitReader;
#[doc = "Field `DATA3_E` writer - 8-bit data register"]
pub type Data3EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_F` reader - 8-bit data register"]
pub type Data3FR = crate::BitReader;
#[doc = "Field `DATA3_F` writer - 8-bit data register"]
pub type Data3FW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_G` reader - 8-bit data register"]
pub type Data3GR = crate::BitReader;
#[doc = "Field `DATA3_G` writer - 8-bit data register"]
pub type Data3GW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA3_DP` reader - 8-bit data register"]
pub type Data3DpR = crate::BitReader;
#[doc = "Field `DATA3_DP` writer - 8-bit data register"]
pub type Data3DpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_a(&self) -> Data3AR {
        Data3AR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_b(&self) -> Data3BR {
        Data3BR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_c(&self) -> Data3CR {
        Data3CR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_d(&self) -> Data3DR {
        Data3DR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_e(&self) -> Data3ER {
        Data3ER::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_f(&self) -> Data3FR {
        Data3FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_g(&self) -> Data3GR {
        Data3GR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_dp(&self) -> Data3DpR {
        Data3DpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_a(&mut self) -> Data3AW<'_, Dr3Spec> {
        Data3AW::new(self, 0)
    }
    #[doc = "Bit 1 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_b(&mut self) -> Data3BW<'_, Dr3Spec> {
        Data3BW::new(self, 1)
    }
    #[doc = "Bit 2 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_c(&mut self) -> Data3CW<'_, Dr3Spec> {
        Data3CW::new(self, 2)
    }
    #[doc = "Bit 3 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_d(&mut self) -> Data3DW<'_, Dr3Spec> {
        Data3DW::new(self, 3)
    }
    #[doc = "Bit 4 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_e(&mut self) -> Data3EW<'_, Dr3Spec> {
        Data3EW::new(self, 4)
    }
    #[doc = "Bit 5 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_f(&mut self) -> Data3FW<'_, Dr3Spec> {
        Data3FW::new(self, 5)
    }
    #[doc = "Bit 6 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_g(&mut self) -> Data3GW<'_, Dr3Spec> {
        Data3GW::new(self, 6)
    }
    #[doc = "Bit 7 - 8-bit data register"]
    #[inline(always)]
    pub fn data3_dp(&mut self) -> Data3DpW<'_, Dr3Spec> {
        Data3DpW::new(self, 7)
    }
}
#[doc = "Data3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr3Spec;
impl crate::RegisterSpec for Dr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr3::R`](R) reader structure"]
impl crate::Readable for Dr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dr3::W`](W) writer structure"]
impl crate::Writable for Dr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR3 to value 0"]
impl crate::Resettable for Dr3Spec {}
