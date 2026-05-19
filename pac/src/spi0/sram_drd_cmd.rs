#[doc = "Register `SRAM_DRD_CMD` reader"]
pub type R = crate::R<SramDrdCmdSpec>;
#[doc = "Register `SRAM_DRD_CMD` writer"]
pub type W = crate::W<SramDrdCmdSpec>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` reader - For SPI0,When cache mode is enable it is the read command value of command phase for sram."]
pub type CacheSramUsrRdCmdValueR = crate::FieldReader<u16>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_VALUE` writer - For SPI0,When cache mode is enable it is the read command value of command phase for sram."]
pub type CacheSramUsrRdCmdValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` reader - For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1)."]
pub type CacheSramUsrRdCmdBitlenR = crate::FieldReader;
#[doc = "Field `CACHE_SRAM_USR_RD_CMD_BITLEN` writer - For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1)."]
pub type CacheSramUsrRdCmdBitlenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - For SPI0,When cache mode is enable it is the read command value of command phase for sram."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_value(&self) -> CacheSramUsrRdCmdValueR {
        CacheSramUsrRdCmdValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_bitlen(&self) -> CacheSramUsrRdCmdBitlenR {
        CacheSramUsrRdCmdBitlenR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - For SPI0,When cache mode is enable it is the read command value of command phase for sram."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_value(&mut self) -> CacheSramUsrRdCmdValueW<'_, SramDrdCmdSpec> {
        CacheSramUsrRdCmdValueW::new(self, 0)
    }
    #[doc = "Bits 28:31 - For SPI0,When cache mode is enable it is the length in bits of command phase for sram. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_rd_cmd_bitlen(&mut self) -> CacheSramUsrRdCmdBitlenW<'_, SramDrdCmdSpec> {
        CacheSramUsrRdCmdBitlenW::new(self, 28)
    }
}
#[doc = "SPI0 external RAM DDR read command control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_drd_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_drd_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramDrdCmdSpec;
impl crate::RegisterSpec for SramDrdCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_drd_cmd::R`](R) reader structure"]
impl crate::Readable for SramDrdCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_drd_cmd::W`](W) writer structure"]
impl crate::Writable for SramDrdCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_DRD_CMD to value 0"]
impl crate::Resettable for SramDrdCmdSpec {}
