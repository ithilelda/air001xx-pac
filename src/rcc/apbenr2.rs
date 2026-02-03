#[doc = "Register `APBENR2` reader"]
pub type R = crate::R<Apbenr2Spec>;
#[doc = "Register `APBENR2` writer"]
pub type W = crate::W<Apbenr2Spec>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG, COMP and VREFBUF clock enable"]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG, COMP and VREFBUF clock enable"]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type Tim1enR = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14EN` reader - TIM14 timer clock enable"]
pub type Tim14enR = crate::BitReader;
#[doc = "Field `TIM14EN` writer - TIM14 timer clock enable"]
pub type Tim14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub type Tim16enR = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM16 timer clock enable"]
pub type Tim17enR = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM16 timer clock enable"]
pub type Tim17enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1EN` reader - COMP1 clock enable"]
pub type Comp1enR = crate::BitReader;
#[doc = "Field `COMP1EN` writer - COMP1 clock enable"]
pub type Comp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2EN` reader - COMP2 clock enable"]
pub type Comp2enR = crate::BitReader;
#[doc = "Field `COMP2EN` writer - COMP2 clock enable"]
pub type Comp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDEN` reader - LED clock enable"]
pub type LedenR = crate::BitReader;
#[doc = "Field `LEDEN` writer - LED clock enable"]
pub type LedenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> Tim14enR {
        Tim14enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> Tim17enR {
        Tim17enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> Comp1enR {
        Comp1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> Comp2enR {
        Comp2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LED clock enable"]
    #[inline(always)]
    pub fn leden(&self) -> LedenR {
        LedenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, Apbenr2Spec> {
        SyscfgenW::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> Tim1enW<'_, Apbenr2Spec> {
        Tim1enW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<'_, Apbenr2Spec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<'_, Apbenr2Spec> {
        Usart1enW::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> Tim14enW<'_, Apbenr2Spec> {
        Tim14enW::new(self, 15)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<'_, Apbenr2Spec> {
        Tim16enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> Tim17enW<'_, Apbenr2Spec> {
        Tim17enW::new(self, 18)
    }
    #[doc = "Bit 20 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, Apbenr2Spec> {
        AdcenW::new(self, 20)
    }
    #[doc = "Bit 21 - COMP1 clock enable"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> Comp1enW<'_, Apbenr2Spec> {
        Comp1enW::new(self, 21)
    }
    #[doc = "Bit 22 - COMP2 clock enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> Comp2enW<'_, Apbenr2Spec> {
        Comp2enW::new(self, 22)
    }
    #[doc = "Bit 23 - LED clock enable"]
    #[inline(always)]
    pub fn leden(&mut self) -> LedenW<'_, Apbenr2Spec> {
        LedenW::new(self, 23)
    }
}
#[doc = "APB peripheral clock enable register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbenr2Spec;
impl crate::RegisterSpec for Apbenr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbenr2::R`](R) reader structure"]
impl crate::Readable for Apbenr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbenr2::W`](W) writer structure"]
impl crate::Writable for Apbenr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBENR2 to value 0"]
impl crate::Resettable for Apbenr2Spec {}
