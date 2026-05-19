#[doc = "Register `GMAC_CTRL0` reader"]
pub type R = crate::R<GmacCtrl0Spec>;
#[doc = "Register `GMAC_CTRL0` writer"]
pub type W = crate::W<GmacCtrl0Spec>;
#[doc = "Field `PTP_PPS` reader - N/A"]
pub type PtpPpsR = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` reader - N/A"]
pub type SbdFlowctrlR = crate::BitReader;
#[doc = "Field `SBD_FLOWCTRL` writer - N/A"]
pub type SbdFlowctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_INTF_SEL` reader - N/A"]
pub type PhyIntfSelR = crate::FieldReader;
#[doc = "Field `PHY_INTF_SEL` writer - N/A"]
pub type PhyIntfSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` reader - N/A"]
pub type GmacMemClkForceOnR = crate::BitReader;
#[doc = "Field `GMAC_MEM_CLK_FORCE_ON` writer - N/A"]
pub type GmacMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC_RST_CLK_TX_N` reader - N/A"]
pub type GmacRstClkTxNR = crate::BitReader;
#[doc = "Field `GMAC_RST_CLK_RX_N` reader - N/A"]
pub type GmacRstClkRxNR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn ptp_pps(&self) -> PtpPpsR {
        PtpPpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn sbd_flowctrl(&self) -> SbdFlowctrlR {
        SbdFlowctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - N/A"]
    #[inline(always)]
    pub fn phy_intf_sel(&self) -> PhyIntfSelR {
        PhyIntfSelR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&self) -> GmacMemClkForceOnR {
        GmacMemClkForceOnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn gmac_rst_clk_tx_n(&self) -> GmacRstClkTxNR {
        GmacRstClkTxNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn gmac_rst_clk_rx_n(&self) -> GmacRstClkRxNR {
        GmacRstClkRxNR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn sbd_flowctrl(&mut self) -> SbdFlowctrlW<'_, GmacCtrl0Spec> {
        SbdFlowctrlW::new(self, 1)
    }
    #[doc = "Bits 2:4 - N/A"]
    #[inline(always)]
    pub fn phy_intf_sel(&mut self) -> PhyIntfSelW<'_, GmacCtrl0Spec> {
        PhyIntfSelW::new(self, 2)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn gmac_mem_clk_force_on(&mut self) -> GmacMemClkForceOnW<'_, GmacCtrl0Spec> {
        GmacMemClkForceOnW::new(self, 5)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacCtrl0Spec;
impl crate::RegisterSpec for GmacCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl0::R`](R) reader structure"]
impl crate::Readable for GmacCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gmac_ctrl0::W`](W) writer structure"]
impl crate::Writable for GmacCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC_CTRL0 to value 0"]
impl crate::Resettable for GmacCtrl0Spec {}
