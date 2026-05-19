#[doc = "Register `OTG_CONF` reader"]
pub type R = crate::R<OtgConfSpec>;
#[doc = "Register `OTG_CONF` writer"]
pub type W = crate::W<OtgConfSpec>;
#[doc = "Field `SRP_SESSEND_OVERRIDE` reader - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software."]
pub type SrpSessendOverrideR = crate::BitReader;
#[doc = "Field `SRP_SESSEND_OVERRIDE` writer - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software."]
pub type SrpSessendOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRP_SESSEND_VALUE` reader - Software over-ride value of srp session end signal."]
pub type SrpSessendValueR = crate::BitReader;
#[doc = "Field `SRP_SESSEND_VALUE` writer - Software over-ride value of srp session end signal."]
pub type SrpSessendValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_SEL` reader - Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY."]
pub type PhySelR = crate::BitReader;
#[doc = "Field `PHY_SEL` writer - Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY."]
pub type PhySelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFO_FORCE_PD` reader - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DfifoForcePdR = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PD` writer - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DfifoForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCE_FLTR_BYPASS` reader - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DbnceFltrBypassR = crate::BitReader;
#[doc = "Field `DBNCE_FLTR_BYPASS` writer - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
pub type DbnceFltrBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCHG_PINS_OVERRIDE` reader - Enable software controlle USB D+ D- exchange"]
pub type ExchgPinsOverrideR = crate::BitReader;
#[doc = "Field `EXCHG_PINS_OVERRIDE` writer - Enable software controlle USB D+ D- exchange"]
pub type ExchgPinsOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCHG_PINS` reader - USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-."]
pub type ExchgPinsR = crate::BitReader;
#[doc = "Field `EXCHG_PINS` writer - USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-."]
pub type ExchgPinsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFH` reader - Control single-end input high threshold,1.76V to 2V, step 80mV."]
pub type VrefhR = crate::FieldReader;
#[doc = "Field `VREFH` writer - Control single-end input high threshold,1.76V to 2V, step 80mV."]
pub type VrefhW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREFL` reader - Control single-end input low threshold,0.8V to 1.04V, step 80mV."]
pub type VreflR = crate::FieldReader;
#[doc = "Field `VREFL` writer - Control single-end input low threshold,0.8V to 1.04V, step 80mV."]
pub type VreflW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VREF_OVERRIDE` reader - Enable software controlle input threshold."]
pub type VrefOverrideR = crate::BitReader;
#[doc = "Field `VREF_OVERRIDE` writer - Enable software controlle input threshold."]
pub type VrefOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_PULL_OVERRIDE` reader - Enable software controlle USB D+ D- pullup pulldown."]
pub type PadPullOverrideR = crate::BitReader;
#[doc = "Field `PAD_PULL_OVERRIDE` writer - Enable software controlle USB D+ D- pullup pulldown."]
pub type PadPullOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLUP` reader - Controlle USB D+ pullup."]
pub type DpPullupR = crate::BitReader;
#[doc = "Field `DP_PULLUP` writer - Controlle USB D+ pullup."]
pub type DpPullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP_PULLDOWN` reader - Controlle USB D+ pulldown."]
pub type DpPulldownR = crate::BitReader;
#[doc = "Field `DP_PULLDOWN` writer - Controlle USB D+ pulldown."]
pub type DpPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLUP` reader - Controlle USB D+ pullup."]
pub type DmPullupR = crate::BitReader;
#[doc = "Field `DM_PULLUP` writer - Controlle USB D+ pullup."]
pub type DmPullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_PULLDOWN` reader - Controlle USB D+ pulldown."]
pub type DmPulldownR = crate::BitReader;
#[doc = "Field `DM_PULLDOWN` writer - Controlle USB D+ pulldown."]
pub type DmPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLUP_VALUE` reader - Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K."]
pub type PullupValueR = crate::BitReader;
#[doc = "Field `PULLUP_VALUE` writer - Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K."]
pub type PullupValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_PAD_ENABLE` reader - Enable USB pad function."]
pub type UsbPadEnableR = crate::BitReader;
#[doc = "Field `USB_PAD_ENABLE` writer - Enable USB pad function."]
pub type UsbPadEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_CLK_FORCE_ON` reader - Force ahb clock always on."]
pub type AhbClkForceOnR = crate::BitReader;
#[doc = "Field `AHB_CLK_FORCE_ON` writer - Force ahb clock always on."]
pub type AhbClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CLK_FORCE_ON` reader - Force phy clock always on."]
pub type PhyClkForceOnR = crate::BitReader;
#[doc = "Field `PHY_CLK_FORCE_ON` writer - Force phy clock always on."]
pub type PhyClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TX_EDGE_SEL` reader - Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge."]
pub type PhyTxEdgeSelR = crate::BitReader;
#[doc = "Field `PHY_TX_EDGE_SEL` writer - Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge."]
pub type PhyTxEdgeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFIFO_FORCE_PU` reader - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DfifoForcePuR = crate::BitReader;
#[doc = "Field `DFIFO_FORCE_PU` writer - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
pub type DfifoForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Disable auto clock gating of CSR registers."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Disable auto clock gating of CSR registers."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&self) -> SrpSessendOverrideR {
        SrpSessendOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&self) -> SrpSessendValueR {
        SrpSessendValueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&self) -> PhySelR {
        PhySelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&self) -> DfifoForcePdR {
        DfifoForcePdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&self) -> DbnceFltrBypassR {
        DbnceFltrBypassR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> ExchgPinsOverrideR {
        ExchgPinsOverrideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-."]
    #[inline(always)]
    pub fn exchg_pins(&self) -> ExchgPinsR {
        ExchgPinsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV."]
    #[inline(always)]
    pub fn vrefh(&self) -> VrefhR {
        VrefhR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV."]
    #[inline(always)]
    pub fn vrefl(&self) -> VreflR {
        VreflR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold."]
    #[inline(always)]
    pub fn vref_override(&self) -> VrefOverrideR {
        VrefOverrideR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown."]
    #[inline(always)]
    pub fn pad_pull_override(&self) -> PadPullOverrideR {
        PadPullOverrideR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup."]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DpPullupR {
        DpPullupR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown."]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DpPulldownR {
        DpPulldownR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup."]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DmPullupR {
        DmPullupR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown."]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DmPulldownR {
        DmPulldownR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&self) -> PullupValueR {
        PullupValueR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> UsbPadEnableR {
        UsbPadEnableR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force ahb clock always on."]
    #[inline(always)]
    pub fn ahb_clk_force_on(&self) -> AhbClkForceOnR {
        AhbClkForceOnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Force phy clock always on."]
    #[inline(always)]
    pub fn phy_clk_force_on(&self) -> PhyClkForceOnR {
        PhyClkForceOnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&self) -> PhyTxEdgeSelR {
        PhyTxEdgeSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&self) -> DfifoForcePuR {
        DfifoForcePuR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software."]
    #[inline(always)]
    pub fn srp_sessend_override(&mut self) -> SrpSessendOverrideW<'_, OtgConfSpec> {
        SrpSessendOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Software over-ride value of srp session end signal."]
    #[inline(always)]
    pub fn srp_sessend_value(&mut self) -> SrpSessendValueW<'_, OtgConfSpec> {
        SrpSessendValueW::new(self, 1)
    }
    #[doc = "Bit 2 - Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY."]
    #[inline(always)]
    pub fn phy_sel(&mut self) -> PhySelW<'_, OtgConfSpec> {
        PhySelW::new(self, 2)
    }
    #[doc = "Bit 3 - Force the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pd(&mut self) -> DfifoForcePdW<'_, OtgConfSpec> {
        DfifoForcePdW::new(self, 3)
    }
    #[doc = "Bit 4 - Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals"]
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&mut self) -> DbnceFltrBypassW<'_, OtgConfSpec> {
        DbnceFltrBypassW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable software controlle USB D+ D- exchange"]
    #[inline(always)]
    pub fn exchg_pins_override(&mut self) -> ExchgPinsOverrideW<'_, OtgConfSpec> {
        ExchgPinsOverrideW::new(self, 5)
    }
    #[doc = "Bit 6 - USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-."]
    #[inline(always)]
    pub fn exchg_pins(&mut self) -> ExchgPinsW<'_, OtgConfSpec> {
        ExchgPinsW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Control single-end input high threshold,1.76V to 2V, step 80mV."]
    #[inline(always)]
    pub fn vrefh(&mut self) -> VrefhW<'_, OtgConfSpec> {
        VrefhW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Control single-end input low threshold,0.8V to 1.04V, step 80mV."]
    #[inline(always)]
    pub fn vrefl(&mut self) -> VreflW<'_, OtgConfSpec> {
        VreflW::new(self, 9)
    }
    #[doc = "Bit 11 - Enable software controlle input threshold."]
    #[inline(always)]
    pub fn vref_override(&mut self) -> VrefOverrideW<'_, OtgConfSpec> {
        VrefOverrideW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable software controlle USB D+ D- pullup pulldown."]
    #[inline(always)]
    pub fn pad_pull_override(&mut self) -> PadPullOverrideW<'_, OtgConfSpec> {
        PadPullOverrideW::new(self, 12)
    }
    #[doc = "Bit 13 - Controlle USB D+ pullup."]
    #[inline(always)]
    pub fn dp_pullup(&mut self) -> DpPullupW<'_, OtgConfSpec> {
        DpPullupW::new(self, 13)
    }
    #[doc = "Bit 14 - Controlle USB D+ pulldown."]
    #[inline(always)]
    pub fn dp_pulldown(&mut self) -> DpPulldownW<'_, OtgConfSpec> {
        DpPulldownW::new(self, 14)
    }
    #[doc = "Bit 15 - Controlle USB D+ pullup."]
    #[inline(always)]
    pub fn dm_pullup(&mut self) -> DmPullupW<'_, OtgConfSpec> {
        DmPullupW::new(self, 15)
    }
    #[doc = "Bit 16 - Controlle USB D+ pulldown."]
    #[inline(always)]
    pub fn dm_pulldown(&mut self) -> DmPulldownW<'_, OtgConfSpec> {
        DmPulldownW::new(self, 16)
    }
    #[doc = "Bit 17 - Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K."]
    #[inline(always)]
    pub fn pullup_value(&mut self) -> PullupValueW<'_, OtgConfSpec> {
        PullupValueW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable USB pad function."]
    #[inline(always)]
    pub fn usb_pad_enable(&mut self) -> UsbPadEnableW<'_, OtgConfSpec> {
        UsbPadEnableW::new(self, 18)
    }
    #[doc = "Bit 19 - Force ahb clock always on."]
    #[inline(always)]
    pub fn ahb_clk_force_on(&mut self) -> AhbClkForceOnW<'_, OtgConfSpec> {
        AhbClkForceOnW::new(self, 19)
    }
    #[doc = "Bit 20 - Force phy clock always on."]
    #[inline(always)]
    pub fn phy_clk_force_on(&mut self) -> PhyClkForceOnW<'_, OtgConfSpec> {
        PhyClkForceOnW::new(self, 20)
    }
    #[doc = "Bit 21 - Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge."]
    #[inline(always)]
    pub fn phy_tx_edge_sel(&mut self) -> PhyTxEdgeSelW<'_, OtgConfSpec> {
        PhyTxEdgeSelW::new(self, 21)
    }
    #[doc = "Bit 22 - Disable the dfifo to go into low power mode. The data in dfifo will not lost."]
    #[inline(always)]
    pub fn dfifo_force_pu(&mut self) -> DfifoForcePuW<'_, OtgConfSpec> {
        DfifoForcePuW::new(self, 22)
    }
    #[doc = "Bit 31 - Disable auto clock gating of CSR registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, OtgConfSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "USB wrapper configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgConfSpec;
impl crate::RegisterSpec for OtgConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_conf::R`](R) reader structure"]
impl crate::Readable for OtgConfSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_conf::W`](W) writer structure"]
impl crate::Writable for OtgConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_CONF to value 0x0010_0000"]
impl crate::Resettable for OtgConfSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
