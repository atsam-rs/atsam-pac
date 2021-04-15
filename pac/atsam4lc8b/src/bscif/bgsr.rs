#[doc = "Reader of register BGSR"]
pub type R = crate::R<u32, super::BGSR>;
#[doc = "Bandgap Buffer Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BGBUFRDY_A {
    #[doc = "1: `1`"]
    FLASH = 1,
    #[doc = "2: `10`"]
    PLL = 2,
    #[doc = "4: `100`"]
    VREG = 4,
    #[doc = "8: `1000`"]
    BUFRR = 8,
    #[doc = "16: `10000`"]
    ADC = 16,
    #[doc = "32: `100000`"]
    LCD = 32,
}
impl From<BGBUFRDY_A> for u8 {
    #[inline(always)]
    fn from(variant: BGBUFRDY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BGBUFRDY`"]
pub type BGBUFRDY_R = crate::R<u8, BGBUFRDY_A>;
impl BGBUFRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BGBUFRDY_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BGBUFRDY_A::FLASH),
            2 => Val(BGBUFRDY_A::PLL),
            4 => Val(BGBUFRDY_A::VREG),
            8 => Val(BGBUFRDY_A::BUFRR),
            16 => Val(BGBUFRDY_A::ADC),
            32 => Val(BGBUFRDY_A::LCD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == BGBUFRDY_A::FLASH
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == BGBUFRDY_A::PLL
    }
    #[doc = "Checks if the value of the field is `VREG`"]
    #[inline(always)]
    pub fn is_vreg(&self) -> bool {
        *self == BGBUFRDY_A::VREG
    }
    #[doc = "Checks if the value of the field is `BUFRR`"]
    #[inline(always)]
    pub fn is_bufrr(&self) -> bool {
        *self == BGBUFRDY_A::BUFRR
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == BGBUFRDY_A::ADC
    }
    #[doc = "Checks if the value of the field is `LCD`"]
    #[inline(always)]
    pub fn is_lcd(&self) -> bool {
        *self == BGBUFRDY_A::LCD
    }
}
#[doc = "Reader of field `BGRDY`"]
pub type BGRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPBGRDY`"]
pub type LPBGRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREF`"]
pub type VREF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Bandgap Buffer Ready"]
    #[inline(always)]
    pub fn bgbufrdy(&self) -> BGBUFRDY_R {
        BGBUFRDY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn bgrdy(&self) -> BGRDY_R {
        BGRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&self) -> LPBGRDY_R {
        LPBGRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Voltage Reference Used by the System"]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
