#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RDRF_R = crate::BitReader<RDRFSELECT_A>;
#[doc = "Receive Data Register Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRFSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<RDRFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RDRFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRFSELECT_A {
        match self.bits {
            false => RDRFSELECT_A::_0,
            true => RDRFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRFSELECT_A::_1
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Interrupt Mask"]
pub type TDRE_R = crate::BitReader<TDRESELECT_A>;
#[doc = "Transmit Data Register Empty Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRESELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<TDRESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TDRESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRESELECT_A {
        match self.bits {
            false => TDRESELECT_A::_0,
            true => TDRESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRESELECT_A::_1
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error Interrupt Mask"]
pub type MODF_R = crate::BitReader<MODFSELECT_A>;
#[doc = "Mode Fault Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<MODFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFSELECT_A {
        match self.bits {
            false => MODFSELECT_A::_0,
            true => MODFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFSELECT_A::_1
    }
}
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OVRES_R = crate::BitReader<OVRESSELECT_A>;
#[doc = "Overrun Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<OVRESSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVRESSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRESSELECT_A {
        match self.bits {
            false => OVRESSELECT_A::_0,
            true => OVRESSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRESSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRESSELECT_A::_1
    }
}
#[doc = "Field `ENDRX` reader - End of Receive Buffer Interrupt Mask"]
pub type ENDRX_R = crate::BitReader<ENDRXSELECT_A>;
#[doc = "End of Receive Buffer Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRXSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<ENDRXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRXSELECT_A {
        match self.bits {
            false => ENDRXSELECT_A::_0,
            true => ENDRXSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDRXSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDRXSELECT_A::_1
    }
}
#[doc = "Field `ENDTX` reader - End of Transmit Buffer Interrupt Mask"]
pub type ENDTX_R = crate::BitReader<ENDTXSELECT_A>;
#[doc = "End of Transmit Buffer Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDTXSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<ENDTXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTXSELECT_A {
        match self.bits {
            false => ENDTXSELECT_A::_0,
            true => ENDTXSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDTXSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDTXSELECT_A::_1
    }
}
#[doc = "Field `RXBUFF` reader - Receive Buffer Full Interrupt Mask"]
pub type RXBUFF_R = crate::BitReader<RXBUFFSELECT_A>;
#[doc = "Receive Buffer Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<RXBUFFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUFFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXBUFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBUFFSELECT_A {
        match self.bits {
            false => RXBUFFSELECT_A::_0,
            true => RXBUFFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBUFFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBUFFSELECT_A::_1
    }
}
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty Interrupt Mask"]
pub type TXBUFE_R = crate::BitReader<TXBUFESELECT_A>;
#[doc = "Transmit Buffer Empty Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<TXBUFESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXBUFESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXBUFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBUFESELECT_A {
        match self.bits {
            false => TXBUFESELECT_A::_0,
            true => TXBUFESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXBUFESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXBUFESELECT_A::_1
    }
}
#[doc = "Field `NSSR` reader - NSS Rising Interrupt Mask"]
pub type NSSR_R = crate::BitReader<NSSRSELECT_A>;
#[doc = "NSS Rising Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSRSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<NSSRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NSSRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSRSELECT_A {
        match self.bits {
            false => NSSRSELECT_A::_0,
            true => NSSRSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NSSRSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NSSRSELECT_A::_1
    }
}
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTYSELECT_A>;
#[doc = "Transmission Registers Empty Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_A {
    #[doc = "0: The corresponding interrupt is not enabled."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt is enabled."]
    _1 = 1,
}
impl From<TXEMPTYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTYSELECT_A {
        match self.bits {
            false => TXEMPTYSELECT_A::_0,
            true => TXEMPTYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTYSELECT_A::_1
    }
}
#[doc = "Field `UNDES` reader - Underrun Error Interrupt Mask"]
pub type UNDES_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Receive Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Transmit Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
