#[doc = "Register `CMD_MODE_CFG` reader"]
pub type R = crate::R<CmdModeCfgSpec>;
#[doc = "Register `CMD_MODE_CFG` writer"]
pub type W = crate::W<CmdModeCfgSpec>;
#[doc = "Field `TEAR_FX_EN` reader - NA"]
pub type TearFxEnR = crate::BitReader;
#[doc = "Field `TEAR_FX_EN` writer - NA"]
pub type TearFxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_RQST_EN` reader - NA"]
pub type AckRqstEnR = crate::BitReader;
#[doc = "Field `ACK_RQST_EN` writer - NA"]
pub type AckRqstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_0P_TX` reader - NA"]
pub type GenSw0pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_0P_TX` writer - NA"]
pub type GenSw0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_1P_TX` reader - NA"]
pub type GenSw1pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_1P_TX` writer - NA"]
pub type GenSw1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_2P_TX` reader - NA"]
pub type GenSw2pTxR = crate::BitReader;
#[doc = "Field `GEN_SW_2P_TX` writer - NA"]
pub type GenSw2pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_0P_TX` reader - NA"]
pub type GenSr0pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_0P_TX` writer - NA"]
pub type GenSr0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_1P_TX` reader - NA"]
pub type GenSr1pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_1P_TX` writer - NA"]
pub type GenSr1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_2P_TX` reader - NA"]
pub type GenSr2pTxR = crate::BitReader;
#[doc = "Field `GEN_SR_2P_TX` writer - NA"]
pub type GenSr2pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_LW_TX` reader - NA"]
pub type GenLwTxR = crate::BitReader;
#[doc = "Field `GEN_LW_TX` writer - NA"]
pub type GenLwTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_0P_TX` reader - NA"]
pub type DcsSw0pTxR = crate::BitReader;
#[doc = "Field `DCS_SW_0P_TX` writer - NA"]
pub type DcsSw0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_1P_TX` reader - NA"]
pub type DcsSw1pTxR = crate::BitReader;
#[doc = "Field `DCS_SW_1P_TX` writer - NA"]
pub type DcsSw1pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SR_0P_TX` reader - NA"]
pub type DcsSr0pTxR = crate::BitReader;
#[doc = "Field `DCS_SR_0P_TX` writer - NA"]
pub type DcsSr0pTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_LW_TX` reader - NA"]
pub type DcsLwTxR = crate::BitReader;
#[doc = "Field `DCS_LW_TX` writer - NA"]
pub type DcsLwTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_RD_PKT_SIZE` reader - NA"]
pub type MaxRdPktSizeR = crate::BitReader;
#[doc = "Field `MAX_RD_PKT_SIZE` writer - NA"]
pub type MaxRdPktSizeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tear_fx_en(&self) -> TearFxEnR {
        TearFxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ack_rqst_en(&self) -> AckRqstEnR {
        AckRqstEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn gen_sw_0p_tx(&self) -> GenSw0pTxR {
        GenSw0pTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn gen_sw_1p_tx(&self) -> GenSw1pTxR {
        GenSw1pTxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn gen_sw_2p_tx(&self) -> GenSw2pTxR {
        GenSw2pTxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn gen_sr_0p_tx(&self) -> GenSr0pTxR {
        GenSr0pTxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn gen_sr_1p_tx(&self) -> GenSr1pTxR {
        GenSr1pTxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn gen_sr_2p_tx(&self) -> GenSr2pTxR {
        GenSr2pTxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn gen_lw_tx(&self) -> GenLwTxR {
        GenLwTxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dcs_sw_0p_tx(&self) -> DcsSw0pTxR {
        DcsSw0pTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dcs_sw_1p_tx(&self) -> DcsSw1pTxR {
        DcsSw1pTxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn dcs_sr_0p_tx(&self) -> DcsSr0pTxR {
        DcsSr0pTxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dcs_lw_tx(&self) -> DcsLwTxR {
        DcsLwTxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn max_rd_pkt_size(&self) -> MaxRdPktSizeR {
        MaxRdPktSizeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tear_fx_en(&mut self) -> TearFxEnW<'_, CmdModeCfgSpec> {
        TearFxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ack_rqst_en(&mut self) -> AckRqstEnW<'_, CmdModeCfgSpec> {
        AckRqstEnW::new(self, 1)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn gen_sw_0p_tx(&mut self) -> GenSw0pTxW<'_, CmdModeCfgSpec> {
        GenSw0pTxW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn gen_sw_1p_tx(&mut self) -> GenSw1pTxW<'_, CmdModeCfgSpec> {
        GenSw1pTxW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn gen_sw_2p_tx(&mut self) -> GenSw2pTxW<'_, CmdModeCfgSpec> {
        GenSw2pTxW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn gen_sr_0p_tx(&mut self) -> GenSr0pTxW<'_, CmdModeCfgSpec> {
        GenSr0pTxW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn gen_sr_1p_tx(&mut self) -> GenSr1pTxW<'_, CmdModeCfgSpec> {
        GenSr1pTxW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn gen_sr_2p_tx(&mut self) -> GenSr2pTxW<'_, CmdModeCfgSpec> {
        GenSr2pTxW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn gen_lw_tx(&mut self) -> GenLwTxW<'_, CmdModeCfgSpec> {
        GenLwTxW::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dcs_sw_0p_tx(&mut self) -> DcsSw0pTxW<'_, CmdModeCfgSpec> {
        DcsSw0pTxW::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dcs_sw_1p_tx(&mut self) -> DcsSw1pTxW<'_, CmdModeCfgSpec> {
        DcsSw1pTxW::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn dcs_sr_0p_tx(&mut self) -> DcsSr0pTxW<'_, CmdModeCfgSpec> {
        DcsSr0pTxW::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dcs_lw_tx(&mut self) -> DcsLwTxW<'_, CmdModeCfgSpec> {
        DcsLwTxW::new(self, 19)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn max_rd_pkt_size(&mut self) -> MaxRdPktSizeW<'_, CmdModeCfgSpec> {
        MaxRdPktSizeW::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_mode_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_mode_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdModeCfgSpec;
impl crate::RegisterSpec for CmdModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_mode_cfg::R`](R) reader structure"]
impl crate::Readable for CmdModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_mode_cfg::W`](W) writer structure"]
impl crate::Writable for CmdModeCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD_MODE_CFG to value 0"]
impl crate::Resettable for CmdModeCfgSpec {}
