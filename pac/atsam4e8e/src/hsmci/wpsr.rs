#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Write Protection Violation Status"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WP_VS_A {
    #[doc = "0: No Write Protection Violation occurred since the last read of this register (WP_SR)"]
    NONE = 0,
    #[doc = "1: Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    WRITE = 1,
    #[doc = "2: Software reset had been performed while Write Protection was enabled (since the last read)."]
    RESET = 2,
    #[doc = "3: Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    BOTH = 3,
}
impl From<WP_VS_A> for u8 {
    #[inline(always)]
    fn from(variant: WP_VS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WP_VS`"]
pub type WP_VS_R = crate::R<u8, WP_VS_A>;
impl WP_VS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WP_VS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WP_VS_A::NONE),
            1 => Val(WP_VS_A::WRITE),
            2 => Val(WP_VS_A::RESET),
            3 => Val(WP_VS_A::BOTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WP_VS_A::NONE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == WP_VS_A::WRITE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WP_VS_A::RESET
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == WP_VS_A::BOTH
    }
}
#[doc = "Reader of field `WP_VSRC`"]
pub type WP_VSRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wp_vs(&self) -> WP_VS_R {
        WP_VS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline(always)]
    pub fn wp_vsrc(&self) -> WP_VSRC_R {
        WP_VSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
