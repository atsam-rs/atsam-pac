#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_A {
    #[doc = "0: Receiver is effectively disabled, following a CR.RXDIS or CR.SWRST request"]
    OFF = 0,
    #[doc = "1: Receiver is effectively enabled, following a CR.RXEN request"]
    ON = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, RXEN_A>;
impl RXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::OFF,
            true => RXEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RXEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RXEN_A::ON
    }
}
#[doc = "Receive Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_A {
    #[doc = "0: The register RHR is empty and can't be read"]
    EMPTY = 0,
    #[doc = "1: The register RHR is full and is ready to be read"]
    FULL = 1,
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
            false => RXRDY_A::EMPTY,
            true => RXRDY_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXRDY_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXRDY_A::FULL
    }
}
#[doc = "Receive Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOR_A {
    #[doc = "0: No overrun"]
    NO = 0,
    #[doc = "1: The previous received data has not been read. This data is lost"]
    YES = 1,
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
            false => RXOR_A::NO,
            true => RXOR_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == RXOR_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == RXOR_A::YES
    }
}
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_A {
    #[doc = "0: Transmitter is effectively disabled, following a CR.TXDIS or CR.SWRST request"]
    OFF = 0,
    #[doc = "1: Transmitter is effectively enabled, following a CR.TXEN request"]
    ON = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, TXEN_A>;
impl TXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::OFF,
            true => TXEN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TXEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == TXEN_A::ON
    }
}
#[doc = "Transmit Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: The register THR is full and can't be written"]
    FULL = 0,
    #[doc = "1: The register THR is empty and is ready to be written"]
    EMPTY = 1,
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
            false => TXRDY_A::FULL,
            true => TXRDY_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXRDY_A::FULL
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXRDY_A::EMPTY
    }
}
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_A {
    #[doc = "0: No underrun"]
    NO = 0,
    #[doc = "1: The last bit of the last data written to the register THR has been set. Until the next write to THR, data will be sent according to MR.TXSAME field"]
    YES = 1,
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
            false => TXUR_A::NO,
            true => TXUR_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == TXUR_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == TXUR_A::YES
    }
}
#[doc = "Receive Overrun Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXORCH_A {
    #[doc = "0: Overrun first occurred on left channel"]
    LEFT = 0,
    #[doc = "1: Overrun first occurred on right channel"]
    RIGHT = 1,
}
impl From<RXORCH_A> for u8 {
    #[inline(always)]
    fn from(variant: RXORCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXORCH`"]
pub type RXORCH_R = crate::R<u8, RXORCH_A>;
impl RXORCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXORCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXORCH_A::LEFT),
            1 => Val(RXORCH_A::RIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == RXORCH_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == RXORCH_A::RIGHT
    }
}
#[doc = "Transmit Underrun Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXURCH_A {
    #[doc = "0: Underrun first occurred on left channel"]
    LEFT = 0,
    #[doc = "1: Underrun first occurred on right channel"]
    RIGHT = 1,
}
impl From<TXURCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TXURCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXURCH`"]
pub type TXURCH_R = crate::R<u8, TXURCH_A>;
impl TXURCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXURCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXURCH_A::LEFT),
            1 => Val(TXURCH_A::RIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == TXURCH_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == TXURCH_A::RIGHT
    }
}
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channels"]
    #[inline(always)]
    pub fn rxorch(&self) -> RXORCH_R {
        RXORCH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channels"]
    #[inline(always)]
    pub fn txurch(&self) -> TXURCH_R {
        TXURCH_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
