#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Write Protection Violation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WPVS_A {
    #[doc = "1: The Write Protection has blocked a Write access to a protected register (since the last read)."]
    WRITE_WITH_WP = 1,
    #[doc = "2: Software Reset has been performed while Write Protection was enabled (since the last read or since the last write access on MR, IER, IDR or CSRx)."]
    SWRST_WITH_WP = 2,
    #[doc = "4: Write accesses have been detected on MR (while a chip select was active) or on CSRi (while the Chip Select \"i\" was active) since the last read."]
    UNEXPECTED_WRITE = 4,
}
impl From<WPVS_A> for u8 {
    #[inline(always)]
    fn from(variant: WPVS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WPVS`"]
pub type WPVS_R = crate::R<u8, WPVS_A>;
impl WPVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WPVS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(WPVS_A::WRITE_WITH_WP),
            2 => Val(WPVS_A::SWRST_WITH_WP),
            4 => Val(WPVS_A::UNEXPECTED_WRITE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITE_WITH_WP`"]
    #[inline(always)]
    pub fn is_write_with_wp(&self) -> bool {
        *self == WPVS_A::WRITE_WITH_WP
    }
    #[doc = "Checks if the value of the field is `SWRST_WITH_WP`"]
    #[inline(always)]
    pub fn is_swrst_with_wp(&self) -> bool {
        *self == WPVS_A::SWRST_WITH_WP
    }
    #[doc = "Checks if the value of the field is `UNEXPECTED_WRITE`"]
    #[inline(always)]
    pub fn is_unexpected_write(&self) -> bool {
        *self == WPVS_A::UNEXPECTED_WRITE
    }
}
#[doc = "Reader of field `WPVSRC`"]
pub type WPVSRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
