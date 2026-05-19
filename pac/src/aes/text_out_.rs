#[doc = "Register `TEXT_OUT_%s` reader"]
pub type R = crate::R<TextOut_Spec>;
#[doc = "Register `TEXT_OUT_%s` writer"]
pub type W = crate::W<TextOut_Spec>;
#[doc = "Field `TEXT_OUT_0` reader - This bits stores text_out_0 that is a part of result text material."]
pub type TextOut0R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_OUT_0` writer - This bits stores text_out_0 that is a part of result text material."]
pub type TextOut0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_out_0 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_0(&self) -> TextOut0R {
        TextOut0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_out_0 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_0(&mut self) -> TextOut0W<'_, TextOut_Spec> {
        TextOut0W::new(self, 0)
    }
}
#[doc = "Result text data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_out_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_out_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TextOut_Spec;
impl crate::RegisterSpec for TextOut_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_out_::R`](R) reader structure"]
impl crate::Readable for TextOut_Spec {}
#[doc = "`write(|w| ..)` method takes [`text_out_::W`](W) writer structure"]
impl crate::Writable for TextOut_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEXT_OUT_%s to value 0"]
impl crate::Resettable for TextOut_Spec {}
