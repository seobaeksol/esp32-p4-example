#[doc = "Register `TX_CRC_EN_ADDR` reader"]
pub type R = crate::R<TxCrcEnAddrSpec>;
#[doc = "Register `TX_CRC_EN_ADDR` writer"]
pub type W = crate::W<TxCrcEnAddrSpec>;
#[doc = "Field `TX_CRC_EN_ADDR` reader - reserved"]
pub type TxCrcEnAddrR = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_EN_ADDR` writer - reserved"]
pub type TxCrcEnAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_en_addr(&self) -> TxCrcEnAddrR {
        TxCrcEnAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_en_addr(&mut self) -> TxCrcEnAddrW<'_, TxCrcEnAddrSpec> {
        TxCrcEnAddrW::new(self, 0)
    }
}
#[doc = "This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCrcEnAddrSpec;
impl crate::RegisterSpec for TxCrcEnAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_en_addr::R`](R) reader structure"]
impl crate::Readable for TxCrcEnAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_en_addr::W`](W) writer structure"]
impl crate::Writable for TxCrcEnAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_EN_ADDR to value 0"]
impl crate::Resettable for TxCrcEnAddrSpec {}
