#[doc = "Reader of register PBAMASK"]
pub type R = crate::R<u32, super::PBAMASK>;
#[doc = "Writer for register PBAMASK"]
pub type W = crate::W<u32, super::PBAMASK>;
#[doc = "Register PBAMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::PBAMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IISC_`"]
pub type IISC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IISC_`"]
pub struct IISC__W<'a> {
    w: &'a mut W,
}
impl<'a> IISC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SPI_`"]
pub type SPI__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_`"]
pub struct SPI__W<'a> {
    w: &'a mut W,
}
impl<'a> SPI__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TC0_`"]
pub type TC0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC0_`"]
pub struct TC0__W<'a> {
    w: &'a mut W,
}
impl<'a> TC0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TC1_`"]
pub type TC1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC1_`"]
pub struct TC1__W<'a> {
    w: &'a mut W,
}
impl<'a> TC1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TWIM0_`"]
pub type TWIM0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIM0_`"]
pub struct TWIM0__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TWIS0_`"]
pub type TWIS0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIS0_`"]
pub struct TWIS0__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TWIM1_`"]
pub type TWIM1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIM1_`"]
pub struct TWIM1__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TWIS1_`"]
pub type TWIS1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIS1_`"]
pub struct TWIS1__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `USART0_`"]
pub type USART0__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0_`"]
pub struct USART0__W<'a> {
    w: &'a mut W,
}
impl<'a> USART0__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `USART1_`"]
pub type USART1__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1_`"]
pub struct USART1__W<'a> {
    w: &'a mut W,
}
impl<'a> USART1__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USART2_`"]
pub type USART2__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2_`"]
pub struct USART2__W<'a> {
    w: &'a mut W,
}
impl<'a> USART2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `USART3_`"]
pub type USART3__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3_`"]
pub struct USART3__W<'a> {
    w: &'a mut W,
}
impl<'a> USART3__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADCIFE_`"]
pub type ADCIFE__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCIFE_`"]
pub struct ADCIFE__W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFE__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DACC_`"]
pub type DACC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACC_`"]
pub struct DACC__W<'a> {
    w: &'a mut W,
}
impl<'a> DACC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ACIFC_`"]
pub type ACIFC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACIFC_`"]
pub struct ACIFC__W<'a> {
    w: &'a mut W,
}
impl<'a> ACIFC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GLOC_`"]
pub type GLOC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOC_`"]
pub struct GLOC__W<'a> {
    w: &'a mut W,
}
impl<'a> GLOC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ABDACB_`"]
pub type ABDACB__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABDACB_`"]
pub struct ABDACB__W<'a> {
    w: &'a mut W,
}
impl<'a> ABDACB__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRNG_`"]
pub type TRNG__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_`"]
pub struct TRNG__W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PARC_`"]
pub type PARC__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARC_`"]
pub struct PARC__W<'a> {
    w: &'a mut W,
}
impl<'a> PARC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CATB_`"]
pub type CATB__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CATB_`"]
pub struct CATB__W<'a> {
    w: &'a mut W,
}
impl<'a> CATB__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TWIM2_`"]
pub type TWIM2__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIM2_`"]
pub struct TWIM2__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TWIM3_`"]
pub type TWIM3__R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TWIM3_`"]
pub struct TWIM3__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM3__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    pub fn iisc_(&self) -> IISC__R {
        IISC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    pub fn spi_(&self) -> SPI__R {
        SPI__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    pub fn twim0_(&self) -> TWIM0__R {
        TWIM0__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    pub fn twis0_(&self) -> TWIS0__R {
        TWIS0__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    pub fn twim1_(&self) -> TWIM1__R {
        TWIM1__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    pub fn twis1_(&self) -> TWIS1__R {
        TWIS1__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    pub fn usart0_(&self) -> USART0__R {
        USART0__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    pub fn usart1_(&self) -> USART1__R {
        USART1__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    pub fn usart2_(&self) -> USART2__R {
        USART2__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    pub fn usart3_(&self) -> USART3__R {
        USART3__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    pub fn adcife_(&self) -> ADCIFE__R {
        ADCIFE__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    pub fn dacc_(&self) -> DACC__R {
        DACC__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    pub fn acifc_(&self) -> ACIFC__R {
        ACIFC__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    pub fn gloc_(&self) -> GLOC__R {
        GLOC__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    pub fn abdacb_(&self) -> ABDACB__R {
        ABDACB__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    pub fn parc_(&self) -> PARC__R {
        PARC__R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    pub fn catb_(&self) -> CATB__R {
        CATB__R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    pub fn twim2_(&self) -> TWIM2__R {
        TWIM2__R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    pub fn twim3_(&self) -> TWIM3__R {
        TWIM3__R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    pub fn iisc_(&mut self) -> IISC__W {
        IISC__W { w: self }
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    pub fn spi_(&mut self) -> SPI__W {
        SPI__W { w: self }
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> TC0__W {
        TC0__W { w: self }
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> TC1__W {
        TC1__W { w: self }
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    pub fn twim0_(&mut self) -> TWIM0__W {
        TWIM0__W { w: self }
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    pub fn twis0_(&mut self) -> TWIS0__W {
        TWIS0__W { w: self }
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    pub fn twim1_(&mut self) -> TWIM1__W {
        TWIM1__W { w: self }
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    pub fn twis1_(&mut self) -> TWIS1__W {
        TWIS1__W { w: self }
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    pub fn usart0_(&mut self) -> USART0__W {
        USART0__W { w: self }
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    pub fn usart1_(&mut self) -> USART1__W {
        USART1__W { w: self }
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    pub fn usart2_(&mut self) -> USART2__W {
        USART2__W { w: self }
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    pub fn usart3_(&mut self) -> USART3__W {
        USART3__W { w: self }
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    pub fn adcife_(&mut self) -> ADCIFE__W {
        ADCIFE__W { w: self }
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    pub fn dacc_(&mut self) -> DACC__W {
        DACC__W { w: self }
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    pub fn acifc_(&mut self) -> ACIFC__W {
        ACIFC__W { w: self }
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    pub fn gloc_(&mut self) -> GLOC__W {
        GLOC__W { w: self }
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    pub fn abdacb_(&mut self) -> ABDACB__W {
        ABDACB__W { w: self }
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&mut self) -> TRNG__W {
        TRNG__W { w: self }
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    pub fn parc_(&mut self) -> PARC__W {
        PARC__W { w: self }
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    pub fn catb_(&mut self) -> CATB__W {
        CATB__W { w: self }
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    pub fn twim2_(&mut self) -> TWIM2__W {
        TWIM2__W { w: self }
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    pub fn twim3_(&mut self) -> TWIM3__W {
        TWIM3__W { w: self }
    }
}
