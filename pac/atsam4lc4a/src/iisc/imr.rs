#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Receive Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, RXRDY_A>;
impl RXRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDY_A {
        match self.bits {
            false => RXRDY_A::DISABLED,
            true => RXRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXRDY_A::ENABLED
    }
}
#[doc = "Receive Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOR_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXOR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXOR`"]
pub type RXOR_R = crate::R<bool, RXOR_A>;
impl RXOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOR_A {
        match self.bits {
            false => RXOR_A::DISABLED,
            true => RXOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXOR_A::ENABLED
    }
}
#[doc = "Transmit Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<TXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, TXRDY_A>;
impl TXRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDY_A {
        match self.bits {
            false => TXRDY_A::DISABLED,
            true => TXRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXRDY_A::ENABLED
    }
}
#[doc = "Transmit Underrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<TXUR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXUR`"]
pub type TXUR_R = crate::R<bool, TXUR_A>;
impl TXUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUR_A {
        match self.bits {
            false => TXUR_A::DISABLED,
            true => TXUR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXUR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXUR_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 1 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun Interrupt Mask"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
