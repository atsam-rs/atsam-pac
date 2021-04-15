#[doc = "Reader of register CPUMASK"]
pub type R = crate::R<u32, super::CPUMASK>;
#[doc = "Writer for register CPUMASK"]
pub type W = crate::W<u32, super::CPUMASK>;
#[doc = "Register CPUMASK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CPUMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `OCD_`"]
pub type OCD__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCD_`"]
pub struct OCD__W<'a> {
    w: &'a mut W,
}
impl<'a> OCD__W<'a> {
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
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    pub fn ocd_(&self) -> OCD__R {
        OCD__R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    pub fn ocd_(&mut self) -> OCD__W {
        OCD__W { w: self }
    }
}
