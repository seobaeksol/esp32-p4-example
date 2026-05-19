#[doc = "Register `TX_CRC_EN_WR_DATA` reader"]
pub type R = crate::R<TxCrcEnWrDataSpec>;
#[doc = "Register `TX_CRC_EN_WR_DATA` writer"]
pub type W = crate::W<TxCrcEnWrDataSpec>;
#[doc = "Field `TX_CRC_EN_WR_DATA` reader - This register is used to enable tx ch0 crc 32bit on/off"]
pub type TxCrcEnWrDataR = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_EN_WR_DATA` writer - This register is used to enable tx ch0 crc 32bit on/off"]
pub type TxCrcEnWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to enable tx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data(&self) -> TxCrcEnWrDataR {
        TxCrcEnWrDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to enable tx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data(&mut self) -> TxCrcEnWrDataW<'_, TxCrcEnWrDataSpec> {
        TxCrcEnWrDataW::new(self, 0)
    }
}
#[doc = "This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_wr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_wr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCrcEnWrDataSpec;
impl crate::RegisterSpec for TxCrcEnWrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_en_wr_data::R`](R) reader structure"]
impl crate::Readable for TxCrcEnWrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_en_wr_data::W`](W) writer structure"]
impl crate::Writable for TxCrcEnWrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_EN_WR_DATA to value 0"]
impl crate::Resettable for TxCrcEnWrDataSpec {}
