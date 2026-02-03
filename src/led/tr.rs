#[doc = "Register `TR` reader"]
pub type R = crate::R<TrSpec>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TrSpec>;
#[doc = "Field `T1` reader - Light on time"]
pub type T1R = crate::FieldReader;
#[doc = "Field `T1` writer - Light on time"]
pub type T1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `T2` reader - Switch time"]
pub type T2R = crate::FieldReader;
#[doc = "Field `T2` writer - Switch time"]
pub type T2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Light on time"]
    #[inline(always)]
    pub fn t1(&self) -> T1R {
        T1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Switch time"]
    #[inline(always)]
    pub fn t2(&self) -> T2R {
        T2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Light on time"]
    #[inline(always)]
    pub fn t1(&mut self) -> T1W<'_, TrSpec> {
        T1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Switch time"]
    #[inline(always)]
    pub fn t2(&mut self) -> T2W<'_, TrSpec> {
        T2W::new(self, 8)
    }
}
#[doc = "Time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrSpec;
impl crate::RegisterSpec for TrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TrSpec {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TrSpec {}
