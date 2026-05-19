#[doc = "Register `PHY_HS_LOOPBACK_CTRL` reader"]
pub type R = crate::R<PhyHsLoopbackCtrlSpec>;
#[doc = "Register `PHY_HS_LOOPBACK_CTRL` writer"]
pub type W = crate::W<PhyHsLoopbackCtrlSpec>;
#[doc = "Field `PHY_HS_TXDATAHS_1` reader - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxdatahs1R = crate::FieldReader;
#[doc = "Field `PHY_HS_TXDATAHS_1` writer - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxdatahs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_1` reader - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequestdatahs1R = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_1` writer - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequestdatahs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_BASEDIR_1` reader - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsBasedir1R = crate::BitReader;
#[doc = "Field `PHY_HS_BASEDIR_1` writer - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsBasedir1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_TXDATAHS_0` reader - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxdatahs0R = crate::FieldReader;
#[doc = "Field `PHY_HS_TXDATAHS_0` writer - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxdatahs0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_0` reader - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequestdatahs0R = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_0` writer - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequestdatahs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_BASEDIR_0` reader - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsBasedir0R = crate::BitReader;
#[doc = "Field `PHY_HS_BASEDIR_0` writer - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PhyHsBasedir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_TXREQUESTHSCLK` reader - txrequesthsclk when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequesthsclkR = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTHSCLK` writer - txrequesthsclk when enable dsi phy hs_loopback_test"]
pub type PhyHsTxrequesthsclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_CHECK` writer - dsi phy hs_loopback test start check"]
pub type PhyHsLoopbackCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_CHECK_DONE` reader - dsi phy hs_loopback test check done"]
pub type PhyHsLoopbackCheckDoneR = crate::BitReader;
#[doc = "Field `PHY_HS_LOOPBACK_EN` reader - dsi phy hs_loopback ctrl en"]
pub type PhyHsLoopbackEnR = crate::BitReader;
#[doc = "Field `PHY_HS_LOOPBACK_EN` writer - dsi phy hs_loopback ctrl en"]
pub type PhyHsLoopbackEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_OK` reader - result of dsi phy hs_loopback test"]
pub type PhyHsLoopbackOkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_1(&self) -> PhyHsTxdatahs1R {
        PhyHsTxdatahs1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_1(&self) -> PhyHsTxrequestdatahs1R {
        PhyHsTxrequestdatahs1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_1(&self) -> PhyHsBasedir1R {
        PhyHsBasedir1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_0(&self) -> PhyHsTxdatahs0R {
        PhyHsTxdatahs0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_0(&self) -> PhyHsTxrequestdatahs0R {
        PhyHsTxrequestdatahs0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_0(&self) -> PhyHsBasedir0R {
        PhyHsBasedir0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - txrequesthsclk when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequesthsclk(&self) -> PhyHsTxrequesthsclkR {
        PhyHsTxrequesthsclkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - dsi phy hs_loopback test check done"]
    #[inline(always)]
    pub fn phy_hs_loopback_check_done(&self) -> PhyHsLoopbackCheckDoneR {
        PhyHsLoopbackCheckDoneR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - dsi phy hs_loopback ctrl en"]
    #[inline(always)]
    pub fn phy_hs_loopback_en(&self) -> PhyHsLoopbackEnR {
        PhyHsLoopbackEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - result of dsi phy hs_loopback test"]
    #[inline(always)]
    pub fn phy_hs_loopback_ok(&self) -> PhyHsLoopbackOkR {
        PhyHsLoopbackOkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_1(&mut self) -> PhyHsTxdatahs1W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsTxdatahs1W::new(self, 0)
    }
    #[doc = "Bit 8 - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_1(
        &mut self,
    ) -> PhyHsTxrequestdatahs1W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsTxrequestdatahs1W::new(self, 8)
    }
    #[doc = "Bit 9 - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_1(&mut self) -> PhyHsBasedir1W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsBasedir1W::new(self, 9)
    }
    #[doc = "Bits 16:23 - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_0(&mut self) -> PhyHsTxdatahs0W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsTxdatahs0W::new(self, 16)
    }
    #[doc = "Bit 24 - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_0(
        &mut self,
    ) -> PhyHsTxrequestdatahs0W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsTxrequestdatahs0W::new(self, 24)
    }
    #[doc = "Bit 25 - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_0(&mut self) -> PhyHsBasedir0W<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsBasedir0W::new(self, 25)
    }
    #[doc = "Bit 27 - txrequesthsclk when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequesthsclk(&mut self) -> PhyHsTxrequesthsclkW<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsTxrequesthsclkW::new(self, 27)
    }
    #[doc = "Bit 28 - dsi phy hs_loopback test start check"]
    #[inline(always)]
    pub fn phy_hs_loopback_check(&mut self) -> PhyHsLoopbackCheckW<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsLoopbackCheckW::new(self, 28)
    }
    #[doc = "Bit 30 - dsi phy hs_loopback ctrl en"]
    #[inline(always)]
    pub fn phy_hs_loopback_en(&mut self) -> PhyHsLoopbackEnW<'_, PhyHsLoopbackCtrlSpec> {
        PhyHsLoopbackEnW::new(self, 30)
    }
}
#[doc = "dsi phy hp_loopback test ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_hs_loopback_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_hs_loopback_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyHsLoopbackCtrlSpec;
impl crate::RegisterSpec for PhyHsLoopbackCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_hs_loopback_ctrl::R`](R) reader structure"]
impl crate::Readable for PhyHsLoopbackCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_hs_loopback_ctrl::W`](W) writer structure"]
impl crate::Writable for PhyHsLoopbackCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_HS_LOOPBACK_CTRL to value 0x0200"]
impl crate::Resettable for PhyHsLoopbackCtrlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
