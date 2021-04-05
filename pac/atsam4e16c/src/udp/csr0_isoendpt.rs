#[doc = "Reader of register CSR0_ISOENDPT"]
pub type R = crate::R<u32, super::CSR0_ISOENDPT>;
#[doc = "Writer for register CSR0_ISOENDPT"]
pub type W = crate::W<u32, super::CSR0_ISOENDPT>;
#[doc = "Reader of field `TXCOMP`"]
pub type TXCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCOMP`"]
pub struct TXCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCOMP_W<'a> {
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
#[doc = "Reader of field `RX_DATA_BK0`"]
pub type RX_DATA_BK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_BK0`"]
pub struct RX_DATA_BK0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_BK0_W<'a> {
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
#[doc = "Reader of field `RXSETUP`"]
pub type RXSETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXSETUP`"]
pub struct RXSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSETUP_W<'a> {
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
#[doc = "Reader of field `ISOERROR`"]
pub type ISOERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOERROR`"]
pub struct ISOERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOERROR_W<'a> {
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
#[doc = "Reader of field `TXPKTRDY`"]
pub type TXPKTRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPKTRDY`"]
pub struct TXPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTRDY_W<'a> {
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
#[doc = "Reader of field `FORCESTALL`"]
pub type FORCESTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCESTALL`"]
pub struct FORCESTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCESTALL_W<'a> {
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
#[doc = "Reader of field `RX_DATA_BK1`"]
pub type RX_DATA_BK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_BK1`"]
pub struct RX_DATA_BK1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_BK1_W<'a> {
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
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Endpoint Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous OUT"]
    ISO_OUT = 1,
    #[doc = "5: Isochronous IN"]
    ISO_IN = 5,
    #[doc = "2: Bulk OUT"]
    BULK_OUT = 2,
    #[doc = "6: Bulk IN"]
    BULK_IN = 6,
    #[doc = "3: Interrupt OUT"]
    INT_OUT = 3,
    #[doc = "7: Interrupt IN"]
    INT_IN = 7,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EPTYPE`"]
pub type EPTYPE_R = crate::R<u8, EPTYPE_A>;
impl EPTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EPTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EPTYPE_A::CTRL),
            1 => Val(EPTYPE_A::ISO_OUT),
            5 => Val(EPTYPE_A::ISO_IN),
            2 => Val(EPTYPE_A::BULK_OUT),
            6 => Val(EPTYPE_A::BULK_IN),
            3 => Val(EPTYPE_A::INT_OUT),
            7 => Val(EPTYPE_A::INT_IN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO_OUT`"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == EPTYPE_A::ISO_OUT
    }
    #[doc = "Checks if the value of the field is `ISO_IN`"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == EPTYPE_A::ISO_IN
    }
    #[doc = "Checks if the value of the field is `BULK_OUT`"]
    #[inline(always)]
    pub fn is_bulk_out(&self) -> bool {
        *self == EPTYPE_A::BULK_OUT
    }
    #[doc = "Checks if the value of the field is `BULK_IN`"]
    #[inline(always)]
    pub fn is_bulk_in(&self) -> bool {
        *self == EPTYPE_A::BULK_IN
    }
    #[doc = "Checks if the value of the field is `INT_OUT`"]
    #[inline(always)]
    pub fn is_int_out(&self) -> bool {
        *self == EPTYPE_A::INT_OUT
    }
    #[doc = "Checks if the value of the field is `INT_IN`"]
    #[inline(always)]
    pub fn is_int_in(&self) -> bool {
        *self == EPTYPE_A::INT_IN
    }
}
#[doc = "Write proxy for field `EPTYPE`"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPE_A::CTRL)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_OUT)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_IN)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulk_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_OUT)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulk_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_IN)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn int_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_OUT)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn int_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_IN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTGLE`"]
pub type DTGLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTGLE`"]
pub struct DTGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLE_W<'a> {
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
#[doc = "Reader of field `EPEDS`"]
pub type EPEDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEDS`"]
pub struct EPEDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEDS_W<'a> {
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
#[doc = "Reader of field `RXBYTECNT`"]
pub type RXBYTECNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXBYTECNT`"]
pub struct RXBYTECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBYTECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&self) -> RX_DATA_BK0_R {
        RX_DATA_BK0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&self) -> RXSETUP_R {
        RXSETUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&self) -> ISOERROR_R {
        ISOERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TXPKTRDY_R {
        TXPKTRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&self) -> FORCESTALL_R {
        FORCESTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&self) -> RX_DATA_BK1_R {
        RX_DATA_BK1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline(always)]
    pub fn dtgle(&self) -> DTGLE_R {
        DTGLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&self) -> EPEDS_R {
        EPEDS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline(always)]
    pub fn rxbytecnt(&self) -> RXBYTECNT_R {
        RXBYTECNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TXCOMP_W {
        TXCOMP_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&mut self) -> RX_DATA_BK0_W {
        RX_DATA_BK0_W { w: self }
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&mut self) -> RXSETUP_W {
        RXSETUP_W { w: self }
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&mut self) -> ISOERROR_W {
        ISOERROR_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TXPKTRDY_W {
        TXPKTRDY_W { w: self }
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&mut self) -> FORCESTALL_W {
        FORCESTALL_W { w: self }
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&mut self) -> RX_DATA_BK1_W {
        RX_DATA_BK1_W { w: self }
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints)"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 8:10 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bit 11 - Data Toggle"]
    #[inline(always)]
    pub fn dtgle(&mut self) -> DTGLE_W {
        DTGLE_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&mut self) -> EPEDS_W {
        EPEDS_W { w: self }
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO"]
    #[inline(always)]
    pub fn rxbytecnt(&mut self) -> RXBYTECNT_W {
        RXBYTECNT_W { w: self }
    }
}
