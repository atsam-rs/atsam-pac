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
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<u8, CMDSELECT_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_A {
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
impl From<CMDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSELECT_A> {
        match self.bits {
            0 => Some(CMDSELECT_A::NOP),
            1 => Some(CMDSELECT_A::WP),
            2 => Some(CMDSELECT_A::EP),
            3 => Some(CMDSELECT_A::CPB),
            4 => Some(CMDSELECT_A::LP),
            5 => Some(CMDSELECT_A::UP),
            6 => Some(CMDSELECT_A::EA),
            7 => Some(CMDSELECT_A::WGPB),
            8 => Some(CMDSELECT_A::EGPB),
            9 => Some(CMDSELECT_A::SSB),
            10 => Some(CMDSELECT_A::PGPFB),
            11 => Some(CMDSELECT_A::EAGPF),
            12 => Some(CMDSELECT_A::QPR),
            13 => Some(CMDSELECT_A::WUP),
            14 => Some(CMDSELECT_A::EUP),
            15 => Some(CMDSELECT_A::QPRUP),
            16 => Some(CMDSELECT_A::HSEN),
            17 => Some(CMDSELECT_A::HSDIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == CMDSELECT_A::NOP
    }
    #[doc = "Checks if the value of the field is `WP`"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == CMDSELECT_A::WP
    }
    #[doc = "Checks if the value of the field is `EP`"]
    #[inline(always)]
    pub fn is_ep(&self) -> bool {
        *self == CMDSELECT_A::EP
    }
    #[doc = "Checks if the value of the field is `CPB`"]
    #[inline(always)]
    pub fn is_cpb(&self) -> bool {
        *self == CMDSELECT_A::CPB
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == CMDSELECT_A::LP
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == CMDSELECT_A::UP
    }
    #[doc = "Checks if the value of the field is `EA`"]
    #[inline(always)]
    pub fn is_ea(&self) -> bool {
        *self == CMDSELECT_A::EA
    }
    #[doc = "Checks if the value of the field is `WGPB`"]
    #[inline(always)]
    pub fn is_wgpb(&self) -> bool {
        *self == CMDSELECT_A::WGPB
    }
    #[doc = "Checks if the value of the field is `EGPB`"]
    #[inline(always)]
    pub fn is_egpb(&self) -> bool {
        *self == CMDSELECT_A::EGPB
    }
    #[doc = "Checks if the value of the field is `SSB`"]
    #[inline(always)]
    pub fn is_ssb(&self) -> bool {
        *self == CMDSELECT_A::SSB
    }
    #[doc = "Checks if the value of the field is `PGPFB`"]
    #[inline(always)]
    pub fn is_pgpfb(&self) -> bool {
        *self == CMDSELECT_A::PGPFB
    }
    #[doc = "Checks if the value of the field is `EAGPF`"]
    #[inline(always)]
    pub fn is_eagpf(&self) -> bool {
        *self == CMDSELECT_A::EAGPF
    }
    #[doc = "Checks if the value of the field is `QPR`"]
    #[inline(always)]
    pub fn is_qpr(&self) -> bool {
        *self == CMDSELECT_A::QPR
    }
    #[doc = "Checks if the value of the field is `WUP`"]
    #[inline(always)]
    pub fn is_wup(&self) -> bool {
        *self == CMDSELECT_A::WUP
    }
    #[doc = "Checks if the value of the field is `EUP`"]
    #[inline(always)]
    pub fn is_eup(&self) -> bool {
        *self == CMDSELECT_A::EUP
    }
    #[doc = "Checks if the value of the field is `QPRUP`"]
    #[inline(always)]
    pub fn is_qprup(&self) -> bool {
        *self == CMDSELECT_A::QPRUP
    }
    #[doc = "Checks if the value of the field is `HSEN`"]
    #[inline(always)]
    pub fn is_hsen(&self) -> bool {
        *self == CMDSELECT_A::HSEN
    }
    #[doc = "Checks if the value of the field is `HSDIS`"]
    #[inline(always)]
    pub fn is_hsdis(&self) -> bool {
        *self == CMDSELECT_A::HSDIS
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCMD_SPEC, u8, CMDSELECT_A, 6, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "No Operation"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(CMDSELECT_A::NOP)
    }
    #[doc = "Write Page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDSELECT_A::WP)
    }
    #[doc = "Erase Page"]
    #[inline(always)]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMDSELECT_A::EP)
    }
    #[doc = "Clear Page Buffer"]
    #[inline(always)]
    pub fn cpb(self) -> &'a mut W {
        self.variant(CMDSELECT_A::CPB)
    }
    #[doc = "Lock Region containing page"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(CMDSELECT_A::LP)
    }
    #[doc = "Unlock Region containing page"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(CMDSELECT_A::UP)
    }
    #[doc = "Erase All, including secuity and fuse bits"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(CMDSELECT_A::EA)
    }
    #[doc = "Write General-Purpose fuse Bit"]
    #[inline(always)]
    pub fn wgpb(self) -> &'a mut W {
        self.variant(CMDSELECT_A::WGPB)
    }
    #[doc = "Erase General-Purpose fuse Bit"]
    #[inline(always)]
    pub fn egpb(self) -> &'a mut W {
        self.variant(CMDSELECT_A::EGPB)
    }
    #[doc = "Set Security Bit"]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMDSELECT_A::SSB)
    }
    #[doc = "Program GPFuse Byte"]
    #[inline(always)]
    pub fn pgpfb(self) -> &'a mut W {
        self.variant(CMDSELECT_A::PGPFB)
    }
    #[doc = "Erase All GP Fuses"]
    #[inline(always)]
    pub fn eagpf(self) -> &'a mut W {
        self.variant(CMDSELECT_A::EAGPF)
    }
    #[doc = "Quick Page Read"]
    #[inline(always)]
    pub fn qpr(self) -> &'a mut W {
        self.variant(CMDSELECT_A::QPR)
    }
    #[doc = "Write User Page"]
    #[inline(always)]
    pub fn wup(self) -> &'a mut W {
        self.variant(CMDSELECT_A::WUP)
    }
    #[doc = "Erase User Page"]
    #[inline(always)]
    pub fn eup(self) -> &'a mut W {
        self.variant(CMDSELECT_A::EUP)
    }
    #[doc = "Quick Page Read User Page"]
    #[inline(always)]
    pub fn qprup(self) -> &'a mut W {
        self.variant(CMDSELECT_A::QPRUP)
    }
    #[doc = "High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsen(self) -> &'a mut W {
        self.variant(CMDSELECT_A::HSEN)
    }
    #[doc = "High Speed Mode Disable"]
    #[inline(always)]
    pub fn hsdis(self) -> &'a mut W {
        self.variant(CMDSELECT_A::HSDIS)
    }
}
#[doc = "Field `PAGEN` reader - Page number"]
pub type PAGEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAGEN` writer - Page number"]
pub type PAGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCMD_SPEC, u16, u16, 16, O>;
#[doc = "Field `KEY` reader - Write protection key"]
pub type KEY_R = crate::FieldReader<u8, KEYSELECT_A>;
#[doc = "Write protection key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "165: `10100101`"]
    KEY = 165,
}
impl From<KEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_A) -> Self {
        variant as _
    }
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSELECT_A> {
        match self.bits {
            165 => Some(KEYSELECT_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEYSELECT_A::KEY
    }
}
#[doc = "Field `KEY` writer - Write protection key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCMD_SPEC, u8, KEYSELECT_A, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "`10100101`"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEYSELECT_A::KEY)
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
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 8:23 - Page number"]
    #[inline(always)]
    #[must_use]
    pub fn pagen(&mut self) -> PAGEN_W<8> {
        PAGEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Write protection key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCMD to value 0"]
impl crate::Resettable for FCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
