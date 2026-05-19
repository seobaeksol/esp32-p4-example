#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `RXFIFO_WM` reader - The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt."]
pub type RxfifoWmR = crate::BitReader;
#[doc = "Field `TXFIFO_WM` reader - The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt."]
pub type TxfifoWmR = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt."]
pub type RxfifoOvfR = crate::BitReader;
#[doc = "Field `END_DETECT` reader - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
pub type EndDetectR = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE` reader - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
pub type ByteTransDoneR = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ArbitrationLostR = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF` reader - The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt."]
pub type MstTxfifoUdfR = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TransCompleteR = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - The masked interrupt status status of the I2C_TIME_OUT_INT interrupt."]
pub type TimeOutR = crate::BitReader;
#[doc = "Field `TRANS_START` reader - The masked interrupt status status of the I2C_TRANS_START_INT interrupt."]
pub type TransStartR = crate::BitReader;
#[doc = "Field `NACK` reader - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
pub type NackR = crate::BitReader;
#[doc = "Field `TXFIFO_OVF` reader - The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt."]
pub type TxfifoOvfR = crate::BitReader;
#[doc = "Field `RXFIFO_UDF` reader - The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt."]
pub type RxfifoUdfR = crate::BitReader;
#[doc = "Field `SCL_ST_TO` reader - The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt."]
pub type SclStToR = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO` reader - The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SclMainStToR = crate::BitReader;
#[doc = "Field `DET_START` reader - The masked interrupt status status of I2C_DET_START_INT interrupt."]
pub type DetStartR = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH` reader - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
pub type SlaveStretchR = crate::BitReader;
#[doc = "Field `GENERAL_CALL` reader - The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt."]
pub type GeneralCallR = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_UNMATCH` reader - The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
pub type SlaveAddrUnmatchR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status status of I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RxfifoWmR {
        RxfifoWmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status status of I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TxfifoWmR {
        TxfifoWmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status status of I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RxfifoOvfR {
        RxfifoOvfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> EndDetectR {
        EndDetectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&self) -> ByteTransDoneR {
        ByteTransDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status status of the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ArbitrationLostR {
        ArbitrationLostR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status status of I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MstTxfifoUdfR {
        MstTxfifoUdfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status status of the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TransCompleteR {
        TransCompleteR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status status of the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TimeOutR {
        TimeOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status status of the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TransStartR {
        TransStartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status status of I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TxfifoOvfR {
        TxfifoOvfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status status of I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RxfifoUdfR {
        RxfifoUdfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status status of I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&self) -> SclStToR {
        SclStToR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SclMainStToR {
        SclMainStToR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status status of I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&self) -> DetStartR {
        DetStartR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status status of I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&self) -> SlaveStretchR {
        SlaveStretchR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status status of I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&self) -> GeneralCallR {
        GeneralCallR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status status of I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&self) -> SlaveAddrUnmatchR {
        SlaveAddrUnmatchR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Status of captured I2C communication events\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
