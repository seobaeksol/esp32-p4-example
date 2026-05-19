#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `RX_DONE` writer - Set this bit to clear the i2s_rx_done_int interrupt"]
pub type RxDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_DONE` writer - Set this bit to clear the i2s_tx_done_int interrupt"]
pub type TxDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_HUNG` writer - Set this bit to clear the i2s_rx_hung_int interrupt"]
pub type RxHungW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_HUNG` writer - Set this bit to clear the i2s_tx_hung_int interrupt"]
pub type TxHungW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RxDoneW<'_, IntClrSpec> {
        RxDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TxDoneW<'_, IntClrSpec> {
        TxDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RxHungW<'_, IntClrSpec> {
        RxHungW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TxHungW<'_, IntClrSpec> {
        TxHungW::new(self, 3)
    }
}
#[doc = "I2S interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
