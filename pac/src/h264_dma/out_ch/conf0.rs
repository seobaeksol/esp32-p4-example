#[doc = "Register `CONF0` reader"]
pub type R = crate::R<Conf0Spec>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<Conf0Spec>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OutAutoWrbackR = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
pub type OutAutoWrbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OutEofModeR = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
pub type OutEofModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OutdscrBurstEnR = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
pub type OutdscrBurstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ECC_AES_EN` reader - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OutEccAesEnR = crate::BitReader;
#[doc = "Field `OUT_ECC_AES_EN` writer - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
pub type OutEccAesEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OutCheckOwnerR = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OutCheckOwnerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_MEM_BURST_LENGTH` reader - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OutMemBurstLengthR = crate::FieldReader;
#[doc = "Field `OUT_MEM_BURST_LENGTH` writer - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
pub type OutMemBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OUT_PAGE_BOUND_EN` reader - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OutPageBoundEnR = crate::BitReader;
#[doc = "Field `OUT_PAGE_BOUND_EN` writer - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
pub type OutPageBoundEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_REORDER_EN` reader - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type OutReorderEnR = crate::BitReader;
#[doc = "Field `OUT_REORDER_EN` writer - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
pub type OutReorderEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST` reader - Write 1 then write 0 to this bit to reset TX channel"]
pub type OutRstR = crate::BitReader;
#[doc = "Field `OUT_RST` writer - Write 1 then write 0 to this bit to reset TX channel"]
pub type OutRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_CMD_DISABLE` reader - Write 1 before reset and write 0 after reset"]
pub type OutCmdDisableR = crate::BitReader;
#[doc = "Field `OUT_CMD_DISABLE` writer - Write 1 before reset and write 0 after reset"]
pub type OutCmdDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS` reader - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OutArbWeightOptDisR = crate::BitReader;
#[doc = "Field `OUT_ARB_WEIGHT_OPT_DIS` writer - Set this bit to 1 to disable arbiter optimum weight function."]
pub type OutArbWeightOptDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OutAutoWrbackR {
        OutAutoWrbackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OutEofModeR {
        OutEofModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OutdscrBurstEnR {
        OutdscrBurstEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn out_ecc_aes_en(&self) -> OutEccAesEnR {
        OutEccAesEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OutCheckOwnerR {
        OutCheckOwnerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn out_mem_burst_length(&self) -> OutMemBurstLengthR {
        OutMemBurstLengthR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn out_page_bound_en(&self) -> OutPageBoundEnR {
        OutPageBoundEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn out_reorder_en(&self) -> OutReorderEnR {
        OutReorderEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset TX channel"]
    #[inline(always)]
    pub fn out_rst(&self) -> OutRstR {
        OutRstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn out_cmd_disable(&self) -> OutCmdDisableR {
        OutCmdDisableR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn out_arb_weight_opt_dis(&self) -> OutArbWeightOptDisR {
        OutArbWeightOptDisR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable automatic outlink-writeback when all the data pointed by outlink descriptor has been received."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OutAutoWrbackW<'_, Conf0Spec> {
        OutAutoWrbackW::new(self, 0)
    }
    #[doc = "Bit 1 - EOF flag generation mode when receiving data. 1: EOF flag for Tx channel 0 is generated when data need to read has been popped from FIFO in DMA"]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OutEofModeW<'_, Conf0Spec> {
        OutEofModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable INCR burst transfer for Tx channel 0 reading link descriptor when accessing internal SRAM."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OutdscrBurstEnW<'_, Conf0Spec> {
        OutdscrBurstEnW::new(self, 2)
    }
    #[doc = "Bit 3 - When access address space is ecc/aes area, this bit should be set to 1. In this case, the start address of square should be 16-bit aligned. The width of square multiply byte number of one pixel should be 16-bit aligned."]
    #[inline(always)]
    pub fn out_ecc_aes_en(&mut self) -> OutEccAesEnW<'_, Conf0Spec> {
        OutEccAesEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&mut self) -> OutCheckOwnerW<'_, Conf0Spec> {
        OutCheckOwnerW::new(self, 4)
    }
    #[doc = "Bits 6:8 - Block size of Tx channel 0. 0: single 1: 16 bytes 2: 32 bytes 3: 64 bytes 4: 128 bytes"]
    #[inline(always)]
    pub fn out_mem_burst_length(&mut self) -> OutMemBurstLengthW<'_, Conf0Spec> {
        OutMemBurstLengthW::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to 1 to make sure AXI read data don't cross the address boundary which define by mem_burst_length"]
    #[inline(always)]
    pub fn out_page_bound_en(&mut self) -> OutPageBoundEnW<'_, Conf0Spec> {
        OutPageBoundEnW::new(self, 12)
    }
    #[doc = "Bit 16 - Enable TX channel 0 macro block reorder when set to 1, only channel0 have this selection"]
    #[inline(always)]
    pub fn out_reorder_en(&mut self) -> OutReorderEnW<'_, Conf0Spec> {
        OutReorderEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Write 1 then write 0 to this bit to reset TX channel"]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OutRstW<'_, Conf0Spec> {
        OutRstW::new(self, 24)
    }
    #[doc = "Bit 25 - Write 1 before reset and write 0 after reset"]
    #[inline(always)]
    pub fn out_cmd_disable(&mut self) -> OutCmdDisableW<'_, Conf0Spec> {
        OutCmdDisableW::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to 1 to disable arbiter optimum weight function."]
    #[inline(always)]
    pub fn out_arb_weight_opt_dis(&mut self) -> OutArbWeightOptDisW<'_, Conf0Spec> {
        OutArbWeightOptDisW::new(self, 26)
    }
}
#[doc = "TX CHx config0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf0Spec;
impl crate::RegisterSpec for Conf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for Conf0Spec {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for Conf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0 to value 0x02"]
impl crate::Resettable for Conf0Spec {
    const RESET_VALUE: u32 = 0x02;
}
