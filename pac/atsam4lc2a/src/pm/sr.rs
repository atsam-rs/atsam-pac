#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFD` reader - Clock Failure Detected"]
pub struct CFD_R(crate::FieldReader<bool, bool>);
impl CFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCP` reader - Over Clock Detected"]
pub struct OCP_R(crate::FieldReader<bool, bool>);
impl OCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKRDY` reader - Clock Ready"]
pub struct CKRDY_R(crate::FieldReader<bool, bool>);
impl CKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wake up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disable Interrupt."]
    _1 = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Wake up"]
pub struct WAKE_R(crate::FieldReader<bool, WAKE_A>);
impl WAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::_0,
            true => WAKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAKE_A::_1
    }
}
impl core::ops::Deref for WAKE_R {
    type Target = crate::FieldReader<bool, WAKE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRDY` reader - Peripheral Ready"]
pub struct PERRDY_R(crate::FieldReader<bool, bool>);
impl PERRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE` reader - Access Error"]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detected"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Over Clock Detected"]
    #[inline(always)]
    pub fn ocp(&self) -> OCP_R {
        OCP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Ready"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CKRDY_R {
        CKRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake up"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral Ready"]
    #[inline(always)]
    pub fn perrdy(&self) -> PERRDY_R {
        PERRDY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
