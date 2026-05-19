#[doc = "Register `INT_ST1` reader"]
pub type R = crate::R<IntSt1Spec>;
#[doc = "Field `TO_HS_TX` reader - NA"]
pub type ToHsTxR = crate::BitReader;
#[doc = "Field `TO_LP_RX` reader - NA"]
pub type ToLpRxR = crate::BitReader;
#[doc = "Field `ECC_SINGLE_ERR` reader - NA"]
pub type EccSingleErrR = crate::BitReader;
#[doc = "Field `ECC_MILTI_ERR` reader - NA"]
pub type EccMiltiErrR = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - NA"]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `PKT_SIZE_ERR` reader - NA"]
pub type PktSizeErrR = crate::BitReader;
#[doc = "Field `EOPT_ERR` reader - NA"]
pub type EoptErrR = crate::BitReader;
#[doc = "Field `DPI_PLD_WR_ERR` reader - NA"]
pub type DpiPldWrErrR = crate::BitReader;
#[doc = "Field `GEN_CMD_WR_ERR` reader - NA"]
pub type GenCmdWrErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_WR_ERR` reader - NA"]
pub type GenPldWrErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_SEND_ERR` reader - NA"]
pub type GenPldSendErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_RD_ERR` reader - NA"]
pub type GenPldRdErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_RECEV_ERR` reader - NA"]
pub type GenPldRecevErrR = crate::BitReader;
#[doc = "Field `DPI_BUFF_PLD_UNDER` reader - NA"]
pub type DpiBuffPldUnderR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn to_hs_tx(&self) -> ToHsTxR {
        ToHsTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn to_lp_rx(&self) -> ToLpRxR {
        ToLpRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ecc_single_err(&self) -> EccSingleErrR {
        EccSingleErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ecc_milti_err(&self) -> EccMiltiErrR {
        EccMiltiErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn pkt_size_err(&self) -> PktSizeErrR {
        PktSizeErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn eopt_err(&self) -> EoptErrR {
        EoptErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn dpi_pld_wr_err(&self) -> DpiPldWrErrR {
        DpiPldWrErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn gen_cmd_wr_err(&self) -> GenCmdWrErrR {
        GenCmdWrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn gen_pld_wr_err(&self) -> GenPldWrErrR {
        GenPldWrErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn gen_pld_send_err(&self) -> GenPldSendErrR {
        GenPldSendErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn gen_pld_rd_err(&self) -> GenPldRdErrR {
        GenPldRdErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn gen_pld_recev_err(&self) -> GenPldRecevErrR {
        GenPldRecevErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dpi_buff_pld_under(&self) -> DpiBuffPldUnderR {
        DpiBuffPldUnderR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSt1Spec;
impl crate::RegisterSpec for IntSt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st1::R`](R) reader structure"]
impl crate::Readable for IntSt1Spec {}
#[doc = "`reset()` method sets INT_ST1 to value 0"]
impl crate::Resettable for IntSt1Spec {}
