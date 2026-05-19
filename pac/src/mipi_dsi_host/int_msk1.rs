#[doc = "Register `INT_MSK1` reader"]
pub type R = crate::R<IntMsk1Spec>;
#[doc = "Register `INT_MSK1` writer"]
pub type W = crate::W<IntMsk1Spec>;
#[doc = "Field `MASK_TO_HS_TX` reader - NA"]
pub type MaskToHsTxR = crate::BitReader;
#[doc = "Field `MASK_TO_HS_TX` writer - NA"]
pub type MaskToHsTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_TO_LP_RX` reader - NA"]
pub type MaskToLpRxR = crate::BitReader;
#[doc = "Field `MASK_TO_LP_RX` writer - NA"]
pub type MaskToLpRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ECC_SINGLE_ERR` reader - NA"]
pub type MaskEccSingleErrR = crate::BitReader;
#[doc = "Field `MASK_ECC_SINGLE_ERR` writer - NA"]
pub type MaskEccSingleErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_ECC_MILTI_ERR` reader - NA"]
pub type MaskEccMiltiErrR = crate::BitReader;
#[doc = "Field `MASK_ECC_MILTI_ERR` writer - NA"]
pub type MaskEccMiltiErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_CRC_ERR` reader - NA"]
pub type MaskCrcErrR = crate::BitReader;
#[doc = "Field `MASK_CRC_ERR` writer - NA"]
pub type MaskCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_PKT_SIZE_ERR` reader - NA"]
pub type MaskPktSizeErrR = crate::BitReader;
#[doc = "Field `MASK_PKT_SIZE_ERR` writer - NA"]
pub type MaskPktSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_EOPT_ERR` reader - NA"]
pub type MaskEoptErrR = crate::BitReader;
#[doc = "Field `MASK_EOPT_ERR` writer - NA"]
pub type MaskEoptErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_DPI_PLD_WR_ERR` reader - NA"]
pub type MaskDpiPldWrErrR = crate::BitReader;
#[doc = "Field `MASK_DPI_PLD_WR_ERR` writer - NA"]
pub type MaskDpiPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_GEN_CMD_WR_ERR` reader - NA"]
pub type MaskGenCmdWrErrR = crate::BitReader;
#[doc = "Field `MASK_GEN_CMD_WR_ERR` writer - NA"]
pub type MaskGenCmdWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_GEN_PLD_WR_ERR` reader - NA"]
pub type MaskGenPldWrErrR = crate::BitReader;
#[doc = "Field `MASK_GEN_PLD_WR_ERR` writer - NA"]
pub type MaskGenPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_GEN_PLD_SEND_ERR` reader - NA"]
pub type MaskGenPldSendErrR = crate::BitReader;
#[doc = "Field `MASK_GEN_PLD_SEND_ERR` writer - NA"]
pub type MaskGenPldSendErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_GEN_PLD_RD_ERR` reader - NA"]
pub type MaskGenPldRdErrR = crate::BitReader;
#[doc = "Field `MASK_GEN_PLD_RD_ERR` writer - NA"]
pub type MaskGenPldRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_GEN_PLD_RECEV_ERR` reader - NA"]
pub type MaskGenPldRecevErrR = crate::BitReader;
#[doc = "Field `MASK_GEN_PLD_RECEV_ERR` writer - NA"]
pub type MaskGenPldRecevErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_DPI_BUFF_PLD_UNDER` reader - NA"]
pub type MaskDpiBuffPldUnderR = crate::BitReader;
#[doc = "Field `MASK_DPI_BUFF_PLD_UNDER` writer - NA"]
pub type MaskDpiBuffPldUnderW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_to_hs_tx(&self) -> MaskToHsTxR {
        MaskToHsTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_to_lp_rx(&self) -> MaskToLpRxR {
        MaskToLpRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn mask_ecc_single_err(&self) -> MaskEccSingleErrR {
        MaskEccSingleErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mask_ecc_milti_err(&self) -> MaskEccMiltiErrR {
        MaskEccMiltiErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn mask_crc_err(&self) -> MaskCrcErrR {
        MaskCrcErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn mask_pkt_size_err(&self) -> MaskPktSizeErrR {
        MaskPktSizeErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn mask_eopt_err(&self) -> MaskEoptErrR {
        MaskEoptErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn mask_dpi_pld_wr_err(&self) -> MaskDpiPldWrErrR {
        MaskDpiPldWrErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn mask_gen_cmd_wr_err(&self) -> MaskGenCmdWrErrR {
        MaskGenCmdWrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_wr_err(&self) -> MaskGenPldWrErrR {
        MaskGenPldWrErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_send_err(&self) -> MaskGenPldSendErrR {
        MaskGenPldSendErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_rd_err(&self) -> MaskGenPldRdErrR {
        MaskGenPldRdErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_recev_err(&self) -> MaskGenPldRecevErrR {
        MaskGenPldRecevErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn mask_dpi_buff_pld_under(&self) -> MaskDpiBuffPldUnderR {
        MaskDpiBuffPldUnderR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn mask_to_hs_tx(&mut self) -> MaskToHsTxW<'_, IntMsk1Spec> {
        MaskToHsTxW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn mask_to_lp_rx(&mut self) -> MaskToLpRxW<'_, IntMsk1Spec> {
        MaskToLpRxW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn mask_ecc_single_err(&mut self) -> MaskEccSingleErrW<'_, IntMsk1Spec> {
        MaskEccSingleErrW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mask_ecc_milti_err(&mut self) -> MaskEccMiltiErrW<'_, IntMsk1Spec> {
        MaskEccMiltiErrW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn mask_crc_err(&mut self) -> MaskCrcErrW<'_, IntMsk1Spec> {
        MaskCrcErrW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn mask_pkt_size_err(&mut self) -> MaskPktSizeErrW<'_, IntMsk1Spec> {
        MaskPktSizeErrW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn mask_eopt_err(&mut self) -> MaskEoptErrW<'_, IntMsk1Spec> {
        MaskEoptErrW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn mask_dpi_pld_wr_err(&mut self) -> MaskDpiPldWrErrW<'_, IntMsk1Spec> {
        MaskDpiPldWrErrW::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn mask_gen_cmd_wr_err(&mut self) -> MaskGenCmdWrErrW<'_, IntMsk1Spec> {
        MaskGenCmdWrErrW::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_wr_err(&mut self) -> MaskGenPldWrErrW<'_, IntMsk1Spec> {
        MaskGenPldWrErrW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_send_err(&mut self) -> MaskGenPldSendErrW<'_, IntMsk1Spec> {
        MaskGenPldSendErrW::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_rd_err(&mut self) -> MaskGenPldRdErrW<'_, IntMsk1Spec> {
        MaskGenPldRdErrW::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn mask_gen_pld_recev_err(&mut self) -> MaskGenPldRecevErrW<'_, IntMsk1Spec> {
        MaskGenPldRecevErrW::new(self, 12)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn mask_dpi_buff_pld_under(&mut self) -> MaskDpiBuffPldUnderW<'_, IntMsk1Spec> {
        MaskDpiBuffPldUnderW::new(self, 19)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_msk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_msk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMsk1Spec;
impl crate::RegisterSpec for IntMsk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_msk1::R`](R) reader structure"]
impl crate::Readable for IntMsk1Spec {}
#[doc = "`write(|w| ..)` method takes [`int_msk1::W`](W) writer structure"]
impl crate::Writable for IntMsk1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MSK1 to value 0"]
impl crate::Resettable for IntMsk1Spec {}
