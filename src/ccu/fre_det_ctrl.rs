#[doc = "Register `FRE_DET_CTRL` reader"]
pub struct R(crate::R<FRE_DET_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRE_DET_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRE_DET_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRE_DET_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRE_DET_CTRL` writer"]
pub struct W(crate::W<FRE_DET_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRE_DET_CTRL_SPEC>;
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
impl From<crate::W<FRE_DET_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRE_DET_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_FLAG_A {
    #[doc = "0: `0`"]
    W0C = 0,
    #[doc = "1: `1`"]
    ERROR = 1,
}
impl From<ERROR_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR_FLAG` reader - Error Flag"]
pub struct ERROR_FLAG_R(crate::FieldReader<bool, ERROR_FLAG_A>);
impl ERROR_FLAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_FLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_FLAG_A {
        match self.bits {
            false => ERROR_FLAG_A::W0C,
            true => ERROR_FLAG_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `W0C`"]
    #[inline(always)]
    pub fn is_w0c(&self) -> bool {
        **self == ERROR_FLAG_A::W0C
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == ERROR_FLAG_A::ERROR
    }
}
impl core::ops::Deref for ERROR_FLAG_R {
    type Target = crate::FieldReader<bool, ERROR_FLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_FLAG` writer - Error Flag"]
pub struct ERROR_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_FLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn w0c(self) -> &'a mut W {
        self.variant(ERROR_FLAG_A::W0C)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERROR_FLAG_A::ERROR)
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
#[doc = "Field `DET_TIME` reader - Detect Time"]
pub struct DET_TIME_R(crate::FieldReader<u8, u8>);
impl DET_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DET_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_TIME` writer - Detect Time"]
pub struct DET_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DET_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Frequence Detect IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_DET_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FRE_DET_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_DET_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRE_DET_IRQ_EN` reader - Frequence Detect IRQ Enable"]
pub struct FRE_DET_IRQ_EN_R(crate::FieldReader<bool, FRE_DET_IRQ_EN_A>);
impl FRE_DET_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRE_DET_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_DET_IRQ_EN_A {
        match self.bits {
            false => FRE_DET_IRQ_EN_A::DISABLE,
            true => FRE_DET_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FRE_DET_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRE_DET_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for FRE_DET_IRQ_EN_R {
    type Target = crate::FieldReader<bool, FRE_DET_IRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRE_DET_IRQ_EN` writer - Frequence Detect IRQ Enable"]
pub struct FRE_DET_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRE_DET_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRE_DET_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRE_DET_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRE_DET_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Frequence Detect Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_DET_FUN_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FRE_DET_FUN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_DET_FUN_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRE_DET_FUN_EN` reader - Frequence Detect Function Enable"]
pub struct FRE_DET_FUN_EN_R(crate::FieldReader<bool, FRE_DET_FUN_EN_A>);
impl FRE_DET_FUN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRE_DET_FUN_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_DET_FUN_EN_A {
        match self.bits {
            false => FRE_DET_FUN_EN_A::DISABLE,
            true => FRE_DET_FUN_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FRE_DET_FUN_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRE_DET_FUN_EN_A::ENABLE
    }
}
impl core::ops::Deref for FRE_DET_FUN_EN_R {
    type Target = crate::FieldReader<bool, FRE_DET_FUN_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRE_DET_FUN_EN` writer - Frequence Detect Function Enable"]
pub struct FRE_DET_FUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRE_DET_FUN_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRE_DET_FUN_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRE_DET_FUN_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRE_DET_FUN_EN_A::ENABLE)
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
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error_flag(&self) -> ERROR_FLAG_R {
        ERROR_FLAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - Detect Time"]
    #[inline(always)]
    pub fn det_time(&self) -> DET_TIME_R {
        DET_TIME_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - Frequence Detect IRQ Enable"]
    #[inline(always)]
    pub fn fre_det_irq_en(&self) -> FRE_DET_IRQ_EN_R {
        FRE_DET_IRQ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Frequence Detect Function Enable"]
    #[inline(always)]
    pub fn fre_det_fun_en(&self) -> FRE_DET_FUN_EN_R {
        FRE_DET_FUN_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Error Flag"]
    #[inline(always)]
    pub fn error_flag(&mut self) -> ERROR_FLAG_W {
        ERROR_FLAG_W { w: self }
    }
    #[doc = "Bits 4:8 - Detect Time"]
    #[inline(always)]
    pub fn det_time(&mut self) -> DET_TIME_W {
        DET_TIME_W { w: self }
    }
    #[doc = "Bit 1 - Frequence Detect IRQ Enable"]
    #[inline(always)]
    pub fn fre_det_irq_en(&mut self) -> FRE_DET_IRQ_EN_W {
        FRE_DET_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 0 - Frequence Detect Function Enable"]
    #[inline(always)]
    pub fn fre_det_fun_en(&mut self) -> FRE_DET_FUN_EN_W {
        FRE_DET_FUN_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fre_det_ctrl](index.html) module"]
pub struct FRE_DET_CTRL_SPEC;
impl crate::RegisterSpec for FRE_DET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fre_det_ctrl::R](R) reader structure"]
impl crate::Readable for FRE_DET_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fre_det_ctrl::W](W) writer structure"]
impl crate::Writable for FRE_DET_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRE_DET_CTRL to value 0"]
impl crate::Resettable for FRE_DET_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
