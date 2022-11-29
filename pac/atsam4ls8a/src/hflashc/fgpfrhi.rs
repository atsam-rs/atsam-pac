#[doc = "Register `FGPFRHI` reader"]
pub struct R(crate::R<FGPFRHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGPFRHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGPFRHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGPFRHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGPFRHI` writer"]
pub struct W(crate::W<FGPFRHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGPFRHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FGPFRHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGPFRHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPF32` reader - General Purpose Fuse 32"]
pub type GPF32_R = crate::BitReader<bool>;
#[doc = "Field `GPF33` reader - General Purpose Fuse 33"]
pub type GPF33_R = crate::BitReader<bool>;
#[doc = "Field `GPF34` reader - General Purpose Fuse 34"]
pub type GPF34_R = crate::BitReader<bool>;
#[doc = "Field `GPF35` reader - General Purpose Fuse 35"]
pub type GPF35_R = crate::BitReader<bool>;
#[doc = "Field `GPF36` reader - General Purpose Fuse 36"]
pub type GPF36_R = crate::BitReader<bool>;
#[doc = "Field `GPF37` reader - General Purpose Fuse 37"]
pub type GPF37_R = crate::BitReader<bool>;
#[doc = "Field `GPF38` reader - General Purpose Fuse 38"]
pub type GPF38_R = crate::BitReader<bool>;
#[doc = "Field `GPF39` reader - General Purpose Fuse 39"]
pub type GPF39_R = crate::BitReader<bool>;
#[doc = "Field `GPF40` reader - General Purpose Fuse 40"]
pub type GPF40_R = crate::BitReader<bool>;
#[doc = "Field `GPF41` reader - General Purpose Fuse 41"]
pub type GPF41_R = crate::BitReader<bool>;
#[doc = "Field `GPF42` reader - General Purpose Fuse 42"]
pub type GPF42_R = crate::BitReader<bool>;
#[doc = "Field `GPF43` reader - General Purpose Fuse 43"]
pub type GPF43_R = crate::BitReader<bool>;
#[doc = "Field `GPF44` reader - General Purpose Fuse 44"]
pub type GPF44_R = crate::BitReader<bool>;
#[doc = "Field `GPF45` reader - General Purpose Fuse 45"]
pub type GPF45_R = crate::BitReader<bool>;
#[doc = "Field `GPF46` reader - General Purpose Fuse 46"]
pub type GPF46_R = crate::BitReader<bool>;
#[doc = "Field `GPF47` reader - General Purpose Fuse 47"]
pub type GPF47_R = crate::BitReader<bool>;
#[doc = "Field `GPF48` reader - General Purpose Fuse 48"]
pub type GPF48_R = crate::BitReader<bool>;
#[doc = "Field `GPF49` reader - General Purpose Fuse 49"]
pub type GPF49_R = crate::BitReader<bool>;
#[doc = "Field `GPF50` reader - General Purpose Fuse 50"]
pub type GPF50_R = crate::BitReader<bool>;
#[doc = "Field `GPF51` reader - General Purpose Fuse 51"]
pub type GPF51_R = crate::BitReader<bool>;
#[doc = "Field `GPF52` reader - General Purpose Fuse 52"]
pub type GPF52_R = crate::BitReader<bool>;
#[doc = "Field `GPF53` reader - General Purpose Fuse 53"]
pub type GPF53_R = crate::BitReader<bool>;
#[doc = "Field `GPF54` reader - General Purpose Fuse 54"]
pub type GPF54_R = crate::BitReader<bool>;
#[doc = "Field `GPF55` reader - General Purpose Fuse 55"]
pub type GPF55_R = crate::BitReader<bool>;
#[doc = "Field `GPF56` reader - General Purpose Fuse 56"]
pub type GPF56_R = crate::BitReader<bool>;
#[doc = "Field `GPF57` reader - General Purpose Fuse 57"]
pub type GPF57_R = crate::BitReader<bool>;
#[doc = "Field `GPF58` reader - General Purpose Fuse 58"]
pub type GPF58_R = crate::BitReader<bool>;
#[doc = "Field `GPF59` reader - General Purpose Fuse 59"]
pub type GPF59_R = crate::BitReader<bool>;
#[doc = "Field `GPF60` reader - General Purpose Fuse 60"]
pub type GPF60_R = crate::BitReader<bool>;
#[doc = "Field `GPF61` reader - General Purpose Fuse 61"]
pub type GPF61_R = crate::BitReader<bool>;
#[doc = "Field `GPF62` reader - General Purpose Fuse 62"]
pub type GPF62_R = crate::BitReader<bool>;
#[doc = "Field `GPF63` reader - General Purpose Fuse 63"]
pub type GPF63_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - General Purpose Fuse 32"]
    #[inline(always)]
    pub fn gpf32(&self) -> GPF32_R {
        GPF32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Purpose Fuse 33"]
    #[inline(always)]
    pub fn gpf33(&self) -> GPF33_R {
        GPF33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Purpose Fuse 34"]
    #[inline(always)]
    pub fn gpf34(&self) -> GPF34_R {
        GPF34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Purpose Fuse 35"]
    #[inline(always)]
    pub fn gpf35(&self) -> GPF35_R {
        GPF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Purpose Fuse 36"]
    #[inline(always)]
    pub fn gpf36(&self) -> GPF36_R {
        GPF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose Fuse 37"]
    #[inline(always)]
    pub fn gpf37(&self) -> GPF37_R {
        GPF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Purpose Fuse 38"]
    #[inline(always)]
    pub fn gpf38(&self) -> GPF38_R {
        GPF38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - General Purpose Fuse 39"]
    #[inline(always)]
    pub fn gpf39(&self) -> GPF39_R {
        GPF39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - General Purpose Fuse 40"]
    #[inline(always)]
    pub fn gpf40(&self) -> GPF40_R {
        GPF40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - General Purpose Fuse 41"]
    #[inline(always)]
    pub fn gpf41(&self) -> GPF41_R {
        GPF41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - General Purpose Fuse 42"]
    #[inline(always)]
    pub fn gpf42(&self) -> GPF42_R {
        GPF42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General Purpose Fuse 43"]
    #[inline(always)]
    pub fn gpf43(&self) -> GPF43_R {
        GPF43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - General Purpose Fuse 44"]
    #[inline(always)]
    pub fn gpf44(&self) -> GPF44_R {
        GPF44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - General Purpose Fuse 45"]
    #[inline(always)]
    pub fn gpf45(&self) -> GPF45_R {
        GPF45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - General Purpose Fuse 46"]
    #[inline(always)]
    pub fn gpf46(&self) -> GPF46_R {
        GPF46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - General Purpose Fuse 47"]
    #[inline(always)]
    pub fn gpf47(&self) -> GPF47_R {
        GPF47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose Fuse 48"]
    #[inline(always)]
    pub fn gpf48(&self) -> GPF48_R {
        GPF48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Purpose Fuse 49"]
    #[inline(always)]
    pub fn gpf49(&self) -> GPF49_R {
        GPF49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose Fuse 50"]
    #[inline(always)]
    pub fn gpf50(&self) -> GPF50_R {
        GPF50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General Purpose Fuse 51"]
    #[inline(always)]
    pub fn gpf51(&self) -> GPF51_R {
        GPF51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - General Purpose Fuse 52"]
    #[inline(always)]
    pub fn gpf52(&self) -> GPF52_R {
        GPF52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - General Purpose Fuse 53"]
    #[inline(always)]
    pub fn gpf53(&self) -> GPF53_R {
        GPF53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - General Purpose Fuse 54"]
    #[inline(always)]
    pub fn gpf54(&self) -> GPF54_R {
        GPF54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - General Purpose Fuse 55"]
    #[inline(always)]
    pub fn gpf55(&self) -> GPF55_R {
        GPF55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Purpose Fuse 56"]
    #[inline(always)]
    pub fn gpf56(&self) -> GPF56_R {
        GPF56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Purpose Fuse 57"]
    #[inline(always)]
    pub fn gpf57(&self) -> GPF57_R {
        GPF57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - General Purpose Fuse 58"]
    #[inline(always)]
    pub fn gpf58(&self) -> GPF58_R {
        GPF58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - General Purpose Fuse 59"]
    #[inline(always)]
    pub fn gpf59(&self) -> GPF59_R {
        GPF59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - General Purpose Fuse 60"]
    #[inline(always)]
    pub fn gpf60(&self) -> GPF60_R {
        GPF60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - General Purpose Fuse 61"]
    #[inline(always)]
    pub fn gpf61(&self) -> GPF61_R {
        GPF61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - General Purpose Fuse 62"]
    #[inline(always)]
    pub fn gpf62(&self) -> GPF62_R {
        GPF62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - General Purpose Fuse 63"]
    #[inline(always)]
    pub fn gpf63(&self) -> GPF63_R {
        GPF63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller General Purpose Fuse Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfrhi](index.html) module"]
pub struct FGPFRHI_SPEC;
impl crate::RegisterSpec for FGPFRHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgpfrhi::R](R) reader structure"]
impl crate::Readable for FGPFRHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgpfrhi::W](W) writer structure"]
impl crate::Writable for FGPFRHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGPFRHI to value 0"]
impl crate::Resettable for FGPFRHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
