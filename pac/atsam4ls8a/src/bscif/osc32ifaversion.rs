#[doc = "Reader of register OSC32IFAVERSION"]
pub type R = crate::R<u32, super::OSC32IFAVERSION>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Variant nubmer"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
