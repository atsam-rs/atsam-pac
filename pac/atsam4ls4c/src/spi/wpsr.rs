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
#[doc = "Write Protection Violation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WPVS_A {
    #[doc = "1: The Write Protection has blocked a Write access to a protected register (since the last read)."]
    WRITE_WITH_WP = 1,
    #[doc = "2: Software Reset has been performed while Write Protection was enabled (since the last read or since the last write access on MR, IER, IDR or CSRx)."]
    SWRST_WITH_WP = 2,
    #[doc = "4: Write accesses have been detected on MR (while a chip select was active) or on CSRi (while the Chip Select \"i\" was active) since the last read."]
    UNEXPECTED_WRITE = 4,
}
impl From<WPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: WPVS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub struct WPVS_R(crate::FieldReader<u8, WPVS_A>);
impl WPVS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WPVS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPVS_A> {
        match self.bits {
            1 => Some(WPVS_A::WRITE_WITH_WP),
            2 => Some(WPVS_A::SWRST_WITH_WP),
            4 => Some(WPVS_A::UNEXPECTED_WRITE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_WITH_WP`"]
    #[inline(always)]
    pub fn is_write_with_wp(&self) -> bool {
        **self == WPVS_A::WRITE_WITH_WP
    }
    #[doc = "Checks if the value of the field is `SWRST_WITH_WP`"]
    #[inline(always)]
    pub fn is_swrst_with_wp(&self) -> bool {
        **self == WPVS_A::SWRST_WITH_WP
    }
    #[doc = "Checks if the value of the field is `UNEXPECTED_WRITE`"]
    #[inline(always)]
    pub fn is_unexpected_write(&self) -> bool {
        **self == WPVS_A::UNEXPECTED_WRITE
    }
}
impl core::ops::Deref for WPVS_R {
    type Target = crate::FieldReader<u8, WPVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub struct WPVSRC_R(crate::FieldReader<u8, u8>);
impl WPVSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WPVSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPVSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Write Protection status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
