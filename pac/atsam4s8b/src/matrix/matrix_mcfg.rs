#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MCFG_SPEC>;
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
impl From<crate::W<MATRIX_MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type ULBT_R = crate::FieldReader<u8, ULBT_A>;
#[doc = "Undefined Length Burst Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ULBT_A {
    #[doc = "0: No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    INFINITE = 0,
    #[doc = "1: The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    SINGLE = 1,
    #[doc = "2: The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    FOUR_BEAT = 2,
    #[doc = "3: The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    EIGHT_BEAT = 3,
    #[doc = "4: The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    SIXTEEN_BEAT = 4,
}
impl From<ULBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBT_A) -> Self {
        variant as _
    }
}
impl ULBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ULBT_A> {
        match self.bits {
            0 => Some(ULBT_A::INFINITE),
            1 => Some(ULBT_A::SINGLE),
            2 => Some(ULBT_A::FOUR_BEAT),
            3 => Some(ULBT_A::EIGHT_BEAT),
            4 => Some(ULBT_A::SIXTEEN_BEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INFINITE`"]
    #[inline(always)]
    pub fn is_infinite(&self) -> bool {
        *self == ULBT_A::INFINITE
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ULBT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR_BEAT`"]
    #[inline(always)]
    pub fn is_four_beat(&self) -> bool {
        *self == ULBT_A::FOUR_BEAT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BEAT`"]
    #[inline(always)]
    pub fn is_eight_beat(&self) -> bool {
        *self == ULBT_A::EIGHT_BEAT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BEAT`"]
    #[inline(always)]
    pub fn is_sixteen_beat(&self) -> bool {
        *self == ULBT_A::SIXTEEN_BEAT
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type ULBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATRIX_MCFG_SPEC, u8, ULBT_A, 3, O>;
impl<'a, const O: u8> ULBT_W<'a, O> {
    #[doc = "No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    #[inline(always)]
    pub fn infinite(self) -> &'a mut W {
        self.variant(ULBT_A::INFINITE)
    }
    #[doc = "The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBT_A::SINGLE)
    }
    #[doc = "The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    #[inline(always)]
    pub fn four_beat(self) -> &'a mut W {
        self.variant(ULBT_A::FOUR_BEAT)
    }
    #[doc = "The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    #[inline(always)]
    pub fn eight_beat(self) -> &'a mut W {
        self.variant(ULBT_A::EIGHT_BEAT)
    }
    #[doc = "The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    #[inline(always)]
    pub fn sixteen_beat(self) -> &'a mut W {
        self.variant(ULBT_A::SIXTEEN_BEAT)
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
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](index.html) module"]
pub struct MATRIX_MCFG_SPEC;
impl crate::RegisterSpec for MATRIX_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mcfg::R](R) reader structure"]
impl crate::Readable for MATRIX_MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](W) writer structure"]
impl crate::Writable for MATRIX_MCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
