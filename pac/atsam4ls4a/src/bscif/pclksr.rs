#[doc = "Register `PCLKSR` reader"]
pub struct R(crate::R<PCLKSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLKSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLKSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLKSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSC32RDY` reader - 32kHz Oscillator Ready"]
pub struct OSC32RDY_R(crate::FieldReader<bool, bool>);
impl OSC32RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSC32RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32KRDY` reader - 32kHz RC Oscillator Ready"]
pub struct RC32KRDY_R(crate::FieldReader<bool, bool>);
impl RC32KRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32KRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32KRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32KLOCK` reader - 32kHz RC Oscillator Lock"]
pub struct RC32KLOCK_R(crate::FieldReader<bool, bool>);
impl RC32KLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32KLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32KLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32KREFE` reader - 32kHz RC Oscillator Reference Error"]
pub struct RC32KREFE_R(crate::FieldReader<bool, bool>);
impl RC32KREFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32KREFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32KREFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32KSAT` reader - 32kHz RC Oscillator Saturation"]
pub struct RC32KSAT_R(crate::FieldReader<bool, bool>);
impl RC32KSAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC32KSAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32KSAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33DET` reader - BOD33 Detected"]
pub struct BOD33DET_R(crate::FieldReader<bool, bool>);
impl BOD33DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD33DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD18DET` reader - BOD18 Detected"]
pub struct BOD18DET_R(crate::FieldReader<bool, bool>);
impl BOD18DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD18DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD18DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33SYNRDY` reader - BOD33 Synchronization Ready"]
pub struct BOD33SYNRDY_R(crate::FieldReader<bool, bool>);
impl BOD33SYNRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD33SYNRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33SYNRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD18SYNRDY` reader - BOD18 Synchronization Ready"]
pub struct BOD18SYNRDY_R(crate::FieldReader<bool, bool>);
impl BOD18SYNRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD18SYNRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD18SYNRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSWRDY` reader - VREG Stop Switching Ready"]
pub struct SSWRDY_R(crate::FieldReader<bool, bool>);
impl SSWRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSWRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSWRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGOK` reader - Main VREG OK"]
pub struct VREGOK_R(crate::FieldReader<bool, bool>);
impl VREGOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC1MRDY` reader - RC 1MHz Oscillator Ready"]
pub struct RC1MRDY_R(crate::FieldReader<bool, bool>);
impl RC1MRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RC1MRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC1MRDY_R {
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
impl R {
    #[doc = "Bit 0 - 32kHz Oscillator Ready"]
    #[inline(always)]
    pub fn osc32rdy(&self) -> OSC32RDY_R {
        OSC32RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32kHz RC Oscillator Ready"]
    #[inline(always)]
    pub fn rc32krdy(&self) -> RC32KRDY_R {
        RC32KRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32kHz RC Oscillator Lock"]
    #[inline(always)]
    pub fn rc32klock(&self) -> RC32KLOCK_R {
        RC32KLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 32kHz RC Oscillator Reference Error"]
    #[inline(always)]
    pub fn rc32krefe(&self) -> RC32KREFE_R {
        RC32KREFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 32kHz RC Oscillator Saturation"]
    #[inline(always)]
    pub fn rc32ksat(&self) -> RC32KSAT_R {
        RC32KSAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOD33 Detected"]
    #[inline(always)]
    pub fn bod33det(&self) -> BOD33DET_R {
        BOD33DET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOD18 Detected"]
    #[inline(always)]
    pub fn bod18det(&self) -> BOD18DET_R {
        BOD18DET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn bod33synrdy(&self) -> BOD33SYNRDY_R {
        BOD33SYNRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BOD18 Synchronization Ready"]
    #[inline(always)]
    pub fn bod18synrdy(&self) -> BOD18SYNRDY_R {
        BOD18SYNRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VREG Stop Switching Ready"]
    #[inline(always)]
    pub fn sswrdy(&self) -> SSWRDY_R {
        SSWRDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Main VREG OK"]
    #[inline(always)]
    pub fn vregok(&self) -> VREGOK_R {
        VREGOK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RC 1MHz Oscillator Ready"]
    #[inline(always)]
    pub fn rc1mrdy(&self) -> RC1MRDY_R {
        RC1MRDY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&self) -> LPBGRDY_R {
        LPBGRDY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "Power and Clocks Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksr](index.html) module"]
pub struct PCLKSR_SPEC;
impl crate::RegisterSpec for PCLKSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclksr::R](R) reader structure"]
impl crate::Readable for PCLKSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCLKSR to value 0"]
impl crate::Resettable for PCLKSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
