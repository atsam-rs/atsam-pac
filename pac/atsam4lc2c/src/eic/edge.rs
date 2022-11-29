#[doc = "Register `EDGE` reader"]
pub struct R(crate::R<EDGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDGE` writer"]
pub struct W(crate::W<EDGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDGE_SPEC>;
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
impl From<crate::W<EDGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI` reader - External Non Maskable CPU interrupt"]
pub type NMI_R = crate::BitReader<bool>;
#[doc = "Field `NMI` writer - External Non Maskable CPU interrupt"]
pub type NMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT1` reader - External Interrupt 1"]
pub type INT1_R = crate::BitReader<INT1SELECT_A>;
#[doc = "External Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT1SELECT_A {
    #[doc = "0: Triggers on falling edge"]
    _0 = 0,
    #[doc = "1: Triggers on rising edge."]
    _1 = 1,
}
impl From<INT1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INT1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1SELECT_A {
        match self.bits {
            false => INT1SELECT_A::_0,
            true => INT1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT1SELECT_A::_1
    }
}
#[doc = "Field `INT1` writer - External Interrupt 1"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, INT1SELECT_A, O>;
impl<'a, const O: u8> INT1_W<'a, O> {
    #[doc = "Triggers on falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT1SELECT_A::_0)
    }
    #[doc = "Triggers on rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT1SELECT_A::_1)
    }
}
#[doc = "Field `INT2` reader - External Interrupt 2"]
pub type INT2_R = crate::BitReader<INT2SELECT_A>;
#[doc = "External Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT2SELECT_A {
    #[doc = "0: Triggers on falling edge"]
    _0 = 0,
    #[doc = "1: Triggers on rising edge."]
    _1 = 1,
}
impl From<INT2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INT2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2SELECT_A {
        match self.bits {
            false => INT2SELECT_A::_0,
            true => INT2SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT2SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT2SELECT_A::_1
    }
}
#[doc = "Field `INT2` writer - External Interrupt 2"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, INT2SELECT_A, O>;
impl<'a, const O: u8> INT2_W<'a, O> {
    #[doc = "Triggers on falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT2SELECT_A::_0)
    }
    #[doc = "Triggers on rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT2SELECT_A::_1)
    }
}
#[doc = "Field `INT3` reader - External Interrupt 3"]
pub type INT3_R = crate::BitReader<INT3SELECT_A>;
#[doc = "External Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT3SELECT_A {
    #[doc = "0: Triggers on falling edge"]
    _0 = 0,
    #[doc = "1: Triggers on rising edge."]
    _1 = 1,
}
impl From<INT3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INT3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3SELECT_A {
        match self.bits {
            false => INT3SELECT_A::_0,
            true => INT3SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT3SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT3SELECT_A::_1
    }
}
#[doc = "Field `INT3` writer - External Interrupt 3"]
pub type INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, INT3SELECT_A, O>;
impl<'a, const O: u8> INT3_W<'a, O> {
    #[doc = "Triggers on falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3SELECT_A::_0)
    }
    #[doc = "Triggers on rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3SELECT_A::_1)
    }
}
#[doc = "Field `INT4` reader - External Interrupt 4"]
pub type INT4_R = crate::BitReader<INT4SELECT_A>;
#[doc = "External Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT4SELECT_A {
    #[doc = "0: Triggers on falling edge"]
    _0 = 0,
    #[doc = "1: Triggers on rising edge."]
    _1 = 1,
}
impl From<INT4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INT4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4SELECT_A {
        match self.bits {
            false => INT4SELECT_A::_0,
            true => INT4SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INT4SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INT4SELECT_A::_1
    }
}
#[doc = "Field `INT4` writer - External Interrupt 4"]
pub type INT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, INT4SELECT_A, O>;
impl<'a, const O: u8> INT4_W<'a, O> {
    #[doc = "Triggers on falling edge"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT4SELECT_A::_0)
    }
    #[doc = "Triggers on rising edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT4SELECT_A::_1)
    }
}
#[doc = "Field `INT5` reader - External Interrupt 5"]
pub type INT5_R = crate::BitReader<bool>;
#[doc = "Field `INT5` writer - External Interrupt 5"]
pub type INT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT6` reader - External Interrupt 6"]
pub type INT6_R = crate::BitReader<bool>;
#[doc = "Field `INT6` writer - External Interrupt 6"]
pub type INT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT7` reader - External Interrupt 7"]
pub type INT7_R = crate::BitReader<bool>;
#[doc = "Field `INT7` writer - External Interrupt 7"]
pub type INT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT8` reader - External Interrupt 8"]
pub type INT8_R = crate::BitReader<bool>;
#[doc = "Field `INT8` writer - External Interrupt 8"]
pub type INT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT9` reader - External Interrupt 9"]
pub type INT9_R = crate::BitReader<bool>;
#[doc = "Field `INT9` writer - External Interrupt 9"]
pub type INT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT10` reader - External Interrupt 10"]
pub type INT10_R = crate::BitReader<bool>;
#[doc = "Field `INT10` writer - External Interrupt 10"]
pub type INT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT11` reader - External Interrupt 11"]
pub type INT11_R = crate::BitReader<bool>;
#[doc = "Field `INT11` writer - External Interrupt 11"]
pub type INT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT12` reader - External Interrupt 12"]
pub type INT12_R = crate::BitReader<bool>;
#[doc = "Field `INT12` writer - External Interrupt 12"]
pub type INT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT13` reader - External Interrupt 13"]
pub type INT13_R = crate::BitReader<bool>;
#[doc = "Field `INT13` writer - External Interrupt 13"]
pub type INT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT14` reader - External Interrupt 14"]
pub type INT14_R = crate::BitReader<bool>;
#[doc = "Field `INT14` writer - External Interrupt 14"]
pub type INT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
#[doc = "Field `INT15` reader - External Interrupt 15"]
pub type INT15_R = crate::BitReader<bool>;
#[doc = "Field `INT15` writer - External Interrupt 15"]
pub type INT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDGE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<0> {
        NMI_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<1> {
        INT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<2> {
        INT2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<3> {
        INT3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<4> {
        INT4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<5> {
        INT5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<6> {
        INT6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<7> {
        INT7_W::new(self)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<8> {
        INT8_W::new(self)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<9> {
        INT9_W::new(self)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<10> {
        INT10_W::new(self)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<11> {
        INT11_W::new(self)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<12> {
        INT12_W::new(self)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<13> {
        INT13_W::new(self)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<14> {
        INT14_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<15> {
        INT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Edge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edge](index.html) module"]
pub struct EDGE_SPEC;
impl crate::RegisterSpec for EDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edge::R](R) reader structure"]
impl crate::Readable for EDGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edge::W](W) writer structure"]
impl crate::Writable for EDGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDGE to value 0"]
impl crate::Resettable for EDGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
