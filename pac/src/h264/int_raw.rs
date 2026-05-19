#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `DB_TMP_READY` reader - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
pub type DbTmpReadyR = crate::BitReader;
#[doc = "Field `DB_TMP_READY` writer - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
pub type DbTmpReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REC_READY` reader - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
pub type RecReadyR = crate::BitReader;
#[doc = "Field `REC_READY` writer - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
pub type RecReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_DONE` reader - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` reader - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
pub type DmaMove2mbLineDoneR = crate::BitReader;
#[doc = "Field `DMA_MOVE_2MB_LINE_DONE` writer - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
pub type DmaMove2mbLineDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_BUFFER_OVERFLOW` reader - Raw status bit: The raw interrupt status of H264_BS_BUFFER_OVERFLOW_INT. Triggered when H264 bit stream buffer overflow."]
pub type BsBufferOverflowR = crate::BitReader;
#[doc = "Field `BS_BUFFER_OVERFLOW` writer - Raw status bit: The raw interrupt status of H264_BS_BUFFER_OVERFLOW_INT. Triggered when H264 bit stream buffer overflow."]
pub type BsBufferOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
    #[inline(always)]
    pub fn db_tmp_ready(&self) -> DbTmpReadyR {
        DbTmpReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
    #[inline(always)]
    pub fn rec_ready(&self) -> RecReadyR {
        RecReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&self) -> DmaMove2mbLineDoneR {
        DmaMove2mbLineDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of H264_BS_BUFFER_OVERFLOW_INT. Triggered when H264 bit stream buffer overflow."]
    #[inline(always)]
    pub fn bs_buffer_overflow(&self) -> BsBufferOverflowR {
        BsBufferOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of H264_DB_TMP_READY_INT. Triggered when H264 written enough db tmp pixel."]
    #[inline(always)]
    pub fn db_tmp_ready(&mut self) -> DbTmpReadyW<'_, IntRawSpec> {
        DbTmpReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of H264_REC_READY_INT. Triggered when H264 encoding enough reconstruct pixel."]
    #[inline(always)]
    pub fn rec_ready(&mut self) -> RecReadyW<'_, IntRawSpec> {
        RecReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of H264_FRAME_DONE_INT. Triggered when H264 encoding one frame done."]
    #[inline(always)]
    pub fn frame_done(&mut self) -> FrameDoneW<'_, IntRawSpec> {
        FrameDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of H264_DMA_MOVE_2MB_LINE_DONE_INT. Triggered when H264 move two MB lines of reference frame from external mem to internal mem done."]
    #[inline(always)]
    pub fn dma_move_2mb_line_done(&mut self) -> DmaMove2mbLineDoneW<'_, IntRawSpec> {
        DmaMove2mbLineDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of H264_BS_BUFFER_OVERFLOW_INT. Triggered when H264 bit stream buffer overflow."]
    #[inline(always)]
    pub fn bs_buffer_overflow(&mut self) -> BsBufferOverflowW<'_, IntRawSpec> {
        BsBufferOverflowW::new(self, 4)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
