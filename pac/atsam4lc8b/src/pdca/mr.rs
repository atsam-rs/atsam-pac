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
#[doc = "Transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: `0`"]
    BYTE = 0,
    #[doc = "1: `1`"]
    HALF_WORD = 1,
    #[doc = "2: `10`"]
    WORD = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SIZE` reader - Transfer size"]
pub struct SIZE_R(crate::FieldReader<u8, SIZE_A>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::BYTE),
            1 => Some(SIZE_A::HALF_WORD),
            2 => Some(SIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        **self == SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        **self == SIZE_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == SIZE_A::WORD
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Transfer size"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZE_A::BYTE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SIZE_A::HALF_WORD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ETRIG` reader - Event trigger"]
pub struct ETRIG_R(crate::FieldReader<bool, bool>);
impl ETRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRIG` writer - Event trigger"]
pub struct ETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRIG_W<'a> {
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
#[doc = "Field `RING` reader - Ring Buffer"]
pub struct RING_R(crate::FieldReader<bool, bool>);
impl RING_R {
    pub(crate) fn new(bits: bool) -> Self {
        RING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RING` writer - Ring Buffer"]
pub struct RING_W<'a> {
    w: &'a mut W,
}
impl<'a> RING_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    pub fn etrig(&self) -> ETRIG_R {
        ETRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    pub fn ring(&self) -> RING_R {
        RING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 2 - Event trigger"]
    #[inline(always)]
    pub fn etrig(&mut self) -> ETRIG_W {
        ETRIG_W { w: self }
    }
    #[doc = "Bit 3 - Ring Buffer"]
    #[inline(always)]
    pub fn ring(&mut self) -> RING_W {
        RING_W { w: self }
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
}
#[doc = "`reset()` method sets MR%s to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
