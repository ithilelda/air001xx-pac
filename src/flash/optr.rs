#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OptrSpec>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OptrSpec>;
#[doc = "Field `RDP` reader - Read Protection"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Read Protection"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BOREN` reader - BOR reset Level"]
pub type BorenR = crate::BitReader;
#[doc = "Field `BOREN` writer - BOR reset Level"]
pub type BorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORF_LEV` reader - These bits contain the VDD supply level threshold that activates the reset"]
pub type BorfLevR = crate::FieldReader;
#[doc = "Field `BORF_LEV` writer - These bits contain the VDD supply level threshold that activates the reset"]
pub type BorfLevW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IdwgSwR = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IdwgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WwdgSwR = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WwdgSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_MODE` reader - NRST_MODE"]
pub type NrstModeR = crate::BitReader;
#[doc = "Field `NRST_MODE` writer - NRST_MODE"]
pub type NrstModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type NBoot1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&self) -> BorenR {
        BorenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&self) -> BorfLevR {
        BorfLevR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IdwgSwR {
        IdwgSwR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WwdgSwR {
        WwdgSwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&self) -> NrstModeR {
        NrstModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Protection"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RdpW<'_, OptrSpec> {
        RdpW::new(self, 0)
    }
    #[doc = "Bit 8 - BOR reset Level"]
    #[inline(always)]
    pub fn boren(&mut self) -> BorenW<'_, OptrSpec> {
        BorenW::new(self, 8)
    }
    #[doc = "Bits 9:11 - These bits contain the VDD supply level threshold that activates the reset"]
    #[inline(always)]
    pub fn borf_lev(&mut self) -> BorfLevW<'_, OptrSpec> {
        BorfLevW::new(self, 9)
    }
    #[doc = "Bit 12 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IdwgSwW<'_, OptrSpec> {
        IdwgSwW::new(self, 12)
    }
    #[doc = "Bit 13 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WwdgSwW<'_, OptrSpec> {
        WwdgSwW::new(self, 13)
    }
    #[doc = "Bit 14 - NRST_MODE"]
    #[inline(always)]
    pub fn nrst_mode(&mut self) -> NrstModeW<'_, OptrSpec> {
        NrstModeW::new(self, 14)
    }
    #[doc = "Bit 15 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBoot1W<'_, OptrSpec> {
        NBoot1W::new(self, 15)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptrSpec;
impl crate::RegisterSpec for OptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OptrSpec {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTR to value 0x4f55_b0aa"]
impl crate::Resettable for OptrSpec {
    const RESET_VALUE: u32 = 0x4f55_b0aa;
}
