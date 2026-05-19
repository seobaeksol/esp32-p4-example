#[doc = "Register `USER1` reader"]
pub type R = crate::R<User1Spec>;
#[doc = "Register `USER1` writer"]
pub type W = crate::W<User1Spec>;
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type UsrDummyCyclelenR = crate::FieldReader;
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type UsrDummyCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type MstWfullErrEndEnR = crate::BitReader;
#[doc = "Field `MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type MstWfullErrEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type CsSetupTimeR = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type CsSetupTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type CsHoldTimeR = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type CsHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type UsrAddrBitlenR = crate::FieldReader;
#[doc = "Field `USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type UsrAddrBitlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> UsrDummyCyclelenR {
        UsrDummyCyclelenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MstWfullErrEndEnR {
        MstWfullErrEndEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CsSetupTimeR {
        CsSetupTimeR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CsHoldTimeR {
        CsHoldTimeR::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> UsrAddrBitlenR {
        UsrAddrBitlenR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> UsrDummyCyclelenW<'_, User1Spec> {
        UsrDummyCyclelenW::new(self, 0)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&mut self) -> MstWfullErrEndEnW<'_, User1Spec> {
        MstWfullErrEndEnW::new(self, 16)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CsSetupTimeW<'_, User1Spec> {
        CsSetupTimeW::new(self, 17)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CsHoldTimeW<'_, User1Spec> {
        CsHoldTimeW::new(self, 22)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> UsrAddrBitlenW<'_, User1Spec> {
        UsrAddrBitlenW::new(self, 27)
    }
}
#[doc = "SPI USER control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct User1Spec;
impl crate::RegisterSpec for User1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user1::R`](R) reader structure"]
impl crate::Readable for User1Spec {}
#[doc = "`write(|w| ..)` method takes [`user1::W`](W) writer structure"]
impl crate::Writable for User1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER1 to value 0xb841_0007"]
impl crate::Resettable for User1Spec {
    const RESET_VALUE: u32 = 0xb841_0007;
}
