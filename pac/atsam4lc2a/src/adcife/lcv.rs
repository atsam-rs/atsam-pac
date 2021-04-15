#[doc = "Reader of register LCV"]
pub type R = crate::R<u32, super::LCV>;
#[doc = "Reader of field `LCV`"]
pub type LCV_R = crate::R<u16, u16>;
#[doc = "Reader of field `LCPC`"]
pub type LCPC_R = crate::R<u8, u8>;
#[doc = "Reader of field `LCNC`"]
pub type LCNC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Last converted value"]
    #[inline(always)]
    pub fn lcv(&self) -> LCV_R {
        LCV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Last converted positive channel"]
    #[inline(always)]
    pub fn lcpc(&self) -> LCPC_R {
        LCPC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Last converted negative channel"]
    #[inline(always)]
    pub fn lcnc(&self) -> LCNC_R {
        LCNC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
