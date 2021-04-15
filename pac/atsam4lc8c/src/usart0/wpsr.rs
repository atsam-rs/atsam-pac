#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Write Protect Violation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPV_A {
    #[doc = "0: No Write Protect Violation has occurred since the last read of the WPSR register"]
    _0 = 0,
    #[doc = "1: A Write Protect Violation has occurred since the last read of the WPSR register. If this violation is an unauthorized attempt to write a protected register, the associated violation is reported into field WPVSRC"]
    _1 = 1,
}
impl From<WPV_A> for bool {
    #[inline(always)]
    fn from(variant: WPV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPV`"]
pub type WPV_R = crate::R<bool, WPV_A>;
impl WPV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPV_A {
        match self.bits {
            false => WPV_A::_0,
            true => WPV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPV_A::_1
    }
}
#[doc = "Reader of field `WPVSRC`"]
pub type WPVSRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpv(&self) -> WPV_R {
        WPV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:23 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
