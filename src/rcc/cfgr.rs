#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE` reader - APB prescaler"]
pub type PpreR = crate::FieldReader;
#[doc = "Field `PPRE` writer - APB prescaler"]
pub type PpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type McoselR = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type McopreR = crate::FieldReader;
#[doc = "Field `MCOPRE` writer - Microcontroller clock output prescaler"]
pub type McopreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn ppre(&self) -> PpreR {
        PpreR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> McopreR {
        McopreR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<'_, CfgrSpec> {
        HpreW::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn ppre(&mut self) -> PpreW<'_, CfgrSpec> {
        PpreW::new(self, 12)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> McoselW<'_, CfgrSpec> {
        McoselW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> McopreW<'_, CfgrSpec> {
        McopreW::new(self, 28)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
