#[doc = "Register `FEATURES` reader"]
pub struct R(crate::R<FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NCS` reader - Number of Chip Selects"]
pub type NCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCONF` reader - Polarity is Configurable"]
pub type PCONF_R = crate::BitReader<bool>;
#[doc = "Field `PPNCONF` reader - Polarity is Positive if Polarity is not Configurable"]
pub type PPNCONF_R = crate::BitReader<bool>;
#[doc = "Field `PHCONF` reader - Phase is Configurable"]
pub type PHCONF_R = crate::BitReader<bool>;
#[doc = "Field `PHZNCONF` reader - Phase is Zero if Phase is not Configurable"]
pub type PHZNCONF_R = crate::BitReader<bool>;
#[doc = "Field `LENCONF` reader - Character Length is Configurable"]
pub type LENCONF_R = crate::BitReader<bool>;
#[doc = "Field `LENNCONF` reader - Character Length if not Configurable"]
pub type LENNCONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTDEC` reader - External Decoder is True"]
pub type EXTDEC_R = crate::BitReader<bool>;
#[doc = "Field `CSNAATIMPL` reader - CSNAAT Features are Implemented"]
pub type CSNAATIMPL_R = crate::BitReader<bool>;
#[doc = "Field `BRPBHSB` reader - Bridge Type is PB to HSB"]
pub type BRPBHSB_R = crate::BitReader<bool>;
#[doc = "Field `FIFORIMPL` reader - FIFO in Reception is Implemented"]
pub type FIFORIMPL_R = crate::BitReader<bool>;
#[doc = "Field `SWPIMPL` reader - Spurious Write Protection is Implemented"]
pub type SWPIMPL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Number of Chip Selects"]
    #[inline(always)]
    pub fn ncs(&self) -> NCS_R {
        NCS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Polarity is Configurable"]
    #[inline(always)]
    pub fn pconf(&self) -> PCONF_R {
        PCONF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity is Positive if Polarity is not Configurable"]
    #[inline(always)]
    pub fn ppnconf(&self) -> PPNCONF_R {
        PPNCONF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Phase is Configurable"]
    #[inline(always)]
    pub fn phconf(&self) -> PHCONF_R {
        PHCONF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Phase is Zero if Phase is not Configurable"]
    #[inline(always)]
    pub fn phznconf(&self) -> PHZNCONF_R {
        PHZNCONF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Character Length is Configurable"]
    #[inline(always)]
    pub fn lenconf(&self) -> LENCONF_R {
        LENCONF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Character Length if not Configurable"]
    #[inline(always)]
    pub fn lennconf(&self) -> LENNCONF_R {
        LENNCONF_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - External Decoder is True"]
    #[inline(always)]
    pub fn extdec(&self) -> EXTDEC_R {
        EXTDEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSNAAT Features are Implemented"]
    #[inline(always)]
    pub fn csnaatimpl(&self) -> CSNAATIMPL_R {
        CSNAATIMPL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bridge Type is PB to HSB"]
    #[inline(always)]
    pub fn brpbhsb(&self) -> BRPBHSB_R {
        BRPBHSB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FIFO in Reception is Implemented"]
    #[inline(always)]
    pub fn fiforimpl(&self) -> FIFORIMPL_R {
        FIFORIMPL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Spurious Write Protection is Implemented"]
    #[inline(always)]
    pub fn swpimpl(&self) -> SWPIMPL_R {
        SWPIMPL_R::new(((self.bits >> 20) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
