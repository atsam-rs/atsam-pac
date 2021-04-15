#[doc = "Register `MCCTRL` reader"]
pub struct R(crate::R<MCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MCCTRL_SPEC>> for R {
    fn from(reader: crate::R<MCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCCTRL` writer"]
pub struct W(crate::W<MCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCCTRL_SPEC>;
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
impl core::convert::From<crate::W<MCCTRL_SPEC>> for W {
    fn from(writer: crate::W<MCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCSEL` reader - Main Clock Select"]
pub struct MCSEL_R(crate::FieldReader<u8, u8>);
impl MCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCSEL` writer - Main Clock Select"]
pub struct MCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline(always)]
    pub fn mcsel(&mut self) -> MCSEL_W {
        MCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcctrl](index.html) module"]
pub struct MCCTRL_SPEC;
impl crate::RegisterSpec for MCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcctrl::R](R) reader structure"]
impl crate::Readable for MCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcctrl::W](W) writer structure"]
impl crate::Writable for MCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCCTRL to value 0"]
impl crate::Resettable for MCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
