#[doc = "Register `STATUS2` reader"]
pub type R = crate::R<Status2Spec>;
#[doc = "Field `SOURCE_PIXEL` reader - source pixels fetched from dma"]
pub type SourcePixelR = crate::FieldReader<u32>;
#[doc = "Field `LAST_BLOCK` reader - indicate the encoding process for the last mcu of the picture"]
pub type LastBlockR = crate::BitReader;
#[doc = "Field `LAST_MCU` reader - indicate the encoding process for the last block of the picture"]
pub type LastMcuR = crate::BitReader;
#[doc = "Field `LAST_DC` reader - indicate the encoding process is at the header of the last block of the picture"]
pub type LastDcR = crate::BitReader;
#[doc = "Field `PACKFIFO_READY` reader - the jpeg pack_fifo ready signal, high active"]
pub type PackfifoReadyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - source pixels fetched from dma"]
    #[inline(always)]
    pub fn source_pixel(&self) -> SourcePixelR {
        SourcePixelR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - indicate the encoding process for the last mcu of the picture"]
    #[inline(always)]
    pub fn last_block(&self) -> LastBlockR {
        LastBlockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - indicate the encoding process for the last block of the picture"]
    #[inline(always)]
    pub fn last_mcu(&self) -> LastMcuR {
        LastMcuR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - indicate the encoding process is at the header of the last block of the picture"]
    #[inline(always)]
    pub fn last_dc(&self) -> LastDcR {
        LastDcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the jpeg pack_fifo ready signal, high active"]
    #[inline(always)]
    pub fn packfifo_ready(&self) -> PackfifoReadyR {
        PackfifoReadyR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status2Spec;
impl crate::RegisterSpec for Status2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status2::R`](R) reader structure"]
impl crate::Readable for Status2Spec {}
#[doc = "`reset()` method sets STATUS2 to value 0x0800_0000"]
impl crate::Resettable for Status2Spec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
