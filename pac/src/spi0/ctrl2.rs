#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CS_SETUP_TIME` reader - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type CsSetupTimeR = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
pub type CsSetupTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_HOLD_TIME` reader - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type CsHoldTimeR = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
pub type CsHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ECC_CS_HOLD_TIME` reader - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
pub type EccCsHoldTimeR = crate::FieldReader;
#[doc = "Field `ECC_CS_HOLD_TIME` writer - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
pub type EccCsHoldTimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` reader - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type EccSkipPageCornerR = crate::BitReader;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` writer - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
pub type EccSkipPageCornerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_16TO18_BYTE_EN` reader - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type Ecc16to18ByteEnR = crate::BitReader;
#[doc = "Field `ECC_16TO18_BYTE_EN` writer - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
pub type Ecc16to18ByteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLIT_TRANS_EN` reader - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SplitTransEnR = crate::BitReader;
#[doc = "Field `SPLIT_TRANS_EN` writer - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
pub type SplitTransEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CsHoldDelayR = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CsHoldDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYNC_RESET` writer - The spi0_mst_st and spi0_slv_st will be reset."]
pub type SyncResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CsSetupTimeR {
        CsSetupTimeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CsHoldTimeR {
        CsHoldTimeR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&self) -> EccCsHoldTimeR {
        EccCsHoldTimeR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&self) -> EccSkipPageCornerR {
        EccSkipPageCornerR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&self) -> Ecc16to18ByteEnR {
        Ecc16to18ByteEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn split_trans_en(&self) -> SplitTransEnR {
        SplitTransEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CsHoldDelayR {
        CsHoldDelayR::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by SPI Bus clock, this bits are combined with SPI_MEM_CS_SETUP bit."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CsSetupTimeW<'_, Ctrl2Spec> {
        CsSetupTimeW::new(self, 0)
    }
    #[doc = "Bits 5:9 - SPI CS signal is delayed to inactive by SPI bus clock, this bits are combined with SPI_MEM_CS_HOLD bit."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CsHoldTimeW<'_, Ctrl2Spec> {
        CsHoldTimeW::new(self, 5)
    }
    #[doc = "Bits 10:12 - SPI_MEM_CS_HOLD_TIME + SPI_MEM_ECC_CS_HOLD_TIME is the SPI0 CS hold cycle in ECC mode when accessed flash."]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&mut self) -> EccCsHoldTimeW<'_, Ctrl2Spec> {
        EccCsHoldTimeW::new(self, 10)
    }
    #[doc = "Bit 13 - 1: SPI0 and SPI1 skip page corner when accesses flash. 0: Not skip page corner when accesses flash."]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&mut self) -> EccSkipPageCornerW<'_, Ctrl2Spec> {
        EccSkipPageCornerW::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to enable SPI0 and SPI1 ECC 16 bytes data with 2 ECC bytes mode when accesses flash."]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&mut self) -> Ecc16to18ByteEnW<'_, Ctrl2Spec> {
        Ecc16to18ByteEnW::new(self, 14)
    }
    #[doc = "Bit 24 - Set this bit to enable SPI0 split one AXI read flash transfer into two SPI transfers when one transfer will cross flash or EXT_RAM page corner, valid no matter whether there is an ECC region or not."]
    #[inline(always)]
    pub fn split_trans_en(&mut self) -> SplitTransEnW<'_, Ctrl2Spec> {
        SplitTransEnW::new(self, 24)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CsHoldDelayW<'_, Ctrl2Spec> {
        CsHoldDelayW::new(self, 25)
    }
    #[doc = "Bit 31 - The spi0_mst_st and spi0_slv_st will be reset."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SyncResetW<'_, Ctrl2Spec> {
        SyncResetW::new(self, 31)
    }
}
#[doc = "SPI0 control2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0x0100_2c21"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x0100_2c21;
}
