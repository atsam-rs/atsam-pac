#[doc = "Register `WER` reader"]
pub struct R(crate::R<WER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WER` writer"]
pub struct W(crate::W<WER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WER_SPEC>;
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
impl From<crate::W<WER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub type OVF_R = crate::BitReader<OVFSELECT_A>;
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSELECT_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<OVFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFSELECT_A {
        match self.bits {
            false => OVFSELECT_A::_0,
            true => OVFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFSELECT_A::_1
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WER_SPEC, OVFSELECT_A, O>;
impl<'a, const O: u8> OVF_W<'a, O> {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFSELECT_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFSELECT_A::_1)
    }
}
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type ALARM0_R = crate::BitReader<ALARM0SELECT_A>;
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM0SELECT_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<ALARM0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM0SELECT_A {
        match self.bits {
            false => ALARM0SELECT_A::_0,
            true => ALARM0SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM0SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM0SELECT_A::_1
    }
}
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub type ALARM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WER_SPEC, ALARM0SELECT_A, O>;
impl<'a, const O: u8> ALARM0_W<'a, O> {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0SELECT_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM0SELECT_A::_1)
    }
}
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub type ALARM1_R = crate::BitReader<ALARM1SELECT_A>;
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM1SELECT_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<ALARM1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1SELECT_A {
        match self.bits {
            false => ALARM1SELECT_A::_0,
            true => ALARM1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM1SELECT_A::_1
    }
}
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub type ALARM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WER_SPEC, ALARM1SELECT_A, O>;
impl<'a, const O: u8> ALARM1_W<'a, O> {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1SELECT_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM1SELECT_A::_1)
    }
}
#[doc = "Field `PER0` reader - Periodic 0"]
pub type PER0_R = crate::BitReader<PER0SELECT_A>;
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER0SELECT_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<PER0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PER0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER0SELECT_A {
        match self.bits {
            false => PER0SELECT_A::_0,
            true => PER0SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER0SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER0SELECT_A::_1
    }
}
#[doc = "Field `PER0` writer - Periodic 0"]
pub type PER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WER_SPEC, PER0SELECT_A, O>;
impl<'a, const O: u8> PER0_W<'a, O> {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0SELECT_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER0SELECT_A::_1)
    }
}
#[doc = "Field `PER1` reader - Periodic 1"]
pub type PER1_R = crate::BitReader<PER1SELECT_A>;
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER1SELECT_A {
    #[doc = "0: The corresponing event will not wake up the CPU from sleep mode"]
    _0 = 0,
    #[doc = "1: The corresponding event will wake up the CPU from sleep mode"]
    _1 = 1,
}
impl From<PER1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PER1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER1SELECT_A {
        match self.bits {
            false => PER1SELECT_A::_0,
            true => PER1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER1SELECT_A::_1
    }
}
#[doc = "Field `PER1` writer - Periodic 1"]
pub type PER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WER_SPEC, PER1SELECT_A, O>;
impl<'a, const O: u8> PER1_W<'a, O> {
    #[doc = "The corresponing event will not wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1SELECT_A::_0)
    }
    #[doc = "The corresponding event will wake up the CPU from sleep mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER1SELECT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<8> {
        ALARM0_W::new(self)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> ALARM1_W<9> {
        ALARM1_W::new(self)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    #[must_use]
    pub fn per0(&mut self) -> PER0_W<16> {
        PER0_W::new(self)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    #[must_use]
    pub fn per1(&mut self) -> PER1_W<17> {
        PER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wer](index.html) module"]
pub struct WER_SPEC;
impl crate::RegisterSpec for WER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wer::R](R) reader structure"]
impl crate::Readable for WER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wer::W](W) writer structure"]
impl crate::Writable for WER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WER to value 0"]
impl crate::Resettable for WER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
