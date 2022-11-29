#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CFD` reader - Clock Failure Detected"]
pub type CFD_R = crate::BitReader<bool>;
#[doc = "Field `OCP` reader - Over Clock Detected"]
pub type OCP_R = crate::BitReader<bool>;
#[doc = "Field `CKRDY` reader - Clock Ready"]
pub type CKRDY_R = crate::BitReader<bool>;
#[doc = "Field `WAKE` reader - Wake up"]
pub type WAKE_R = crate::BitReader<WAKESELECT_A>;
#[doc = "Wake up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKESELECT_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Disable Interrupt."]
    _1 = 1,
}
impl From<WAKESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKESELECT_A {
        match self.bits {
            false => WAKESELECT_A::_0,
            true => WAKESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKESELECT_A::_1
    }
}
#[doc = "Field `PERRDY` reader - Peripheral Ready"]
pub type PERRDY_R = crate::BitReader<bool>;
#[doc = "Field `AE` reader - Access Error"]
pub type AE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detected"]
    #[inline(always)]
    pub fn cfd(&self) -> CFD_R {
        CFD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Over Clock Detected"]
    #[inline(always)]
    pub fn ocp(&self) -> OCP_R {
        OCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Ready"]
    #[inline(always)]
    pub fn ckrdy(&self) -> CKRDY_R {
        CKRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake up"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral Ready"]
    #[inline(always)]
    pub fn perrdy(&self) -> PERRDY_R {
        PERRDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
