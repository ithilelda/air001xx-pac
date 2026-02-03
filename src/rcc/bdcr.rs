#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BdcrSpec>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BdcrSpec>;
#[doc = "Field `LSEON` reader - LSE oscillator enable"]
pub type LseonR = crate::BitReader;
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSERDY` writer - LSE oscillator ready"]
pub type LserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LsebypR = crate::BitReader;
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSON` reader - LSE CSS enable"]
pub type LsecssonR = crate::BitReader;
#[doc = "Field `LSECSSON` writer - LSE CSS enable"]
pub type LsecssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSD` reader - LSE CSS detect"]
pub type LsecssdR = crate::BitReader;
#[doc = "Field `LSECSSD` writer - LSE CSS detect"]
pub type LsecssdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RtcselR = crate::FieldReader;
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RtcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock source enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock source enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDRST` reader - RTC domain software reset"]
pub type BdrstR = crate::BitReader;
#[doc = "Field `BDRST` writer - RTC domain software reset"]
pub type BdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable"]
pub type LscoenR = crate::BitReader;
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable"]
pub type LscoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection"]
pub type LscoselR = crate::BitReader;
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection"]
pub type LscoselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - LSE CSS enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LsecssonR {
        LsecssonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE CSS detect"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LsecssdR {
        LsecssdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RtcselR {
        RtcselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock source enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BdrstR {
        BdrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn lscoen(&self) -> LscoenR {
        LscoenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&self) -> LscoselR {
        LscoselR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<'_, BdcrSpec> {
        LseonW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&mut self) -> LserdyW<'_, BdcrSpec> {
        LserdyW::new(self, 1)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LsebypW<'_, BdcrSpec> {
        LsebypW::new(self, 2)
    }
    #[doc = "Bit 5 - LSE CSS enable"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LsecssonW<'_, BdcrSpec> {
        LsecssonW::new(self, 5)
    }
    #[doc = "Bit 6 - LSE CSS detect"]
    #[inline(always)]
    pub fn lsecssd(&mut self) -> LsecssdW<'_, BdcrSpec> {
        LsecssdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RtcselW<'_, BdcrSpec> {
        RtcselW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock source enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, BdcrSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BdrstW<'_, BdcrSpec> {
        BdrstW::new(self, 16)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable"]
    #[inline(always)]
    pub fn lscoen(&mut self) -> LscoenW<'_, BdcrSpec> {
        LscoenW::new(self, 24)
    }
    #[doc = "Bit 25 - Low-speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&mut self) -> LscoselW<'_, BdcrSpec> {
        LscoselW::new(self, 25)
    }
}
#[doc = "RTC domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdcrSpec;
impl crate::RegisterSpec for BdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BdcrSpec {}
