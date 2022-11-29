#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEN` reader - Slave Enable"]
pub type SEN_R = crate::BitReader<bool>;
#[doc = "Field `SEN` writer - Slave Enable"]
pub type SEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMEN` reader - SMBus Mode Enable"]
pub type SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SMEN` writer - SMBus Mode Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMATCH` reader - Slave Address Match"]
pub type SMATCH_R = crate::BitReader<bool>;
#[doc = "Field `SMATCH` writer - Slave Address Match"]
pub type SMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `GCMATCH` reader - General Call Address Match"]
pub type GCMATCH_R = crate::BitReader<bool>;
#[doc = "Field `GCMATCH` writer - General Call Address Match"]
pub type GCMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STREN` reader - Clock Stretch Enable"]
pub type STREN_R = crate::BitReader<bool>;
#[doc = "Field `STREN` writer - Clock Stretch Enable"]
pub type STREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMBALERT` reader - SMBus Alert"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - SMBus Alert"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub type SMDA_R = crate::BitReader<bool>;
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub type SMDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub type SMHH_R = crate::BitReader<bool>;
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub type SMHH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PECEN` reader - Packet Error Checking Enable"]
pub type PECEN_R = crate::BitReader<bool>;
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ACK` reader - Slave Receiver Data Phase ACK Value"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - Slave Receiver Data Phase ACK Value"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CUP` reader - NBYTES Count Up"]
pub type CUP_R = crate::BitReader<bool>;
#[doc = "Field `CUP` writer - NBYTES Count Up"]
pub type CUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SOAM` reader - Stretch Clock on Address Match"]
pub type SOAM_R = crate::BitReader<bool>;
#[doc = "Field `SOAM` writer - Stretch Clock on Address Match"]
pub type SOAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SODR` reader - Stretch Clock on Data Byte Reception"]
pub type SODR_R = crate::BitReader<bool>;
#[doc = "Field `SODR` writer - Stretch Clock on Data Byte Reception"]
pub type SODR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ADR` reader - Slave Address"]
pub type ADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADR` writer - Slave Address"]
pub type ADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, u16, 10, O>;
#[doc = "Field `TENBIT` reader - Ten Bit Address Match"]
pub type TENBIT_R = crate::BitReader<bool>;
#[doc = "Field `TENBIT` writer - Ten Bit Address Match"]
pub type TENBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BRIDGE` reader - Bridge Control Enable"]
pub type BRIDGE_R = crate::BitReader<bool>;
#[doc = "Field `BRIDGE` writer - Bridge Control Enable"]
pub type BRIDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    pub fn smatch(&self) -> SMATCH_R {
        SMATCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    pub fn gcmatch(&self) -> GCMATCH_R {
        GCMATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    pub fn cup(&self) -> CUP_R {
        CUP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    pub fn soam(&self) -> SOAM_R {
        SOAM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    pub fn sodr(&self) -> SODR_R {
        SODR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    pub fn bridge(&self) -> BRIDGE_R {
        BRIDGE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<0> {
        SEN_W::new(self)
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn smatch(&mut self) -> SMATCH_W<2> {
        SMATCH_W::new(self)
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn gcmatch(&mut self) -> GCMATCH_W<3> {
        GCMATCH_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stren(&mut self) -> STREN_W<4> {
        STREN_W::new(self)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<8> {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    #[must_use]
    pub fn smda(&mut self) -> SMDA_W<9> {
        SMDA_W::new(self)
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    #[must_use]
    pub fn smhh(&mut self) -> SMHH_W<10> {
        SMHH_W::new(self)
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<11> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<12> {
        ACK_W::new(self)
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    #[must_use]
    pub fn cup(&mut self) -> CUP_W<13> {
        CUP_W::new(self)
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn soam(&mut self) -> SOAM_W<14> {
        SOAM_W::new(self)
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    #[must_use]
    pub fn sodr(&mut self) -> SODR_W<15> {
        SODR_W::new(self)
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn adr(&mut self) -> ADR_W<16> {
        ADR_W::new(self)
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn tenbit(&mut self) -> TENBIT_W<26> {
        TENBIT_W::new(self)
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bridge(&mut self) -> BRIDGE_W<27> {
        BRIDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
