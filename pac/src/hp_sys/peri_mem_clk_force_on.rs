#[doc = "Register `PERI_MEM_CLK_FORCE_ON` reader"]
pub type R = crate::R<PeriMemClkForceOnSpec>;
#[doc = "Register `PERI_MEM_CLK_FORCE_ON` writer"]
pub type W = crate::W<PeriMemClkForceOnSpec>;
#[doc = "Field `RMT_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in rmt"]
pub type RmtMemClkForceOnR = crate::BitReader;
#[doc = "Field `RMT_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in rmt"]
pub type RmtMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` reader - Set this bit to force on tx mem clk in bitscrambler"]
pub type BitscramblerTxMemClkForceOnR = crate::BitReader;
#[doc = "Field `BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` writer - Set this bit to force on tx mem clk in bitscrambler"]
pub type BitscramblerTxMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` reader - Set this bit to force on rx mem clk in bitscrambler"]
pub type BitscramblerRxMemClkForceOnR = crate::BitReader;
#[doc = "Field `BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` writer - Set this bit to force on rx mem clk in bitscrambler"]
pub type BitscramblerRxMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in gdma"]
pub type GdmaMemClkForceOnR = crate::BitReader;
#[doc = "Field `GDMA_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in gdma"]
pub type GdmaMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&self) -> RmtMemClkForceOnR {
        RmtMemClkForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_tx_mem_clk_force_on(&self) -> BitscramblerTxMemClkForceOnR {
        BitscramblerTxMemClkForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_rx_mem_clk_force_on(&self) -> BitscramblerRxMemClkForceOnR {
        BitscramblerRxMemClkForceOnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    pub fn gdma_mem_clk_force_on(&self) -> GdmaMemClkForceOnR {
        GdmaMemClkForceOnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&mut self) -> RmtMemClkForceOnW<'_, PeriMemClkForceOnSpec> {
        RmtMemClkForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_tx_mem_clk_force_on(
        &mut self,
    ) -> BitscramblerTxMemClkForceOnW<'_, PeriMemClkForceOnSpec> {
        BitscramblerTxMemClkForceOnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_rx_mem_clk_force_on(
        &mut self,
    ) -> BitscramblerRxMemClkForceOnW<'_, PeriMemClkForceOnSpec> {
        BitscramblerRxMemClkForceOnW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    pub fn gdma_mem_clk_force_on(&mut self) -> GdmaMemClkForceOnW<'_, PeriMemClkForceOnSpec> {
        GdmaMemClkForceOnW::new(self, 3)
    }
}
#[doc = "hp peri mem clk force on regpster\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_mem_clk_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_mem_clk_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriMemClkForceOnSpec;
impl crate::RegisterSpec for PeriMemClkForceOnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_mem_clk_force_on::R`](R) reader structure"]
impl crate::Readable for PeriMemClkForceOnSpec {}
#[doc = "`write(|w| ..)` method takes [`peri_mem_clk_force_on::W`](W) writer structure"]
impl crate::Writable for PeriMemClkForceOnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_MEM_CLK_FORCE_ON to value 0"]
impl crate::Resettable for PeriMemClkForceOnSpec {}
