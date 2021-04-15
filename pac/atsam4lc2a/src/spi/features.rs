#[doc = "Register `FEATURES` reader"]
pub struct R(crate::R<FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FEATURES_SPEC>> for R {
    fn from(reader: crate::R<FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NCS` reader - Number of Chip Selects"]
pub struct NCS_R(crate::FieldReader<u8, u8>);
impl NCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCONF` reader - Polarity is Configurable"]
pub struct PCONF_R(crate::FieldReader<bool, bool>);
impl PCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPNCONF` reader - Polarity is Positive if Polarity is not Configurable"]
pub struct PPNCONF_R(crate::FieldReader<bool, bool>);
impl PPNCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPNCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPNCONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHCONF` reader - Phase is Configurable"]
pub struct PHCONF_R(crate::FieldReader<bool, bool>);
impl PHCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHCONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHZNCONF` reader - Phase is Zero if Phase is not Configurable"]
pub struct PHZNCONF_R(crate::FieldReader<bool, bool>);
impl PHZNCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHZNCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHZNCONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENCONF` reader - Character Length is Configurable"]
pub struct LENCONF_R(crate::FieldReader<bool, bool>);
impl LENCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LENCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENCONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENNCONF` reader - Character Length if not Configurable"]
pub struct LENNCONF_R(crate::FieldReader<u8, u8>);
impl LENNCONF_R {
    pub(crate) fn new(bits: u8) -> Self {
        LENNCONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENNCONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTDEC` reader - External Decoder is True"]
pub struct EXTDEC_R(crate::FieldReader<bool, bool>);
impl EXTDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSNAATIMPL` reader - CSNAAT Features are Implemented"]
pub struct CSNAATIMPL_R(crate::FieldReader<bool, bool>);
impl CSNAATIMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSNAATIMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSNAATIMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRPBHSB` reader - Bridge Type is PB to HSB"]
pub struct BRPBHSB_R(crate::FieldReader<bool, bool>);
impl BRPBHSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRPBHSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRPBHSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFORIMPL` reader - FIFO in Reception is Implemented"]
pub struct FIFORIMPL_R(crate::FieldReader<bool, bool>);
impl FIFORIMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFORIMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFORIMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPIMPL` reader - Spurious Write Protection is Implemented"]
pub struct SWPIMPL_R(crate::FieldReader<bool, bool>);
impl SWPIMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWPIMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWPIMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Chip Selects"]
    #[inline(always)]
    pub fn ncs(&self) -> NCS_R {
        NCS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Polarity is Configurable"]
    #[inline(always)]
    pub fn pconf(&self) -> PCONF_R {
        PCONF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Polarity is Positive if Polarity is not Configurable"]
    #[inline(always)]
    pub fn ppnconf(&self) -> PPNCONF_R {
        PPNCONF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Phase is Configurable"]
    #[inline(always)]
    pub fn phconf(&self) -> PHCONF_R {
        PHCONF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Phase is Zero if Phase is not Configurable"]
    #[inline(always)]
    pub fn phznconf(&self) -> PHZNCONF_R {
        PHZNCONF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Character Length is Configurable"]
    #[inline(always)]
    pub fn lenconf(&self) -> LENCONF_R {
        LENCONF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - Character Length if not Configurable"]
    #[inline(always)]
    pub fn lennconf(&self) -> LENNCONF_R {
        LENNCONF_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - External Decoder is True"]
    #[inline(always)]
    pub fn extdec(&self) -> EXTDEC_R {
        EXTDEC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CSNAAT Features are Implemented"]
    #[inline(always)]
    pub fn csnaatimpl(&self) -> CSNAATIMPL_R {
        CSNAATIMPL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Bridge Type is PB to HSB"]
    #[inline(always)]
    pub fn brpbhsb(&self) -> BRPBHSB_R {
        BRPBHSB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FIFO in Reception is Implemented"]
    #[inline(always)]
    pub fn fiforimpl(&self) -> FIFORIMPL_R {
        FIFORIMPL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Spurious Write Protection is Implemented"]
    #[inline(always)]
    pub fn swpimpl(&self) -> SWPIMPL_R {
        SWPIMPL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "Features Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [features](index.html) module"]
pub struct FEATURES_SPEC;
impl crate::RegisterSpec for FEATURES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [features::R](R) reader structure"]
impl crate::Readable for FEATURES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FEATURES to value 0"]
impl crate::Resettable for FEATURES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
