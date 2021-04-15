#[doc = "Register `CFDCTRL` reader"]
pub struct R(crate::R<CFDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFDCTRL_SPEC>> for R {
    fn from(reader: crate::R<CFDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCTRL` writer"]
pub struct W(crate::W<CFDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCTRL_SPEC>;
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
impl core::convert::From<crate::W<CFDCTRL_SPEC>> for W {
    fn from(writer: crate::W<CFDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEN` reader - Clock Failure Detection Enable"]
pub struct CFDEN_R(crate::FieldReader<bool, bool>);
impl CFDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEN` writer - Clock Failure Detection Enable"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
#[doc = "Field `SFV` reader - Store Final Value"]
pub struct SFV_R(crate::FieldReader<bool, bool>);
impl SFV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFV` writer - Store Final Value"]
pub struct SFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&mut self) -> SFV_W {
        SFV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Failure Detector Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdctrl](index.html) module"]
pub struct CFDCTRL_SPEC;
impl crate::RegisterSpec for CFDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdctrl::R](R) reader structure"]
impl crate::Readable for CFDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdctrl::W](W) writer structure"]
impl crate::Writable for CFDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFDCTRL to value 0"]
impl crate::Resettable for CFDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
