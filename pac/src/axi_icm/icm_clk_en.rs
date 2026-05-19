#[doc = "Register `ICM_CLK_EN` reader"]
pub type R = crate::R<IcmClkEnSpec>;
#[doc = "Register `ICM_CLK_EN` writer"]
pub type W = crate::W<IcmClkEnSpec>;
#[doc = "Field `CLK_EN` reader - "]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, IcmClkEnSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "ICM clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmClkEnSpec;
impl crate::RegisterSpec for IcmClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_clk_en::R`](R) reader structure"]
impl crate::Readable for IcmClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`icm_clk_en::W`](W) writer structure"]
impl crate::Writable for IcmClkEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_CLK_EN to value 0"]
impl crate::Resettable for IcmClkEnSpec {}
