#[doc = "Register `GET_LINE_CODE_W1` reader"]
pub type R = crate::R<GetLineCodeW1Spec>;
#[doc = "Register `GET_LINE_CODE_W1` writer"]
pub type W = crate::W<GetLineCodeW1Spec>;
#[doc = "Field `GET_BDATA_BITS` reader - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type GetBdataBitsR = crate::FieldReader;
#[doc = "Field `GET_BDATA_BITS` writer - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type GetBdataBitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GET_BPARITY_TYPE` reader - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type GetBparityTypeR = crate::FieldReader;
#[doc = "Field `GET_BPARITY_TYPE` writer - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type GetBparityTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GET_BCHAR_FORMAT` reader - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type GetBcharFormatR = crate::FieldReader;
#[doc = "Field `GET_BCHAR_FORMAT` writer - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type GetBcharFormatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bdata_bits(&self) -> GetBdataBitsR {
        GetBdataBitsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bparity_type(&self) -> GetBparityTypeR {
        GetBparityTypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bchar_format(&self) -> GetBcharFormatR {
        GetBcharFormatR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bdata_bits(&mut self) -> GetBdataBitsW<'_, GetLineCodeW1Spec> {
        GetBdataBitsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bparity_type(&mut self) -> GetBparityTypeW<'_, GetLineCodeW1Spec> {
        GetBparityTypeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn get_bchar_format(&mut self) -> GetBcharFormatW<'_, GetLineCodeW1Spec> {
        GetBcharFormatW::new(self, 16)
    }
}
#[doc = "W1 of GET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`get_line_code_w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`get_line_code_w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GetLineCodeW1Spec;
impl crate::RegisterSpec for GetLineCodeW1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`get_line_code_w1::R`](R) reader structure"]
impl crate::Readable for GetLineCodeW1Spec {}
#[doc = "`write(|w| ..)` method takes [`get_line_code_w1::W`](W) writer structure"]
impl crate::Writable for GetLineCodeW1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GET_LINE_CODE_W1 to value 0"]
impl crate::Resettable for GetLineCodeW1Spec {}
