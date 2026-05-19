#[doc = "Register `INT_FORCE1` reader"]
pub type R = crate::R<IntForce1Spec>;
#[doc = "Register `INT_FORCE1` writer"]
pub type W = crate::W<IntForce1Spec>;
#[doc = "Field `FORCE_TO_HS_TX` reader - NA"]
pub type ForceToHsTxR = crate::BitReader;
#[doc = "Field `FORCE_TO_HS_TX` writer - NA"]
pub type ForceToHsTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TO_LP_RX` reader - NA"]
pub type ForceToLpRxR = crate::BitReader;
#[doc = "Field `FORCE_TO_LP_RX` writer - NA"]
pub type ForceToLpRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ECC_SINGLE_ERR` reader - NA"]
pub type ForceEccSingleErrR = crate::BitReader;
#[doc = "Field `FORCE_ECC_SINGLE_ERR` writer - NA"]
pub type ForceEccSingleErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ECC_MILTI_ERR` reader - NA"]
pub type ForceEccMiltiErrR = crate::BitReader;
#[doc = "Field `FORCE_ECC_MILTI_ERR` writer - NA"]
pub type ForceEccMiltiErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CRC_ERR` reader - NA"]
pub type ForceCrcErrR = crate::BitReader;
#[doc = "Field `FORCE_CRC_ERR` writer - NA"]
pub type ForceCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PKT_SIZE_ERR` reader - NA"]
pub type ForcePktSizeErrR = crate::BitReader;
#[doc = "Field `FORCE_PKT_SIZE_ERR` writer - NA"]
pub type ForcePktSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_EOPT_ERR` reader - NA"]
pub type ForceEoptErrR = crate::BitReader;
#[doc = "Field `FORCE_EOPT_ERR` writer - NA"]
pub type ForceEoptErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DPI_PLD_WR_ERR` reader - NA"]
pub type ForceDpiPldWrErrR = crate::BitReader;
#[doc = "Field `FORCE_DPI_PLD_WR_ERR` writer - NA"]
pub type ForceDpiPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_CMD_WR_ERR` reader - NA"]
pub type ForceGenCmdWrErrR = crate::BitReader;
#[doc = "Field `FORCE_GEN_CMD_WR_ERR` writer - NA"]
pub type ForceGenCmdWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_WR_ERR` reader - NA"]
pub type ForceGenPldWrErrR = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_WR_ERR` writer - NA"]
pub type ForceGenPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_SEND_ERR` reader - NA"]
pub type ForceGenPldSendErrR = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_SEND_ERR` writer - NA"]
pub type ForceGenPldSendErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_RD_ERR` reader - NA"]
pub type ForceGenPldRdErrR = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_RD_ERR` writer - NA"]
pub type ForceGenPldRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_RECEV_ERR` reader - NA"]
pub type ForceGenPldRecevErrR = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_RECEV_ERR` writer - NA"]
pub type ForceGenPldRecevErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DPI_BUFF_PLD_UNDER` reader - NA"]
pub type ForceDpiBuffPldUnderR = crate::BitReader;
#[doc = "Field `FORCE_DPI_BUFF_PLD_UNDER` writer - NA"]
pub type ForceDpiBuffPldUnderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_to_hs_tx(&self) -> ForceToHsTxR {
        ForceToHsTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_to_lp_rx(&self) -> ForceToLpRxR {
        ForceToLpRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_ecc_single_err(&self) -> ForceEccSingleErrR {
        ForceEccSingleErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_ecc_milti_err(&self) -> ForceEccMiltiErrR {
        ForceEccMiltiErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_crc_err(&self) -> ForceCrcErrR {
        ForceCrcErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_pkt_size_err(&self) -> ForcePktSizeErrR {
        ForcePktSizeErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_eopt_err(&self) -> ForceEoptErrR {
        ForceEoptErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_dpi_pld_wr_err(&self) -> ForceDpiPldWrErrR {
        ForceDpiPldWrErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_gen_cmd_wr_err(&self) -> ForceGenCmdWrErrR {
        ForceGenCmdWrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_wr_err(&self) -> ForceGenPldWrErrR {
        ForceGenPldWrErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_send_err(&self) -> ForceGenPldSendErrR {
        ForceGenPldSendErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_rd_err(&self) -> ForceGenPldRdErrR {
        ForceGenPldRdErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_recev_err(&self) -> ForceGenPldRecevErrR {
        ForceGenPldRecevErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn force_dpi_buff_pld_under(&self) -> ForceDpiBuffPldUnderR {
        ForceDpiBuffPldUnderR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_to_hs_tx(&mut self) -> ForceToHsTxW<'_, IntForce1Spec> {
        ForceToHsTxW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_to_lp_rx(&mut self) -> ForceToLpRxW<'_, IntForce1Spec> {
        ForceToLpRxW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_ecc_single_err(&mut self) -> ForceEccSingleErrW<'_, IntForce1Spec> {
        ForceEccSingleErrW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_ecc_milti_err(&mut self) -> ForceEccMiltiErrW<'_, IntForce1Spec> {
        ForceEccMiltiErrW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_crc_err(&mut self) -> ForceCrcErrW<'_, IntForce1Spec> {
        ForceCrcErrW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_pkt_size_err(&mut self) -> ForcePktSizeErrW<'_, IntForce1Spec> {
        ForcePktSizeErrW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_eopt_err(&mut self) -> ForceEoptErrW<'_, IntForce1Spec> {
        ForceEoptErrW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_dpi_pld_wr_err(&mut self) -> ForceDpiPldWrErrW<'_, IntForce1Spec> {
        ForceDpiPldWrErrW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_gen_cmd_wr_err(&mut self) -> ForceGenCmdWrErrW<'_, IntForce1Spec> {
        ForceGenCmdWrErrW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_wr_err(&mut self) -> ForceGenPldWrErrW<'_, IntForce1Spec> {
        ForceGenPldWrErrW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_send_err(&mut self) -> ForceGenPldSendErrW<'_, IntForce1Spec> {
        ForceGenPldSendErrW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_rd_err(&mut self) -> ForceGenPldRdErrW<'_, IntForce1Spec> {
        ForceGenPldRdErrW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_recev_err(&mut self) -> ForceGenPldRecevErrW<'_, IntForce1Spec> {
        ForceGenPldRecevErrW::new(self, 12)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn force_dpi_buff_pld_under(&mut self) -> ForceDpiBuffPldUnderW<'_, IntForce1Spec> {
        ForceDpiBuffPldUnderW::new(self, 19)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForce1Spec;
impl crate::RegisterSpec for IntForce1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force1::R`](R) reader structure"]
impl crate::Readable for IntForce1Spec {}
#[doc = "`write(|w| ..)` method takes [`int_force1::W`](W) writer structure"]
impl crate::Writable for IntForce1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_FORCE1 to value 0"]
impl crate::Resettable for IntForce1Spec {}
