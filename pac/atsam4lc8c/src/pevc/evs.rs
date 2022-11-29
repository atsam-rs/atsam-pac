#[doc = "Register `EVS%s` reader"]
pub struct R(crate::R<EVS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVS%s` writer"]
pub struct W(crate::W<EVS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVS_SPEC>;
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
impl From<crate::W<EVS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Event Shaper Enable"]
pub type EN_R = crate::BitReader<ENSELECT_A>;
#[doc = "Event Shaper Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSELECT_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Event Shaper enable"]
    _1 = 1,
}
impl From<ENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENSELECT_A {
        match self.bits {
            false => ENSELECT_A::_0,
            true => ENSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENSELECT_A::_1
    }
}
#[doc = "Field `EN` writer - Event Shaper Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVS_SPEC, ENSELECT_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENSELECT_A::_0)
    }
    #[doc = "Event Shaper enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENSELECT_A::_1)
    }
}
#[doc = "Field `IGFR` reader - Input Glitch Filter Rise"]
pub type IGFR_R = crate::BitReader<IGFRSELECT_A>;
#[doc = "Input Glitch Filter Rise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IGFRSELECT_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a rising edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: IGFRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl IGFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFRSELECT_A {
        match self.bits {
            false => IGFRSELECT_A::_0,
            true => IGFRSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGFRSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGFRSELECT_A::_1
    }
}
#[doc = "Field `IGFR` writer - Input Glitch Filter Rise"]
pub type IGFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVS_SPEC, IGFRSELECT_A, O>;
impl<'a, const O: u8> IGFR_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFRSELECT_A::_0)
    }
    #[doc = "Input Glitch Filter : a rising edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFRSELECT_A::_1)
    }
}
#[doc = "Field `IGFF` reader - Input Glitch Filter Fall"]
pub type IGFF_R = crate::BitReader<IGFFSELECT_A>;
#[doc = "Input Glitch Filter Fall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IGFFSELECT_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a falling edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: IGFFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl IGFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFFSELECT_A {
        match self.bits {
            false => IGFFSELECT_A::_0,
            true => IGFFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGFFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGFFSELECT_A::_1
    }
}
#[doc = "Field `IGFF` writer - Input Glitch Filter Fall"]
pub type IGFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVS_SPEC, IGFFSELECT_A, O>;
impl<'a, const O: u8> IGFF_W<'a, O> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFFSELECT_A::_0)
    }
    #[doc = "Input Glitch Filter : a falling edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFFSELECT_A::_1)
    }
}
#[doc = "Field `IGFON` reader - Input Glitch Filter Status"]
pub type IGFON_R = crate::BitReader<bool>;
#[doc = "Field `IGFON` writer - Input Glitch Filter Status"]
pub type IGFON_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    pub fn igfr(&self) -> IGFR_R {
        IGFR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    pub fn igff(&self) -> IGFF_R {
        IGFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    pub fn igfon(&self) -> IGFON_R {
        IGFON_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    #[must_use]
    pub fn igfr(&mut self) -> IGFR_W<16> {
        IGFR_W::new(self)
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    #[must_use]
    pub fn igff(&mut self) -> IGFF_W<17> {
        IGFF_W::new(self)
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    #[must_use]
    pub fn igfon(&mut self) -> IGFON_W<18> {
        IGFON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Shaper\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evs](index.html) module"]
pub struct EVS_SPEC;
impl crate::RegisterSpec for EVS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evs::R](R) reader structure"]
impl crate::Readable for EVS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evs::W](W) writer structure"]
impl crate::Writable for EVS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVS%s to value 0"]
impl crate::Resettable for EVS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
