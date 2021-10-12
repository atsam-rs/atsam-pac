#[doc = "Register `DFLL0RATIO` reader"]
pub struct R(crate::R<DFLL0RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RATIODIFF` reader - Multiplication Ratio Difference"]
pub struct RATIODIFF_R(crate::FieldReader<u16, u16>);
impl RATIODIFF_R {
    pub(crate) fn new(bits: u16) -> Self {
        RATIODIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RATIODIFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Multiplication Ratio Difference"]
    #[inline(always)]
    pub fn ratiodiff(&self) -> RATIODIFF_R {
        RATIODIFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFLL0 Ratio Registe\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0ratio](index.html) module"]
pub struct DFLL0RATIO_SPEC;
impl crate::RegisterSpec for DFLL0RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0ratio::R](R) reader structure"]
impl crate::Readable for DFLL0RATIO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFLL0RATIO to value 0"]
impl crate::Resettable for DFLL0RATIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
