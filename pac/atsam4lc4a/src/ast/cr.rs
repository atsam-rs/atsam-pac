#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<ENSELECT_A>;
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSELECT_A {
    #[doc = "0: The AST is disabled."]
    _0 = 0,
    #[doc = "1: The AST is enabled"]
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
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ENSELECT_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "The AST is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENSELECT_A::_0)
    }
    #[doc = "The AST is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENSELECT_A::_1)
    }
}
#[doc = "Field `PCLR` reader - Prescaler Clear"]
pub type PCLR_R = crate::BitReader<bool>;
#[doc = "Field `PCLR` writer - Prescaler Clear"]
pub type PCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CAL` reader - Calendar mode"]
pub type CAL_R = crate::BitReader<bool>;
#[doc = "Field `CAL` writer - Calendar mode"]
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CA0` reader - Clear on Alarm 0"]
pub type CA0_R = crate::BitReader<bool>;
#[doc = "Field `CA0` writer - Clear on Alarm 0"]
pub type CA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CA1` reader - Clear on Alarm 1"]
pub type CA1_R = crate::BitReader<bool>;
#[doc = "Field `CA1` writer - Clear on Alarm 1"]
pub type CA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prescaler Clear"]
    #[inline(always)]
    pub fn pclr(&self) -> PCLR_R {
        PCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Calendar mode"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear on Alarm 0"]
    #[inline(always)]
    pub fn ca0(&self) -> CA0_R {
        CA0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear on Alarm 1"]
    #[inline(always)]
    pub fn ca1(&self) -> CA1_R {
        CA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Prescaler Clear"]
    #[inline(always)]
    #[must_use]
    pub fn pclr(&mut self) -> PCLR_W<1> {
        PCLR_W::new(self)
    }
    #[doc = "Bit 2 - Calendar mode"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<2> {
        CAL_W::new(self)
    }
    #[doc = "Bit 8 - Clear on Alarm 0"]
    #[inline(always)]
    #[must_use]
    pub fn ca0(&mut self) -> CA0_W<8> {
        CA0_W::new(self)
    }
    #[doc = "Bit 9 - Clear on Alarm 1"]
    #[inline(always)]
    #[must_use]
    pub fn ca1(&mut self) -> CA1_W<9> {
        CA1_W::new(self)
    }
    #[doc = "Bits 16:20 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<16> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
