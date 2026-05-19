#[doc = "Register `SEARCH_ENABLE` reader"]
pub type R = crate::R<SearchEnableSpec>;
#[doc = "Register `SEARCH_ENABLE` writer"]
pub type W = crate::W<SearchEnableSpec>;
#[doc = "Field `SEARCH_ENABLE` reader - Configures the search option. \\\\ 0: No acceleration (default)\\\\ 1: Acceleration\\\\ This option should be used together with RSA_SEARCH_POS_REG."]
pub type SearchEnableR = crate::BitReader;
#[doc = "Field `SEARCH_ENABLE` writer - Configures the search option. \\\\ 0: No acceleration (default)\\\\ 1: Acceleration\\\\ This option should be used together with RSA_SEARCH_POS_REG."]
pub type SearchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the search option. \\\\ 0: No acceleration (default)\\\\ 1: Acceleration\\\\ This option should be used together with RSA_SEARCH_POS_REG."]
    #[inline(always)]
    pub fn search_enable(&self) -> SearchEnableR {
        SearchEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the search option. \\\\ 0: No acceleration (default)\\\\ 1: Acceleration\\\\ This option should be used together with RSA_SEARCH_POS_REG."]
    #[inline(always)]
    pub fn search_enable(&mut self) -> SearchEnableW<'_, SearchEnableSpec> {
        SearchEnableW::new(self, 0)
    }
}
#[doc = "Configures the search option\n\nYou can [`read`](crate::Reg::read) this register and get [`search_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`search_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SearchEnableSpec;
impl crate::RegisterSpec for SearchEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`search_enable::R`](R) reader structure"]
impl crate::Readable for SearchEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`search_enable::W`](W) writer structure"]
impl crate::Writable for SearchEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEARCH_ENABLE to value 0"]
impl crate::Resettable for SearchEnableSpec {}
