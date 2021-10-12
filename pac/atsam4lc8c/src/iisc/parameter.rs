#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAMETER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Data protocol format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: I2S format, stereo with IWS low for left channel"]
    I2S = 0,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORMAT` reader - Data protocol format"]
pub struct FORMAT_R(crate::FieldReader<bool, FORMAT_A>);
impl FORMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            false => Some(FORMAT_A::I2S),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == FORMAT_A::I2S
    }
}
impl core::ops::Deref for FORMAT_R {
    type Target = crate::FieldReader<bool, FORMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBCHAN` reader - Maximum number of channels - 1"]
pub struct NBCHAN_R(crate::FieldReader<u8, u8>);
impl NBCHAN_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBCHAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBCHAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Data protocol format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Maximum number of channels - 1"]
    #[inline(always)]
    pub fn nbchan(&self) -> NBCHAN_R {
        NBCHAN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](index.html) module"]
pub struct PARAMETER_SPEC;
impl crate::RegisterSpec for PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parameter::R](R) reader structure"]
impl crate::Readable for PARAMETER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAMETER to value 0x0001_0000"]
impl crate::Resettable for PARAMETER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
