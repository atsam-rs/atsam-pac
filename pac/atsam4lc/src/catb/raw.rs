#[doc = "Reader of register RAW"]
pub type R = crate::R<u32, super::RAW>;
#[doc = "Reader of field `RAWA`"]
pub type RAWA_R = crate::R<u8, u8>;
#[doc = "Reader of field `RAWB`"]
pub type RAWB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - Current Sensor Raw Value"]
    #[inline(always)]
    pub fn rawa(&self) -> RAWA_R {
        RAWA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Last Sensor Raw Value"]
    #[inline(always)]
    pub fn rawb(&self) -> RAWB_R {
        RAWB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
