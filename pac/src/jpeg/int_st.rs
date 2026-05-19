#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `DONE` reader - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DoneR = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR` reader - The status interrupt bit to sign that rle parallel error when decoding."]
pub type RleParallelErrR = crate::BitReader;
#[doc = "Field `CID_ERR` reader - The status interrupt bit to sign that scan id check with component fails when decoding."]
pub type CidErrR = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR` reader - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type CDhtDcIdErrR = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR` reader - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type CDhtAcIdErrR = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR` reader - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type CDqtIdErrR = crate::BitReader;
#[doc = "Field `RST_UXP_ERR` reader - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RstUxpErrR = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR` reader - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RstCheckNoneErrR = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR` reader - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RstCheckPosErrR = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OutEofR = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR` reader - The status interrupt bit to sign that the selected source color mode is not supported."]
pub type SrColorModeErrR = crate::BitReader;
#[doc = "Field `DCT_DONE` reader - The status interrupt bit to sign that one dct calculation is finished."]
pub type DctDoneR = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF` reader - The status interrupt bit to sign that the coding process for last block is finished."]
pub type BsLastBlockEofR = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR` reader - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type ScanCheckNoneErrR = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR` reader - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type ScanCheckPosErrR = crate::BitReader;
#[doc = "Field `UXP_DET` reader - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UxpDetR = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR` reader - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EnFrameEofErrR = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK` reader - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EnFrameEofLackR = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR` reader - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DeFrameEofErrR = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK` reader - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DeFrameEofLackR = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR` reader - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SosUnmatchErrR = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN` reader - The status interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MarkerErrFstScanR = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` reader - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MarkerErrOtherScanR = crate::BitReader;
#[doc = "Field `UNDET` reader - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UndetR = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT` reader - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DecodeTimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err(&self) -> RleParallelErrR {
        RleParallelErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err(&self) -> CidErrR {
        CidErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err(&self) -> CDhtDcIdErrR {
        CDhtDcIdErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err(&self) -> CDhtAcIdErrR {
        CDhtAcIdErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err(&self) -> CDqtIdErrR {
        CDqtIdErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err(&self) -> RstUxpErrR {
        RstUxpErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err(&self) -> RstCheckNoneErrR {
        RstCheckNoneErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err(&self) -> RstCheckPosErrR {
        RstCheckPosErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof(&self) -> OutEofR {
        OutEofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err(&self) -> SrColorModeErrR {
        SrColorModeErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done(&self) -> DctDoneR {
        DctDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof(&self) -> BsLastBlockEofR {
        BsLastBlockEofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err(&self) -> ScanCheckNoneErrR {
        ScanCheckNoneErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err(&self) -> ScanCheckPosErrR {
        ScanCheckPosErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det(&self) -> UxpDetR {
        UxpDetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err(&self) -> EnFrameEofErrR {
        EnFrameEofErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack(&self) -> EnFrameEofLackR {
        EnFrameEofLackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err(&self) -> DeFrameEofErrR {
        DeFrameEofErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack(&self) -> DeFrameEofLackR {
        DeFrameEofLackR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err(&self) -> SosUnmatchErrR {
        SosUnmatchErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The status interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan(&self) -> MarkerErrFstScanR {
        MarkerErrFstScanR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan(&self) -> MarkerErrOtherScanR {
        MarkerErrOtherScanR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet(&self) -> UndetR {
        UndetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout(&self) -> DecodeTimeoutR {
        DecodeTimeoutR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
