#[doc = "Register `RTOR` reader"]
pub struct R(crate::R<RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTOR_SPEC>> for R {
    fn from(reader: crate::R<RTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTOR` writer"]
pub struct W(crate::W<RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTOR_SPEC>;
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
impl core::convert::From<crate::W<RTOR_SPEC>> for W {
    fn from(writer: crate::W<RTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Time-out Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TO_A {
    #[doc = "0: Disables the RX Time-out function"]
    DISABLE = 0,
}
impl From<TO_A> for u32 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TO` reader - Time-out Value"]
pub struct TO_R(crate::FieldReader<u32, TO_A>);
impl TO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TO_A> {
        match self.bits {
            0 => Some(TO_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TO_A::DISABLE
    }
}
impl core::ops::Deref for TO_R {
    type Target = crate::FieldReader<u32, TO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO` writer - Time-out Value"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the RX Time-out function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TO_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](index.html) module"]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtor::R](R) reader structure"]
impl crate::Readable for RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtor::W](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
