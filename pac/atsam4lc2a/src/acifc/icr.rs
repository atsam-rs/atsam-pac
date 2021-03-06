#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACINT0` writer - AC0 Interrupt Status Clear"]
pub struct ACINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SUTINT0` writer - AC0 Startup Time Interrupt Status Clear"]
pub struct SUTINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ACINT1` writer - AC1 Interrupt Status Clear"]
pub struct ACINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SUTINT1` writer - AC1 Startup Time Interrupt Status Clear"]
pub struct SUTINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ACINT2` writer - AC2 Interrupt Status Clear"]
pub struct ACINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SUTINT2` writer - AC2 Startup Time Interrupt Status Clear"]
pub struct SUTINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ACINT3` writer - AC3 Interrupt Status Clear"]
pub struct ACINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SUTINT3` writer - AC3 Startup Time Interrupt Status Clear"]
pub struct SUTINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ACINT4` writer - AC4 Interrupt Status Clear"]
pub struct ACINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SUTINT4` writer - AC4 Startup Time Interrupt Status Clear"]
pub struct SUTINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ACINT5` writer - AC5 Interrupt Status Clear"]
pub struct ACINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SUTINT5` writer - AC5 Startup Time Interrupt Status Clear"]
pub struct SUTINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ACINT6` writer - AC6 Interrupt Status Clear"]
pub struct ACINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SUTINT6` writer - AC6 Startup Time Interrupt Status Clear"]
pub struct SUTINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ACINT7` writer - AC7 Interrupt Status Clear"]
pub struct ACINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SUTINT7` writer - AC7 Startup Time Interrupt Status Clear"]
pub struct SUTINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `WFINT0` writer - Window0 Mode Interrupt Status Clear"]
pub struct WFINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `WFINT1` writer - Window1 Mode Interrupt Status Clear"]
pub struct WFINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `WFINT2` writer - Window2 Mode Interrupt Status Clear"]
pub struct WFINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `WFINT3` writer - Window3 Mode Interrupt Status Clear"]
pub struct WFINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - AC0 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint0(&mut self) -> ACINT0_W {
        ACINT0_W { w: self }
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint0(&mut self) -> SUTINT0_W {
        SUTINT0_W { w: self }
    }
    #[doc = "Bit 2 - AC1 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint1(&mut self) -> ACINT1_W {
        ACINT1_W { w: self }
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint1(&mut self) -> SUTINT1_W {
        SUTINT1_W { w: self }
    }
    #[doc = "Bit 4 - AC2 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint2(&mut self) -> ACINT2_W {
        ACINT2_W { w: self }
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint2(&mut self) -> SUTINT2_W {
        SUTINT2_W { w: self }
    }
    #[doc = "Bit 6 - AC3 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint3(&mut self) -> ACINT3_W {
        ACINT3_W { w: self }
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint3(&mut self) -> SUTINT3_W {
        SUTINT3_W { w: self }
    }
    #[doc = "Bit 8 - AC4 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint4(&mut self) -> ACINT4_W {
        ACINT4_W { w: self }
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint4(&mut self) -> SUTINT4_W {
        SUTINT4_W { w: self }
    }
    #[doc = "Bit 10 - AC5 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint5(&mut self) -> ACINT5_W {
        ACINT5_W { w: self }
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint5(&mut self) -> SUTINT5_W {
        SUTINT5_W { w: self }
    }
    #[doc = "Bit 12 - AC6 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint6(&mut self) -> ACINT6_W {
        ACINT6_W { w: self }
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint6(&mut self) -> SUTINT6_W {
        SUTINT6_W { w: self }
    }
    #[doc = "Bit 14 - AC7 Interrupt Status Clear"]
    #[inline(always)]
    pub fn acint7(&mut self) -> ACINT7_W {
        ACINT7_W { w: self }
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Status Clear"]
    #[inline(always)]
    pub fn sutint7(&mut self) -> SUTINT7_W {
        SUTINT7_W { w: self }
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Status Clear"]
    #[inline(always)]
    pub fn wfint0(&mut self) -> WFINT0_W {
        WFINT0_W { w: self }
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Status Clear"]
    #[inline(always)]
    pub fn wfint1(&mut self) -> WFINT1_W {
        WFINT1_W { w: self }
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Status Clear"]
    #[inline(always)]
    pub fn wfint2(&mut self) -> WFINT2_W {
        WFINT2_W { w: self }
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Status Clear"]
    #[inline(always)]
    pub fn wfint3(&mut self) -> WFINT3_W {
        WFINT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
