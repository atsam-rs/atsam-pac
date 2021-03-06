#[doc = "Register `TBQBAPQ[%s]` reader"]
pub struct R(crate::R<TBQBAPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBQBAPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBQBAPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBQBAPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBQBAPQ[%s]` writer"]
pub struct W(crate::W<TBQBAPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBQBAPQ_SPEC>;
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
impl From<crate::W<TBQBAPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBQBAPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBQBA` reader - Transmit Buffer Queue Base Address"]
pub struct TXBQBA_R(crate::FieldReader<u32, u32>);
impl TXBQBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXBQBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBQBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBQBA` writer - Transmit Buffer Queue Base Address"]
pub struct TXBQBA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBQBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&self) -> TXBQBA_R {
        TXBQBA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&mut self) -> TXBQBA_W {
        TXBQBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Queue Base Address Register Priority Queue (index = 1) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqbapq](index.html) module"]
pub struct TBQBAPQ_SPEC;
impl crate::RegisterSpec for TBQBAPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbqbapq::R](R) reader structure"]
impl crate::Readable for TBQBAPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbqbapq::W](W) writer structure"]
impl crate::Writable for TBQBAPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBQBAPQ[%s]
to value 0"]
impl crate::Resettable for TBQBAPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
