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
#[doc = "Field `ADCISEL` reader - ADC Input Selection"]
pub type ADCISEL_R = crate::FieldReader<u8, ADCISELSELECT_A>;
#[doc = "ADC Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCISELSELECT_A {
    #[doc = "0: `0`"]
    DIS = 0,
    #[doc = "1: `1`"]
    VTEMP = 1,
    #[doc = "2: `10`"]
    VREF = 2,
}
impl From<ADCISELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCISELSELECT_A) -> Self {
        variant as _
    }
}
impl ADCISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCISELSELECT_A> {
        match self.bits {
            0 => Some(ADCISELSELECT_A::DIS),
            1 => Some(ADCISELSELECT_A::VTEMP),
            2 => Some(ADCISELSELECT_A::VREF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCISELSELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline(always)]
    pub fn is_vtemp(&self) -> bool {
        *self == ADCISELSELECT_A::VTEMP
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == ADCISELSELECT_A::VREF
    }
}
#[doc = "Field `ADCISEL` writer - ADC Input Selection"]
pub type ADCISEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BGCTRL_SPEC, u8, ADCISELSELECT_A, 2, O>;
impl<'a, const O: u8> ADCISEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCISELSELECT_A::DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(ADCISELSELECT_A::VTEMP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(ADCISELSELECT_A::VREF)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Enable"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - Temperature Sensor Enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BGCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    pub fn adcisel(&self) -> ADCISEL_R {
        ADCISEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcisel(&mut self) -> ADCISEL_W<0> {
        ADCISEL_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<8> {
        TSEN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCTRL to value 0"]
impl crate::Resettable for BGCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
