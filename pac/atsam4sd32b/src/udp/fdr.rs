#[doc = "Reader of register FDR[%s]"]
pub type R = crate::R<u32, super::FDR>;
#[doc = "Writer for register FDR[%s]"]
pub type W = crate::W<u32, super::FDR>;
#[doc = "Reader of field `FIFO_DATA`"]
pub type FIFO_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_DATA`"]
pub struct FIFO_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FIFO_DATA_R {
        FIFO_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    pub fn fifo_data(&mut self) -> FIFO_DATA_W {
        FIFO_DATA_W { w: self }
    }
}
