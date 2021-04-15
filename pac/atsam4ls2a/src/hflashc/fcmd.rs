#[doc = "Reader of register FCMD"]
pub type R = crate::R<u32, super::FCMD>;
#[doc = "Writer for register FCMD"]
pub type W = crate::W<u32, super::FCMD>;
#[doc = "Register FCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::FCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Operation"]
    NOP = 0,
    #[doc = "1: Write Page"]
    WP = 1,
    #[doc = "2: Erase Page"]
    EP = 2,
    #[doc = "3: Clear Page Buffer"]
    CPB = 3,
    #[doc = "4: Lock Region containing page"]
    LP = 4,
    #[doc = "5: Unlock Region containing page"]
    UP = 5,
    #[doc = "6: Erase All, including secuity and fuse bits"]
    EA = 6,
    #[doc = "7: Write General-Purpose fuse Bit"]
    WGPB = 7,
    #[doc = "8: Erase General-Purpose fuse Bit"]
    EGPB = 8,
    #[doc = "9: Set Security Bit"]
    SSB = 9,
    #[doc = "10: Program GPFuse Byte"]
    PGPFB = 10,
    #[doc = "11: Erase All GP Fuses"]
    EAGPF = 11,
    #[doc = "12: Quick Page Read"]
    QPR = 12,
    #[doc = "13: Write User Page"]
    WUP = 13,
    #[doc = "14: Erase User Page"]
    EUP = 14,
    #[doc = "15: Quick Page Read User Page"]
    QPRUP = 15,
    #[doc = "16: High Speed Mode Enable"]
    HSEN = 16,
    #[doc = "17: High Speed Mode Disable"]
    HSDIS = 17,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, CMD_A>;
impl CMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_A::NOP),
            1 => Val(CMD_A::WP),
            2 => Val(CMD_A::EP),
            3 => Val(CMD_A::CPB),
            4 => Val(CMD_A::LP),
            5 => Val(CMD_A::UP),
            6 => Val(CMD_A::EA),
            7 => Val(CMD_A::WGPB),
            8 => Val(CMD_A::EGPB),
            9 => Val(CMD_A::SSB),
            10 => Val(CMD_A::PGPFB),
            11 => Val(CMD_A::EAGPF),
            12 => Val(CMD_A::QPR),
            13 => Val(CMD_A::WUP),
            14 => Val(CMD_A::EUP),
            15 => Val(CMD_A::QPRUP),
            16 => Val(CMD_A::HSEN),
            17 => Val(CMD_A::HSDIS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == CMD_A::NOP
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == CMD_A::WP
    }
    #[doc = "Checks if the value of the field is `EP`"]
    #[inline(always)]
    pub fn is_ep(&self) -> bool {
        *self == CMD_A::EP
    }
    #[doc = "Checks if the value of the field is `CPB`"]
    #[inline(always)]
    pub fn is_cpb(&self) -> bool {
        *self == CMD_A::CPB
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == CMD_A::LP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CMD_A::UP
    }
    #[doc = "Checks if the value of the field is `EA`"]
    #[inline(always)]
    pub fn is_ea(&self) -> bool {
        *self == CMD_A::EA
    }
    #[doc = "Checks if the value of the field is `WGPB`"]
    #[inline(always)]
    pub fn is_wgpb(&self) -> bool {
        *self == CMD_A::WGPB
    }
    #[doc = "Checks if the value of the field is `EGPB`"]
    #[inline(always)]
    pub fn is_egpb(&self) -> bool {
        *self == CMD_A::EGPB
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        *self == CMD_A::SSB
    }
    #[doc = "Checks if the value of the field is `PGPFB`"]
    #[inline(always)]
    pub fn is_pgpfb(&self) -> bool {
        *self == CMD_A::PGPFB
    }
    #[doc = "Checks if the value of the field is `EAGPF`"]
    #[inline(always)]
    pub fn is_eagpf(&self) -> bool {
        *self == CMD_A::EAGPF
    }
    #[doc = "Checks if the value of the field is `QPR`"]
    #[inline(always)]
    pub fn is_qpr(&self) -> bool {
        *self == CMD_A::QPR
    }
    #[doc = "Checks if the value of the field is `WUP`"]
    #[inline(always)]
    pub fn is_wup(&self) -> bool {
        *self == CMD_A::WUP
    }
    #[doc = "Checks if the value of the field is `EUP`"]
    #[inline(always)]
    pub fn is_eup(&self) -> bool {
        *self == CMD_A::EUP
    }
    #[doc = "Checks if the value of the field is `QPRUP`"]
    #[inline(always)]
    pub fn is_qprup(&self) -> bool {
        *self == CMD_A::QPRUP
    }
    #[doc = "Checks if the value of the field is `HSEN`"]
    #[inline(always)]
    pub fn is_hsen(&self) -> bool {
        *self == CMD_A::HSEN
    }
    #[doc = "Checks if the value of the field is `HSDIS`"]
    #[inline(always)]
    pub fn is_hsdis(&self) -> bool {
        *self == CMD_A::HSDIS
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(CMD_A::NOP)
    }
    #[doc = "Write Page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMD_A::WP)
    }
    #[doc = "Erase Page"]
    #[inline(always)]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMD_A::EP)
    }
    #[doc = "Clear Page Buffer"]
    #[inline(always)]
    pub fn cpb(self) -> &'a mut W {
        self.variant(CMD_A::CPB)
    }
    #[doc = "Lock Region containing page"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(CMD_A::LP)
    }
    #[doc = "Unlock Region containing page"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CMD_A::UP)
    }
    #[doc = "Erase All, including secuity and fuse bits"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(CMD_A::EA)
    }
    #[doc = "Write General-Purpose fuse Bit"]
    #[inline(always)]
    pub fn wgpb(self) -> &'a mut W {
        self.variant(CMD_A::WGPB)
    }
    #[doc = "Erase General-Purpose fuse Bit"]
    #[inline(always)]
    pub fn egpb(self) -> &'a mut W {
        self.variant(CMD_A::EGPB)
    }
    #[doc = "Set Security Bit"]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMD_A::SSB)
    }
    #[doc = "Program GPFuse Byte"]
    #[inline(always)]
    pub fn pgpfb(self) -> &'a mut W {
        self.variant(CMD_A::PGPFB)
    }
    #[doc = "Erase All GP Fuses"]
    #[inline(always)]
    pub fn eagpf(self) -> &'a mut W {
        self.variant(CMD_A::EAGPF)
    }
    #[doc = "Quick Page Read"]
    #[inline(always)]
    pub fn qpr(self) -> &'a mut W {
        self.variant(CMD_A::QPR)
    }
    #[doc = "Write User Page"]
    #[inline(always)]
    pub fn wup(self) -> &'a mut W {
        self.variant(CMD_A::WUP)
    }
    #[doc = "Erase User Page"]
    #[inline(always)]
    pub fn eup(self) -> &'a mut W {
        self.variant(CMD_A::EUP)
    }
    #[doc = "Quick Page Read User Page"]
    #[inline(always)]
    pub fn qprup(self) -> &'a mut W {
        self.variant(CMD_A::QPRUP)
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsen(self) -> &'a mut W {
        self.variant(CMD_A::HSEN)
    }
    #[doc = "High Speed Mode Disable"]
    #[inline(always)]
    pub fn hsdis(self) -> &'a mut W {
        self.variant(CMD_A::HSDIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PAGEN`"]
pub type PAGEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PAGEN`"]
pub struct PAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Write protection key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: `10100101`"]
    KEY = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(KEY_A::KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEY_A::KEY
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10100101`"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:23 - Page number"]
    #[inline(always)]
    pub fn pagen(&self) -> PAGEN_R {
        PAGEN_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Write protection key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 8:23 - Page number"]
    #[inline(always)]
    pub fn pagen(&mut self) -> PAGEN_W {
        PAGEN_W { w: self }
    }
    #[doc = "Bits 24:31 - Write protection key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
