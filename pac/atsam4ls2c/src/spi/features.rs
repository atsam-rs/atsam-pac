#[doc = "Reader of register FEATURES"]
pub type R = crate::R<u32, super::FEATURES>;
#[doc = "Reader of field `NCS`"]
pub type NCS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PCONF`"]
pub type PCONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PPNCONF`"]
pub type PPNCONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHCONF`"]
pub type PHCONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PHZNCONF`"]
pub type PHZNCONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LENCONF`"]
pub type LENCONF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LENNCONF`"]
pub type LENNCONF_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTDEC`"]
pub type EXTDEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSNAATIMPL`"]
pub type CSNAATIMPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRPBHSB`"]
pub type BRPBHSB_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFORIMPL`"]
pub type FIFORIMPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWPIMPL`"]
pub type SWPIMPL_R = crate::R<bool, bool>;
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
