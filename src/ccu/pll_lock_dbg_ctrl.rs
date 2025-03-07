#[doc = "Register `PLL_LOCK_DBG_CTRL` reader"]
pub struct R(crate::R<PLL_LOCK_DBG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_LOCK_DBG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_LOCK_DBG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_LOCK_DBG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_LOCK_DBG_CTRL` writer"]
pub struct W(crate::W<PLL_LOCK_DBG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_LOCK_DBG_CTRL_SPEC>;
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
impl From<crate::W<PLL_LOCK_DBG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_LOCK_DBG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_FLAG_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_LOCK_FLAG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_FLAG_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_FLAG_EN` reader - Debug Enable"]
pub struct PLL_LOCK_FLAG_EN_R(crate::FieldReader<bool, PLL_LOCK_FLAG_EN_A>);
impl PLL_LOCK_FLAG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LOCK_FLAG_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_FLAG_EN_A {
        match self.bits {
            false => PLL_LOCK_FLAG_EN_A::DISABLE,
            true => PLL_LOCK_FLAG_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PLL_LOCK_FLAG_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLL_LOCK_FLAG_EN_A::ENABLE
    }
}
impl core::ops::Deref for PLL_LOCK_FLAG_EN_R {
    type Target = crate::FieldReader<bool, PLL_LOCK_FLAG_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LOCK_FLAG_EN` writer - Debug Enable"]
pub struct PLL_LOCK_FLAG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK_FLAG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_LOCK_FLAG_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_LOCK_FLAG_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_LOCK_FLAG_EN_A::ENABLE)
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
    PLL_CPUX = 0,
    #[doc = "1: `1`"]
    PLL_DDR = 1,
    #[doc = "2: `10`"]
    PLL_PERI_2X = 2,
    #[doc = "3: `11`"]
    PLL_VIDEO0_4X = 3,
    #[doc = "4: `100`"]
    PLL_VIDEO1_4X = 4,
    #[doc = "5: `101`"]
    PLL_VE = 5,
    #[doc = "6: `110`"]
    PLL_AUDIO0 = 6,
    #[doc = "7: `111`"]
    PLL_AUDIO1 = 7,
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
    pub fn variant(&self) -> CLK_SRC_SEL_A {
        match self.bits {
            0 => CLK_SRC_SEL_A::PLL_CPUX,
            1 => CLK_SRC_SEL_A::PLL_DDR,
            2 => CLK_SRC_SEL_A::PLL_PERI_2X,
            3 => CLK_SRC_SEL_A::PLL_VIDEO0_4X,
            4 => CLK_SRC_SEL_A::PLL_VIDEO1_4X,
            5 => CLK_SRC_SEL_A::PLL_VE,
            6 => CLK_SRC_SEL_A::PLL_AUDIO0,
            7 => CLK_SRC_SEL_A::PLL_AUDIO1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLL_CPUX`"]
    #[inline(always)]
    pub fn is_pll_cpux(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_CPUX
    }
    #[doc = "Checks if the value of the field is `PLL_DDR`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_4X`"]
    #[inline(always)]
    pub fn is_pll_video0_4x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO0_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_4X`"]
    #[inline(always)]
    pub fn is_pll_video1_4x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO1_4X
    }
    #[doc = "Checks if the value of the field is `PLL_VE`"]
    #[inline(always)]
    pub fn is_pll_ve(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VE
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO0`"]
    #[inline(always)]
    pub fn is_pll_audio0(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_AUDIO0
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1`"]
    #[inline(always)]
    pub fn is_pll_audio1(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_AUDIO1
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
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_cpux(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_CPUX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_video0_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_4X)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pll_video1_4x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_4X)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pll_ve(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VE)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pll_audio0(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pll_audio1(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Debug Enable"]
    #[inline(always)]
    pub fn pll_lock_flag_en(&self) -> PLL_LOCK_FLAG_EN_R {
        PLL_LOCK_FLAG_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Debug Enable"]
    #[inline(always)]
    pub fn pll_lock_flag_en(&mut self) -> PLL_LOCK_FLAG_EN_W {
        PLL_LOCK_FLAG_EN_W { w: self }
    }
    #[doc = "Bits 20:22 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Lock Debug Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_lock_dbg_ctrl](index.html) module"]
pub struct PLL_LOCK_DBG_CTRL_SPEC;
impl crate::RegisterSpec for PLL_LOCK_DBG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_lock_dbg_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_LOCK_DBG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_lock_dbg_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_LOCK_DBG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_LOCK_DBG_CTRL to value 0"]
impl crate::Resettable for PLL_LOCK_DBG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
