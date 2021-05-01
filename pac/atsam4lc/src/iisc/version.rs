#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Reserved. Value subject to change. No functionality associated. This is the Atmel internal version of the macrocell."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Reserved. Value subject to change. No functionality associated."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
