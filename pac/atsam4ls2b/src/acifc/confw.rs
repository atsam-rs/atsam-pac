#[doc = "Register `CONFW%s` reader"]
pub struct R(crate::R<CONFW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFW%s` writer"]
pub struct W(crate::W<CONFW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFW_SPEC>;
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
impl From<crate::W<CONFW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIS` reader - Window Mode Interrupt Settings"]
pub type WIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIS` writer - Window Mode Interrupt Settings"]
pub type WIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFW_SPEC, u8, u8, 3, O>;
#[doc = "Field `WEVSRC` reader - Peripheral Event Sourse Selection for Window Mode"]
pub type WEVSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEVSRC` writer - Peripheral Event Sourse Selection for Window Mode"]
pub type WEVSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFW_SPEC, u8, u8, 3, O>;
#[doc = "Field `WEVEN` reader - Window Peripheral Event Enable"]
pub type WEVEN_R = crate::BitReader<bool>;
#[doc = "Field `WEVEN` writer - Window Peripheral Event Enable"]
pub type WEVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFW_SPEC, bool, O>;
#[doc = "Field `WFEN` reader - Window Mode Enable"]
pub type WFEN_R = crate::BitReader<bool>;
#[doc = "Field `WFEN` writer - Window Mode Enable"]
pub type WFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    pub fn wis(&self) -> WIS_R {
        WIS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    pub fn wevsrc(&self) -> WEVSRC_R {
        WEVSRC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    pub fn weven(&self) -> WEVEN_R {
        WEVEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Mode Interrupt Settings"]
    #[inline(always)]
    #[must_use]
    pub fn wis(&mut self) -> WIS_W<0> {
        WIS_W::new(self)
    }
    #[doc = "Bits 8:10 - Peripheral Event Sourse Selection for Window Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wevsrc(&mut self) -> WEVSRC_W<8> {
        WEVSRC_W::new(self)
    }
    #[doc = "Bit 11 - Window Peripheral Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn weven(&mut self) -> WEVEN_W<11> {
        WEVEN_W::new(self)
    }
    #[doc = "Bit 16 - Window Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfen(&mut self) -> WFEN_W<16> {
        WFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confw](index.html) module"]
pub struct CONFW_SPEC;
impl crate::RegisterSpec for CONFW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confw::R](R) reader structure"]
impl crate::Readable for CONFW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confw::W](W) writer structure"]
impl crate::Writable for CONFW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFW%s to value 0"]
impl crate::Resettable for CONFW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
