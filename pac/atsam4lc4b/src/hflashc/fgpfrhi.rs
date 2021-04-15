#[doc = "Reader of register FGPFRHI"]
pub type R = crate::R<u32, super::FGPFRHI>;
#[doc = "Reader of field `GPF32`"]
pub type GPF32_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF33`"]
pub type GPF33_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF34`"]
pub type GPF34_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF35`"]
pub type GPF35_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF36`"]
pub type GPF36_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF37`"]
pub type GPF37_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF38`"]
pub type GPF38_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF39`"]
pub type GPF39_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF40`"]
pub type GPF40_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF41`"]
pub type GPF41_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF42`"]
pub type GPF42_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF43`"]
pub type GPF43_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF44`"]
pub type GPF44_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF45`"]
pub type GPF45_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF46`"]
pub type GPF46_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF47`"]
pub type GPF47_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF48`"]
pub type GPF48_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF49`"]
pub type GPF49_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF50`"]
pub type GPF50_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF51`"]
pub type GPF51_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF52`"]
pub type GPF52_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF53`"]
pub type GPF53_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF54`"]
pub type GPF54_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF55`"]
pub type GPF55_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF56`"]
pub type GPF56_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF57`"]
pub type GPF57_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF58`"]
pub type GPF58_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF59`"]
pub type GPF59_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF60`"]
pub type GPF60_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF61`"]
pub type GPF61_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF62`"]
pub type GPF62_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF63`"]
pub type GPF63_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - General Purpose Fuse 32"]
    #[inline(always)]
    pub fn gpf32(&self) -> GPF32_R {
        GPF32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - General Purpose Fuse 33"]
    #[inline(always)]
    pub fn gpf33(&self) -> GPF33_R {
        GPF33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - General Purpose Fuse 34"]
    #[inline(always)]
    pub fn gpf34(&self) -> GPF34_R {
        GPF34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - General Purpose Fuse 35"]
    #[inline(always)]
    pub fn gpf35(&self) -> GPF35_R {
        GPF35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General Purpose Fuse 36"]
    #[inline(always)]
    pub fn gpf36(&self) -> GPF36_R {
        GPF36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Purpose Fuse 37"]
    #[inline(always)]
    pub fn gpf37(&self) -> GPF37_R {
        GPF37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - General Purpose Fuse 38"]
    #[inline(always)]
    pub fn gpf38(&self) -> GPF38_R {
        GPF38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - General Purpose Fuse 39"]
    #[inline(always)]
    pub fn gpf39(&self) -> GPF39_R {
        GPF39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - General Purpose Fuse 40"]
    #[inline(always)]
    pub fn gpf40(&self) -> GPF40_R {
        GPF40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - General Purpose Fuse 41"]
    #[inline(always)]
    pub fn gpf41(&self) -> GPF41_R {
        GPF41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - General Purpose Fuse 42"]
    #[inline(always)]
    pub fn gpf42(&self) -> GPF42_R {
        GPF42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - General Purpose Fuse 43"]
    #[inline(always)]
    pub fn gpf43(&self) -> GPF43_R {
        GPF43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - General Purpose Fuse 44"]
    #[inline(always)]
    pub fn gpf44(&self) -> GPF44_R {
        GPF44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - General Purpose Fuse 45"]
    #[inline(always)]
    pub fn gpf45(&self) -> GPF45_R {
        GPF45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - General Purpose Fuse 46"]
    #[inline(always)]
    pub fn gpf46(&self) -> GPF46_R {
        GPF46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - General Purpose Fuse 47"]
    #[inline(always)]
    pub fn gpf47(&self) -> GPF47_R {
        GPF47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - General Purpose Fuse 48"]
    #[inline(always)]
    pub fn gpf48(&self) -> GPF48_R {
        GPF48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - General Purpose Fuse 49"]
    #[inline(always)]
    pub fn gpf49(&self) -> GPF49_R {
        GPF49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - General Purpose Fuse 50"]
    #[inline(always)]
    pub fn gpf50(&self) -> GPF50_R {
        GPF50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - General Purpose Fuse 51"]
    #[inline(always)]
    pub fn gpf51(&self) -> GPF51_R {
        GPF51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - General Purpose Fuse 52"]
    #[inline(always)]
    pub fn gpf52(&self) -> GPF52_R {
        GPF52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - General Purpose Fuse 53"]
    #[inline(always)]
    pub fn gpf53(&self) -> GPF53_R {
        GPF53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - General Purpose Fuse 54"]
    #[inline(always)]
    pub fn gpf54(&self) -> GPF54_R {
        GPF54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - General Purpose Fuse 55"]
    #[inline(always)]
    pub fn gpf55(&self) -> GPF55_R {
        GPF55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General Purpose Fuse 56"]
    #[inline(always)]
    pub fn gpf56(&self) -> GPF56_R {
        GPF56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Purpose Fuse 57"]
    #[inline(always)]
    pub fn gpf57(&self) -> GPF57_R {
        GPF57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - General Purpose Fuse 58"]
    #[inline(always)]
    pub fn gpf58(&self) -> GPF58_R {
        GPF58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - General Purpose Fuse 59"]
    #[inline(always)]
    pub fn gpf59(&self) -> GPF59_R {
        GPF59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - General Purpose Fuse 60"]
    #[inline(always)]
    pub fn gpf60(&self) -> GPF60_R {
        GPF60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - General Purpose Fuse 61"]
    #[inline(always)]
    pub fn gpf61(&self) -> GPF61_R {
        GPF61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - General Purpose Fuse 62"]
    #[inline(always)]
    pub fn gpf62(&self) -> GPF62_R {
        GPF62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - General Purpose Fuse 63"]
    #[inline(always)]
    pub fn gpf63(&self) -> GPF63_R {
        GPF63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
