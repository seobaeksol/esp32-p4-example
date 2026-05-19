#[doc = "Register `TX_CRC_DATA_EN_ADDR_CH%s` reader"]
pub type R = crate::R<TxCrcDataEnAddrChSpec>;
#[doc = "Register `TX_CRC_DATA_EN_ADDR_CH%s` writer"]
pub type W = crate::W<TxCrcDataEnAddrChSpec>;
#[doc = "Field `TX_CRC_DATA_EN_ADDR_CH` reader - reserved"]
pub type TxCrcDataEnAddrChR = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_DATA_EN_ADDR_CH` writer - reserved"]
pub type TxCrcDataEnAddrChW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch(&self) -> TxCrcDataEnAddrChR {
        TxCrcDataEnAddrChR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch(&mut self) -> TxCrcDataEnAddrChW<'_, TxCrcDataEnAddrChSpec> {
        TxCrcDataEnAddrChW::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_addr_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_addr_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCrcDataEnAddrChSpec;
impl crate::RegisterSpec for TxCrcDataEnAddrChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_data_en_addr_ch::R`](R) reader structure"]
impl crate::Readable for TxCrcDataEnAddrChSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_data_en_addr_ch::W`](W) writer structure"]
impl crate::Writable for TxCrcDataEnAddrChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_DATA_EN_ADDR_CH%s to value 0"]
impl crate::Resettable for TxCrcDataEnAddrChSpec {}
