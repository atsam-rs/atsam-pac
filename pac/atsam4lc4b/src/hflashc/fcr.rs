#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDY_A {
    #[doc = "0: Flash Ready does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Flash Ready generates an interrupt"]
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub struct FRDY_R(crate::FieldReader<bool, FRDY_A>);
impl FRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRDY_A::_1
    }
}
impl core::ops::Deref for FRDY_R {
    type Target = crate::FieldReader<bool, FRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub struct FRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash Ready does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDY_A::_0)
    }
    #[doc = "Flash Ready generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDY_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Lock Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKE_A {
    #[doc = "0: Lock Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Lock Error generates an interrupt"]
    _1 = 1,
}
impl From<LOCKE_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKE` reader - Lock Error Interrupt Enable"]
pub struct LOCKE_R(crate::FieldReader<bool, LOCKE_A>);
impl LOCKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKE_A {
        match self.bits {
            false => LOCKE_A::_0,
            true => LOCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOCKE_A::_1
    }
}
impl core::ops::Deref for LOCKE_R {
    type Target = crate::FieldReader<bool, LOCKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKE` writer - Lock Error Interrupt Enable"]
pub struct LOCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Lock Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKE_A::_0)
    }
    #[doc = "Lock Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKE_A::_1)
    }
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
#[doc = "Programming Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROGE_A {
    #[doc = "0: Programming Error does not generate an interrupt"]
    _0 = 0,
    #[doc = "1: Programming Error generates an interrupt"]
    _1 = 1,
}
impl From<PROGE_A> for bool {
    #[inline(always)]
    fn from(variant: PROGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROGE` reader - Programming Error Interrupt Enable"]
pub struct PROGE_R(crate::FieldReader<bool, PROGE_A>);
impl PROGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROGE_A {
        match self.bits {
            false => PROGE_A::_0,
            true => PROGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROGE_A::_1
    }
}
impl core::ops::Deref for PROGE_R {
    type Target = crate::FieldReader<bool, PROGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROGE` writer - Programming Error Interrupt Enable"]
pub struct PROGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PROGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Programming Error does not generate an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROGE_A::_0)
    }
    #[doc = "Programming Error generates an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROGE_A::_1)
    }
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
#[doc = "Flash Wait State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWS_A {
    #[doc = "0: The flash is read with 0 wait states"]
    _0 = 0,
    #[doc = "1: The flash is read with 1 wait states"]
    _1 = 1,
}
impl From<FWS_A> for bool {
    #[inline(always)]
    fn from(variant: FWS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWS` reader - Flash Wait State"]
pub struct FWS_R(crate::FieldReader<bool, FWS_A>);
impl FWS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWS_A {
        match self.bits {
            false => FWS_A::_0,
            true => FWS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FWS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FWS_A::_1
    }
}
impl core::ops::Deref for FWS_R {
    type Target = crate::FieldReader<bool, FWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWS` writer - Flash Wait State"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The flash is read with 0 wait states"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWS_A::_0)
    }
    #[doc = "The flash is read with 1 wait states"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWS_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WS1OPT` reader - Wait State 1 Optimization"]
pub struct WS1OPT_R(crate::FieldReader<bool, bool>);
impl WS1OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WS1OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WS1OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WS1OPT` writer - Wait State 1 Optimization"]
pub struct WS1OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> WS1OPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    pub fn ws1opt(&self) -> WS1OPT_R {
        WS1OPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FRDY_W {
        FRDY_W { w: self }
    }
    #[doc = "Bit 2 - Lock Error Interrupt Enable"]
    #[inline(always)]
    pub fn locke(&mut self) -> LOCKE_W {
        LOCKE_W { w: self }
    }
    #[doc = "Bit 3 - Programming Error Interrupt Enable"]
    #[inline(always)]
    pub fn proge(&mut self) -> PROGE_W {
        PROGE_W { w: self }
    }
    #[doc = "Bit 6 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 7 - Wait State 1 Optimization"]
    #[inline(always)]
    pub fn ws1opt(&mut self) -> WS1OPT_W {
        WS1OPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
