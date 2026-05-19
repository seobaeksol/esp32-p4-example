#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `VADR_NUM_GT` reader - reg_vadr_num is greater than real interrupt raw."]
pub type VadrNumGtR = crate::BitReader;
#[doc = "Field `VADR_NUM_GT` writer - reg_vadr_num is greater than real interrupt raw."]
pub type VadrNumGtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VADR_NUM_LT` reader - reg_vadr_num is less than real interrupt raw."]
pub type VadrNumLtR = crate::BitReader;
#[doc = "Field `VADR_NUM_LT` writer - reg_vadr_num is less than real interrupt raw."]
pub type VadrNumLtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD` reader - an incomplete frame of data was sent interrupt raw."]
pub type DiscardR = crate::BitReader;
#[doc = "Field `DISCARD` writer - an incomplete frame of data was sent interrupt raw."]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_BUF_OVERRUN` reader - buffer overrun interrupt raw."]
pub type CsiBufOverrunR = crate::BitReader;
#[doc = "Field `CSI_BUF_OVERRUN` writer - buffer overrun interrupt raw."]
pub type CsiBufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_ASYNC_FIFO_OVF` reader - buffer overflow interrupt raw."]
pub type CsiAsyncFifoOvfR = crate::BitReader;
#[doc = "Field `CSI_ASYNC_FIFO_OVF` writer - buffer overflow interrupt raw."]
pub type CsiAsyncFifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CFG_HAS_UPDATED` reader - dma configuration update complete interrupt raw."]
pub type DmaCfgHasUpdatedR = crate::BitReader;
#[doc = "Field `DMA_CFG_HAS_UPDATED` writer - dma configuration update complete interrupt raw."]
pub type DmaCfgHasUpdatedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_gt(&self) -> VadrNumGtR {
        VadrNumGtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_lt(&self) -> VadrNumLtR {
        VadrNumLtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt raw."]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - buffer overrun interrupt raw."]
    #[inline(always)]
    pub fn csi_buf_overrun(&self) -> CsiBufOverrunR {
        CsiBufOverrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer overflow interrupt raw."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&self) -> CsiAsyncFifoOvfR {
        CsiAsyncFifoOvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt raw."]
    #[inline(always)]
    pub fn dma_cfg_has_updated(&self) -> DmaCfgHasUpdatedR {
        DmaCfgHasUpdatedR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_gt(&mut self) -> VadrNumGtW<'_, IntRawSpec> {
        VadrNumGtW::new(self, 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt raw."]
    #[inline(always)]
    pub fn vadr_num_lt(&mut self) -> VadrNumLtW<'_, IntRawSpec> {
        VadrNumLtW::new(self, 1)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt raw."]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<'_, IntRawSpec> {
        DiscardW::new(self, 2)
    }
    #[doc = "Bit 3 - buffer overrun interrupt raw."]
    #[inline(always)]
    pub fn csi_buf_overrun(&mut self) -> CsiBufOverrunW<'_, IntRawSpec> {
        CsiBufOverrunW::new(self, 3)
    }
    #[doc = "Bit 4 - buffer overflow interrupt raw."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf(&mut self) -> CsiAsyncFifoOvfW<'_, IntRawSpec> {
        CsiAsyncFifoOvfW::new(self, 4)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt raw."]
    #[inline(always)]
    pub fn dma_cfg_has_updated(&mut self) -> DmaCfgHasUpdatedW<'_, IntRawSpec> {
        DmaCfgHasUpdatedW::new(self, 5)
    }
}
#[doc = "csi bridge interrupt raw.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
