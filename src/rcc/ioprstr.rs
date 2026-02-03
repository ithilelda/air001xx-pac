#[doc = "Register `IOPRSTR` reader"]
pub type R = crate::R<IoprstrSpec>;
#[doc = "Register `IOPRSTR` writer"]
pub type W = crate::W<IoprstrSpec>;
#[doc = "Field `GPIOARST` reader - I/O port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - I/O port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - I/O port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - I/O port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - I/O port F reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - I/O port F reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<'_, IoprstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<'_, IoprstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GpiofrstW<'_, IoprstrSpec> {
        GpiofrstW::new(self, 5)
    }
}
#[doc = "GPIO reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoprstrSpec;
impl crate::RegisterSpec for IoprstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioprstr::R`](R) reader structure"]
impl crate::Readable for IoprstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure"]
impl crate::Writable for IoprstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IoprstrSpec {}
