#[doc = "Register `SMHC_IDST` reader"]
pub struct R(crate::R<SMHC_IDST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_IDST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_IDST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_IDST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_IDST` writer"]
pub struct W(crate::W<SMHC_IDST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_IDST_SPEC>;
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
impl From<crate::W<SMHC_IDST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_IDST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDMAC_ERR_STA_A {
    #[doc = "1: Host Abort received during the transmission"]
    TRANSMISSION = 1,
    #[doc = "2: Host Abort received during the reception"]
    RECEPTION = 2,
}
impl From<IDMAC_ERR_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: IDMAC_ERR_STA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDMAC_ERR_STA` reader - Error Bits"]
pub struct IDMAC_ERR_STA_R(crate::FieldReader<u8, IDMAC_ERR_STA_A>);
impl IDMAC_ERR_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDMAC_ERR_STA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDMAC_ERR_STA_A> {
        match self.bits {
            1 => Some(IDMAC_ERR_STA_A::TRANSMISSION),
            2 => Some(IDMAC_ERR_STA_A::RECEPTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSMISSION`"]
    #[inline(always)]
    pub fn is_transmission(&self) -> bool {
        **self == IDMAC_ERR_STA_A::TRANSMISSION
    }
    #[doc = "Checks if the value of the field is `RECEPTION`"]
    #[inline(always)]
    pub fn is_reception(&self) -> bool {
        **self == IDMAC_ERR_STA_A::RECEPTION
    }
}
impl core::ops::Deref for IDMAC_ERR_STA_R {
    type Target = crate::FieldReader<u8, IDMAC_ERR_STA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABN_INT_SUM` reader - Abnormal Interrupt Summary"]
pub struct ABN_INT_SUM_R(crate::FieldReader<bool, bool>);
impl ABN_INT_SUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABN_INT_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABN_INT_SUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABN_INT_SUM` writer - Abnormal Interrupt Summary"]
pub struct ABN_INT_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ABN_INT_SUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `NOR_INT_SUM` reader - Normal Interrupt Summary"]
pub struct NOR_INT_SUM_R(crate::FieldReader<bool, bool>);
impl NOR_INT_SUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOR_INT_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOR_INT_SUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOR_INT_SUM` writer - Normal Interrupt Summary"]
pub struct NOR_INT_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> NOR_INT_SUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ERR_FLAG_SUM` reader - Card Error Summary"]
pub struct ERR_FLAG_SUM_R(crate::FieldReader<bool, bool>);
impl ERR_FLAG_SUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR_FLAG_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_FLAG_SUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_FLAG_SUM` writer - Card Error Summary"]
pub struct ERR_FLAG_SUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_FLAG_SUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DES_UNAVL_INT` reader - Descriptor Unavailable Interrupt"]
pub struct DES_UNAVL_INT_R(crate::FieldReader<bool, bool>);
impl DES_UNAVL_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DES_UNAVL_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DES_UNAVL_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DES_UNAVL_INT` writer - Descriptor Unavailable Interrupt"]
pub struct DES_UNAVL_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> DES_UNAVL_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FATAL_BERR_INT` reader - Fatal Bus Error Interrupt"]
pub struct FATAL_BERR_INT_R(crate::FieldReader<bool, bool>);
impl FATAL_BERR_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FATAL_BERR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FATAL_BERR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FATAL_BERR_INT` writer - Fatal Bus Error Interrupt"]
pub struct FATAL_BERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> FATAL_BERR_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RX_INT` reader - Receive Interrupt"]
pub struct RX_INT_R(crate::FieldReader<bool, bool>);
impl RX_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_INT` writer - Receive Interrupt"]
pub struct RX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INT_W<'a> {
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
#[doc = "Field `TX_INT` reader - Transmit Interrupt"]
pub struct TX_INT_R(crate::FieldReader<bool, bool>);
impl TX_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_INT` writer - Transmit Interrupt"]
pub struct TX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_INT_W<'a> {
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
    #[doc = "Bits 10:12 - Error Bits"]
    #[inline(always)]
    pub fn idmac_err_sta(&self) -> IDMAC_ERR_STA_R {
        IDMAC_ERR_STA_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn abn_int_sum(&self) -> ABN_INT_SUM_R {
        ABN_INT_SUM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nor_int_sum(&self) -> NOR_INT_SUM_R {
        NOR_INT_SUM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary"]
    #[inline(always)]
    pub fn err_flag_sum(&self) -> ERR_FLAG_SUM_R {
        ERR_FLAG_SUM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    pub fn des_unavl_int(&self) -> DES_UNAVL_INT_R {
        DES_UNAVL_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fatal_berr_int(&self) -> FATAL_BERR_INT_R {
        FATAL_BERR_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int(&self) -> RX_INT_R {
        RX_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int(&self) -> TX_INT_R {
        TX_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn abn_int_sum(&mut self) -> ABN_INT_SUM_W {
        ABN_INT_SUM_W { w: self }
    }
    #[doc = "Bit 8 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nor_int_sum(&mut self) -> NOR_INT_SUM_W {
        NOR_INT_SUM_W { w: self }
    }
    #[doc = "Bit 5 - Card Error Summary"]
    #[inline(always)]
    pub fn err_flag_sum(&mut self) -> ERR_FLAG_SUM_W {
        ERR_FLAG_SUM_W { w: self }
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    pub fn des_unavl_int(&mut self) -> DES_UNAVL_INT_W {
        DES_UNAVL_INT_W { w: self }
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fatal_berr_int(&mut self) -> FATAL_BERR_INT_W {
        FATAL_BERR_INT_W { w: self }
    }
    #[doc = "Bit 1 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int(&mut self) -> RX_INT_W {
        RX_INT_W { w: self }
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int(&mut self) -> TX_INT_W {
        TX_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_idst](index.html) module"]
pub struct SMHC_IDST_SPEC;
impl crate::RegisterSpec for SMHC_IDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_idst::R](R) reader structure"]
impl crate::Readable for SMHC_IDST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_idst::W](W) writer structure"]
impl crate::Writable for SMHC_IDST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_IDST to value 0"]
impl crate::Resettable for SMHC_IDST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
