#[doc = "Register `sclaim` reader"]
pub struct R(crate::R<SCLAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sclaim` writer"]
pub struct W(crate::W<SCLAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLAIM_SPEC>;
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
impl From<crate::W<SCLAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLAIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sclaim` reader - "]
pub struct SCLAIM_R(crate::FieldReader<u16, u16>);
impl SCLAIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SCLAIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLAIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sclaim` writer - "]
pub struct SCLAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLAIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sclaim(&self) -> SCLAIM_R {
        SCLAIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sclaim(&mut self) -> SCLAIM_W {
        SCLAIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supervisor Mode Claim/Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclaim](index.html) module"]
pub struct SCLAIM_SPEC;
impl crate::RegisterSpec for SCLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclaim::R](R) reader structure"]
impl crate::Readable for SCLAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclaim::W](W) writer structure"]
impl crate::Writable for SCLAIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sclaim to value 0"]
impl crate::Resettable for SCLAIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
