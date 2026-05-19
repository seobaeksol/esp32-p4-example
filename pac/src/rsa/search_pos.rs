#[doc = "Register `SEARCH_POS` reader"]
pub type R = crate::R<SearchPosSpec>;
#[doc = "Register `SEARCH_POS` writer"]
pub type W = crate::W<SearchPosSpec>;
#[doc = "Field `SEARCH_POS` reader - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE_REG. The field is only valid when RSA_SEARCH_ENABLE is high."]
pub type SearchPosR = crate::FieldReader<u16>;
#[doc = "Field `SEARCH_POS` writer - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE_REG. The field is only valid when RSA_SEARCH_ENABLE is high."]
pub type SearchPosW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE_REG. The field is only valid when RSA_SEARCH_ENABLE is high."]
    #[inline(always)]
    pub fn search_pos(&self) -> SearchPosR {
        SearchPosR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the starting address to start search. This field should be used together with RSA_SEARCH_ENABLE_REG. The field is only valid when RSA_SEARCH_ENABLE is high."]
    #[inline(always)]
    pub fn search_pos(&mut self) -> SearchPosW<'_, SearchPosSpec> {
        SearchPosW::new(self, 0)
    }
}
#[doc = "Configures the search position\n\nYou can [`read`](crate::Reg::read) this register and get [`search_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`search_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SearchPosSpec;
impl crate::RegisterSpec for SearchPosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`search_pos::R`](R) reader structure"]
impl crate::Readable for SearchPosSpec {}
#[doc = "`write(|w| ..)` method takes [`search_pos::W`](W) writer structure"]
impl crate::Writable for SearchPosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEARCH_POS to value 0"]
impl crate::Resettable for SearchPosSpec {}
