#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - Channel x Transfer Type"]
pub type TYPE_R = crate::BitReader<TYPESELECT_A>;
#[doc = "Channel x Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TYPESELECT_A {
    #[doc = "0: Self triggered mode (Memory to Memory Transfer)."]
    MEM_TRAN = 0,
    #[doc = "1: Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    PER_TRAN = 1,
}
impl From<TYPESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TYPESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPESELECT_A {
        match self.bits {
            false => TYPESELECT_A::MEM_TRAN,
            true => TYPESELECT_A::PER_TRAN,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_TRAN`"]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == TYPESELECT_A::MEM_TRAN
    }
    #[doc = "Checks if the value of the field is `PER_TRAN`"]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == TYPESELECT_A::PER_TRAN
    }
}
#[doc = "Field `TYPE` writer - Channel x Transfer Type"]
pub type TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, TYPESELECT_A, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut W {
        self.variant(TYPESELECT_A::MEM_TRAN)
    }
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut W {
        self.variant(TYPESELECT_A::PER_TRAN)
    }
}
#[doc = "Field `MBSIZE` reader - Channel x Memory Burst Size"]
pub type MBSIZE_R = crate::FieldReader<u8, MBSIZESELECT_A>;
#[doc = "Channel x Memory Burst Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBSIZESELECT_A {
    #[doc = "0: The memory burst size is set to one."]
    SINGLE = 0,
    #[doc = "1: The memory burst size is set to four."]
    FOUR = 1,
    #[doc = "2: The memory burst size is set to eight."]
    EIGHT = 2,
    #[doc = "3: The memory burst size is set to sixteen."]
    SIXTEEN = 3,
}
impl From<MBSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSIZESELECT_A) -> Self {
        variant as _
    }
}
impl MBSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBSIZESELECT_A {
        match self.bits {
            0 => MBSIZESELECT_A::SINGLE,
            1 => MBSIZESELECT_A::FOUR,
            2 => MBSIZESELECT_A::EIGHT,
            3 => MBSIZESELECT_A::SIXTEEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MBSIZESELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == MBSIZESELECT_A::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == MBSIZESELECT_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == MBSIZESELECT_A::SIXTEEN
    }
}
#[doc = "Field `MBSIZE` writer - Channel x Memory Burst Size"]
pub type MBSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CC_SPEC, u8, MBSIZESELECT_A, 2, O>;
impl<'a, const O: u8> MBSIZE_W<'a, O> {
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBSIZESELECT_A::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(MBSIZESELECT_A::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(MBSIZESELECT_A::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(MBSIZESELECT_A::SIXTEEN)
    }
}
#[doc = "Field `DSYNC` reader - Channel x Synchronization"]
pub type DSYNC_R = crate::BitReader<DSYNCSELECT_A>;
#[doc = "Channel x Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSYNCSELECT_A {
    #[doc = "0: Peripheral to Memory transfer."]
    PER2MEM = 0,
    #[doc = "1: Memory to Peripheral transfer."]
    MEM2PER = 1,
}
impl From<DSYNCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DSYNCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSYNCSELECT_A {
        match self.bits {
            false => DSYNCSELECT_A::PER2MEM,
            true => DSYNCSELECT_A::MEM2PER,
        }
    }
    #[doc = "Checks if the value of the field is `PER2MEM`"]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == DSYNCSELECT_A::PER2MEM
    }
    #[doc = "Checks if the value of the field is `MEM2PER`"]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == DSYNCSELECT_A::MEM2PER
    }
}
#[doc = "Field `DSYNC` writer - Channel x Synchronization"]
pub type DSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, DSYNCSELECT_A, O>;
impl<'a, const O: u8> DSYNC_W<'a, O> {
    #[doc = "Peripheral to Memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut W {
        self.variant(DSYNCSELECT_A::PER2MEM)
    }
    #[doc = "Memory to Peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut W {
        self.variant(DSYNCSELECT_A::MEM2PER)
    }
}
#[doc = "Field `SWREQ` reader - Channel x Software Request Trigger"]
pub type SWREQ_R = crate::BitReader<SWREQSELECT_A>;
#[doc = "Channel x Software Request Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQSELECT_A {
    #[doc = "0: Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED = 0,
    #[doc = "1: Software request is connected to the peripheral request line."]
    SWR_CONNECTED = 1,
}
impl From<SWREQSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWREQSELECT_A {
        match self.bits {
            false => SWREQSELECT_A::HWR_CONNECTED,
            true => SWREQSELECT_A::SWR_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `HWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == SWREQSELECT_A::HWR_CONNECTED
    }
    #[doc = "Checks if the value of the field is `SWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == SWREQSELECT_A::SWR_CONNECTED
    }
}
#[doc = "Field `SWREQ` writer - Channel x Software Request Trigger"]
pub type SWREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, SWREQSELECT_A, O>;
impl<'a, const O: u8> SWREQ_W<'a, O> {
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut W {
        self.variant(SWREQSELECT_A::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut W {
        self.variant(SWREQSELECT_A::SWR_CONNECTED)
    }
}
#[doc = "Field `MEMSET` reader - Channel x Fill Block of memory"]
pub type MEMSET_R = crate::BitReader<MEMSETSELECT_A>;
#[doc = "Channel x Fill Block of memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMSETSELECT_A {
    #[doc = "0: Memset is not activated."]
    NORMAL_MODE = 0,
    #[doc = "1: Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE = 1,
}
impl From<MEMSETSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMSETSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMSETSELECT_A {
        match self.bits {
            false => MEMSETSELECT_A::NORMAL_MODE,
            true => MEMSETSELECT_A::HW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == MEMSETSELECT_A::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `HW_MODE`"]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == MEMSETSELECT_A::HW_MODE
    }
}
#[doc = "Field `MEMSET` writer - Channel x Fill Block of memory"]
pub type MEMSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, MEMSETSELECT_A, O>;
impl<'a, const O: u8> MEMSET_W<'a, O> {
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(MEMSETSELECT_A::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut W {
        self.variant(MEMSETSELECT_A::HW_MODE)
    }
}
#[doc = "Field `CSIZE` reader - Channel x Chunk Size"]
pub type CSIZE_R = crate::FieldReader<u8, CSIZESELECT_A>;
#[doc = "Channel x Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSIZESELECT_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 2 data transferred"]
    CHK_2 = 1,
    #[doc = "2: 4 data transferred"]
    CHK_4 = 2,
    #[doc = "3: 8 data transferred"]
    CHK_8 = 3,
    #[doc = "4: 16 data transferred"]
    CHK_16 = 4,
}
impl From<CSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESELECT_A) -> Self {
        variant as _
    }
}
impl CSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZESELECT_A> {
        match self.bits {
            0 => Some(CSIZESELECT_A::CHK_1),
            1 => Some(CSIZESELECT_A::CHK_2),
            2 => Some(CSIZESELECT_A::CHK_4),
            3 => Some(CSIZESELECT_A::CHK_8),
            4 => Some(CSIZESELECT_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == CSIZESELECT_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_2`"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == CSIZESELECT_A::CHK_2
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == CSIZESELECT_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == CSIZESELECT_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == CSIZESELECT_A::CHK_16
    }
}
#[doc = "Field `CSIZE` writer - Channel x Chunk Size"]
pub type CSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u8, CSIZESELECT_A, 3, O>;
impl<'a, const O: u8> CSIZE_W<'a, O> {
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(CSIZESELECT_A::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut W {
        self.variant(CSIZESELECT_A::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(CSIZESELECT_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(CSIZESELECT_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(CSIZESELECT_A::CHK_16)
    }
}
#[doc = "Field `DWIDTH` reader - Channel x Data Width"]
pub type DWIDTH_R = crate::FieldReader<u8, DWIDTHSELECT_A>;
#[doc = "Channel x Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWIDTHSELECT_A {
    #[doc = "0: The data size is set to 8 bits"]
    BYTE = 0,
    #[doc = "1: The data size is set to 16 bits"]
    HALFWORD = 1,
    #[doc = "2: The data size is set to 32 bits"]
    WORD = 2,
}
impl From<DWIDTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DWIDTHSELECT_A) -> Self {
        variant as _
    }
}
impl DWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWIDTHSELECT_A> {
        match self.bits {
            0 => Some(DWIDTHSELECT_A::BYTE),
            1 => Some(DWIDTHSELECT_A::HALFWORD),
            2 => Some(DWIDTHSELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DWIDTHSELECT_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DWIDTHSELECT_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DWIDTHSELECT_A::WORD
    }
}
#[doc = "Field `DWIDTH` writer - Channel x Data Width"]
pub type DWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u8, DWIDTHSELECT_A, 2, O>;
impl<'a, const O: u8> DWIDTH_W<'a, O> {
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DWIDTHSELECT_A::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DWIDTHSELECT_A::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DWIDTHSELECT_A::WORD)
    }
}
#[doc = "Field `SIF` reader - Channel x Source Interface Identifier"]
pub type SIF_R = crate::BitReader<SIFSELECT_A>;
#[doc = "Channel x Source Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIFSELECT_A {
    #[doc = "0: The data is read through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is read through the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<SIFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SIFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIFSELECT_A {
        match self.bits {
            false => SIFSELECT_A::AHB_IF0,
            true => SIFSELECT_A::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIFSELECT_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIFSELECT_A::AHB_IF1
    }
}
#[doc = "Field `SIF` writer - Channel x Source Interface Identifier"]
pub type SIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, SIFSELECT_A, O>;
impl<'a, const O: u8> SIF_W<'a, O> {
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIFSELECT_A::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIFSELECT_A::AHB_IF1)
    }
}
#[doc = "Field `DIF` reader - Channel x Destination Interface Identifier"]
pub type DIF_R = crate::BitReader<DIFSELECT_A>;
#[doc = "Channel x Destination Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSELECT_A {
    #[doc = "0: The data is written through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is written though the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<DIFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSELECT_A {
        match self.bits {
            false => DIFSELECT_A::AHB_IF0,
            true => DIFSELECT_A::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIFSELECT_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIFSELECT_A::AHB_IF1
    }
}
#[doc = "Field `DIF` writer - Channel x Destination Interface Identifier"]
pub type DIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, DIFSELECT_A, O>;
impl<'a, const O: u8> DIF_W<'a, O> {
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIFSELECT_A::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIFSELECT_A::AHB_IF1)
    }
}
#[doc = "Field `SAM` reader - Channel x Source Addressing Mode"]
pub type SAM_R = crate::FieldReader<u8, SAMSELECT_A>;
#[doc = "Channel x Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMSELECT_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<SAMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMSELECT_A) -> Self {
        variant as _
    }
}
impl SAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMSELECT_A {
        match self.bits {
            0 => SAMSELECT_A::FIXED_AM,
            1 => SAMSELECT_A::INCREMENTED_AM,
            2 => SAMSELECT_A::UBS_AM,
            3 => SAMSELECT_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == SAMSELECT_A::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == SAMSELECT_A::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == SAMSELECT_A::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == SAMSELECT_A::UBS_DS_AM
    }
}
#[doc = "Field `SAM` writer - Channel x Source Addressing Mode"]
pub type SAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CC_SPEC, u8, SAMSELECT_A, 2, O>;
impl<'a, const O: u8> SAM_W<'a, O> {
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(SAMSELECT_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(SAMSELECT_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(SAMSELECT_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(SAMSELECT_A::UBS_DS_AM)
    }
}
#[doc = "Field `DAM` reader - Channel x Destination Addressing Mode"]
pub type DAM_R = crate::FieldReader<u8, DAMSELECT_A>;
#[doc = "Channel x Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAMSELECT_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<DAMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAMSELECT_A) -> Self {
        variant as _
    }
}
impl DAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAMSELECT_A {
        match self.bits {
            0 => DAMSELECT_A::FIXED_AM,
            1 => DAMSELECT_A::INCREMENTED_AM,
            2 => DAMSELECT_A::UBS_AM,
            3 => DAMSELECT_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == DAMSELECT_A::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == DAMSELECT_A::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == DAMSELECT_A::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == DAMSELECT_A::UBS_DS_AM
    }
}
#[doc = "Field `DAM` writer - Channel x Destination Addressing Mode"]
pub type DAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CC_SPEC, u8, DAMSELECT_A, 2, O>;
impl<'a, const O: u8> DAM_W<'a, O> {
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(DAMSELECT_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(DAMSELECT_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(DAMSELECT_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(DAMSELECT_A::UBS_DS_AM)
    }
}
#[doc = "Field `INITD` reader - Channel Initialization Terminated (this bit is read-only)"]
pub type INITD_R = crate::BitReader<INITDSELECT_A>;
#[doc = "Channel Initialization Terminated (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITDSELECT_A {
    #[doc = "0: Channel initialization is in progress."]
    IN_PROGRESS = 0,
    #[doc = "1: Channel initialization is completed."]
    TERMINATED = 1,
}
impl From<INITDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INITDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INITD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITDSELECT_A {
        match self.bits {
            false => INITDSELECT_A::IN_PROGRESS,
            true => INITDSELECT_A::TERMINATED,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == INITDSELECT_A::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TERMINATED`"]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == INITDSELECT_A::TERMINATED
    }
}
#[doc = "Field `INITD` writer - Channel Initialization Terminated (this bit is read-only)"]
pub type INITD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, INITDSELECT_A, O>;
impl<'a, const O: u8> INITD_W<'a, O> {
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(INITDSELECT_A::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut W {
        self.variant(INITDSELECT_A::TERMINATED)
    }
}
#[doc = "Field `RDIP` reader - Read in Progress (this bit is read-only)"]
pub type RDIP_R = crate::BitReader<RDIPSELECT_A>;
#[doc = "Read in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIPSELECT_A {
    #[doc = "0: No Active read transaction on the bus."]
    DONE = 0,
    #[doc = "1: A read transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<RDIPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RDIPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIPSELECT_A {
        match self.bits {
            false => RDIPSELECT_A::DONE,
            true => RDIPSELECT_A::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RDIPSELECT_A::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RDIPSELECT_A::IN_PROGRESS
    }
}
#[doc = "Field `RDIP` writer - Read in Progress (this bit is read-only)"]
pub type RDIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, RDIPSELECT_A, O>;
impl<'a, const O: u8> RDIP_W<'a, O> {
    #[doc = "No Active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(RDIPSELECT_A::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(RDIPSELECT_A::IN_PROGRESS)
    }
}
#[doc = "Field `WRIP` reader - Write in Progress (this bit is read-only)"]
pub type WRIP_R = crate::BitReader<WRIPSELECT_A>;
#[doc = "Write in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRIPSELECT_A {
    #[doc = "0: No Active write transaction on the bus."]
    DONE = 0,
    #[doc = "1: A Write transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<WRIPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WRIPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WRIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRIPSELECT_A {
        match self.bits {
            false => WRIPSELECT_A::DONE,
            true => WRIPSELECT_A::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == WRIPSELECT_A::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == WRIPSELECT_A::IN_PROGRESS
    }
}
#[doc = "Field `WRIP` writer - Write in Progress (this bit is read-only)"]
pub type WRIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC_SPEC, WRIPSELECT_A, O>;
impl<'a, const O: u8> WRIP_W<'a, O> {
    #[doc = "No Active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(WRIPSELECT_A::DONE)
    }
    #[doc = "A Write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(WRIPSELECT_A::IN_PROGRESS)
    }
}
#[doc = "Field `PERID` reader - Channel x Peripheral Hardware Request Line Identifier"]
pub type PERID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERID` writer - Channel x Peripheral Hardware Request Line Identifier"]
pub type PERID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MBSIZE_R {
        MBSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DSYNC_R {
        DSYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DAM_R {
        DAM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WRIP_R {
        WRIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<0> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    #[must_use]
    pub fn mbsize(&mut self) -> MBSIZE_W<1> {
        MBSIZE_W::new(self)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dsync(&mut self) -> DSYNC_W<4> {
        DSYNC_W::new(self)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<6> {
        SWREQ_W::new(self)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    #[must_use]
    pub fn memset(&mut self) -> MEMSET_W<7> {
        MEMSET_W::new(self)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn csize(&mut self) -> CSIZE_W<8> {
        CSIZE_W::new(self)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<11> {
        DWIDTH_W::new(self)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sif(&mut self) -> SIF_W<13> {
        SIF_W::new(self)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn dif(&mut self) -> DIF_W<14> {
        DIF_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<16> {
        SAM_W::new(self)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dam(&mut self) -> DAM_W<18> {
        DAM_W::new(self)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn initd(&mut self) -> INITD_W<21> {
        INITD_W::new(self)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rdip(&mut self) -> RDIP_W<22> {
        RDIP_W::new(self)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn wrip(&mut self) -> WRIP_W<23> {
        WRIP_W::new(self)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn perid(&mut self) -> PERID_W<24> {
        PERID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
