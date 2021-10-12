#[doc = "Register `BGCTRL` reader"]
pub struct R(crate::R<BGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCTRL` writer"]
pub struct W(crate::W<BGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCTRL_SPEC>;
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
impl From<crate::W<BGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCISEL_A {
    #[doc = "0: `0`"]
    DIS = 0,
    #[doc = "1: `1`"]
    VTEMP = 1,
    #[doc = "2: `10`"]
    VREF = 2,
}
impl From<ADCISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCISEL` reader - ADC Input Selection"]
pub struct ADCISEL_R(crate::FieldReader<u8, ADCISEL_A>);
impl ADCISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCISEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCISEL_A> {
        match self.bits {
            0 => Some(ADCISEL_A::DIS),
            1 => Some(ADCISEL_A::VTEMP),
            2 => Some(ADCISEL_A::VREF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ADCISEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline(always)]
    pub fn is_vtemp(&self) -> bool {
        **self == ADCISEL_A::VTEMP
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == ADCISEL_A::VREF
    }
}
impl core::ops::Deref for ADCISEL_R {
    type Target = crate::FieldReader<u8, ADCISEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCISEL` writer - ADC Input Selection"]
pub struct ADCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCISEL_A::DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(ADCISEL_A::VTEMP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(ADCISEL_A::VREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub struct TSEN_R(crate::FieldReader<bool, bool>);
impl TSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    pub fn adcisel(&self) -> ADCISEL_R {
        ADCISEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    pub fn adcisel(&mut self) -> ADCISEL_W {
        ADCISEL_W { w: self }
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgctrl](index.html) module"]
pub struct BGCTRL_SPEC;
impl crate::RegisterSpec for BGCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgctrl::R](R) reader structure"]
impl crate::Readable for BGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgctrl::W](W) writer structure"]
impl crate::Writable for BGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCTRL to value 0"]
impl crate::Resettable for BGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
