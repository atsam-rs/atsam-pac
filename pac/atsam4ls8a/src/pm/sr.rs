#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `CFD`"]
pub type CFD_R = crate::R<bool, bool>;
#[doc = "Reader of field `OCP`"]
pub type OCP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKRDY`"]
pub type CKRDY_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `WAKE`"]
pub type WAKE_R = crate::R<bool, WAKE_A>;
impl WAKE_R {
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
        *self == WAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKE_A::_1
    }
}
#[doc = "Reader of field `PERRDY`"]
pub type PERRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
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
