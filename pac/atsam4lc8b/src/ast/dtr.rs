#[doc = "Register `DTR` reader"]
pub struct R(crate::R<DTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTR` writer"]
pub struct W(crate::W<DTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR_SPEC>;
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
impl From<crate::W<DTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXP` reader - EXP"]
pub type EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXP` writer - EXP"]
pub type EXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `ADD` reader - ADD"]
pub type ADD_R = crate::BitReader<bool>;
#[doc = "Field `ADD` writer - ADD"]
pub type ADD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR_SPEC, bool, O>;
#[doc = "Field `VALUE` reader - VALUE"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - VALUE"]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    #[must_use]
    pub fn exp(&mut self) -> EXP_W<0> {
        EXP_W::new(self)
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<5> {
        ADD_W::new(self)
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<8> {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Tuner Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr](index.html) module"]
pub struct DTR_SPEC;
impl crate::RegisterSpec for DTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtr::R](R) reader structure"]
impl crate::Readable for DTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtr::W](W) writer structure"]
impl crate::Writable for DTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
