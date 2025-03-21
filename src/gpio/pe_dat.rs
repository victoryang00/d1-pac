#[doc = "Register `pe_dat` reader"]
pub struct R(crate::R<PE_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_dat` writer"]
pub struct W(crate::W<PE_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_DAT_SPEC>;
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
impl From<crate::W<PE_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE_DAT` reader - PE Data"]
pub struct PE_DAT_R(crate::FieldReader<u32, u32>);
impl PE_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PE_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE_DAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE_DAT` writer - PE Data"]
pub struct PE_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - PE Data"]
    #[inline(always)]
    pub fn pe_dat(&self) -> PE_DAT_R {
        PE_DAT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - PE Data"]
    #[inline(always)]
    pub fn pe_dat(&mut self) -> PE_DAT_W {
        PE_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_dat](index.html) module"]
pub struct PE_DAT_SPEC;
impl crate::RegisterSpec for PE_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_dat::R](R) reader structure"]
impl crate::Readable for PE_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_dat::W](W) writer structure"]
impl crate::Writable for PE_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pe_dat to value 0"]
impl crate::Resettable for PE_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
