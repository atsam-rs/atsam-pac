#[doc = "Reader of register PBBSEL"]
pub type R = crate::R<u32, super::PBBSEL>;
#[doc = "Writer for register PBBSEL"]
pub type W = crate::W<u32, super::PBBSEL>;
#[doc = "Register PBBSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PBBSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBSEL`"]
pub type PBSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBSEL`"]
pub struct PBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PBSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PBDIV`"]
pub type PBDIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBDIV`"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PBB Clock Select"]
    #[inline(always)]
    pub fn pbsel(&self) -> PBSEL_R {
        PBSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - PBB Division Select"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PBB Clock Select"]
    #[inline(always)]
    pub fn pbsel(&mut self) -> PBSEL_W {
        PBSEL_W { w: self }
    }
    #[doc = "Bit 7 - PBB Division Select"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
}
