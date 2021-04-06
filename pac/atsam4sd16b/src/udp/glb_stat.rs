#[doc = "Reader of register GLB_STAT"]
pub type R = crate::R<u32, super::GLB_STAT>;
#[doc = "Writer for register GLB_STAT"]
pub type W = crate::W<u32, super::GLB_STAT>;
#[doc = "Register GLB_STAT `reset()`'s with value 0x10"]
impl crate::ResetValue for super::GLB_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `FADDEN`"]
pub type FADDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FADDEN`"]
pub struct FADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CONFG`"]
pub type CONFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONFG`"]
pub struct CONFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ESR`"]
pub type ESR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESR`"]
pub struct ESR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RSMINPR`"]
pub type RSMINPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSMINPR`"]
pub struct RSMINPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMINPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RMWUPE`"]
pub type RMWUPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMWUPE`"]
pub struct RMWUPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMWUPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&self) -> FADDEN_R {
        FADDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&self) -> CONFG_R {
        CONFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&self) -> ESR_R {
        ESR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&self) -> RSMINPR_R {
        RSMINPR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&self) -> RMWUPE_R {
        RMWUPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Function Address Enable"]
    #[inline(always)]
    pub fn fadden(&mut self) -> FADDEN_W {
        FADDEN_W { w: self }
    }
    #[doc = "Bit 1 - Configured"]
    #[inline(always)]
    pub fn confg(&mut self) -> CONFG_W {
        CONFG_W { w: self }
    }
    #[doc = "Bit 2 - Enable Send Resume"]
    #[inline(always)]
    pub fn esr(&mut self) -> ESR_W {
        ESR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsminpr(&mut self) -> RSMINPR_W {
        RSMINPR_W { w: self }
    }
    #[doc = "Bit 4 - Remote Wake Up Enable"]
    #[inline(always)]
    pub fn rmwupe(&mut self) -> RMWUPE_W {
        RMWUPE_W { w: self }
    }
}
