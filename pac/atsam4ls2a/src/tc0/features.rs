#[doc = "Reader of register FEATURES"]
pub type R = crate::R<u32, super::FEATURES>;
#[doc = "Reader of field `CTRSIZE`"]
pub type CTRSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `UPDNIMPL`"]
pub type UPDNIMPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRPBHSB`"]
pub type BRPBHSB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - Counter Size"]
    #[inline(always)]
    pub fn ctrsize(&self) -> CTRSIZE_R {
        CTRSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Up Down is Implemented"]
    #[inline(always)]
    pub fn updnimpl(&self) -> UPDNIMPL_R {
        UPDNIMPL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bridge Type is PB to HSB"]
    #[inline(always)]
    pub fn brpbhsb(&self) -> BRPBHSB_R {
        BRPBHSB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
