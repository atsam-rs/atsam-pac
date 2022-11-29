#[doc = "Register `MCFG%s` reader"]
pub struct R(crate::R<MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFG%s` writer"]
pub struct W(crate::W<MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFG_SPEC>;
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
impl From<crate::W<MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader<u8, ULBTSELECT_A>;
#[doc = "Undefined Length Burst Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ULBTSELECT_A {
    #[doc = "0: Infinite Length"]
    INFINITE = 0,
    #[doc = "1: Single Access"]
    SINGLE = 1,
    #[doc = "2: Four Beat Burst"]
    FOUR_BEAT = 2,
    #[doc = "3: Eight Beat Burst"]
    EIGHT_BEAT = 3,
    #[doc = "4: Sixteen Beat Burst"]
    SIXTEEN_BEAT = 4,
}
impl From<ULBTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBTSELECT_A) -> Self {
        variant as _
    }
}
impl ULBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ULBTSELECT_A> {
        match self.bits {
            0 => Some(ULBTSELECT_A::INFINITE),
            1 => Some(ULBTSELECT_A::SINGLE),
            2 => Some(ULBTSELECT_A::FOUR_BEAT),
            3 => Some(ULBTSELECT_A::EIGHT_BEAT),
            4 => Some(ULBTSELECT_A::SIXTEEN_BEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INFINITE`"]
    #[inline(always)]
    pub fn is_infinite(&self) -> bool {
        *self == ULBTSELECT_A::INFINITE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ULBTSELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR_BEAT`"]
    #[inline(always)]
    pub fn is_four_beat(&self) -> bool {
        *self == ULBTSELECT_A::FOUR_BEAT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BEAT`"]
    #[inline(always)]
    pub fn is_eight_beat(&self) -> bool {
        *self == ULBTSELECT_A::EIGHT_BEAT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BEAT`"]
    #[inline(always)]
    pub fn is_sixteen_beat(&self) -> bool {
        *self == ULBTSELECT_A::SIXTEEN_BEAT
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFG_SPEC, u8, ULBTSELECT_A, 3, O>;
impl<'a, const O: u8> ULBT_W<'a, O> {
    #[doc = "Infinite Length"]
    #[inline(always)]
    pub fn infinite(self) -> &'a mut W {
        self.variant(ULBTSELECT_A::INFINITE)
    }
    #[doc = "Single Access"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBTSELECT_A::SINGLE)
    }
    #[doc = "Four Beat Burst"]
    #[inline(always)]
    pub fn four_beat(self) -> &'a mut W {
        self.variant(ULBTSELECT_A::FOUR_BEAT)
    }
    #[doc = "Eight Beat Burst"]
    #[inline(always)]
    pub fn eight_beat(self) -> &'a mut W {
        self.variant(ULBTSELECT_A::EIGHT_BEAT)
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline(always)]
    pub fn sixteen_beat(self) -> &'a mut W {
        self.variant(ULBTSELECT_A::SIXTEEN_BEAT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> ULBT_W<0> {
        ULBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](index.html) module"]
pub struct MCFG_SPEC;
impl crate::RegisterSpec for MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfg::R](R) reader structure"]
impl crate::Readable for MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfg::W](W) writer structure"]
impl crate::Writable for MCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFG%s to value 0x02"]
impl crate::Resettable for MCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
