#[doc = "Register `SMHC_FIFOTH` reader"]
pub struct R(crate::R<SMHC_FIFOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_FIFOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_FIFOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_FIFOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_FIFOTH` writer"]
pub struct W(crate::W<SMHC_FIFOTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_FIFOTH_SPEC>;
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
impl From<crate::W<SMHC_FIFOTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_FIFOTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "sBurst Size of Multiple Transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BSIZE_OF_TRANS_A {
    #[doc = "0: 1 transfer"]
    T1 = 0,
    #[doc = "1: 4 transfers"]
    T4 = 1,
    #[doc = "2: 8 transfers"]
    T8 = 2,
    #[doc = "3: 16 transfers"]
    T16 = 3,
}
impl From<BSIZE_OF_TRANS_A> for u8 {
    #[inline(always)]
    fn from(variant: BSIZE_OF_TRANS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BSIZE_OF_TRANS` reader - sBurst Size of Multiple Transaction"]
pub struct BSIZE_OF_TRANS_R(crate::FieldReader<u8, BSIZE_OF_TRANS_A>);
impl BSIZE_OF_TRANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BSIZE_OF_TRANS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BSIZE_OF_TRANS_A> {
        match self.bits {
            0 => Some(BSIZE_OF_TRANS_A::T1),
            1 => Some(BSIZE_OF_TRANS_A::T4),
            2 => Some(BSIZE_OF_TRANS_A::T8),
            3 => Some(BSIZE_OF_TRANS_A::T16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T1`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        **self == BSIZE_OF_TRANS_A::T1
    }
    #[doc = "Checks if the value of the field is `T4`"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        **self == BSIZE_OF_TRANS_A::T4
    }
    #[doc = "Checks if the value of the field is `T8`"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        **self == BSIZE_OF_TRANS_A::T8
    }
    #[doc = "Checks if the value of the field is `T16`"]
    #[inline(always)]
    pub fn is_t16(&self) -> bool {
        **self == BSIZE_OF_TRANS_A::T16
    }
}
impl core::ops::Deref for BSIZE_OF_TRANS_R {
    type Target = crate::FieldReader<u8, BSIZE_OF_TRANS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSIZE_OF_TRANS` writer - sBurst Size of Multiple Transaction"]
pub struct BSIZE_OF_TRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> BSIZE_OF_TRANS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSIZE_OF_TRANS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 transfer"]
    #[inline(always)]
    pub fn t1(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T1)
    }
    #[doc = "4 transfers"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T4)
    }
    #[doc = "8 transfers"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T8)
    }
    #[doc = "16 transfers"]
    #[inline(always)]
    pub fn t16(self) -> &'a mut W {
        self.variant(BSIZE_OF_TRANS_A::T16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `RX_TL` reader - RX Trigger Level"]
pub struct RX_TL_R(crate::FieldReader<u8, u8>);
impl RX_TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TL` writer - RX Trigger Level"]
pub struct RX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TX_TL` reader - TX Trigger Level"]
pub struct TX_TL_R(crate::FieldReader<u8, u8>);
impl TX_TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TL` writer - TX Trigger Level"]
pub struct TX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - sBurst Size of Multiple Transaction"]
    #[inline(always)]
    pub fn bsize_of_trans(&self) -> BSIZE_OF_TRANS_R {
        BSIZE_OF_TRANS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - RX Trigger Level"]
    #[inline(always)]
    pub fn rx_tl(&self) -> RX_TL_R {
        RX_TL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - TX Trigger Level"]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - sBurst Size of Multiple Transaction"]
    #[inline(always)]
    pub fn bsize_of_trans(&mut self) -> BSIZE_OF_TRANS_W {
        BSIZE_OF_TRANS_W { w: self }
    }
    #[doc = "Bits 16:23 - RX Trigger Level"]
    #[inline(always)]
    pub fn rx_tl(&mut self) -> RX_TL_W {
        RX_TL_W { w: self }
    }
    #[doc = "Bits 0:7 - TX Trigger Level"]
    #[inline(always)]
    pub fn tx_tl(&mut self) -> TX_TL_W {
        TX_TL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Water Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_fifoth](index.html) module"]
pub struct SMHC_FIFOTH_SPEC;
impl crate::RegisterSpec for SMHC_FIFOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_fifoth::R](R) reader structure"]
impl crate::Readable for SMHC_FIFOTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_fifoth::W](W) writer structure"]
impl crate::Writable for SMHC_FIFOTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_FIFOTH to value 0"]
impl crate::Resettable for SMHC_FIFOTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
