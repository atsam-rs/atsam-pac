#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Data protocol format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: I2S format, stereo with IWS low for left channel"]
    I2S = 0,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORMAT`"]
pub type FORMAT_R = crate::R<bool, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FORMAT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(FORMAT_A::I2S),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == FORMAT_A::I2S
    }
}
#[doc = "Reader of field `NBCHAN`"]
pub type NBCHAN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 7 - Data protocol format"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Maximum number of channels - 1"]
    #[inline(always)]
    pub fn nbchan(&self) -> NBCHAN_R {
        NBCHAN_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
