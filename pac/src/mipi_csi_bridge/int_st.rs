#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `VADR_NUM_GT` reader - reg_vadr_num is greater than real interrupt st."]
pub type VadrNumGtR = crate::BitReader;
#[doc = "Field `VADR_NUM_LT` reader - reg_vadr_num is less than real interrupt st."]
pub type VadrNumLtR = crate::BitReader;
#[doc = "Field `DISCARD` reader - an incomplete frame of data was sent interrupt st."]
pub type DiscardR = crate::BitReader;
#[doc = "Field `CSI_BUF_OVERRUN` reader - buffer overrun interrupt st."]
pub type CsiBufOverrunR = crate::BitReader;
#[doc = "Field `CSI_ASYNC_FIFO_OVF` reader - buffer overflow interrupt st."]
pub type CsiAsyncFifoOvfR = crate::BitReader;
#[doc = "Field `DMA_CFG_HAS_UPDATED` reader - dma configuration update complete interrupt st."]
pub type DmaCfgHasUpdatedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt st."]
    #[inline(always)]
    pub fn vadr_num_gt(&self) -> VadrNumGtR {
        VadrNumGtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt st."]
    #[inline(always)]
    pub fn vadr_num_lt(&self) -> VadrNumLtR {
        VadrNumLtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt st."]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - buffer overrun interrupt st."]
    #[inline(always)]
    pub fn csi_buf_overrun(&self) -> CsiBufOverrunR {
        CsiBufOverrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer overflow interrupt st."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&self) -> CsiAsyncFifoOvfR {
        CsiAsyncFifoOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt st."]
    #[inline(always)]
    pub fn dma_cfg_has_updated(&self) -> DmaCfgHasUpdatedR {
        DmaCfgHasUpdatedR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "csi bridge interrupt st.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
