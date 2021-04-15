#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `EVENTEN`"]
pub type EVENTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENTEN`"]
pub struct EVENTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTEN_W<'a> {
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
#[doc = "Reader of field `USTART`"]
pub type USTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USTART`"]
pub struct USTART_W<'a> {
    w: &'a mut W,
}
impl<'a> USTART_W<'a> {
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
#[doc = "Reader of field `ESTART`"]
pub type ESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESTART`"]
pub struct ESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ACTEST`"]
pub type ACTEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTEST`"]
pub struct ACTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST_W<'a> {
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
    #[doc = "Bit 0 - ACIFC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Event Trigger Enable"]
    #[inline(always)]
    pub fn eventen(&self) -> EVENTEN_R {
        EVENTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - User Start Single Comparison"]
    #[inline(always)]
    pub fn ustart(&self) -> USTART_R {
        USTART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral Event Start Single Comparison"]
    #[inline(always)]
    pub fn estart(&self) -> ESTART_R {
        ESTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Test Mode"]
    #[inline(always)]
    pub fn actest(&self) -> ACTEST_R {
        ACTEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACIFC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Event Trigger Enable"]
    #[inline(always)]
    pub fn eventen(&mut self) -> EVENTEN_W {
        EVENTEN_W { w: self }
    }
    #[doc = "Bit 4 - User Start Single Comparison"]
    #[inline(always)]
    pub fn ustart(&mut self) -> USTART_W {
        USTART_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral Event Start Single Comparison"]
    #[inline(always)]
    pub fn estart(&mut self) -> ESTART_W {
        ESTART_W { w: self }
    }
    #[doc = "Bit 7 - Analog Comparator Test Mode"]
    #[inline(always)]
    pub fn actest(&mut self) -> ACTEST_W {
        ACTEST_W { w: self }
    }
}
