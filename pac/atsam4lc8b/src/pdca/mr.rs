#[doc = "Register `MR%s` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR%s` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE` reader - Transfer size"]
pub type SIZE_R = crate::FieldReader<u8, SIZESELECT_A>;
#[doc = "Transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZESELECT_A {
    #[doc = "0: `0`"]
    BYTE = 0,
    #[doc = "1: `1`"]
    HALF_WORD = 1,
    #[doc = "2: `10`"]
    WORD = 2,
}
impl From<SIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZESELECT_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZESELECT_A> {
        match self.bits {
            0 => Some(SIZESELECT_A::BYTE),
            1 => Some(SIZESELECT_A::HALF_WORD),
            2 => Some(SIZESELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZESELECT_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SIZESELECT_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZESELECT_A::WORD
    }
}
#[doc = "Field `SIZE` writer - Transfer size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, SIZESELECT_A, 2, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZESELECT_A::BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SIZESELECT_A::HALF_WORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZESELECT_A::WORD)
    }
}
#[doc = "Field `ETRIG` reader - Event trigger"]
pub type ETRIG_R = crate::BitReader<bool>;
#[doc = "Field `ETRIG` writer - Event trigger"]
pub type ETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `RING` reader - Ring Buffer"]
pub type RING_R = crate::BitReader<bool>;
#[doc = "Field `RING` writer - Ring Buffer"]
pub type RING_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    pub fn etrig(&self) -> ETRIG_R {
        ETRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    pub fn ring(&self) -> RING_R {
        RING_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<0> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    #[must_use]
    pub fn etrig(&mut self) -> ETRIG_W<2> {
        ETRIG_W::new(self)
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ring(&mut self) -> RING_W<3> {
        RING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR%s to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
