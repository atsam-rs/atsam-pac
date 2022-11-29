#[doc = "Register `WPSR` reader"]
pub struct R(crate::R<WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPV` reader - Write Protect Violation Status"]
pub type WPV_R = crate::BitReader<WPVSELECT_A>;
#[doc = "Write Protect Violation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPVSELECT_A {
    #[doc = "0: No Write Protect Violation has occurred since the last read of the WPSR register"]
    _0 = 0,
    #[doc = "1: A Write Protect Violation has occurred since the last read of the WPSR register. If this violation is an unauthorized attempt to write a protected register, the associated violation is reported into field WPVSRC"]
    _1 = 1,
}
impl From<WPVSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WPVSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WPV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPVSELECT_A {
        match self.bits {
            false => WPVSELECT_A::_0,
            true => WPVSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPVSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPVSELECT_A::_1
    }
}
#[doc = "Field `WPVSRC` reader - Write Protect Violation Source"]
pub type WPVSRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpv(&self) -> WPV_R {
        WPV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
