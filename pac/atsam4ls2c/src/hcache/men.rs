#[doc = "Register `MEN` writer"]
pub struct W(crate::W<MEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEN_SPEC>;
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
impl From<crate::W<MEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENABLE_AW {
    #[doc = "0: Disable Monitor Counter"]
    DIS = 0,
    #[doc = "1: Enable Monitor Counter"]
    EN = 1,
}
impl From<MENABLE_AW> for bool {
    #[inline(always)]
    fn from(variant: MENABLE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MENABLE` writer - Monitor Enable"]
pub struct MENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MENABLE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Monitor Counter"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MENABLE_AW::DIS)
    }
    #[doc = "Enable Monitor Counter"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MENABLE_AW::EN)
    }
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
impl W {
    #[doc = "Bit 0 - Monitor Enable"]
    #[inline(always)]
    pub fn menable(&mut self) -> MENABLE_W {
        MENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Monitor Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [men](index.html) module"]
pub struct MEN_SPEC;
impl crate::RegisterSpec for MEN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [men::W](W) writer structure"]
impl crate::Writable for MEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEN to value 0"]
impl crate::Resettable for MEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
