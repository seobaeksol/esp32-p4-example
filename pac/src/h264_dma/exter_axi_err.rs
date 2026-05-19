#[doc = "Register `EXTER_AXI_ERR` reader"]
pub type R = crate::R<ExterAxiErrSpec>;
#[doc = "Field `EXTER_RID_ERR_CNT` reader - AXI read id err cnt"]
pub type ExterRidErrCntR = crate::FieldReader;
#[doc = "Field `EXTER_RRESP_ERR_CNT` reader - AXI read resp err cnt"]
pub type ExterRrespErrCntR = crate::FieldReader;
#[doc = "Field `EXTER_WRESP_ERR_CNT` reader - AXI write resp err cnt"]
pub type ExterWrespErrCntR = crate::FieldReader;
#[doc = "Field `EXTER_RD_FIFO_CNT` reader - AXI read cmd fifo remain cmd count"]
pub type ExterRdFifoCntR = crate::FieldReader;
#[doc = "Field `EXTER_RD_BAK_FIFO_CNT` reader - AXI read backup cmd fifo remain cmd count"]
pub type ExterRdBakFifoCntR = crate::FieldReader;
#[doc = "Field `EXTER_WR_FIFO_CNT` reader - AXI write cmd fifo remain cmd count"]
pub type ExterWrFifoCntR = crate::FieldReader;
#[doc = "Field `EXTER_WR_BAK_FIFO_CNT` reader - AXI write backup cmd fifo remain cmd count"]
pub type ExterWrBakFifoCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AXI read id err cnt"]
    #[inline(always)]
    pub fn exter_rid_err_cnt(&self) -> ExterRidErrCntR {
        ExterRidErrCntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AXI read resp err cnt"]
    #[inline(always)]
    pub fn exter_rresp_err_cnt(&self) -> ExterRrespErrCntR {
        ExterRrespErrCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AXI write resp err cnt"]
    #[inline(always)]
    pub fn exter_wresp_err_cnt(&self) -> ExterWrespErrCntR {
        ExterWrespErrCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - AXI read cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn exter_rd_fifo_cnt(&self) -> ExterRdFifoCntR {
        ExterRdFifoCntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:18 - AXI read backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn exter_rd_bak_fifo_cnt(&self) -> ExterRdBakFifoCntR {
        ExterRdBakFifoCntR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21 - AXI write cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn exter_wr_fifo_cnt(&self) -> ExterWrFifoCntR {
        ExterWrFifoCntR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - AXI write backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn exter_wr_bak_fifo_cnt(&self) -> ExterWrBakFifoCntR {
        ExterWrBakFifoCntR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[doc = "exter memory axi err register\n\nYou can [`read`](crate::Reg::read) this register and get [`exter_axi_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExterAxiErrSpec;
impl crate::RegisterSpec for ExterAxiErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exter_axi_err::R`](R) reader structure"]
impl crate::Readable for ExterAxiErrSpec {}
#[doc = "`reset()` method sets EXTER_AXI_ERR to value 0"]
impl crate::Resettable for ExterAxiErrSpec {}
