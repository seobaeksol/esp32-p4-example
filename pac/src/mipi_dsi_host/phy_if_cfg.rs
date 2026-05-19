#[doc = "Register `PHY_IF_CFG` reader"]
pub type R = crate::R<PhyIfCfgSpec>;
#[doc = "Register `PHY_IF_CFG` writer"]
pub type W = crate::W<PhyIfCfgSpec>;
#[doc = "Field `N_LANES` reader - NA"]
pub type NLanesR = crate::FieldReader;
#[doc = "Field `N_LANES` writer - NA"]
pub type NLanesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_STOP_WAIT_TIME` reader - NA"]
pub type PhyStopWaitTimeR = crate::FieldReader;
#[doc = "Field `PHY_STOP_WAIT_TIME` writer - NA"]
pub type PhyStopWaitTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn n_lanes(&self) -> NLanesR {
        NLanesR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn phy_stop_wait_time(&self) -> PhyStopWaitTimeR {
        PhyStopWaitTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn n_lanes(&mut self) -> NLanesW<'_, PhyIfCfgSpec> {
        NLanesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn phy_stop_wait_time(&mut self) -> PhyStopWaitTimeW<'_, PhyIfCfgSpec> {
        PhyStopWaitTimeW::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_if_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_if_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyIfCfgSpec;
impl crate::RegisterSpec for PhyIfCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_if_cfg::R`](R) reader structure"]
impl crate::Readable for PhyIfCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_if_cfg::W`](W) writer structure"]
impl crate::Writable for PhyIfCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_IF_CFG to value 0x01"]
impl crate::Resettable for PhyIfCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
