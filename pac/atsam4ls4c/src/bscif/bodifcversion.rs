#[doc = "Reader of register BODIFCVERSION"]
pub type R = crate::R<u32, super::BODIFCVERSION>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version Number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Variant Number"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
