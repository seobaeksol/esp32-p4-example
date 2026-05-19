#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `VADR_NUM_GT_REAL` writer - reg_vadr_num is greater than real interrupt clr."]
pub type VadrNumGtRealW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VADR_NUM_LT_REAL` writer - reg_vadr_num is less than real interrupt clr."]
pub type VadrNumLtRealW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DISCARD` writer - an incomplete frame of data was sent interrupt clr."]
pub type DiscardW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN` writer - buffer overrun interrupt clr."]
pub type CsiBufOverrunW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF` writer - buffer overflow interrupt clr."]
pub type CsiAsyncFifoOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED` writer - dma configuration update complete interrupt clr."]
pub type DmaCfgHasUpdatedW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt clr."]
    #[inline(always)]
    pub fn vadr_num_gt_real(&mut self) -> VadrNumGtRealW<'_, IntClrSpec> {
        VadrNumGtRealW::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt clr."]
    #[inline(always)]
    pub fn vadr_num_lt_real(&mut self) -> VadrNumLtRealW<'_, IntClrSpec> {
        VadrNumLtRealW::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt clr."]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<'_, IntClrSpec> {
        DiscardW::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt clr."]
    #[inline(always)]
    pub fn csi_buf_overrun(&mut self) -> CsiBufOverrunW<'_, IntClrSpec> {
        CsiBufOverrunW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt clr."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&mut self) -> CsiAsyncFifoOvfW<'_, IntClrSpec> {
        CsiAsyncFifoOvfW::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt clr."]
    #[inline(always)]
    pub fn dma_cfg_has_updated(&mut self) -> DmaCfgHasUpdatedW<'_, IntClrSpec> {
        DmaCfgHasUpdatedW::new(self, 5)
    }
}
#[doc = "csi bridge interrupt clr.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
