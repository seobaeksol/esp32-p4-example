#[doc = "Register `DPI_CONFIG_UPDATE` writer"]
pub type W = crate::W<DpiConfigUpdateSpec>;
#[doc = "Field `DPI_CONFIG_UPDATE` writer - write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*"]
pub type DpiConfigUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - write 1 to this bit to update dpi config register MIPI_DSI_BRG_DPI_*"]
    #[inline(always)]
    pub fn dpi_config_update(&mut self) -> DpiConfigUpdateW<'_, DpiConfigUpdateSpec> {
        DpiConfigUpdateW::new(self, 0)
    }
}
#[doc = "dsi_bridge dpi config update register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_config_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiConfigUpdateSpec;
impl crate::RegisterSpec for DpiConfigUpdateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dpi_config_update::W`](W) writer structure"]
impl crate::Writable for DpiConfigUpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_CONFIG_UPDATE to value 0"]
impl crate::Resettable for DpiConfigUpdateSpec {}
