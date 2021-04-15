#[doc = "Reader of register ASR%s"]
pub type R = crate::R<u32, super::ASR>;
#[doc = "Writer for register ASR%s"]
pub type W = crate::W<u32, super::ASR>;
#[doc = "Register ASR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::ASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AR`"]
pub type AR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AR`"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
}
