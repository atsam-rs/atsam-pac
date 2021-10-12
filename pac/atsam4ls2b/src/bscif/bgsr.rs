#[doc = "Register `BGSR` reader"]
pub struct R(crate::R<BGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Bandgap Buffer Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BGBUFRDY_A {
    #[doc = "1: `1`"]
    FLASH = 1,
    #[doc = "2: `10`"]
    PLL = 2,
    #[doc = "4: `100`"]
    VREG = 4,
    #[doc = "8: `1000`"]
    BUFRR = 8,
    #[doc = "16: `10000`"]
    ADC = 16,
    #[doc = "32: `100000`"]
    LCD = 32,
}
impl From<BGBUFRDY_A> for u8 {
    #[inline(always)]
    fn from(variant: BGBUFRDY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BGBUFRDY` reader - Bandgap Buffer Ready"]
pub struct BGBUFRDY_R(crate::FieldReader<u8, BGBUFRDY_A>);
impl BGBUFRDY_R {
    pub(crate) fn new(bits: u8) -> Self {
        BGBUFRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BGBUFRDY_A> {
        match self.bits {
            1 => Some(BGBUFRDY_A::FLASH),
            2 => Some(BGBUFRDY_A::PLL),
            4 => Some(BGBUFRDY_A::VREG),
            8 => Some(BGBUFRDY_A::BUFRR),
            16 => Some(BGBUFRDY_A::ADC),
            32 => Some(BGBUFRDY_A::LCD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        **self == BGBUFRDY_A::FLASH
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == BGBUFRDY_A::PLL
    }
    #[doc = "Checks if the value of the field is `VREG`"]
    #[inline(always)]
    pub fn is_vreg(&self) -> bool {
        **self == BGBUFRDY_A::VREG
    }
    #[doc = "Checks if the value of the field is `BUFRR`"]
    #[inline(always)]
    pub fn is_bufrr(&self) -> bool {
        **self == BGBUFRDY_A::BUFRR
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        **self == BGBUFRDY_A::ADC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        **self == BGBUFRDY_A::LCD
    }
}
impl core::ops::Deref for BGBUFRDY_R {
    type Target = crate::FieldReader<u8, BGBUFRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGRDY` reader - Bandgap Voltage Reference Ready"]
pub struct BGRDY_R(crate::FieldReader<bool, bool>);
impl BGRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPBGRDY` reader - Low Power Bandgap Voltage Reference Ready"]
pub struct LPBGRDY_R(crate::FieldReader<bool, bool>);
impl LPBGRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPBGRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPBGRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF` reader - Voltage Reference Used by the System"]
pub struct VREF_R(crate::FieldReader<u8, u8>);
impl VREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bandgap Buffer Ready"]
    #[inline(always)]
    pub fn bgbufrdy(&self) -> BGBUFRDY_R {
        BGBUFRDY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn bgrdy(&self) -> BGRDY_R {
        BGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&self) -> LPBGRDY_R {
        LPBGRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Voltage Reference Used by the System"]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
#[doc = "Bandgap Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgsr](index.html) module"]
pub struct BGSR_SPEC;
impl crate::RegisterSpec for BGSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgsr::R](R) reader structure"]
impl crate::Readable for BGSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BGSR to value 0"]
impl crate::Resettable for BGSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
