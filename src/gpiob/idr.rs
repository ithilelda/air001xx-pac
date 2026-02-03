#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `ID0` reader - Port input data (y = 0..15)"]
pub type Id0R = crate::BitReader;
#[doc = "Field `ID1` reader - Port input data (y = 0..15)"]
pub type Id1R = crate::BitReader;
#[doc = "Field `ID2` reader - Port input data (y = 0..15)"]
pub type Id2R = crate::BitReader;
#[doc = "Field `ID3` reader - Port input data (y = 0..15)"]
pub type Id3R = crate::BitReader;
#[doc = "Field `ID4` reader - Port input data (y = 0..15)"]
pub type Id4R = crate::BitReader;
#[doc = "Field `ID5` reader - Port input data (y = 0..15)"]
pub type Id5R = crate::BitReader;
#[doc = "Field `ID6` reader - Port input data (y = 0..15)"]
pub type Id6R = crate::BitReader;
#[doc = "Field `ID7` reader - Port input data (y = 0..15)"]
pub type Id7R = crate::BitReader;
#[doc = "Field `ID8` reader - Port input data (y = 0..15)"]
pub type Id8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id0(&self) -> Id0R {
        Id0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id1(&self) -> Id1R {
        Id1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id2(&self) -> Id2R {
        Id2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id3(&self) -> Id3R {
        Id3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id4(&self) -> Id4R {
        Id4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id5(&self) -> Id5R {
        Id5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id6(&self) -> Id6R {
        Id6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id7(&self) -> Id7R {
        Id7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn id8(&self) -> Id8R {
        Id8R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
