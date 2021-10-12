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
#[doc = "Receive Data Register Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRF_A {
    #[doc = "0: No data has been received since the last read of RDR"]
    _0 = 0,
    #[doc = "1: Data has been received and the received data has been transferred from the serializer to RDR since the last readof RDR."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full"]
pub struct RDRF_R(crate::FieldReader<bool, RDRF_A>);
impl RDRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RDRF_A::_1
    }
}
impl core::ops::Deref for RDRF_R {
    type Target = crate::FieldReader<bool, RDRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Data Register Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: Data has been written to TDR and not yet transferred to the serializer."]
    _0 = 0,
    #[doc = "1: The last data written in the Transmit Data Register has been transferred to the serializer.TDRE equals zero when the SPI is disabled or at reset. The SPI enable command sets this bit to one."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty"]
pub struct TDRE_R(crate::FieldReader<bool, TDRE_A>);
impl TDRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TDRE_A::_1
    }
}
impl core::ops::Deref for TDRE_R {
    type Target = crate::FieldReader<bool, TDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Mode Fault Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No Mode Fault has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: A Mode Fault occurred since the last read of the SR."]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error"]
pub struct MODF_R(crate::FieldReader<bool, MODF_A>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODF_A::_1
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, MODF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overrun Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRES_A {
    #[doc = "0: No overrun has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: An overrun has occurred since the last read of SR."]
    _1 = 1,
}
impl From<OVRES_A> for bool {
    #[inline(always)]
    fn from(variant: OVRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRES` reader - Overrun Error Status"]
pub struct OVRES_R(crate::FieldReader<bool, OVRES_A>);
impl OVRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRES_A {
        match self.bits {
            false => OVRES_A::_0,
            true => OVRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OVRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVRES_A::_1
    }
}
impl core::ops::Deref for OVRES_R {
    type Target = crate::FieldReader<bool, OVRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of RX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: The Receive Counter Register has not reached 0 since the last write in RCR or RNCR."]
    _0 = 0,
    #[doc = "1: The Receive Counter Register has reached 0 since the last write in RCR or RNCR."]
    _1 = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` reader - End of RX buffer"]
pub struct ENDRX_R(crate::FieldReader<bool, ENDRX_A>);
impl ENDRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDRX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::_0,
            true => ENDRX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENDRX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENDRX_A::_1
    }
}
impl core::ops::Deref for ENDRX_R {
    type Target = crate::FieldReader<bool, ENDRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "End of TX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: The Transmit Counter Register has not reached 0 since the last write in TCR or TNCR."]
    _0 = 0,
    #[doc = "1: The Transmit Counter Register has reached 0 since the last write in TCR or TNCR."]
    _1 = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` reader - End of TX buffer"]
pub struct ENDTX_R(crate::FieldReader<bool, ENDTX_A>);
impl ENDTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDTX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::_0,
            true => ENDTX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENDTX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENDTX_A::_1
    }
}
impl core::ops::Deref for ENDTX_R {
    type Target = crate::FieldReader<bool, ENDTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RX Buffer Full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_A {
    #[doc = "0: RCR or RNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both RCR and RNCR has a value of 0."]
    _1 = 1,
}
impl From<RXBUFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub struct RXBUFF_R(crate::FieldReader<bool, RXBUFF_A>);
impl RXBUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBUFF_A {
        match self.bits {
            false => RXBUFF_A::_0,
            true => RXBUFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXBUFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBUFF_A::_1
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, RXBUFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TX Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_A {
    #[doc = "0: TCR or TNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both TCR and TNCR has a value of 0."]
    _1 = 1,
}
impl From<TXBUFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXBUFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBUFE` reader - TX Buffer Empty"]
pub struct TXBUFE_R(crate::FieldReader<bool, TXBUFE_A>);
impl TXBUFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBUFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBUFE_A {
        match self.bits {
            false => TXBUFE_A::_0,
            true => TXBUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXBUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXBUFE_A::_1
    }
}
impl core::ops::Deref for TXBUFE_R {
    type Target = crate::FieldReader<bool, TXBUFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "NSS Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSR_A {
    #[doc = "0: No rising edge detected on NSS pin since last read."]
    _0 = 0,
    #[doc = "1: A rising edge occurred on NSS pin since last read."]
    _1 = 1,
}
impl From<NSSR_A> for bool {
    #[inline(always)]
    fn from(variant: NSSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSR` reader - NSS Rising"]
pub struct NSSR_R(crate::FieldReader<bool, NSSR_A>);
impl NSSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSR_A {
        match self.bits {
            false => NSSR_A::_0,
            true => NSSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NSSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NSSR_A::_1
    }
}
impl core::ops::Deref for NSSR_R {
    type Target = crate::FieldReader<bool, NSSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmission Registers Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: As soon as data is written in TDR."]
    _0 = 0,
    #[doc = "1: TDR and internal shifter are empty. If a transfer delay has been defined, TXEMPTY is set after the completion ofsuch delay."]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty"]
pub struct TXEMPTY_R(crate::FieldReader<bool, TXEMPTY_A>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            false => TXEMPTY_A::_0,
            true => TXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPTY_A::_1
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, TXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave Mode Only)"]
pub struct UNDES_R(crate::FieldReader<bool, bool>);
impl UNDES_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNDES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SPI Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIENS_A {
    #[doc = "0: SPI is disabled."]
    _0 = 0,
    #[doc = "1: SPI is enabled."]
    _1 = 1,
}
impl From<SPIENS_A> for bool {
    #[inline(always)]
    fn from(variant: SPIENS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub struct SPIENS_R(crate::FieldReader<bool, SPIENS_A>);
impl SPIENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIENS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIENS_A {
        match self.bits {
            false => SPIENS_A::_0,
            true => SPIENS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SPIENS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SPIENS_A::_1
    }
}
impl core::ops::Deref for SPIENS_R {
    type Target = crate::FieldReader<bool, SPIENS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of RX buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of TX buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NSS Rising"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave Mode Only)"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SPIENS_R {
        SPIENS_R::new(((self.bits >> 16) & 0x01) != 0)
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
#[doc = "`reset()` method sets SR to value 0xf0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
