#[doc = "Reader of register UHCON"]
pub type R = crate::R<u32, super::UHCON>;
#[doc = "Writer for register UHCON"]
pub type W = crate::W<u32, super::UHCON>;
#[doc = "Register UHCON `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFE`"]
pub type SOFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFE`"]
pub struct SOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPDCONF`"]
pub type SPDCONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPDCONF`"]
pub struct SPDCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDCONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `TSTJ`"]
pub type TSTJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTJ`"]
pub struct TSTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTJ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSTK`"]
pub type TSTK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTK`"]
pub struct TSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - SOF Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Test J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Test K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SOF Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SOFE_W {
        SOFE_W { w: self }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bits 12:13 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Bit 16 - Test J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W {
        TSTJ_W { w: self }
    }
    #[doc = "Bit 17 - Test K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W {
        TSTK_W { w: self }
    }
}
