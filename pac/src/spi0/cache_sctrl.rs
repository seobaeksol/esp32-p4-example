#[doc = "Register `CACHE_SCTRL` reader"]
pub type R = crate::R<CacheSctrlSpec>;
#[doc = "Register `CACHE_SCTRL` writer"]
pub type W = crate::W<CacheSctrlSpec>;
#[doc = "Field `CACHE_USR_SADDR_4BYTE` reader - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
pub type CacheUsrSaddr4byteR = crate::BitReader;
#[doc = "Field `CACHE_USR_SADDR_4BYTE` writer - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
pub type CacheUsrSaddr4byteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_SRAM_DIO` reader - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
pub type UsrSramDioR = crate::BitReader;
#[doc = "Field `USR_SRAM_DIO` writer - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
pub type UsrSramDioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_SRAM_QIO` reader - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
pub type UsrSramQioR = crate::BitReader;
#[doc = "Field `USR_SRAM_QIO` writer - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
pub type UsrSramQioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_WR_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
pub type UsrWrSramDummyR = crate::BitReader;
#[doc = "Field `USR_WR_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
pub type UsrWrSramDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_RD_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
pub type UsrRdSramDummyR = crate::BitReader;
#[doc = "Field `USR_RD_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
pub type UsrRdSramDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SRAM_USR_RCMD` reader - For SPI0, In the external RAM mode cache read external RAM for user define command."]
pub type CacheSramUsrRcmdR = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_RCMD` writer - For SPI0, In the external RAM mode cache read external RAM for user define command."]
pub type CacheSramUsrRcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
pub type SramRdummyCyclelenR = crate::FieldReader;
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
pub type SramRdummyCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SRAM_ADDR_BITLEN` reader - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SramAddrBitlenR = crate::FieldReader;
#[doc = "Field `SRAM_ADDR_BITLEN` writer - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SramAddrBitlenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CACHE_SRAM_USR_WCMD` reader - For SPI0, In the external RAM mode cache write sram for user define command"]
pub type CacheSramUsrWcmdR = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_WCMD` writer - For SPI0, In the external RAM mode cache write sram for user define command"]
pub type CacheSramUsrWcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_OCT` reader - reserved"]
pub type SramOctR = crate::BitReader;
#[doc = "Field `SRAM_OCT` writer - reserved"]
pub type SramOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
pub type SramWdummyCyclelenR = crate::FieldReader;
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
pub type SramWdummyCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_usr_saddr_4byte(&self) -> CacheUsrSaddr4byteR {
        CacheUsrSaddr4byteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> UsrSramDioR {
        UsrSramDioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> UsrSramQioR {
        UsrSramQioR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> UsrWrSramDummyR {
        UsrWrSramDummyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> UsrRdSramDummyR {
        UsrRdSramDummyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CacheSramUsrRcmdR {
        CacheSramUsrRcmdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&self) -> SramRdummyCyclelenR {
        SramRdummyCyclelenR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SramAddrBitlenR {
        SramAddrBitlenR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CacheSramUsrWcmdR {
        CacheSramUsrWcmdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn sram_oct(&self) -> SramOctR {
        SramOctR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&self) -> SramWdummyCyclelenR {
        SramWdummyCyclelenR::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn cache_usr_saddr_4byte(&mut self) -> CacheUsrSaddr4byteW<'_, CacheSctrlSpec> {
        CacheUsrSaddr4byteW::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&mut self) -> UsrSramDioW<'_, CacheSctrlSpec> {
        UsrSramDioW::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&mut self) -> UsrSramQioW<'_, CacheSctrlSpec> {
        UsrSramQioW::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&mut self) -> UsrWrSramDummyW<'_, CacheSctrlSpec> {
        UsrWrSramDummyW::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&mut self) -> UsrRdSramDummyW<'_, CacheSctrlSpec> {
        UsrRdSramDummyW::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&mut self) -> CacheSramUsrRcmdW<'_, CacheSctrlSpec> {
        CacheSramUsrRcmdW::new(self, 5)
    }
    #[doc = "Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&mut self) -> SramRdummyCyclelenW<'_, CacheSctrlSpec> {
        SramRdummyCyclelenW::new(self, 6)
    }
    #[doc = "Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&mut self) -> SramAddrBitlenW<'_, CacheSctrlSpec> {
        SramAddrBitlenW::new(self, 14)
    }
    #[doc = "Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&mut self) -> CacheSramUsrWcmdW<'_, CacheSctrlSpec> {
        CacheSramUsrWcmdW::new(self, 20)
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn sram_oct(&mut self) -> SramOctW<'_, CacheSctrlSpec> {
        SramOctW::new(self, 21)
    }
    #[doc = "Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&mut self) -> SramWdummyCyclelenW<'_, CacheSctrlSpec> {
        SramWdummyCyclelenW::new(self, 22)
    }
}
#[doc = "SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacheSctrlSpec;
impl crate::RegisterSpec for CacheSctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sctrl::R`](R) reader structure"]
impl crate::Readable for CacheSctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cache_sctrl::W`](W) writer structure"]
impl crate::Writable for CacheSctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_SCTRL to value 0x0055_c070"]
impl crate::Resettable for CacheSctrlSpec {
    const RESET_VALUE: u32 = 0x0055_c070;
}
