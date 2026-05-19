#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `DONE` reader - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE_PARALLEL_ERR` reader - The raw interrupt bit to sign that rle parallel error when decoding."]
pub type RleParallelErrR = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR` writer - The raw interrupt bit to sign that rle parallel error when decoding."]
pub type RleParallelErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CID_ERR` reader - The raw interrupt bit to sign that scan id check with component fails when decoding."]
pub type CidErrR = crate::BitReader;
#[doc = "Field `CID_ERR` writer - The raw interrupt bit to sign that scan id check with component fails when decoding."]
pub type CidErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_DC_ID_ERR` reader - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type CDhtDcIdErrR = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR` writer - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type CDhtDcIdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_AC_ID_ERR` reader - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type CDhtAcIdErrR = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR` writer - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type CDhtAcIdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DQT_ID_ERR` reader - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type CDqtIdErrR = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR` writer - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type CDqtIdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_UXP_ERR` reader - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RstUxpErrR = crate::BitReader;
#[doc = "Field `RST_UXP_ERR` writer - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RstUxpErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_NONE_ERR` reader - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RstCheckNoneErrR = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR` writer - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RstCheckNoneErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_POS_ERR` reader - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RstCheckPosErrR = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR` writer - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RstCheckPosErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OutEofR = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OutEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_COLOR_MODE_ERR` reader - The raw interrupt bit to sign that the selected source color mode is not supported."]
pub type SrColorModeErrR = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR` writer - The raw interrupt bit to sign that the selected source color mode is not supported."]
pub type SrColorModeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCT_DONE` reader - The raw interrupt bit to sign that one dct calculation is finished."]
pub type DctDoneR = crate::BitReader;
#[doc = "Field `DCT_DONE` writer - The raw interrupt bit to sign that one dct calculation is finished."]
pub type DctDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_LAST_BLOCK_EOF` reader - The raw interrupt bit to sign that the coding process for last block is finished."]
pub type BsLastBlockEofR = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF` writer - The raw interrupt bit to sign that the coding process for last block is finished."]
pub type BsLastBlockEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_NONE_ERR` reader - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type ScanCheckNoneErrR = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR` writer - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type ScanCheckNoneErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_POS_ERR` reader - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type ScanCheckPosErrR = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR` writer - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type ScanCheckPosErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UXP_DET` reader - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UxpDetR = crate::BitReader;
#[doc = "Field `UXP_DET` writer - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UxpDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_ERR` reader - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EnFrameEofErrR = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR` writer - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EnFrameEofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_LACK` reader - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EnFrameEofLackR = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK` writer - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EnFrameEofLackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_ERR` reader - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DeFrameEofErrR = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR` writer - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DeFrameEofErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_LACK` reader - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DeFrameEofLackR = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK` writer - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DeFrameEofLackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOS_UNMATCH_ERR` reader - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SosUnmatchErrR = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR` writer - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SosUnmatchErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_FST_SCAN` reader - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MarkerErrFstScanR = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN` writer - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MarkerErrFstScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` reader - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MarkerErrOtherScanR = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` writer - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MarkerErrOtherScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDET` reader - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UndetR = crate::BitReader;
#[doc = "Field `UNDET` writer - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UndetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE_TIMEOUT` reader - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DecodeTimeoutR = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT` writer - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DecodeTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err(&self) -> RleParallelErrR {
        RleParallelErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err(&self) -> CidErrR {
        CidErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err(&self) -> CDhtDcIdErrR {
        CDhtDcIdErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err(&self) -> CDhtAcIdErrR {
        CDhtAcIdErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err(&self) -> CDqtIdErrR {
        CDqtIdErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err(&self) -> RstUxpErrR {
        RstUxpErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err(&self) -> RstCheckNoneErrR {
        RstCheckNoneErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err(&self) -> RstCheckPosErrR {
        RstCheckPosErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof(&self) -> OutEofR {
        OutEofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err(&self) -> SrColorModeErrR {
        SrColorModeErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done(&self) -> DctDoneR {
        DctDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof(&self) -> BsLastBlockEofR {
        BsLastBlockEofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err(&self) -> ScanCheckNoneErrR {
        ScanCheckNoneErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err(&self) -> ScanCheckPosErrR {
        ScanCheckPosErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det(&self) -> UxpDetR {
        UxpDetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err(&self) -> EnFrameEofErrR {
        EnFrameEofErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack(&self) -> EnFrameEofLackR {
        EnFrameEofLackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err(&self) -> DeFrameEofErrR {
        DeFrameEofErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack(&self) -> DeFrameEofLackR {
        DeFrameEofLackR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err(&self) -> SosUnmatchErrR {
        SosUnmatchErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan(&self) -> MarkerErrFstScanR {
        MarkerErrFstScanR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan(&self) -> MarkerErrOtherScanR {
        MarkerErrOtherScanR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet(&self) -> UndetR {
        UndetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout(&self) -> DecodeTimeoutR {
        DecodeTimeoutR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntRawSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err(&mut self) -> RleParallelErrW<'_, IntRawSpec> {
        RleParallelErrW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err(&mut self) -> CidErrW<'_, IntRawSpec> {
        CidErrW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err(&mut self) -> CDhtDcIdErrW<'_, IntRawSpec> {
        CDhtDcIdErrW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err(&mut self) -> CDhtAcIdErrW<'_, IntRawSpec> {
        CDhtAcIdErrW::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err(&mut self) -> CDqtIdErrW<'_, IntRawSpec> {
        CDqtIdErrW::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err(&mut self) -> RstUxpErrW<'_, IntRawSpec> {
        RstUxpErrW::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err(&mut self) -> RstCheckNoneErrW<'_, IntRawSpec> {
        RstCheckNoneErrW::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err(&mut self) -> RstCheckPosErrW<'_, IntRawSpec> {
        RstCheckPosErrW::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OutEofW<'_, IntRawSpec> {
        OutEofW::new(self, 9)
    }
    #[doc = "Bit 10 - The raw interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err(&mut self) -> SrColorModeErrW<'_, IntRawSpec> {
        SrColorModeErrW::new(self, 10)
    }
    #[doc = "Bit 11 - The raw interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done(&mut self) -> DctDoneW<'_, IntRawSpec> {
        DctDoneW::new(self, 11)
    }
    #[doc = "Bit 12 - The raw interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof(&mut self) -> BsLastBlockEofW<'_, IntRawSpec> {
        BsLastBlockEofW::new(self, 12)
    }
    #[doc = "Bit 13 - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err(&mut self) -> ScanCheckNoneErrW<'_, IntRawSpec> {
        ScanCheckNoneErrW::new(self, 13)
    }
    #[doc = "Bit 14 - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err(&mut self) -> ScanCheckPosErrW<'_, IntRawSpec> {
        ScanCheckPosErrW::new(self, 14)
    }
    #[doc = "Bit 15 - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det(&mut self) -> UxpDetW<'_, IntRawSpec> {
        UxpDetW::new(self, 15)
    }
    #[doc = "Bit 16 - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err(&mut self) -> EnFrameEofErrW<'_, IntRawSpec> {
        EnFrameEofErrW::new(self, 16)
    }
    #[doc = "Bit 17 - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack(&mut self) -> EnFrameEofLackW<'_, IntRawSpec> {
        EnFrameEofLackW::new(self, 17)
    }
    #[doc = "Bit 18 - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err(&mut self) -> DeFrameEofErrW<'_, IntRawSpec> {
        DeFrameEofErrW::new(self, 18)
    }
    #[doc = "Bit 19 - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack(&mut self) -> DeFrameEofLackW<'_, IntRawSpec> {
        DeFrameEofLackW::new(self, 19)
    }
    #[doc = "Bit 20 - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err(&mut self) -> SosUnmatchErrW<'_, IntRawSpec> {
        SosUnmatchErrW::new(self, 20)
    }
    #[doc = "Bit 21 - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan(&mut self) -> MarkerErrFstScanW<'_, IntRawSpec> {
        MarkerErrFstScanW::new(self, 21)
    }
    #[doc = "Bit 22 - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan(&mut self) -> MarkerErrOtherScanW<'_, IntRawSpec> {
        MarkerErrOtherScanW::new(self, 22)
    }
    #[doc = "Bit 23 - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet(&mut self) -> UndetW<'_, IntRawSpec> {
        UndetW::new(self, 23)
    }
    #[doc = "Bit 24 - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout(&mut self) -> DecodeTimeoutW<'_, IntRawSpec> {
        DecodeTimeoutW::new(self, 24)
    }
}
#[doc = "Interrupt raw registers\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
