#[doc = "Register `pe_drv0` reader"]
pub struct R(crate::R<PE_DRV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_DRV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_DRV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_DRV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_drv0` writer"]
pub struct W(crate::W<PE_DRV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_DRV0_SPEC>;
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
impl From<crate::W<PE_DRV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_DRV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PE Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PE_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PE_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PE_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `PE(0-7)_DRV` reader - PE Multi_Driving Select"]
pub struct PE_DRV_R(crate::FieldReader<u8, PE_DRV_A>);
impl PE_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PE_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_DRV_A {
        match self.bits {
            0 => PE_DRV_A::L0,
            1 => PE_DRV_A::L1,
            2 => PE_DRV_A::L2,
            3 => PE_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PE_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PE_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PE_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PE_DRV_A::L3
    }
}
impl core::ops::Deref for PE_DRV_R {
    type Target = crate::FieldReader<u8, PE_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `PE(0-7)_DRV` writer - PE Multi_Driving Select"]
pub struct PE_DRV_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PE_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PE_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PE_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PE_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PE_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << self.offset)) | ((value as u32 & 0x03) << self.offset);
        self.w
    }
}
#[doc = "Fields `PE(0-7)_DRV` const generic writer - PE Multi_Driving Select"]
pub struct PE_DRV_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PE_DRV_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PE_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PE_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PE_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PE_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << O)) | ((value as u32 & 0x03) << O);
        self.w
    }
}
impl R {
    #[doc = "PE Multi_Driving Select"]
    #[inline(always)]
    pub unsafe fn pe_drv(&self, n: usize) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> (n * 4)) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe0_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe1_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe2_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe3_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe4_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe5_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe6_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe7_drv(&self) -> PE_DRV_R {
        PE_DRV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "PE Multi_Driving Select"]
    #[inline(always)]
    pub unsafe fn pe_drv(&mut self, n: usize) -> PE_DRV_W {
        PE_DRV_W {
            w: self,
            offset: n * 4,
        }
    }
    #[doc = "Bits 0:1 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe0_drv(&mut self) -> PE_DRV_CGW<0> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 4:5 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe1_drv(&mut self) -> PE_DRV_CGW<4> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 8:9 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe2_drv(&mut self) -> PE_DRV_CGW<8> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 12:13 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe3_drv(&mut self) -> PE_DRV_CGW<12> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 16:17 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe4_drv(&mut self) -> PE_DRV_CGW<16> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 20:21 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe5_drv(&mut self) -> PE_DRV_CGW<20> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 24:25 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe6_drv(&mut self) -> PE_DRV_CGW<24> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Bits 28:29 - PE Multi_Driving Select"]
    #[inline(always)]
    pub fn pe7_drv(&mut self) -> PE_DRV_CGW<28> {
        PE_DRV_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE Multi_Driving Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_drv0](index.html) module"]
pub struct PE_DRV0_SPEC;
impl crate::RegisterSpec for PE_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_drv0::R](R) reader structure"]
impl crate::Readable for PE_DRV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_drv0::W](W) writer structure"]
impl crate::Writable for PE_DRV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pe_drv0 to value 0"]
impl crate::Resettable for PE_DRV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
