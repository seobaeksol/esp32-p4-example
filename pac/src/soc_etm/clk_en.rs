#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<ClkEnSpec>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<ClkEnSpec>;
#[doc = "Field `CLK_EN` reader - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to open register clock gate.\\\\0: Open the clock gate only when application writes registers\\\\1: Force open the clock gate for register"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, ClkEnSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "ETM clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkEnSpec;
impl crate::RegisterSpec for ClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for ClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for ClkEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_EN to value 0"]
impl crate::Resettable for ClkEnSpec {}
