#[doc = "Register `DMA_BLOCK_INTERVAL` reader"]
pub type R = crate::R<DmaBlockIntervalSpec>;
#[doc = "Register `DMA_BLOCK_INTERVAL` writer"]
pub type W = crate::W<DmaBlockIntervalSpec>;
#[doc = "Field `DMA_BLOCK_SLOT` reader - this field configures the max block_slot_cnt"]
pub type DmaBlockSlotR = crate::FieldReader<u16>;
#[doc = "Field `DMA_BLOCK_SLOT` writer - this field configures the max block_slot_cnt"]
pub type DmaBlockSlotW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DMA_BLOCK_INTERVAL` reader - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
pub type DmaBlockIntervalR = crate::FieldReader<u32>;
#[doc = "Field `DMA_BLOCK_INTERVAL` writer - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
pub type DmaBlockIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `RAW_NUM_TOTAL_AUTO_RELOAD` reader - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
pub type RawNumTotalAutoReloadR = crate::BitReader;
#[doc = "Field `RAW_NUM_TOTAL_AUTO_RELOAD` writer - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
pub type RawNumTotalAutoReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - this field configures the max block_slot_cnt"]
    #[inline(always)]
    pub fn dma_block_slot(&self) -> DmaBlockSlotR {
        DmaBlockSlotR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:27 - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
    #[inline(always)]
    pub fn dma_block_interval(&self) -> DmaBlockIntervalR {
        DmaBlockIntervalR::new((self.bits >> 10) & 0x0003_ffff)
    }
    #[doc = "Bit 28 - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn raw_num_total_auto_reload(&self) -> RawNumTotalAutoReloadR {
        RawNumTotalAutoReloadR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - this field configures the max block_slot_cnt"]
    #[inline(always)]
    pub fn dma_block_slot(&mut self) -> DmaBlockSlotW<'_, DmaBlockIntervalSpec> {
        DmaBlockSlotW::new(self, 0)
    }
    #[doc = "Bits 10:27 - this field configures the max block_interval_cnt, block_interval_cnt increased by 1 when block_slot_cnt if full"]
    #[inline(always)]
    pub fn dma_block_interval(&mut self) -> DmaBlockIntervalW<'_, DmaBlockIntervalSpec> {
        DmaBlockIntervalW::new(self, 10)
    }
    #[doc = "Bit 28 - this bit configures enable of auto reload reg_raw_num_total, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn raw_num_total_auto_reload(
        &mut self,
    ) -> RawNumTotalAutoReloadW<'_, DmaBlockIntervalSpec> {
        RawNumTotalAutoReloadW::new(self, 28)
    }
    #[doc = "Bit 29 - this bit configures enable of interval between dma block transfer, 0: disable, 1: enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, DmaBlockIntervalSpec> {
        EnW::new(self, 29)
    }
}
#[doc = "dsi bridge dma block interval control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaBlockIntervalSpec;
impl crate::RegisterSpec for DmaBlockIntervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_block_interval::R`](R) reader structure"]
impl crate::Readable for DmaBlockIntervalSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_block_interval::W`](W) writer structure"]
impl crate::Writable for DmaBlockIntervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_BLOCK_INTERVAL to value 0x3000_2409"]
impl crate::Resettable for DmaBlockIntervalSpec {
    const RESET_VALUE: u32 = 0x3000_2409;
}
