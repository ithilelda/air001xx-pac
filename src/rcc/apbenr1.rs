#[doc = "Register `APBENR1` reader"]
pub type R = crate::R<Apbenr1Spec>;
#[doc = "Register `APBENR1` writer"]
pub type W = crate::W<Apbenr1Spec>;
#[doc = "Field `TIM3EN` reader - TIM3 timer clock enable"]
pub type Tim3enR = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM3 timer clock enable"]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub type RtcapbenR = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub type RtcapbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGEN` reader - WWDG clock enable"]
pub type WwdgenR = crate::BitReader;
#[doc = "Field `WWDGEN` writer - WWDG clock enable"]
pub type WwdgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - I2C clock enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C clock enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - Debug support clock enable"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debug support clock enable"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub type PwrenR = crate::BitReader;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIMEN` reader - LPTIM clock enable"]
pub type LptimenR = crate::BitReader;
#[doc = "Field `LPTIMEN` writer - LPTIM clock enable"]
pub type LptimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RtcapbenR {
        RtcapbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WwdgenR {
        WwdgenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM clock enable"]
    #[inline(always)]
    pub fn lptimen(&self) -> LptimenR {
        LptimenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer clock enable"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> Tim3enW<'_, Apbenr1Spec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RtcapbenW<'_, Apbenr1Spec> {
        RtcapbenW::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WwdgenW<'_, Apbenr1Spec> {
        WwdgenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<'_, Apbenr1Spec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<'_, Apbenr1Spec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 21 - I2C clock enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2cenW<'_, Apbenr1Spec> {
        I2cenW::new(self, 21)
    }
    #[doc = "Bit 27 - Debug support clock enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, Apbenr1Spec> {
        DbgenW::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PwrenW<'_, Apbenr1Spec> {
        PwrenW::new(self, 28)
    }
    #[doc = "Bit 31 - LPTIM clock enable"]
    #[inline(always)]
    pub fn lptimen(&mut self) -> LptimenW<'_, Apbenr1Spec> {
        LptimenW::new(self, 31)
    }
}
#[doc = "APB peripheral clock enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbenr1Spec;
impl crate::RegisterSpec for Apbenr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr1::R`](R) reader structure"]
impl crate::Readable for Apbenr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure"]
impl crate::Writable for Apbenr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBENR1 to value 0"]
impl crate::Resettable for Apbenr1Spec {}
