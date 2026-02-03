#[doc = "Register `IOPENR` reader"]
pub type R = crate::R<IopenrSpec>;
#[doc = "Register `IOPENR` writer"]
pub type W = crate::W<IopenrSpec>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<'_, IopenrSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<'_, IopenrSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 5 - I/O port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GpiofenW<'_, IopenrSpec> {
        GpiofenW::new(self, 5)
    }
}
#[doc = "GPIO clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IopenrSpec;
impl crate::RegisterSpec for IopenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopenr::R`](R) reader structure"]
impl crate::Readable for IopenrSpec {}
#[doc = "`write(|w| ..)` method takes [`iopenr::W`](W) writer structure"]
impl crate::Writable for IopenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IopenrSpec {}
