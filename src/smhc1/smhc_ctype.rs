#[doc = "Register `SMHC_CTYPE` reader"]
pub struct R(crate::R<SMHC_CTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_CTYPE` writer"]
pub struct W(crate::W<SMHC_CTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CTYPE_SPEC>;
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
impl From<crate::W<SMHC_CTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Card Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CARD_WID_A {
    #[doc = "0: 1-bit width"]
    B1 = 0,
    #[doc = "1: 4-bit width"]
    B4 = 1,
    #[doc = "2: 8-bit width"]
    B8 = 2,
}
impl From<CARD_WID_A> for u8 {
    #[inline(always)]
    fn from(variant: CARD_WID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CARD_WID` reader - Card Width"]
pub struct CARD_WID_R(crate::FieldReader<u8, CARD_WID_A>);
impl CARD_WID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CARD_WID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CARD_WID_A> {
        match self.bits {
            0 => Some(CARD_WID_A::B1),
            1 => Some(CARD_WID_A::B4),
            2 => Some(CARD_WID_A::B8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        **self == CARD_WID_A::B1
    }
    #[doc = "Checks if the value of the field is `B4`"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        **self == CARD_WID_A::B4
    }
    #[doc = "Checks if the value of the field is `B8`"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        **self == CARD_WID_A::B8
    }
}
impl core::ops::Deref for CARD_WID_R {
    type Target = crate::FieldReader<u8, CARD_WID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_WID` writer - Card Width"]
pub struct CARD_WID_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_WID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_WID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1-bit width"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(CARD_WID_A::B1)
    }
    #[doc = "4-bit width"]
    #[inline(always)]
    pub fn b4(self) -> &'a mut W {
        self.variant(CARD_WID_A::B4)
    }
    #[doc = "8-bit width"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut W {
        self.variant(CARD_WID_A::B8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    pub fn card_wid(&self) -> CARD_WID_R {
        CARD_WID_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Card Width"]
    #[inline(always)]
    pub fn card_wid(&mut self) -> CARD_WID_W {
        CARD_WID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ctype](index.html) module"]
pub struct SMHC_CTYPE_SPEC;
impl crate::RegisterSpec for SMHC_CTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ctype::R](R) reader structure"]
impl crate::Readable for SMHC_CTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ctype::W](W) writer structure"]
impl crate::Writable for SMHC_CTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_CTYPE to value 0"]
impl crate::Resettable for SMHC_CTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
