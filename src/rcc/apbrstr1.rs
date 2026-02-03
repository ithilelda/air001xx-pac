#[doc = "Register `APBRSTR1` reader"]
pub type R = crate::R<Apbrstr1Spec>;
#[doc = "Register `APBRSTR1` writer"]
pub type W = crate::W<Apbrstr1Spec>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type Tim3rstR = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CRST` reader - I2C reset"]
pub type I2crstR = crate::BitReader;
#[doc = "Field `I2CRST` writer - I2C reset"]
pub type I2crstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRST` reader - Debug support reset"]
pub type DbgrstR = crate::BitReader;
#[doc = "Field `DBGRST` writer - Debug support reset"]
pub type DbgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PwrrstR = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PwrrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMRST` reader - Low Power Timer reset"]
pub type LptimrstR = crate::BitReader;
#[doc = "Field `LPTIMRST` writer - Low Power Timer reset"]
pub type LptimrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C reset"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2crstR {
        I2crstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DbgrstR {
        DbgrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PwrrstR {
        PwrrstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    pub fn lptimrst(&self) -> LptimrstR {
        LptimrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> Tim3rstW<'_, Apbrstr1Spec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> Spi2rstW<'_, Apbrstr1Spec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<'_, Apbrstr1Spec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C reset"]
    #[inline(always)]
    pub fn i2crst(&mut self) -> I2crstW<'_, Apbrstr1Spec> {
        I2crstW::new(self, 21)
    }
    #[doc = "Bit 27 - Debug support reset"]
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DbgrstW<'_, Apbrstr1Spec> {
        DbgrstW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PwrrstW<'_, Apbrstr1Spec> {
        PwrrstW::new(self, 28)
    }
    #[doc = "Bit 31 - Low Power Timer reset"]
    #[inline(always)]
    pub fn lptimrst(&mut self) -> LptimrstW<'_, Apbrstr1Spec> {
        LptimrstW::new(self, 31)
    }
}
#[doc = "APB peripheral reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrstr1Spec;
impl crate::RegisterSpec for Apbrstr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr1::R`](R) reader structure"]
impl crate::Readable for Apbrstr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrstr1::W`](W) writer structure"]
impl crate::Writable for Apbrstr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRSTR1 to value 0"]
impl crate::Resettable for Apbrstr1Spec {}
