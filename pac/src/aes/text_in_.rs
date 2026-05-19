#[doc = "Register `TEXT_IN_%s` reader"]
pub type R = crate::R<TextIn_Spec>;
#[doc = "Register `TEXT_IN_%s` writer"]
pub type W = crate::W<TextIn_Spec>;
#[doc = "Field `TEXT_IN_0` reader - This bits stores text_in_0 that is a part of source text material."]
pub type TextIn0R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_IN_0` writer - This bits stores text_in_0 that is a part of source text material."]
pub type TextIn0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_in_0 that is a part of source text material."]
    #[inline(always)]
    pub fn text_in_0(&self) -> TextIn0R {
        TextIn0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_in_0 that is a part of source text material."]
    #[inline(always)]
    pub fn text_in_0(&mut self) -> TextIn0W<'_, TextIn_Spec> {
        TextIn0W::new(self, 0)
    }
}
#[doc = "Source text data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_in_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_in_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TextIn_Spec;
impl crate::RegisterSpec for TextIn_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_in_::R`](R) reader structure"]
impl crate::Readable for TextIn_Spec {}
#[doc = "`write(|w| ..)` method takes [`text_in_::W`](W) writer structure"]
impl crate::Writable for TextIn_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEXT_IN_%s to value 0"]
impl crate::Resettable for TextIn_Spec {}
