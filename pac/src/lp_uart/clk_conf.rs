#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<ClkConfSpec>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<ClkConfSpec>;
#[doc = "Field `TX_SCLK_EN` reader - Set this bit to enable UART Tx clock."]
pub type TxSclkEnR = crate::BitReader;
#[doc = "Field `TX_SCLK_EN` writer - Set this bit to enable UART Tx clock."]
pub type TxSclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SCLK_EN` reader - Set this bit to enable UART Rx clock."]
pub type RxSclkEnR = crate::BitReader;
#[doc = "Field `RX_SCLK_EN` writer - Set this bit to enable UART Rx clock."]
pub type RxSclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Tx."]
pub type TxRstCoreR = crate::BitReader;
#[doc = "Field `TX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Tx."]
pub type TxRstCoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Rx."]
pub type RxRstCoreR = crate::BitReader;
#[doc = "Field `RX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Rx."]
pub type RxRstCoreW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> TxSclkEnR {
        TxSclkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> RxSclkEnR {
        RxSclkEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset UART Tx."]
    #[inline(always)]
    pub fn tx_rst_core(&self) -> TxRstCoreR {
        TxRstCoreR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit to reset UART Rx."]
    #[inline(always)]
    pub fn rx_rst_core(&self) -> RxRstCoreR {
        RxRstCoreR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    pub fn tx_sclk_en(&mut self) -> TxSclkEnW<'_, ClkConfSpec> {
        TxSclkEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    pub fn rx_sclk_en(&mut self) -> RxSclkEnW<'_, ClkConfSpec> {
        RxSclkEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset UART Tx."]
    #[inline(always)]
    pub fn tx_rst_core(&mut self) -> TxRstCoreW<'_, ClkConfSpec> {
        TxRstCoreW::new(self, 26)
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit to reset UART Rx."]
    #[inline(always)]
    pub fn rx_rst_core(&mut self) -> RxRstCoreW<'_, ClkConfSpec> {
        RxRstCoreW::new(self, 27)
    }
}
#[doc = "UART core clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkConfSpec;
impl crate::RegisterSpec for ClkConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for ClkConfSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for ClkConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0300_0000"]
impl crate::Resettable for ClkConfSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
