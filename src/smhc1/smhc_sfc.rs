#[doc = "Register `SMHC_SFC` reader"]
pub struct R(crate::R<SMHC_SFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_SFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_SFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_SFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_SFC` writer"]
pub struct W(crate::W<SMHC_SFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_SFC_SPEC>;
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
impl From<crate::W<SMHC_SFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_SFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_CLK_CTRL` reader - Stop Clock Control"]
pub struct STOP_CLK_CTRL_R(crate::FieldReader<u8, u8>);
impl STOP_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STOP_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_CLK_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_CLK_CTRL` writer - Stop Clock Control"]
pub struct STOP_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `BYPASS_EN` reader - Bypass enable"]
pub struct BYPASS_EN_R(crate::FieldReader<bool, bool>);
impl BYPASS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS_EN` writer - Bypass enable"]
pub struct BYPASS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    pub fn stop_clk_ctrl(&self) -> STOP_CLK_CTRL_R {
        STOP_CLK_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    pub fn bypass_en(&self) -> BYPASS_EN_R {
        BYPASS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    pub fn stop_clk_ctrl(&mut self) -> STOP_CLK_CTRL_W {
        STOP_CLK_CTRL_W { w: self }
    }
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    pub fn bypass_en(&mut self) -> BYPASS_EN_W {
        BYPASS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_sfc](index.html) module"]
pub struct SMHC_SFC_SPEC;
impl crate::RegisterSpec for SMHC_SFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_sfc::R](R) reader structure"]
impl crate::Readable for SMHC_SFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_sfc::W](W) writer structure"]
impl crate::Writable for SMHC_SFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_SFC to value 0"]
impl crate::Resettable for SMHC_SFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
