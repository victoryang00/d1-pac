#[doc = "Register `SMHC_HWRST` reader"]
pub struct R(crate::R<SMHC_HWRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_HWRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_HWRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_HWRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_HWRST` writer"]
pub struct W(crate::W<SMHC_HWRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_HWRST_SPEC>;
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
impl From<crate::W<SMHC_HWRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_HWRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_RST_A {
    #[doc = "0: Active mode"]
    ACTIVE = 0,
    #[doc = "1: Reset"]
    RESET = 1,
}
impl From<HW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HW_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HW_RST` reader - "]
pub struct HW_RST_R(crate::FieldReader<bool, HW_RST_A>);
impl HW_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HW_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_RST_A {
        match self.bits {
            false => HW_RST_A::ACTIVE,
            true => HW_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == HW_RST_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == HW_RST_A::RESET
    }
}
impl core::ops::Deref for HW_RST_R {
    type Target = crate::FieldReader<bool, HW_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_RST` writer - "]
pub struct HW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(HW_RST_A::ACTIVE)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HW_RST_A::RESET)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hw_rst(&self) -> HW_RST_R {
        HW_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hw_rst(&mut self) -> HW_RST_W {
        HW_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_hwrst](index.html) module"]
pub struct SMHC_HWRST_SPEC;
impl crate::RegisterSpec for SMHC_HWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_hwrst::R](R) reader structure"]
impl crate::Readable for SMHC_HWRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_hwrst::W](W) writer structure"]
impl crate::Writable for SMHC_HWRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_HWRST to value 0"]
impl crate::Resettable for SMHC_HWRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
