#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `FLAG` reader - interrupt flag"]
pub type FlagR = crate::BitReader;
#[doc = "Field `FLAG` writer - interrupt flag"]
pub type FlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt flag"]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag"]
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, IrSpec> {
        FlagW::new(self, 0)
    }
}
#[doc = "Interrupt register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {}
