#[doc = "Register `DSI_CLK` reader"]
pub struct R(crate::R<DSI_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_CLK` writer"]
pub struct W(crate::W<DSI_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_CLK_SPEC>;
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
impl From<crate::W<DSI_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_GATING` reader - Gating Clock"]
pub struct CLK_GATING_R(crate::FieldReader<bool, CLK_GATING_A>);
impl CLK_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK_GATING_A::ON
    }
}
impl core::ops::Deref for CLK_GATING_R {
    type Target = crate::FieldReader<bool, CLK_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_GATING` writer - Gating Clock"]
pub struct CLK_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_GATING_A::ON)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    HOSC = 0,
    #[doc = "1: `1`"]
    PLL_PERI_1X = 1,
    #[doc = "2: `10`"]
    PLL_VIDEO0_2X = 2,
    #[doc = "3: `11`"]
    PLL_VIDEO1_2X = 3,
    #[doc = "4: `100`"]
    PLL_AUDIO1_DIV2 = 4,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select"]
pub struct CLK_SRC_SEL_R(crate::FieldReader<u8, CLK_SRC_SEL_A>);
impl CLK_SRC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::HOSC),
            1 => Some(CLK_SRC_SEL_A::PLL_PERI_1X),
            2 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_2X),
            3 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_2X),
            4 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        **self == CLK_SRC_SEL_A::HOSC
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_1X`"]
    #[inline(always)]
    pub fn is_pll_peri_1x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_PERI_1X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_2X`"]
    #[inline(always)]
    pub fn is_pll_video0_2x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO0_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_2X`"]
    #[inline(always)]
    pub fn is_pll_video1_2x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO1_2X
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
}
impl core::ops::Deref for CLK_SRC_SEL_R {
    type Target = crate::FieldReader<u8, CLK_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select"]
pub struct CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::HOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_peri_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_1X)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_video0_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_video1_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_2X)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FACTOR_M` reader - Factor M"]
pub struct FACTOR_M_R(crate::FieldReader<u8, u8>);
impl FACTOR_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FACTOR_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FACTOR_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FACTOR_M` writer - Factor M"]
pub struct FACTOR_M_W<'a> {
    w: &'a mut W,
}
impl<'a> FACTOR_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&self) -> FACTOR_M_R {
        FACTOR_M_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&mut self) -> CLK_GATING_W {
        CLK_GATING_W { w: self }
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - Factor M"]
    #[inline(always)]
    pub fn factor_m(&mut self) -> FACTOR_M_W {
        FACTOR_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_clk](index.html) module"]
pub struct DSI_CLK_SPEC;
impl crate::RegisterSpec for DSI_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_clk::R](R) reader structure"]
impl crate::Readable for DSI_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_clk::W](W) writer structure"]
impl crate::Writable for DSI_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_CLK to value 0"]
impl crate::Resettable for DSI_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
