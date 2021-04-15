#[doc = "Reader of register IORET"]
pub type R = crate::R<u32, super::IORET>;
#[doc = "Writer for register IORET"]
pub type W = crate::W<u32, super::IORET>;
#[doc = "Register IORET `reset()`'s with value 0"]
impl crate::ResetValue for super::IORET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RET`"]
pub type RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RET`"]
pub struct RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_W<'a> {
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
    #[doc = "Bit 0 - Retention on I/O lines after waking up from the BACKUP mode"]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention on I/O lines after waking up from the BACKUP mode"]
    #[inline(always)]
    pub fn ret(&mut self) -> RET_W {
        RET_W { w: self }
    }
}
