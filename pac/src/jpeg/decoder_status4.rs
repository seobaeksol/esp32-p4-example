#[doc = "Register `DECODER_STATUS4` reader"]
pub type R = crate::R<DecoderStatus4Spec>;
#[doc = "Field `BLOCK_EOF_CNT` reader - Reserved"]
pub type BlockEofCntR = crate::FieldReader<u32>;
#[doc = "Field `DEZIGZAG_READY` reader - Reserved"]
pub type DezigzagReadyR = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_CHECK` reader - Reserved"]
pub type DeFrameEofCheckR = crate::BitReader;
#[doc = "Field `DE_DMA2D_IN_PUSH` reader - Reserved"]
pub type DeDma2dInPushR = crate::BitReader;
impl R {
    #[doc = "Bits 0:25 - Reserved"]
    #[inline(always)]
    pub fn block_eof_cnt(&self) -> BlockEofCntR {
        BlockEofCntR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn dezigzag_ready(&self) -> DezigzagReadyR {
        DezigzagReadyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn de_frame_eof_check(&self) -> DeFrameEofCheckR {
        DeFrameEofCheckR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn de_dma2d_in_push(&self) -> DeDma2dInPushR {
        DeDma2dInPushR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecoderStatus4Spec;
impl crate::RegisterSpec for DecoderStatus4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status4::R`](R) reader structure"]
impl crate::Readable for DecoderStatus4Spec {}
#[doc = "`reset()` method sets DECODER_STATUS4 to value 0"]
impl crate::Resettable for DecoderStatus4Spec {}
