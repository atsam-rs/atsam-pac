#[doc = "Reader of register RC"]
pub type R = crate::R<u32, super::RC>;
#[doc = "Writer for register RC"]
pub type W = crate::W<u32, super::RC>;
#[doc = "Register RC `reset()`'s with value 0"]
impl crate::ResetValue for super::RC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RC`"]
pub type RC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RC`"]
pub struct RC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register C"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W {
        RC_W { w: self }
    }
}