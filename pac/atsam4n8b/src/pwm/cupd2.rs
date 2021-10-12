#[doc = "Register `CUPD2` writer"]
pub struct W(crate::W<CUPD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUPD2_SPEC>;
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
impl From<crate::W<CUPD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUPD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUPD` writer - "]
pub struct CUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cupd(&mut self) -> CUPD_W {
        CUPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd2](index.html) module"]
pub struct CUPD2_SPEC;
impl crate::RegisterSpec for CUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cupd2::W](W) writer structure"]
impl crate::Writable for CUPD2_SPEC {
    type Writer = W;
}
