#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MemModeR = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MemModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C_PA2_ANF` reader - Analog filter enable control driving capability activation bits PA2"]
pub type I2cPa2AnfR = crate::BitReader;
#[doc = "Field `I2C_PA2_ANF` writer - Analog filter enable control driving capability activation bits PA2"]
pub type I2cPa2AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA3_ANF` reader - Analog filter enable control driving capability activation bits PA3"]
pub type I2cPa3AnfR = crate::BitReader;
#[doc = "Field `I2C_PA3_ANF` writer - Analog filter enable control driving capability activation bits PA3"]
pub type I2cPa3AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA7_ANF` reader - Analog filter enable control driving capability activation bits PA7"]
pub type I2cPa7AnfR = crate::BitReader;
#[doc = "Field `I2C_PA7_ANF` writer - Analog filter enable control driving capability activation bits PA7"]
pub type I2cPa7AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA8_ANF` reader - Analog filter enable control driving capability activation bits PA8"]
pub type I2cPa8AnfR = crate::BitReader;
#[doc = "Field `I2C_PA8_ANF` writer - Analog filter enable control driving capability activation bits PA8"]
pub type I2cPa8AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA9_ANF` reader - Analog filter enable control driving capability activation bits PA9"]
pub type I2cPa9AnfR = crate::BitReader;
#[doc = "Field `I2C_PA9_ANF` writer - Analog filter enable control driving capability activation bits PA9"]
pub type I2cPa9AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA10_ANF` reader - Analog filter enable control driving capability activation bits PA10"]
pub type I2cPa10AnfR = crate::BitReader;
#[doc = "Field `I2C_PA10_ANF` writer - Analog filter enable control driving capability activation bits PA10"]
pub type I2cPa10AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA11_ANF` reader - Analog filter enable control driving capability activation bits PA11"]
pub type I2cPa11AnfR = crate::BitReader;
#[doc = "Field `I2C_PA11_ANF` writer - Analog filter enable control driving capability activation bits PA11"]
pub type I2cPa11AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PA12_ANF` reader - Analog filter enable control driving capability activation bits PA12"]
pub type I2cPa12AnfR = crate::BitReader;
#[doc = "Field `I2C_PA12_ANF` writer - Analog filter enable control driving capability activation bits PA12"]
pub type I2cPa12AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB6_ANF` reader - Analog filter enable control driving capability activation bits PB6"]
pub type I2cPb6AnfR = crate::BitReader;
#[doc = "Field `I2C_PB6_ANF` writer - Analog filter enable control driving capability activation bits PB6"]
pub type I2cPb6AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB7_ANF` reader - Analog filter enable control driving capability activation bits PB7"]
pub type I2cPb7AnfR = crate::BitReader;
#[doc = "Field `I2C_PB7_ANF` writer - Analog filter enable control driving capability activation bits PB7"]
pub type I2cPb7AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PB8_ANF` reader - Analog filter enable control driving capability activation bits PB8"]
pub type I2cPb8AnfR = crate::BitReader;
#[doc = "Field `I2C_PB8_ANF` writer - Analog filter enable control driving capability activation bits PB8"]
pub type I2cPb8AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PF0_ANF` reader - Analog filter enable control driving capability activation bits PF0"]
pub type I2cPf0AnfR = crate::BitReader;
#[doc = "Field `I2C_PF0_ANF` writer - Analog filter enable control driving capability activation bits PF0"]
pub type I2cPf0AnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_PF1_ANF` reader - Analog filter enable control driving capability activation bits PF1"]
pub type I2cPf1AnfR = crate::BitReader;
#[doc = "Field `I2C_PF1_ANF` writer - Analog filter enable control driving capability activation bits PF1"]
pub type I2cPf1AnfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MemModeR {
        MemModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    pub fn i2c_pa2_anf(&self) -> I2cPa2AnfR {
        I2cPa2AnfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    pub fn i2c_pa3_anf(&self) -> I2cPa3AnfR {
        I2cPa3AnfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    pub fn i2c_pa7_anf(&self) -> I2cPa7AnfR {
        I2cPa7AnfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    pub fn i2c_pa8_anf(&self) -> I2cPa8AnfR {
        I2cPa8AnfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    pub fn i2c_pa9_anf(&self) -> I2cPa9AnfR {
        I2cPa9AnfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    pub fn i2c_pa10_anf(&self) -> I2cPa10AnfR {
        I2cPa10AnfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    pub fn i2c_pa11_anf(&self) -> I2cPa11AnfR {
        I2cPa11AnfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    pub fn i2c_pa12_anf(&self) -> I2cPa12AnfR {
        I2cPa12AnfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    pub fn i2c_pb6_anf(&self) -> I2cPb6AnfR {
        I2cPb6AnfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    pub fn i2c_pb7_anf(&self) -> I2cPb7AnfR {
        I2cPb7AnfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    pub fn i2c_pb8_anf(&self) -> I2cPb8AnfR {
        I2cPb8AnfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    pub fn i2c_pf0_anf(&self) -> I2cPf0AnfR {
        I2cPf0AnfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    pub fn i2c_pf1_anf(&self) -> I2cPf1AnfR {
        I2cPf1AnfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MemModeW<'_, Cfgr1Spec> {
        MemModeW::new(self, 0)
    }
    #[doc = "Bit 18 - Analog filter enable control driving capability activation bits PA2"]
    #[inline(always)]
    pub fn i2c_pa2_anf(&mut self) -> I2cPa2AnfW<'_, Cfgr1Spec> {
        I2cPa2AnfW::new(self, 18)
    }
    #[doc = "Bit 19 - Analog filter enable control driving capability activation bits PA3"]
    #[inline(always)]
    pub fn i2c_pa3_anf(&mut self) -> I2cPa3AnfW<'_, Cfgr1Spec> {
        I2cPa3AnfW::new(self, 19)
    }
    #[doc = "Bit 20 - Analog filter enable control driving capability activation bits PA7"]
    #[inline(always)]
    pub fn i2c_pa7_anf(&mut self) -> I2cPa7AnfW<'_, Cfgr1Spec> {
        I2cPa7AnfW::new(self, 20)
    }
    #[doc = "Bit 21 - Analog filter enable control driving capability activation bits PA8"]
    #[inline(always)]
    pub fn i2c_pa8_anf(&mut self) -> I2cPa8AnfW<'_, Cfgr1Spec> {
        I2cPa8AnfW::new(self, 21)
    }
    #[doc = "Bit 22 - Analog filter enable control driving capability activation bits PA9"]
    #[inline(always)]
    pub fn i2c_pa9_anf(&mut self) -> I2cPa9AnfW<'_, Cfgr1Spec> {
        I2cPa9AnfW::new(self, 22)
    }
    #[doc = "Bit 23 - Analog filter enable control driving capability activation bits PA10"]
    #[inline(always)]
    pub fn i2c_pa10_anf(&mut self) -> I2cPa10AnfW<'_, Cfgr1Spec> {
        I2cPa10AnfW::new(self, 23)
    }
    #[doc = "Bit 24 - Analog filter enable control driving capability activation bits PA11"]
    #[inline(always)]
    pub fn i2c_pa11_anf(&mut self) -> I2cPa11AnfW<'_, Cfgr1Spec> {
        I2cPa11AnfW::new(self, 24)
    }
    #[doc = "Bit 25 - Analog filter enable control driving capability activation bits PA12"]
    #[inline(always)]
    pub fn i2c_pa12_anf(&mut self) -> I2cPa12AnfW<'_, Cfgr1Spec> {
        I2cPa12AnfW::new(self, 25)
    }
    #[doc = "Bit 26 - Analog filter enable control driving capability activation bits PB6"]
    #[inline(always)]
    pub fn i2c_pb6_anf(&mut self) -> I2cPb6AnfW<'_, Cfgr1Spec> {
        I2cPb6AnfW::new(self, 26)
    }
    #[doc = "Bit 27 - Analog filter enable control driving capability activation bits PB7"]
    #[inline(always)]
    pub fn i2c_pb7_anf(&mut self) -> I2cPb7AnfW<'_, Cfgr1Spec> {
        I2cPb7AnfW::new(self, 27)
    }
    #[doc = "Bit 28 - Analog filter enable control driving capability activation bits PB8"]
    #[inline(always)]
    pub fn i2c_pb8_anf(&mut self) -> I2cPb8AnfW<'_, Cfgr1Spec> {
        I2cPb8AnfW::new(self, 28)
    }
    #[doc = "Bit 29 - Analog filter enable control driving capability activation bits PF0"]
    #[inline(always)]
    pub fn i2c_pf0_anf(&mut self) -> I2cPf0AnfW<'_, Cfgr1Spec> {
        I2cPf0AnfW::new(self, 29)
    }
    #[doc = "Bit 30 - Analog filter enable control driving capability activation bits PF1"]
    #[inline(always)]
    pub fn i2c_pf1_anf(&mut self) -> I2cPf1AnfW<'_, Cfgr1Spec> {
        I2cPf1AnfW::new(self, 30)
    }
}
#[doc = "SYSCFG configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {}
