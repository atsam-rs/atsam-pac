#[doc = "Register `FCMD` reader"]
pub struct R(crate::R<FCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCMD` writer"]
pub struct W(crate::W<FCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCMD_SPEC>;
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
impl From<crate::W<FCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCMD_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CMD` reader - Command"]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NOP),
            1 => Some(CMD_A::WP),
            2 => Some(CMD_A::EP),
            3 => Some(CMD_A::CPB),
            4 => Some(CMD_A::LP),
            5 => Some(CMD_A::UP),
            6 => Some(CMD_A::EA),
            7 => Some(CMD_A::WGPB),
            8 => Some(CMD_A::EGPB),
            9 => Some(CMD_A::SSB),
            10 => Some(CMD_A::PGPFB),
            11 => Some(CMD_A::EAGPF),
            12 => Some(CMD_A::QPR),
            13 => Some(CMD_A::WUP),
            14 => Some(CMD_A::EUP),
            15 => Some(CMD_A::QPRUP),
            16 => Some(CMD_A::HSEN),
            17 => Some(CMD_A::HSDIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        **self == CMD_A::NOP
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        **self == CMD_A::WP
    }
    #[doc = "Checks if the value of the field is `EP`"]
    #[inline(always)]
    pub fn is_ep(&self) -> bool {
        **self == CMD_A::EP
    }
    #[doc = "Checks if the value of the field is `CPB`"]
    #[inline(always)]
    pub fn is_cpb(&self) -> bool {
        **self == CMD_A::CPB
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        **self == CMD_A::LP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        **self == CMD_A::UP
    }
    #[doc = "Checks if the value of the field is `EA`"]
    #[inline(always)]
    pub fn is_ea(&self) -> bool {
        **self == CMD_A::EA
    }
    #[doc = "Checks if the value of the field is `WGPB`"]
    #[inline(always)]
    pub fn is_wgpb(&self) -> bool {
        **self == CMD_A::WGPB
    }
    #[doc = "Checks if the value of the field is `EGPB`"]
    #[inline(always)]
    pub fn is_egpb(&self) -> bool {
        **self == CMD_A::EGPB
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        **self == CMD_A::SSB
    }
    #[doc = "Checks if the value of the field is `PGPFB`"]
    #[inline(always)]
    pub fn is_pgpfb(&self) -> bool {
        **self == CMD_A::PGPFB
    }
    #[doc = "Checks if the value of the field is `EAGPF`"]
    #[inline(always)]
    pub fn is_eagpf(&self) -> bool {
        **self == CMD_A::EAGPF
    }
    #[doc = "Checks if the value of the field is `QPR`"]
    #[inline(always)]
    pub fn is_qpr(&self) -> bool {
        **self == CMD_A::QPR
    }
    #[doc = "Checks if the value of the field is `WUP`"]
    #[inline(always)]
    pub fn is_wup(&self) -> bool {
        **self == CMD_A::WUP
    }
    #[doc = "Checks if the value of the field is `EUP`"]
    #[inline(always)]
    pub fn is_eup(&self) -> bool {
        **self == CMD_A::EUP
    }
    #[doc = "Checks if the value of the field is `QPRUP`"]
    #[inline(always)]
    pub fn is_qprup(&self) -> bool {
        **self == CMD_A::QPRUP
    }
    #[doc = "Checks if the value of the field is `HSEN`"]
    #[inline(always)]
    pub fn is_hsen(&self) -> bool {
        **self == CMD_A::HSEN
    }
    #[doc = "Checks if the value of the field is `HSDIS`"]
    #[inline(always)]
    pub fn is_hsdis(&self) -> bool {
        **self == CMD_A::HSDIS
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command"]
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
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `PAGEN` reader - Page number"]
pub struct PAGEN_R(crate::FieldReader<u16, u16>);
impl PAGEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PAGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGEN` writer - Page number"]
pub struct PAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
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
#[doc = "Field `KEY` reader - Write protection key"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == KEY_A::KEY
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Write protection key"]
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
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcmd](index.html) module"]
pub struct FCMD_SPEC;
impl crate::RegisterSpec for FCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcmd::R](R) reader structure"]
impl crate::Readable for FCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcmd::W](W) writer structure"]
impl crate::Writable for FCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCMD to value 0"]
impl crate::Resettable for FCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
