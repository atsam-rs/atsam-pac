#[doc = "Register `MDH1` reader"]
pub struct R(crate::R<MDH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDH1` writer"]
pub struct W(crate::W<MDH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDH1_SPEC>;
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
impl From<crate::W<MDH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDH` reader - Message Data High Value"]
pub type MDH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MDH` writer - Message Data High Value"]
pub type MDH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDH1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MDH_R {
        MDH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    #[must_use]
    pub fn mdh(&mut self) -> MDH_W<0> {
        MDH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Data High Register (MB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdh1](index.html) module"]
pub struct MDH1_SPEC;
impl crate::RegisterSpec for MDH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdh1::R](R) reader structure"]
impl crate::Readable for MDH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdh1::W](W) writer structure"]
impl crate::Writable for MDH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDH1 to value 0"]
impl crate::Resettable for MDH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
