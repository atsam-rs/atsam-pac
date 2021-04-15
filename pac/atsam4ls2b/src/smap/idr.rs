#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Reader of field `APIDV`"]
pub type APIDV_R = crate::R<u8, u8>;
#[doc = "Reader of field `APID`"]
pub type APID_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLSS`"]
pub type CLSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `IC`"]
pub type IC_R = crate::R<u8, u8>;
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - AP Identification Variant"]
    #[inline(always)]
    pub fn apidv(&self) -> APIDV_R {
        APIDV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AP Identification"]
    #[inline(always)]
    pub fn apid(&self) -> APID_R {
        APID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Class"]
    #[inline(always)]
    pub fn clss(&self) -> CLSS_R {
        CLSS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:23 - JEP-106 Identity Code"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
