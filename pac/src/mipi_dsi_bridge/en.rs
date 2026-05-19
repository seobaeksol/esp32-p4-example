#[doc = "Register `EN` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "Field `DSI_EN` reader - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DsiEnR = crate::BitReader;
#[doc = "Field `DSI_EN` writer - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
pub type DsiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_BRIG_RST` reader - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
pub type DsiBrigRstR = crate::BitReader;
#[doc = "Field `DSI_BRIG_RST` writer - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
pub type DsiBrigRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_en(&self) -> DsiEnR {
        DsiEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
    #[inline(always)]
    pub fn dsi_brig_rst(&self) -> DsiBrigRstR {
        DsiBrigRstR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures module enable of dsi_bridge. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dsi_en(&mut self) -> DsiEnW<'_, EnSpec> {
        DsiEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures software reset of dsi_bridge. 0: release reset, 1: reset"]
    #[inline(always)]
    pub fn dsi_brig_rst(&mut self) -> DsiBrigRstW<'_, EnSpec> {
        DsiBrigRstW::new(self, 1)
    }
}
#[doc = "dsi bridge en register\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EnSpec {}
