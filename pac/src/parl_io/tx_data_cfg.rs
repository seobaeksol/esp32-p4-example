#[doc = "Register `TX_DATA_CFG` reader"]
pub type R = crate::R<TxDataCfgSpec>;
#[doc = "Register `TX_DATA_CFG` writer"]
pub type W = crate::W<TxDataCfgSpec>;
#[doc = "Field `TX_BITLEN` reader - Configures expected byte number of sent data."]
pub type TxBitlenR = crate::FieldReader<u32>;
#[doc = "Field `TX_BITLEN` writer - Configures expected byte number of sent data."]
pub type TxBitlenW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `TX_DATA_ORDER_INV` reader - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TxDataOrderInvR = crate::BitReader;
#[doc = "Field `TX_DATA_ORDER_INV` writer - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
pub type TxDataOrderInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BUS_WID_SEL` reader - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
pub type TxBusWidSelR = crate::FieldReader;
#[doc = "Field `TX_BUS_WID_SEL` writer - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
pub type TxBusWidSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    pub fn tx_bitlen(&self) -> TxBitlenR {
        TxBitlenR::new((self.bits >> 9) & 0x0007_ffff)
    }
    #[doc = "Bit 28 - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    pub fn tx_data_order_inv(&self) -> TxDataOrderInvR {
        TxDataOrderInvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&self) -> TxBusWidSelR {
        TxBusWidSelR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 9:27 - Configures expected byte number of sent data."]
    #[inline(always)]
    pub fn tx_bitlen(&mut self) -> TxBitlenW<'_, TxDataCfgSpec> {
        TxBitlenW::new(self, 9)
    }
    #[doc = "Bit 28 - Write 1 to invert bit order of one byte sent from TX_FIFO to IO data."]
    #[inline(always)]
    pub fn tx_data_order_inv(&mut self) -> TxDataOrderInvW<'_, TxDataCfgSpec> {
        TxDataOrderInvW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Configures the txd bus width. 0: bus width is 1. 1: bus width is 2. 2: bus width is 4. 3: bus width is 8."]
    #[inline(always)]
    pub fn tx_bus_wid_sel(&mut self) -> TxBusWidSelW<'_, TxDataCfgSpec> {
        TxBusWidSelW::new(self, 29)
    }
}
#[doc = "Parallel TX data configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataCfgSpec;
impl crate::RegisterSpec for TxDataCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_cfg::R`](R) reader structure"]
impl crate::Readable for TxDataCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_data_cfg::W`](W) writer structure"]
impl crate::Writable for TxDataCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_DATA_CFG to value 0x6000_0000"]
impl crate::Resettable for TxDataCfgSpec {
    const RESET_VALUE: u32 = 0x6000_0000;
}
