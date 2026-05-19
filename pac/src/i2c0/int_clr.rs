#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `RXFIFO_WM` writer - Write 1 to clear I2C_RXFIFO_WM_INT interrupt."]
pub type RxfifoWmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_WM` writer - Write 1 to clear I2C_TXFIFO_WM_INT interrupt."]
pub type TxfifoWmW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_OVF` writer - Write 1 to clear I2C_RXFIFO_OVF_INT interrupt."]
pub type RxfifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `END_DETECT` writer - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
pub type EndDetectW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BYTE_TRANS_DONE` writer - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
pub type ByteTransDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` writer - Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ArbitrationLostW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_TXFIFO_UDF` writer - Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt."]
pub type MstTxfifoUdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` writer - Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TransCompleteW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TIME_OUT` writer - Write 1 to clear the I2C_TIME_OUT_INT interrupt."]
pub type TimeOutW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TRANS_START` writer - Write 1 to clear the I2C_TRANS_START_INT interrupt."]
pub type TransStartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NACK` writer - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type NackW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFIFO_OVF` writer - Write 1 to clear I2C_TXFIFO_OVF_INT interrupt."]
pub type TxfifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFIFO_UDF` writer - Write 1 to clear I2C_RXFIFO_UDF_INT interrupt."]
pub type RxfifoUdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCL_ST_TO` writer - Write 1 to clear I2C_SCL_ST_TO_INT interrupt."]
pub type SclStToW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SCL_MAIN_ST_TO` writer - Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SclMainStToW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DET_START` writer - Write 1 to clear I2C_DET_START_INT interrupt."]
pub type DetStartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLAVE_STRETCH` writer - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type SlaveStretchW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GENERAL_CALL` writer - Write 1 to clear I2C_GENARAL_CALL_INT interrupt."]
pub type GeneralCallW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLAVE_ADDR_UNMATCH` writer - Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
pub type SlaveAddrUnmatchW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&mut self) -> RxfifoWmW<'_, IntClrSpec> {
        RxfifoWmW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&mut self) -> TxfifoWmW<'_, IntClrSpec> {
        TxfifoWmW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RxfifoOvfW<'_, IntClrSpec> {
        RxfifoOvfW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&mut self) -> EndDetectW<'_, IntClrSpec> {
        EndDetectW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&mut self) -> ByteTransDoneW<'_, IntClrSpec> {
        ByteTransDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&mut self) -> ArbitrationLostW<'_, IntClrSpec> {
        ArbitrationLostW::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&mut self) -> MstTxfifoUdfW<'_, IntClrSpec> {
        MstTxfifoUdfW::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TransCompleteW<'_, IntClrSpec> {
        TransCompleteW::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&mut self) -> TimeOutW<'_, IntClrSpec> {
        TimeOutW::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TransStartW<'_, IntClrSpec> {
        TransStartW::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IntClrSpec> {
        NackW::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to clear I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&mut self) -> TxfifoOvfW<'_, IntClrSpec> {
        TxfifoOvfW::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to clear I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&mut self) -> RxfifoUdfW<'_, IntClrSpec> {
        RxfifoUdfW::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to clear I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&mut self) -> SclStToW<'_, IntClrSpec> {
        SclStToW::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&mut self) -> SclMainStToW<'_, IntClrSpec> {
        SclMainStToW::new(self, 14)
    }
    #[doc = "Bit 15 - Write 1 to clear I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&mut self) -> DetStartW<'_, IntClrSpec> {
        DetStartW::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&mut self) -> SlaveStretchW<'_, IntClrSpec> {
        SlaveStretchW::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to clear I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&mut self) -> GeneralCallW<'_, IntClrSpec> {
        GeneralCallW::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to clear I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&mut self) -> SlaveAddrUnmatchW<'_, IntClrSpec> {
        SlaveAddrUnmatchW::new(self, 18)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0007_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
